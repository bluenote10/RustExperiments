
struct ProjectionUniform {
    view_proj: mat3x3<f32>,
};
@group(0) @binding(0)
var<uniform> projection: ProjectionUniform;

struct VertexOutput {
    @location(0) color: vec4<f32>,
    @builtin(position) position: vec4<f32>,
};

@vertex
fn vs_main(
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
) -> VertexOutput {
    var result: VertexOutput;
    // result.position = vec4<f32>(position, 0.0, 1.0);
    result.position = vec4<f32>((projection.view_proj * vec3<f32>(position, 1.0)).xy, 0.0, 1.0);
    result.color = color;
    return result;
}

@fragment
fn fs_main(vertex: VertexOutput) -> @location(0) vec4<f32> {
    return vertex.color;
}
