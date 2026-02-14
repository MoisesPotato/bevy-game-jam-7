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

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> bg_color: vec4<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var base_color_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var base_color_sampler: sampler;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    const black = vec4(0.,0.,0.,1);
    const red = vec4(1.,0.,0.,1);

    let x = mesh.uv.x;
    let y = mesh.uv.y;
    let angle = atan2(x - 0.5, y - 0.5);
    let angle_adjust = sin(angle * 5.) + cos(angle * 3.) * 0.7 + sin(angle * 7.) * 0.4;
    // return vec4(angle_adjust, 0., 0.,0.);
    let d_sq = 4. * ((x - 0.5) * (x - 0.5) + (y - 0.5) * (y - 0.5));
    var t = d_sq * (4 * sqrt(d_sq) - 1.3);
    // return vec4(t, 0.,0.,0.);
    t = t + angle_adjust * t * 0.3;
    t = clamp(t, 0., 1.);

    var s = clamp(4. * (t - 0.1) * (0.4 - t) + 0.1 * sin(angle * 4.), 0, 1) ;

    return t * bg_color + s * red ;// + (1 - t - s) * black; 
}
