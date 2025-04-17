#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

//–– Uniforms for controlling the look ––
struct StylizedShaderSettings {
    // 0.0 = only original scene colors; 1.0 = only ramp palette
    ramp_mix:      f32,
    // How far each channel splits (in UV space)
    aberr_strength: f32,
    // Strength of the per‑pixel noise
    noise_strength: f32,
}

// Bindings
@group(0) @binding(0) var sceneTex: texture_2d<f32>;
@group(0) @binding(1) var sceneSampler: sampler;
@group(0) @binding(2) var<uniform> settings: StylizedShaderSettings;

// Your 1D palette ramp: sample with x = luminance, y = 0.5
@group(0) @binding(3) var rampTex: texture_2d<f32>;
@group(0) @binding(4) var rampSampler: sampler;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0)    uv:       vec2<f32>,
};

// Simple luminance extractor
fn luminance(c: vec3<f32>) -> f32 {
    return dot(c, vec3<f32>(0.299, 0.587, 0.114));
}

// 2D “random” for a bit of per‑pixel noise
fn rand(uv: vec2<f32>) -> f32 {
    return fract(sin(dot(uv , vec2<f32>(12.9898, 78.233))) * 43758.5453);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;

    // 1) Sample the original scene
    let col = textureSample(sceneTex, sceneSampler, uv).rgb;

    // 2) Compute lum & fetch palette color
    let lum     = luminance(col);
    let rampCol = textureSample(rampTex, rampSampler, vec2<f32>(lum, 0.5)).rgb;

    // 3) Mix ramp & original
    var outCol = mix(rampCol, col, settings.ramp_mix);

    // 4) Chromatic aberration: nudge R and B channels apart
    let caOff = settings.aberr_strength * (uv - vec2<f32>(0.5));
    let r = textureSample(sceneTex, sceneSampler, uv + caOff).r;
    let b = textureSample(sceneTex, sceneSampler, uv - caOff).b;
    outCol.r = r;
    outCol.b = b;

    // 5) Add subtle film‑like noise
    let n = (rand(uv) - 0.5) * settings.noise_strength;
    outCol += n;

    return vec4<f32>(outCol, 1.0);
}
