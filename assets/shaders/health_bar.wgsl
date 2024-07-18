#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct HealthBarMaterial {
    foreground_color: vec4<f32>,
    background_color: vec4<f32>,
    percent: f32,
};

@group(2) @binding(0) var<uniform> material: HealthBarMaterial;


@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    var out: vec4<f32>;

    if mesh.uv.x <= material.percent {
        out = material.foreground_color;
    } else {
        out = material.background_color;
    }
    return out;
}
