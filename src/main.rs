use bevy::{
    core_pipeline::{
        core_3d::graph::{Core3d, Node3d},
        fullscreen_vertex_shader::fullscreen_shader_vertex_state,
    },
    ecs::query::QueryItem,
    prelude::*,
    render::{
        render_graph::{
            NodeRunError, RenderGraphApp, RenderGraphContext, RenderLabel, ViewNode, ViewNodeRunner,
        },
        render_resource::{
            BindGroupEntries, BindGroupLayout, CachedRenderPipelineId, FragmentState, Operations,
            PipelineCache, RenderPassColorAttachment, RenderPassDescriptor,
            RenderPipelineDescriptor, Sampler, SamplerDescriptor,
        },
        renderer::{RenderContext, RenderDevice},
        texture::BevyDefault,
        view::ViewTarget,
        RenderApp,
    },
    ui::graph::NodeUi,
};
use wgpu_types::{
    BindGroupLayoutEntry, BindingType, ColorTargetState, ColorWrites, MultisampleState,
    PrimitiveState, SamplerBindingType, ShaderStages, TextureFormat, TextureSampleType,
    TextureViewDimension,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                mode: AssetMode::Processed,
                ..default()
            }),
            PostprocPlug,
        ))
        .add_systems(Startup, init)
        .run();
}

fn init(mut cmd: Commands) {
    cmd.spawn(Camera3dBundle::default());
}

pub struct PostprocPlug;

impl Plugin for PostprocPlug {
    fn build(&self, app: &mut App) {
        app.get_sub_app_mut(RenderApp)
            .unwrap()
            .add_render_graph_node::<ViewNodeRunner<Node>>(Core3d, Postproc)
            .add_render_graph_edges(Core3d, (NodeUi::UiPass, Postproc, Node3d::Upscaling));
    }

    fn finish(&self, app: &mut App) {
        app.get_sub_app_mut(RenderApp)
            .unwrap()
            .init_resource::<Pipeline>();
    }
}

#[derive(Resource)]
struct Pipeline {
    layout: BindGroupLayout,
    sampler: Sampler,
    id: CachedRenderPipelineId,
}

impl FromWorld for Pipeline {
    fn from_world(world: &mut World) -> Self {
        let device = world.resource::<RenderDevice>();

        let layout = device.create_bind_group_layout(
            "postproc_bind_group_layout",
            &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        );

        let assets = world.resource::<AssetServer>();
        let shader = assets.load("postproc.wgsl");

        Self {
            layout: layout.clone(),
            sampler: device.create_sampler(&SamplerDescriptor::default()),
            id: world.resource_mut::<PipelineCache>().queue_render_pipeline(
                RenderPipelineDescriptor {
                    label: Some("postproc_pipeline".into()),
                    layout: vec![layout],
                    vertex: fullscreen_shader_vertex_state(),
                    fragment: Some(FragmentState {
                        shader,
                        shader_defs: Vec::new(),
                        entry_point: "frag".into(),
                        targets: vec![Some(ColorTargetState {
                            format: TextureFormat::bevy_default(),
                            blend: None,
                            write_mask: ColorWrites::ALL,
                        })],
                    }),
                    primitive: PrimitiveState::default(),
                    depth_stencil: None,
                    multisample: MultisampleState::default(),
                    push_constant_ranges: Vec::new(),
                },
            ),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, RenderLabel)]
struct Postproc;

#[derive(Default)]
struct Node;

impl ViewNode for Node {
    type ViewQuery = &'static ViewTarget;

    fn run(
        &self,
        _: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        view_query: QueryItem<&ViewTarget>,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let pipeline = world.resource::<Pipeline>();

        let Some(render_pipeline) = world
            .resource::<PipelineCache>()
            .get_render_pipeline(pipeline.id)
        else {
            return Ok(());
        };

        let postproc = view_query.post_process_write();

        let bind_group = render_context.render_device().create_bind_group(
            "postproc_bind_group",
            &pipeline.layout,
            &BindGroupEntries::sequential((postproc.source, &pipeline.sampler)),
        );

        let mut pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("postproc_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: postproc.destination,
                resolve_target: None,
                ops: Operations::default(),
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        pass.set_render_pipeline(render_pipeline);
        pass.set_bind_group(0, &bind_group, &[]);
        pass.draw(0..3, 0..1);

        Ok(())
    }
}
