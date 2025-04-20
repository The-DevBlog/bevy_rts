#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput
#import bevy_render::view::View
#import bevy_pbr::view_transformations::uv_to_ndc;

// Constants for the effect
const resolution: vec2<f32> = vec2<f32>(1920.0, 1080.0); // Target resolution (adjust as needed)
const normalThreshold: f32 = 0.01; // How sensitive the outline is to normal changes
const outlineColor: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 1.0); // Outline color (black here)
const outlineThickness: f32 = 2.5; // Outline thickness factor. Increase for a thicker outline.

const MIN_ZOOM: f32 = 0.3;
const MAX_ZOOM: f32 = 2.0;

struct OutlineShaderSettings {
    zoom: f32,
    resolution: vec2<f32>,
    normalThreshold: f32,
    outlineColor: vec4<f32>,
    outlineThickness: f32,
}

// Texture and sampler bindings
@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var scene_sampler: sampler;
@group(0) @binding(2) var<uniform> settings: OutlineShaderSettings;
@group(0) @binding(3) var normal_texture: texture_2d<f32>;
@group(0) @binding(4) var depth_texture: texture_depth_2d;
@group(0) @binding(5) var<uniform> view: View;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

fn prepass_depth(frag_coord: vec2f) -> f32 {
    return textureLoad(depth_texture, vec2i(frag_coord), 0);
}

fn position_ndc_to_world(ndc_pos: vec2<f32>, depth: f32) -> vec3<f32> {
    let world_pos = view.world_from_clip * vec4(ndc_pos, depth, 1.0);
    return world_pos.xyz / world_pos.w;
}

fn uv_to_pos(uv: vec2f) -> vec2f {
    return uv * vec2<f32>(textureDimensions(screen_texture));
}

fn worldspace_camera_view_direction(uv: vec2f) -> vec3f {
    let ndc = uv_to_ndc(uv);
    let ray_point = position_ndc_to_world(ndc, prepass_depth(uv_to_pos(uv)));
    return normalize(ray_point - view.world_position).xyz;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let zoom = clamp(settings.zoom, MIN_ZOOM, MAX_ZOOM);
    let pixelSize = vec2<f32>(1.0) / resolution * zoom;
    // Multiply pixelSize by our thickness factor to sample at a further distance if desired
    let offset = pixelSize * outlineThickness;
    
    // Sample the center normals and neighbors using the offset
    let centerN: vec3<f32> = textureSample(normal_texture, scene_sampler, uv).xyz;
    let upN: vec3<f32> = textureSample(normal_texture, scene_sampler, uv + vec2<f32>(0.0,  offset.y)).xyz;
    let downN: vec3<f32> = textureSample(normal_texture, scene_sampler, uv - vec2<f32>(0.0,  offset.y)).xyz;
    let leftN: vec3<f32> = textureSample(normal_texture, scene_sampler, uv - vec2<f32>(offset.x, 0.0)).xyz;
    let rightN: vec3<f32> = textureSample(normal_texture, scene_sampler, uv + vec2<f32>(offset.x, 0.0)).xyz;
    
    // Compute edge strength by how different the normals are
    let diffUp   = length(centerN - upN);
    let diffDown = length(centerN - downN);
    let diffLeft = length(centerN - leftN);
    let diffRight= length(centerN - rightN);
    
    let maxDiff = max(max(diffUp, diffDown), max(diffLeft, diffRight));
    
    // If the difference in normals exceeds the threshold, draw outline
    if maxDiff > normalThreshold {
        return outlineColor;
    }
    
    // Otherwise, show the original color
    let sceneColor = textureSample(screen_texture, scene_sampler, uv);
    return sceneColor;
}
