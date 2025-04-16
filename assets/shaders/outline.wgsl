#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

// Constants for the effect
// const resolution: vec2<f32> = vec2<f32>(1920.0, 1080.0); // Target resolution (adjust as needed)
// const normalThreshold: f32 = 0.01; // How sensitive the outline is to normal changes
// const outlineColor: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 1.0); // Outline color (black here)
// const outlineThickness: f32 = 2.5; // Outline thickness factor. Increase for a thicker outline.

struct OutlineShaderSettings {
    resolution: vec2<f32>,
    normalThreshold: f32,
    outlineColor: vec4<f32>,
    outlineThickness: f32,
}

// Texture and sampler bindings
@group(0) @binding(0)
var sceneTex: texture_2d<f32>;

@group(0) @binding(1)
var sceneSampler: sampler;

@group(0) @binding(2)
var<uniform> settings: OutlineShaderSettings;


// Normal map from an offscreen pass.
@group(0) @binding(3)
var normalTex: texture_2d<f32>;

@group(0) @binding(4)
var normalSampler: sampler;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let pixelSize = vec2<f32>(1.0) / settings.resolution;
    // Multiply pixelSize by our thickness factor to sample at a further distance if desired
    let offset = pixelSize * settings.outlineThickness;
    
    // Sample the center normals and neighbors using the offset
    let centerN: vec3<f32> = textureSample(normalTex, normalSampler, uv).xyz;
    let upN: vec3<f32> = textureSample(normalTex, normalSampler, uv + vec2<f32>(0.0,  offset.y)).xyz;
    let downN: vec3<f32> = textureSample(normalTex, normalSampler, uv - vec2<f32>(0.0,  offset.y)).xyz;
    let leftN: vec3<f32> = textureSample(normalTex, normalSampler, uv - vec2<f32>(offset.x, 0.0)).xyz;
    let rightN: vec3<f32> = textureSample(normalTex, normalSampler, uv + vec2<f32>(offset.x, 0.0)).xyz;
    
    // Compute edge strength by how different the normals are
    let diffUp   = length(centerN - upN);
    let diffDown = length(centerN - downN);
    let diffLeft = length(centerN - leftN);
    let diffRight= length(centerN - rightN);
    
    let maxDiff = max(max(diffUp, diffDown), max(diffLeft, diffRight));
    
    // If the difference in normals exceeds the threshold, draw outline
    if maxDiff > settings.normalThreshold {
        return settings.outlineColor;
    }
    
    // Otherwise, show the original color
    let sceneColor = textureSample(sceneTex, sceneSampler, uv);
    return sceneColor;
}
