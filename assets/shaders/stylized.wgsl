#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

struct StylizedShaderSettings {
    horizon:        f32,
    softness:       f32,
    ground_color:   vec3<f32>,
    sky_color:      vec3<f32>,

    dark_tone:      vec3<f32>,
    mid_tone:       vec3<f32>,
    light_tone:     vec3<f32>,
    tone_thresh:    vec2<f32>,  // (dark→mid, mid→light)
    tone_strength:  f32,

    mix_amount:     f32,
    grain_strength: f32,

    // New!
    saturation:     f32,  // 1.0 = no change, >1 boost, <1 desat
    contrast:       f32,  // 1.0 = no change, >1 punch, <1 flatten
}

//–– bindings
@group(0) @binding(0) var sceneTex:     texture_2d<f32>;
@group(0) @binding(1) var sceneSampler: sampler;
@group(0) @binding(2) var<uniform> settings: StylizedShaderSettings;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0)    uv:       vec2<f32>,
};

// Rec.709 luminance
fn luminance(c: vec3<f32>) -> f32 {
    return dot(c, vec3<f32>(0.2126, 0.7152, 0.0722));
}

// Simple HSV‑style saturation adjust
fn adjust_saturation(c: vec3<f32>, s: f32) -> vec3<f32> {
    let y = luminance(c);
    return mix(vec3<f32>(y), c, s);
}

// Contrast about mid‑gray
fn adjust_contrast(c: vec3<f32>, contrast: f32) -> vec3<f32> {
    return (c - vec3<f32>(0.5)) * contrast + vec3<f32>(0.5);
}

// Film grain
fn grain(uv: vec2<f32>) -> f32 {
    let x = dot(uv * 123.456, vec2<f32>(78.233, 37.719));
    return fract(sin(x) * 43758.5453) - 0.5;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;

    // 0) Sample once
    let scene4 = textureSample(sceneTex, sceneSampler, uv);
    var col   = scene4.rgb;

    // 1) Tri‐tone *only* for dark and mid zones
    let lum = luminance(col);
    if lum < settings.tone_thresh.x {
        col = mix(col, settings.dark_tone, settings.tone_strength);
    } else if lum < settings.tone_thresh.y {
        col = mix(col, settings.mid_tone,  settings.tone_strength);
    }
    // (pixels brighter than tone_thresh.y keep their original color)

    // 2) Sky/ground tint *only* near the horizon
    let h0 = settings.horizon - settings.softness;
    let h1 = settings.horizon + settings.softness;
    let blend = smoothstep(h0, h1, uv.y);
    // ground only below horizon
    col = mix(col, settings.ground_color, settings.mix_amount * (1.0 - blend));
    // sky only above horizon
    col = mix(col, settings.sky_color, settings.mix_amount * blend);

    // 3) Saturation & contrast boost
    col = adjust_saturation(col, settings.saturation);
    col = adjust_contrast(col, settings.contrast);

    // 4) Film grain (subtle)
    col += grain(uv) * settings.grain_strength;

    return vec4<f32>(col, scene4.a);
}
