//#import bevy_sprite::mesh2d_view_bind_group
//#import bevy_sprite::mesh2d_struct
//
//struct View {
//    view_proj: mat4x4<f32>;
//    world_position: vec3<f32>;
//};
//[[group(0), binding(0)]]
//var<uniform> view: View;

struct DebugMaterial {
    color: vec4<f32>;
};
[[group(1), binding(0)]]
var<uniform> material: DebugMaterial;

[[stage(fragment)]]
fn fragment() -> [[location(0)]] vec4<f32> {
    return material.color;
}
