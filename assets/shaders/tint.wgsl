#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

struct TintPostProcessSettings {
    tint: vec4<f32>,
    tint_strength: f32,

}
@group(0) @binding(2) var<uniform> settings: TintPostProcessSettings;

// The screen texture is the rendered scene provided as a texture.
@group(0) @binding(0)
var screen_texture: texture_2d<f32>;

// Sampler for the screen texture.
@group(0) @binding(1)
var texture_sampler: sampler;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    // Sample the rendered scene from the screen texture using the provided UV coordinates.
    let original_color = textureSample(screen_texture, texture_sampler, in.uv);
    
    // Linearly interpolate (mix) between the original scene color and the tint color,
    // using tint_strength as the blending factor.
    let output_color = mix(original_color, settings.tint, settings.tint_strength);
    
    return output_color;
}
