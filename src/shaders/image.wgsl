struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
}

struct Output {
    @builtin(position) clip_pos: vec4<f32>
}

@vertex
fn vs_main(vertex: Vertex) -> Output {
    var out: Output;
    out.clip_pos = vec4<f32>(vertex.position, 1.0);
    return out;
}

@fragment
fn fs_main(in: Output) -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 1.0, 1.0, 1.0);
}