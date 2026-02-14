#import bevy_sprite::mesh2d_vertex_output::VertexOutput
// we can import items from shader modules in the assets folder with a quoted path
// #import "shaders/custom_material_import.wgsl"::COLOR_MULTIPLIER

// https://github.com/bevyengine/bevy/blob/4430ca99b3253c0c7761c5c90d909054ab51f936/crates/bevy_sprite_render/src/mesh2d/mesh2d_vertex_output.wgsl
// struct VertexOutput {
//     // this is `clip position` when the struct is used as a vertex stage output 
//     // and `frag coord` when used as a fragment stage input
//     @builtin(position) position: vec4<f32>,
//     @location(0) world_position: vec4<f32>,
//     @location(1) world_normal: vec3<f32>,
//     @location(2) uv: vec2<f32>,
//     #ifdef VERTEX_TANGENTS
//     @location(3) world_tangent: vec4<f32>,
//     #endif
//     #ifdef VERTEX_COLORS
//     @location(4) color: vec4<f32>,
//     #endif
// }

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> material_color: vec4<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var base_color_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var base_color_sampler: sampler;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    return vec4(mesh.uv.x,mesh.uv.y,0.,1.);
}
