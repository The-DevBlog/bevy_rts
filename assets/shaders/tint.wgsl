#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

const BLACK_THRESHOLD: f32 = 0.05;

struct TintShaderSettings {
    tint:          vec4<f32>,
    tint_strength: f32,
}

@group(0) @binding(0)
var screen_texture: texture_2d<f32>;
@group(0) @binding(1)
var texture_sampler: sampler;
@group(0) @binding(2)
var<uniform> settings: TintShaderSettings;

// Simple luminance function
fn luminance(c: vec3<f32>) -> f32 {
    return dot(c, vec3<f32>(0.299, 0.587, 0.114));
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let original_color = textureSample(screen_texture, texture_sampler, in.uv);
    let lum = luminance(original_color.rgb);

    // If nearly black, skip tint completely.
    if lum < BLACK_THRESHOLD {
        return original_color;
    }

    // Otherwise blend toward the tint.
    let output_color = mix(original_color, settings.tint, settings.tint_strength);
    return output_color;
}
