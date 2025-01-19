struct CameraUniform {
    position: vec2<f32>,
    size: vec2<f32>
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct Vertex {
    @location(0) position: vec2<f32>,
    @location(1) uv: vec2<f32>,
}

struct Output {
    @builtin(position) clip_pos: vec4<f32>
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

@vertex
fn vs_main(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(vertex.position * camera.size + camera.position, 0.0, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.3, 0.2, 0.1, 1.0);
}