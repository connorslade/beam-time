struct VertexOutput {
    @builtin(position)
    pos: vec4<f32>,
    @location(0)
    uv: vec2<f32>,
    @location(1)
    color: vec3<f32>,
};

@vertex
fn vert(
    @location(0) pos: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) color: vec3<f32>,
) -> VertexOutput {
    var out: VertexOutput;
    out.pos = vec4(2.0 * pos.x - 1.0, 2.0 * pos.y - 1.0, pos.z, 1.0);
    out.uv = uv;
    out.color = color;
    return out;
}

@group(0) @binding(0)
var texture: texture_2d<f32>;
@group(0) @binding(1)
var texture_sampler: sampler;

@fragment
fn frag(in: VertexOutput) -> @location(0) vec4<f32> {
    let sample = textureSample(texture, texture_sampler, in.uv);
    if sample.w == 0.0 { discard; }

    return sample * vec4(in.color, 1.0);
}