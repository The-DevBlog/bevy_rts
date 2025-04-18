#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

//–––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––
// uniforms
struct StylizedShaderSettings {
    // horizon parameters
    horizon:        f32,    // y‐coordinate (0..1) of the horizon line
    softness:       f32,    // how soft the horizon blend is

    // sky/ground colors
    ground_color:   vec3<f32>,
    sky_color:      vec3<f32>,

    // tri‐tone ramp
    dark_tone:      vec3<f32>,
    mid_tone:       vec3<f32>,
    light_tone:     vec3<f32>,
    tone_thresh:    vec2<f32>,  // (dark→mid, mid→light) luminance thresholds
    tone_strength:  f32,        // how strongly to apply tri‐tone

    // overall mix between original and sky/ground tint
    mix_amount:     f32,

    // grain
    grain_strength: f32,
}

//–– bindings
@group(0) @binding(0) var sceneTex:     texture_2d<f32>;
@group(0) @binding(1) var sceneSampler: sampler;
@group(0) @binding(2) var<uniform> settings: StylizedShaderSettings;

//–––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––
// helpers

fn luminance(c: vec3<f32>) -> f32 {
    // standard Rec. 709 luma
    return dot(c, vec3<f32>(0.2126, 0.7152, 0.0722));
}

// simple film‑grain
fn grain(uv: vec2<f32>) -> f32 {
    let x = dot(uv * 123.456, vec2<f32>(78.233, 37.719));
    return fract(sin(x) * 43758.5453) - 0.5;
}

//–––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––    
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0)    uv:       vec2<f32>,
};

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    // 1) sample scene
    var col : vec3<f32> = textureSample(sceneTex, sceneSampler, uv).rgb;

    // 2) tri‐tone ramp
    let lum = luminance(col);
    var tone: vec3<f32>;
    if lum < settings.tone_thresh.x {
        tone = settings.dark_tone;
    } else if lum < settings.tone_thresh.y {
        tone = settings.mid_tone;
    } else {
        tone = settings.light_tone;
    }
    col = mix(col, tone, settings.tone_strength);

    // 3) sky/ground split
    //    compute a smooth blend factor at the horizon
    let h0 = settings.horizon - settings.softness;
    let h1 = settings.horizon + settings.softness;
    let blend = smoothstep(h0, h1, uv.y);

    //    interpolate ground_color → sky_color
    let region_color = mix(settings.ground_color, settings.sky_color, blend);

    //    mix with your tri‐toned color
    col = mix(col, region_color, settings.mix_amount);

    // 4) add grain
    col += grain(uv) * settings.grain_strength;

    return vec4<f32>(col, 1.0);
}
