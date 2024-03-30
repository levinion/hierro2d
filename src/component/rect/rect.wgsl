struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) position: vec4<f32>,
};


struct RectUniform{
    position: vec2<f32>,
    size: vec2<f32>,
    radius: f32,
}

@group(0) @binding(0)
var<uniform> rect: RectUniform;

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    out.position = vec4<f32>(model.position, 0.0, 1.0);
    out.clip_position = vec4<f32>(model.position, 0.0, 1.0);
    return out;
}

fn rectSDF(p: vec2f , b: vec2f, r: f32) -> f32 {
    let d = abs(p) - b + r;
    return min(max(d.x, d.y), 0.0) + length(vec2f( max(d.x, 0.0), max(d.y, 0.0) )) - r;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let coords = in.position.xy;
    let pos = coords * rect.size/2.0;
    let center = vec2<f32>(rect.size.x / 2.0 + rect.position.x, rect.size.y / 2.0 - rect.position.y);
    let l = rectSDF(coords, rect.size/2.0 - center, rect.radius);
    if l > 0.0{
        discard;
    }
    return vec4<f32>(in.color);
}

