use bevy::{
    color::palettes::css::*,
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    scene::SceneInstanceReady,
};

use crate::*;

pub struct ShadersPlugin;

impl Plugin for ShadersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<
            ExtendedMaterial<StandardMaterial, MyExtension>,
        >::default())
            .add_observer(customize_scene_materials);
    }
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct MyExtension {
    // 0 - 99 reserved for base material
    #[uniform(100)]
    pub base_color: LinearRgba,

    #[uniform(101)]
    pub tint: LinearRgba,

    #[uniform(102)]
    pub tint_strength: f32,
}

impl MaterialExtension for MyExtension {
    fn fragment_shader() -> ShaderRef {
        "shaders/lighting_extended.wgsl".into()
    }

    fn vertex_shader() -> ShaderRef {
        "shaders/lighting_extended.wgsl".into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        "shaders/lighting_extended.wgsl".into()
    }
}

fn customize_scene_materials(
    trigger: Trigger<SceneInstanceReady>,
    mut extended_materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, MyExtension>>>,
    standard_materials: Res<Assets<StandardMaterial>>,
    mut cmds: Commands,
    q_children: Query<&Children>,
    q_mesh_material: Query<(Entity, &MeshMaterial3d<StandardMaterial>)>,
) {
    // Traverse the spawned SceneRoot's descendants.
    for entity in q_children.iter_descendants(trigger.entity()) {
        // Try to get a MeshMaterial3d<StandardMaterial> component on this entity.
        if let Ok((ent, mesh_mat)) = q_mesh_material.get(entity) {
            // Use the handle from the MeshMaterial3d to fetch the StandardMaterial.
            if let Some(std_mat) = standard_materials.get(mesh_mat.id()) {
                // Optionally, clone and modify the StandardMaterial.
                let modified_std = std_mat.clone();
                // (For example, you could change the base color here before wrapping.)
                let base_color = modified_std.base_color;
                // Now create an ExtendedMaterial that wraps the StandardMaterial.
                let new_extended_handle = extended_materials.add(ExtendedMaterial {
                    base: modified_std,
                    extension: MyExtension {
                        base_color: base_color.into(),
                        tint: TINT_CLR.into(), // Your desired tint color.
                        tint_strength: TINT_STRENGTH, // How strongly to apply the tint.
                    },
                });

                // Replace the material component on this entity:
                // Option 1: Remove the old material component and insert the new one.
                cmds.entity(ent)
                    .remove::<MeshMaterial3d<StandardMaterial>>() // Remove the original.
                    .insert(MeshMaterial3d(new_extended_handle)); // Insert the new, extended one.
            }
        }
    }
}
