// #import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

// // Screen texture: the rendered scene.
// @group(0) @binding(0)
// var scene_texture: texture_2d<f32>;

// // Sampler for the screen texture.
// @group(0) @binding(1)
// var scene_sampler: sampler;

// // A uniform containing our settings for the stylized effect.
// struct Settings {
//     // Number of quantization levels per channel.
//     quantization_levels: u32,
//     // Blend factor: 0.0 will show the original image, 1.0 will show the fully stylized image.
//     blend_factor: f32,
//     // Saturation boost: 1.0 means no change, values greater than 1.0 increase saturation.
//     saturation_boost: f32,
// }
// @group(0) @binding(2)
// var<uniform> settings: Settings;

// // Function to quantize a color into a set number of levels.
// fn quantize_color(c: vec3<f32>, levels: u32) -> vec3<f32> {
//     let levels_f = f32(levels);
//     // Multiply, add a half-step to round, floor, then scale back
//     return floor(c * levels_f + 0.5) / levels_f;
// }

// // Function to boost saturation by mixing the color with its grayscale version.
// fn saturate_color(c: vec3<f32>, boost: f32) -> vec3<f32> {
//     // Compute perceived luminance using standard coefficients.
//     let l = dot(c, vec3<f32>(0.299, 0.587, 0.114));
//     // Mix between gray (no saturation) and the original color.
//     return mix(vec3<f32>(l), c, boost);
// }

// @fragment
// fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
//     // Sample the original rendered scene.
//     let original_color = textureSample(scene_texture, scene_sampler, in.uv).rgb;
    
//     // Quantize the original color for a "posterized" look.
//     let quantized = quantize_color(original_color, settings.quantization_levels);
    
//     // Apply a saturation boost to the quantized color.
//     let sat_quantized = saturate_color(quantized, settings.saturation_boost);
    
//     // Blend between the original color and the stylized version.
//     let final_color = mix(original_color, sat_quantized, settings.blend_factor);
    
//     return vec4<f32>(final_color, 1.0);
// }

// outline.wgsl – Converted from GLSL to WGSL
//
// NOTE: This conversion assumes that the functions for depth conversion are defined
//       in a similar spirit as GLSL’s packing functions. Adjust the implementations if needed.

