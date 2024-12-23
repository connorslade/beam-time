// Vertex Shader //

struct Vertex {
    @location(0) pos: vec4<f32>,
    @location(1) uv: vec2<f32>,
}

struct Instance {
    @location(2) transform_0: vec4<f32>,
    @location(3) transform_1: vec4<f32>,
    @location(4) transform_2: vec4<f32>,
    @location(5) transform_3: vec4<f32>,
    // // uv: vec4<f32>, // (ax, ay, bx, by)
    // uv: vec2<f32>,
    // uv_size: vec2<f32>,

    // color: vec3<f32>,
    // clip: vec4<f32>, // (ax, ay, bx, by)
}

struct VertexOutput {
    @builtin(position)
    pos: vec4<f32>,
    @location(0)
    uv: vec2<f32>
};


@vertex
fn vert(
    vertex: Vertex,
    instance: Instance
) -> VertexOutput {
    var transform = mat4x4<f32>(
        instance.transform_0,
        instance.transform_1,
        instance.transform_2,
        instance.transform_3,
    );

    var out: VertexOutput;
    out.pos = vertex.pos * transform;
    // out.uv = ctx.uv + vec2(uv.x * ctx.uv_size.x, uv.y * ctx.uv_size.y);// uv * ctx.uv_size;
    out.uv = vertex.uv;
    return out;
}

// Fragment Shader //

@group(0) @binding(0)
var texture: texture_2d<f32>;
@group(0) @binding(1)
var texture_sampler: sampler;

@fragment
fn frag(in: VertexOutput) -> @location(0) vec4<f32> {
    let sample = textureSample(texture, texture_sampler, in.uv);
    if sample.w == 0.0 { discard; }

    return sample * vec4(1.0);
}