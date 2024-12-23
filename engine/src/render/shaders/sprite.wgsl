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

    @location(6) uv: vec2<f32>,
    @location(7) uv_size: vec2<f32>,

    @location(8) color: vec3<f32>,
    @location(9) clip: vec4<f32>, // (ax, ay, bx, by)
}

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec3<f32>
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

    out.color = instance.color;
    out.pos = vertex.pos * transform;
    
    out.uv = instance.uv + vec2(
        vertex.uv.x * instance.uv_size.x,
        vertex.uv.y * instance.uv_size.y
    );

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

    return sample * vec4(in.color, 1.0);
}