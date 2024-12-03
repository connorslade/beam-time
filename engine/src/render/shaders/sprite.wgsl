@group(0) @binding(0)
var<uniform> context: Uniform;
@group(0) @binding(1)
var texture: texture_2d<f32>;
@group(0) @binding(2)
var texture_sampler: sampler;

// struct Uniform {
//     transform: mat4x4<f32>,
//     uv: array<vec2<f32>, 2>,
//     color: vec3<f32>,
//     clip: array<vec2<f32>, 2>,
// }

struct Uniform {
    transform: mat4x4<f32>,
    uv: vec4<f32>, // (ax, ay, bx, by)
    color: vec3<f32>,
    clip: vec4<f32>, // (ax, ay, bx, by)
}


struct VertexOutput {
    @builtin(position)
    pos: vec4<f32>,
    @location(0)
    uv: vec2<f32>
};

@vertex
fn vert(
    @location(0) pos: vec4<f32>,
    @location(1) uv: vec2<f32>,
) -> VertexOutput {
    var out: VertexOutput;
    out.pos = pos;
    out.uv = uv;
    return out;
}

@fragment
fn frag(in: VertexOutput) -> @location(0) vec4<f32> {
    let sample = textureSample(texture, texture_sampler, in.uv);
    if sample.w == 0.0 { discard; }

    return sample * vec4(context.color, 1.0);
}