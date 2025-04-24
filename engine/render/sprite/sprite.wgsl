// Vertex Shader //

struct Instance {
    @location(1) point_0: vec4<f32>,
    @location(2) point_1: vec4<f32>,
    @location(3) uv: vec4<f32>,

    @location(4) layer: f32,
    @location(5) color: vec3<f32>,
    @location(6) clip: vec4<f32>
}

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) clip_pos: vec2<f32>,

    @location(1) uv: vec2<f32>,
    @location(2) color: vec3<f32>,
    @location(3) clip: vec4<f32>,
};


@vertex
fn vert(
    @builtin(vertex_index) index: u32,
    instance: Instance
) -> VertexOutput {
    var uvs = array(
        vec2(0.0, 1.0), vec2(0.0, 0.0),
        vec2(1.0, 0.0), vec2(1.0, 1.0)
    );
    var points = array(
        instance.point_0.xy,
        instance.point_0.zw,
        instance.point_1.xy,
        instance.point_1.zw
    );

    var out: VertexOutput;

    out.clip_pos = points[index];
    var pos = out.clip_pos * 2.0 - vec2(1.0);
    out.pos = vec4(pos, instance.layer, 1.0);

    var uv = uvs[index];
    out.uv = instance.uv.xy + vec2(
        uv.x * instance.uv.z,
        uv.y * instance.uv.w
    );

    out.color = instance.color;
    out.clip = instance.clip;

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
    if sample.w == 0.0 || (
        in.clip_pos.x < in.clip.x
        || in.clip_pos.x > in.clip.z
        || in.clip_pos.y < in.clip.y
        || in.clip_pos.y > in.clip.w
    ) { discard; }

    return vec4(sample.xyz * in.color, 1.0);
}
