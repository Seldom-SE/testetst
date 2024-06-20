use bevy::{
    prelude::*,
    render::{
        graph::CameraDriverLabel,
        render_asset::{RenderAssetUsages, RenderAssets},
        render_graph::{self, NodeRunError, RenderGraph, RenderGraphContext, RenderLabel},
        render_resource::{
            Buffer, BufferDescriptor, BufferUsages, CommandEncoderDescriptor, Extent3d,
            ImageCopyBuffer, ImageDataLayout, Maintain, MapMode, TextureDimension, TextureFormat,
            TextureUsages,
        },
        renderer::{RenderContext, RenderDevice, RenderQueue},
        texture::{BevyDefault, TextureFormatPixelInfo},
        view::RenderLayers,
        Extract, Render, RenderApp, RenderSet,
    },
};
use crossbeam_channel::{Receiver, Sender};
use seldom_pixel::{
    prelude::*,
    screen::Screen,
    sprite::{Dither, DitherAlgorithm, ImageToSprite, ThresholdMap},
};

#[derive(Resource, Deref)]
struct RenderReceiver(Receiver<Vec<u8>>);

#[derive(Resource, Deref)]
struct RenderSender(Sender<Vec<u8>>);

fn main() {
    let mut app = App::new();

    let (sender, receiver) = crossbeam_channel::unbounded();

    let render_app = app
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            PxPlugin::<Layer>::new(ScreenSize::MinPixels(10_000), "palette.palette.png"),
        ))
        .insert_resource(RenderReceiver(receiver))
        .insert_resource(Msaa::Off)
        .add_systems(Startup, init)
        .add_systems(Update, (resize, rotato, dither_settings))
        .add_systems(PostUpdate, update)
        .sub_app_mut(RenderApp);

    let mut graph = render_app.world.resource_mut::<RenderGraph>();
    graph.add_node(CopyImageLabel, CopyImage);
    graph.add_node_edge(CameraDriverLabel, CopyImageLabel);

    render_app
        .insert_resource(RenderSender(sender))
        .add_systems(ExtractSchedule, image_copy_extract)
        .add_systems(Render, receive_image_from_buffer.after(RenderSet::Render));

    app.run();
}

fn init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    render_device: Res<RenderDevice>,
) {
    let size = Extent3d {
        width: 100,
        height: 100,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut render_target_image = Image::new_fill(
        size,
        TextureDimension::D2,
        &[0; 4],
        TextureFormat::bevy_default(),
        RenderAssetUsages::default(),
    );

    render_target_image.texture_descriptor.usage |=
        TextureUsages::COPY_SRC | TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING;
    let render_target_image_handle = images.add(render_target_image);

    // This is the texture that will be copied to.
    let cpu_image = Image::new_fill(
        size,
        TextureDimension::D2,
        &[0; 4],
        TextureFormat::bevy_default(),
        RenderAssetUsages::default(),
    );
    let cpu_image_handle = images.add(cpu_image);

    commands.insert_resource(ImageCopier::new(
        render_target_image_handle.clone(),
        size,
        &render_device,
    ));

    commands.insert_resource(ImageToSave(cpu_image_handle.clone()));

    commands.spawn((
        PxSpriteBundle::<Layer> {
            anchor: PxAnchor::BottomLeft,
            ..default()
        },
        ImageToSprite {
            image: cpu_image_handle,
            dither: Some(Dither {
                algorithm: DitherAlgorithm::Ordered,
                threshold: 0.,
                threshold_map: ThresholdMap::X2_2,
            }),
        },
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(4., 4., 4.)),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.8, 0.7, 0.6),
                reflectance: 0.02,
                unlit: false,
                ..default()
            }),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
        },
        Cube,
        RenderLayers::layer(1),
    ));

    commands.spawn((
        PointLightBundle {
            transform: Transform::from_xyz(0., 0., 10.),
            ..default()
        },
        RenderLayers::layer(1),
    ));

    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                order: -1,
                // render to image
                target: render_target_image_handle.into(),
                clear_color: Color::WHITE.into(),
                ..default()
            },
            transform: Transform::from_xyz(0., 0., 15.).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        RenderLayers::layer(1),
    ));

    commands.spawn(Camera2dBundle::default());
}

/// Used by `ImageCopyDriver` for copying from render target to buffer
#[derive(Clone, Resource)]
struct ImageCopier {
    buffer: Buffer,
    src_image: Handle<Image>,
}

impl ImageCopier {
    fn buffer(size: Extent3d, render_device: &RenderDevice) -> Buffer {
        let padded_bytes_per_row =
            RenderDevice::align_copy_bytes_per_row((size.width) as usize) * 4;

        render_device.create_buffer(&BufferDescriptor {
            label: None,
            size: padded_bytes_per_row as u64 * size.height as u64,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }

    pub fn new(
        src_image: Handle<Image>,
        size: Extent3d,
        render_device: &RenderDevice,
    ) -> ImageCopier {
        ImageCopier {
            buffer: Self::buffer(size, render_device),
            src_image,
        }
    }
}

/// Extracting `ImageCopier`s into render world, because `ImageCopyDriver` accesses them
fn image_copy_extract(mut commands: Commands, image_copiers: Extract<Res<ImageCopier>>) {
    commands.insert_resource(image_copiers.clone());
}

/// `RenderGraph` label for `ImageCopyDriver`
#[derive(Debug, PartialEq, Eq, Clone, Hash, RenderLabel)]
struct CopyImageLabel;

/// `RenderGraph` node
#[derive(Default)]
struct CopyImage;

// Copies image content from render target to buffer
impl render_graph::Node for CopyImage {
    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let image_copier = world.get_resource::<ImageCopier>().unwrap();
        let gpu_images = world.get_resource::<RenderAssets<Image>>().unwrap();

        let src_image = gpu_images.get(&image_copier.src_image).unwrap();

        let mut encoder = render_context
            .render_device()
            .create_command_encoder(&CommandEncoderDescriptor::default());

        let block_dimensions = src_image.texture_format.block_dimensions();
        let block_size = src_image.texture_format.block_copy_size(None).unwrap();

        // Calculating correct size of image row because
        // copy_texture_to_buffer can copy image only by rows aligned wgpu::COPY_BYTES_PER_ROW_ALIGNMENT
        // That's why image in buffer can be little bit wider
        // This should be taken into account at copy from buffer stage
        let padded_bytes_per_row = RenderDevice::align_copy_bytes_per_row(
            (src_image.size.x as usize / block_dimensions.0 as usize) * block_size as usize,
        );

        let texture_extent = Extent3d {
            width: src_image.size.x as u32,
            height: src_image.size.y as u32,
            depth_or_array_layers: 1,
        };

        encoder.copy_texture_to_buffer(
            src_image.texture.as_image_copy(),
            ImageCopyBuffer {
                buffer: &image_copier.buffer,
                layout: ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(
                        std::num::NonZeroU32::new(padded_bytes_per_row as u32)
                            .unwrap()
                            .into(),
                    ),
                    rows_per_image: None,
                },
            },
            texture_extent,
        );

        let render_queue = world.get_resource::<RenderQueue>().unwrap();
        render_queue.submit(std::iter::once(encoder.finish()));

        Ok(())
    }
}

/// runs in render world after Render stage to send image from buffer via channel (receiver is in main world)
fn receive_image_from_buffer(
    image_copier: Res<ImageCopier>,
    render_device: Res<RenderDevice>,
    sender: Res<RenderSender>,
) {
    // Finally time to get our data back from the gpu.
    // First we get a buffer slice which represents a chunk of the buffer (which we
    // can't access yet).
    // We want the whole thing so use unbounded range.
    let buffer_slice = image_copier.buffer.slice(..);

    // Now things get complicated. WebGPU, for safety reasons, only allows either the GPU
    // or CPU to access a buffer's contents at a time. We need to "map" the buffer which means
    // flipping ownership of the buffer over to the CPU and making access legal. We do this
    // with `BufferSlice::map_async`.
    //
    // The problem is that map_async is not an async function so we can't await it. What
    // we need to do instead is pass in a closure that will be executed when the slice is
    // either mapped or the mapping has failed.
    //
    // The problem with this is that we don't have a reliable way to wait in the main
    // code for the buffer to be mapped and even worse, calling get_mapped_range or
    // get_mapped_range_mut prematurely will cause a panic, not return an error.
    //
    // Using channels solves this as awaiting the receiving of a message from
    // the passed closure will force the outside code to wait. It also doesn't hurt
    // if the closure finishes before the outside code catches up as the message is
    // buffered and receiving will just pick that up.
    //
    // It may also be worth noting that although on native, the usage of asynchronous
    // channels is wholly unnecessary, for the sake of portability to WASM
    // we'll use async channels that work on both native and WASM.

    let (s, r) = crossbeam_channel::bounded(1);

    // Maps the buffer so it can be read on the cpu
    buffer_slice.map_async(MapMode::Read, move |r| match r {
        // This will execute once the gpu is ready, so after the call to poll()
        Ok(r) => s.send(r).expect("Failed to send map update"),
        Err(err) => panic!("Failed to map buffer {err}"),
    });

    // In order for the mapping to be completed, one of three things must happen.
    // One of those can be calling `Device::poll`. This isn't necessary on the web as devices
    // are polled automatically but natively, we need to make sure this happens manually.
    // `Maintain::Wait` will cause the thread to wait on native but not on WebGpu.

    // This blocks until the gpu is done executing everything
    render_device.poll(Maintain::wait()).panic_on_timeout();

    // This blocks until the buffer is mapped
    r.recv().expect("Failed to receive the map_async message");

    // This could fail on app exit, if Main world clears resources (including receiver) while Render world still renders
    let _ = sender.send(buffer_slice.get_mapped_range().to_vec());

    // We need to make sure all `BufferView`'s are dropped before we do what we're about
    // to do.
    // Unmap so that we can copy to the staging buffer in the next iteration.
    image_copier.buffer.unmap();
}

/// CPU-side image for saving
#[derive(Resource, Deref, DerefMut)]
struct ImageToSave(Handle<Image>);

// Takes from channel image content sent from render world and saves it to disk
fn update(
    images_to_save: Res<ImageToSave>,
    receiver: Res<RenderReceiver>,
    mut images: ResMut<Assets<Image>>,
) {
    // We don't want to block the main world on this,
    // so we use try_recv which attempts to receive without blocking
    let mut image_data = Vec::new();
    while let Ok(data) = receiver.try_recv() {
        // image generation could be faster than saving to fs,
        // that's why use only last of them
        image_data = data;
    }
    if !image_data.is_empty() {
        // Fill correct data from channel to image
        let img_bytes = images.get_mut(images_to_save.id()).unwrap();

        // We need to ensure that this works regardless of the image dimensions
        // If the image became wider when copying from the texture to the buffer,
        // then the data is reduced to its original size when copying from the buffer to the image.
        let row_bytes =
            img_bytes.width() as usize * img_bytes.texture_descriptor.format.pixel_size();
        let aligned_row_bytes = RenderDevice::align_copy_bytes_per_row(row_bytes);
        // shrink data to original image size
        img_bytes.data = image_data
            .chunks(aligned_row_bytes)
            .take(img_bytes.height() as usize)
            .flat_map(|row| &row[..row_bytes.min(row.len())])
            .cloned()
            .collect();
    }
}

#[derive(Component)]
struct Cube;

fn rotato(mut cubes: Query<&mut Transform, With<Cube>>, time: Res<Time>) {
    cubes.iter_mut().for_each(|mut tf| {
        let delta = time.delta_seconds();
        tf.rotate_x(1.5 * delta);
        tf.rotate_z(1.3 * delta);
    });
}

fn resize(
    texture: Res<ImageToSave>,
    mut images: ResMut<Assets<Image>>,
    screen: Res<Screen>,
    mut copier: ResMut<ImageCopier>,
    render_device: Res<RenderDevice>,
) {
    let texture = images.get_mut(&**texture).unwrap();
    let screen_size = screen.size();
    if texture.size() != screen_size {
        let extent = Extent3d {
            width: screen_size.x,
            height: screen_size.y,
            ..default()
        };

        texture.resize(extent);

        images.get_mut(&copier.src_image).unwrap().resize(extent);
        copier.buffer = ImageCopier::buffer(extent, &render_device);
    }
}

fn dither_settings(
    mut images: Query<&mut ImageToSprite>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    use DitherAlgorithm::*;
    use ThresholdMap::*;

    let mut image = images.single_mut();
    let dither = image.dither.as_mut().unwrap();

    if keys.just_pressed(KeyCode::Space) {
        dither.algorithm = match dither.algorithm {
            Ordered => Pattern,
            Pattern => Ordered,
        };
    }

    dither.threshold_map = match (
        dither.threshold_map,
        keys.just_pressed(KeyCode::ArrowRight) as i32
            - keys.just_pressed(KeyCode::ArrowLeft) as i32,
    ) {
        (map, 0) => map,
        (X2_2, -1) | (X4_4, -1) => X2_2,
        (X2_2, 1) | (X8_8, -1) => X4_4,
        (X4_4, 1) | (X8_8, 1) => X8_8,
        _ => unreachable!(),
    };

    dither.threshold += (keys.pressed(KeyCode::ArrowUp) as i8
        - keys.pressed(KeyCode::ArrowDown) as i8) as f32
        * time.delta_seconds()
        * 0.5;
}

#[px_layer]
struct Layer;
