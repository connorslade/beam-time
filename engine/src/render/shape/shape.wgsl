// Vertex Shader //

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>
}

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(1) color: vec3<f32>,
};


@vertex
fn vert(in: VertexInput) -> VertexOutput {
    let pos = in.position.xy * 2.0 - vec2(1.0);
    return VertexOutput(vec4(pos, in.position.z, 1.0), in.color);
}

// Fragment Shader //

@fragment
fn frag(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4(in.color, 1.0);
}
