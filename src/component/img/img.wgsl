struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) tex_coords: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};


struct RectUniform {
    position: vec2<f32>,
    size: vec2<f32>,
    radius: f32,
    resolution: vec2<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.position = vec4<f32>(model.position, 0.0, 1.0);
    return out;
}

fn rectSDF(p: vec2f, b: vec2f, r: f32) -> f32 {
    let d = abs(p) - b + r;
    return min(max(d.x, d.y), 0.0) + length(vec2(max(d.x, 0.0), max(d.y, 0.0))) - r;
}

@group(1) @binding(0)
var<uniform> rect: RectUniform;

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.position.xy / rect.resolution.xy;
    let ratio = rect.resolution.x / rect.resolution.y;
    let center = vec2(rect.position.x + rect.size.x / 2, rect.position.y - rect.size.y / 2.0);
    let coords = (uv * 2 - 1 - vec2(center.x, -center.y)) * vec2(ratio, 1);
    let half_size = rect.size / 2 * vec2(ratio, 1);
    let distance = rectSDF(coords, half_size, rect.radius);
    if distance > 0.0 {
        discard;
    }
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
