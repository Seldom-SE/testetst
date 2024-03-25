#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen: texture_2d<f32>;
@group(0) @binding(1) var tex_sampler: sampler;

@fragment
fn frag(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    return vec4(1., 0., 1., 1.);
}
