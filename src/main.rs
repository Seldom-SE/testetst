use bevy::{
    prelude::*,
    render::{
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
};
use seldom_pixel::{prelude::*, sprite::ImageToSprite};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PxPlugin::<Layer>::new(UVec2::splat(512), "palette.palette.png"),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (rotator_system, list_all_colors))
        .run();
}

// Marks the first pass cube (rendered to a texture.)
#[derive(Component)]
struct FirstPassCube;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let size = Extent3d {
        width: 512,
        height: 512,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);

    let image_handle = images.add(image);

    let cube_handle = meshes.add(Cuboid::new(4.0, 4.0, 4.0));
    let cube_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.8, 0.7, 0.6),
        reflectance: 0.02,
        unlit: false,
        ..default()
    });

    // This specifies the layer used for the first pass, which will be attached to the first pass camera and cube.
    let first_pass_layer = RenderLayers::layer(1);

    // The cube that will be rendered to the texture.
    commands.spawn((
        PbrBundle {
            mesh: cube_handle,
            material: cube_material_handle,
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..default()
        },
        FirstPassCube,
        first_pass_layer,
    ));

    // Light
    // NOTE: we add the light to all layers so it affects both the rendered-to-texture cube, and the cube on which we display the texture
    // Setting the layer to RenderLayers::layer(0) would cause the main view to be lit, but the rendered-to-texture cube to be unlit.
    // Setting the layer to RenderLayers::layer(1) would cause the rendered-to-texture cube to be lit, but the main view to be unlit.
    commands.spawn((
        PointLightBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
            ..default()
        },
        RenderLayers::all(),
    ));

    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                // render before the "main pass" camera
                order: -1,
                target: image_handle.clone().into(),
                clear_color: Color::WHITE.into(),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 15.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        first_pass_layer,
    ));

    commands.spawn(SpriteBundle {
        texture: image_handle.clone(),
        transform: Transform::from_translation(Vec3::Z),
        ..default()
    });

    commands.spawn((
        PxSpriteBundle::<Layer> {
            anchor: PxAnchor::BottomLeft,
            ..default()
        },
        ImageToSprite(image_handle.clone()),
    ));

    // The main pass camera.
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(MyImage(image_handle));
}

/// Rotates the inner cube (first pass)
fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<FirstPassCube>>) {
    for mut transform in &mut query {
        transform.rotate_x(1.5 * time.delta_seconds());
        transform.rotate_z(1.3 * time.delta_seconds());
    }
}

#[derive(Resource)]
struct MyImage(Handle<Image>);

fn list_all_colors(my_image: Res<MyImage>, images: Res<Assets<Image>>) {
    info!("listing all colors");

    let MyImage(ref image_handle) = *my_image;
    for pixel in images.get(image_handle).unwrap().data.chunks_exact(4) {
        if pixel != [0, 0, 0, 0] {
            let &[blue, green, red, alpha] = pixel else {
                panic!();
            };

            info!("found a color: R{red} G{green} B{blue} A{alpha}");
        }
    }
}

#[px_layer]
struct Layer;
