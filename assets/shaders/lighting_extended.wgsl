#import bevy_pbr::mesh_functions::{get_world_from_local, mesh_position_local_to_clip}

// --- Vertex Input/Output Structures ---
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_normal: vec3<f32>,
    // UV coordinates from the vertex shader.
    @location(1) uv: vec2<f32>,
};

// --- Uniforms ---
// Camera view-projection matrix. (Provided in group 0, binding 0)
@group(0) @binding(0)
var<uniform> ViewProj: mat4x4<f32>;

// Custom material extension uniforms in group 2.
// We’re using custom bindings 101 and 102 for our tint parameters.
@group(2) @binding(101)
var<uniform> tint: vec4<f32>;

@group(2) @binding(102)
var<uniform> tint_strength: f32;

// Declare the base texture and its sampler that come from your StandardMaterial’s texture.
// These binding numbers must match the pipeline layout or material extension setup.
@group(2) @binding(1)
var base_color_texture: texture_2d<f32>;
@group(2) @binding(2)
var base_color_sampler: sampler;

// Optionally, if you want a fallback for objects that have no texture,
// you can also use a uniform color. (This is not used if the texture is available.)
@group(2) @binding(100)
var<uniform> base_color: vec4<f32>;

@vertex
fn vertex(
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
) -> VertexOutput {
    var out: VertexOutput;
    // Transform the vertex position to clip space.
    out.clip_position = mesh_position_local_to_clip(
        get_world_from_local(instance_index),
        vec4<f32>(position, 1.0),
    );

    out.world_normal = normalize((get_world_from_local(instance_index) * vec4<f32>(normal, 0.0)).xyz);
    out.uv = uv;
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // --- Lighting Computation ---
    let light_direction = normalize(vec3<f32>(0.5, 0.5, 1.0));
    let normal = normalize(in.world_normal);
    let diffuse = max(dot(normal, light_direction), 0.0);
    let ambient = 0.1;
    let lighting = ambient + diffuse;

    // --- Base Color from Texture ---
    // Sample the base texture using the UV coordinates.
    let tex_color = textureSample(base_color_texture, base_color_sampler, in.uv);
    // Multiply the texture sample by the fallback base_color uniform.
    // For objects without an explicit texture, base_color holds the StandardMaterial base color (green).
    let effective_color = tex_color.rgb * base_color.rgb;
    
    // Apply the lighting.
    let lit_color = effective_color * lighting;

    // --- Tint Application ---
    let tinted_color = lit_color * tint.rgb;
    let final_color = mix(lit_color, tinted_color, tint_strength);

    return vec4<f32>(final_color, tex_color.a);
}
