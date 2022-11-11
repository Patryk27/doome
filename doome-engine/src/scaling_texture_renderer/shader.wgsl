struct Locals {
    scale: vec4<f32>,
}

@group(0) @binding(2) var<uniform> locals: Locals;


// -----

struct VertexOutput {
    @location(0) input_xy: vec2<f32>,
    @builtin(position) output_xy: vec4<f32>,
}

@vertex
fn vs_main(
    @location(0) position: vec2<f32>,
) -> VertexOutput {
    var out: VertexOutput;

    out.input_xy = fma(position, vec2(0.5, -0.5), vec2(0.5, 0.5)) * locals.scale.xy;
    out.output_xy = vec4(position, 1.0, 1.0);

    return out;
}

// -----

@group(0) @binding(0) var tex: texture_2d<f32>;
@group(0) @binding(1) var tex_sampler: sampler;

@fragment
fn fs_main(@location(0) input_xy: vec2<f32>) -> @location(0) vec4<f32> {
    return textureSample(tex, tex_sampler, input_xy);
}
