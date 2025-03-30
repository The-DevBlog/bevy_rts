use bevy::prelude::*;

mod vehicle_depot;

use vehicle_depot::VehicleDepotPlugin;

use crate::{
    components::{
        structures::{SelectedStructure, Structure, StructureType},
        SelectBorder,
    },
    events::SelectStructureEv,
    resources::{DbgOptions, GameCommands, MyAssets, StructuresBuilt},
};

pub struct StructuresPlugin;

impl Plugin for StructuresPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(VehicleDepotPlugin)
            .add_systems(Update, mark_structure_built)
            .add_observer(select_structure);
    }
}

// modifies the 'StructuresBuilt' resource, whenever a structure is placed or removed (destroyed)
pub fn mark_structure_built(
    mut structures_built: ResMut<StructuresBuilt>,
    q_structure_added: Query<&StructureType, Added<Structure>>,
) {
    for structure in q_structure_added.iter() {
        match structure {
            StructureType::Cannon => structures_built.cannon += 1,
            StructureType::Barracks => structures_built.barracks += 1,
            StructureType::VehicleDepot => structures_built.vehicle_depot += 1,
            StructureType::ResearchCenter => structures_built.research_center += 1,
            StructureType::SatelliteDish => structures_built.satellite_dish += 1,
        }
    }
}

fn select_structure(
    trigger: Trigger<SelectStructureEv>,
    dbg: Res<DbgOptions>,
    mut cmds: Commands,
    game_cmds: Res<GameCommands>,
    my_assets: Res<MyAssets>,
) {
    dbg.print("Structure selected");

    if game_cmds.hvr_cmd_interface {
        return;
    }

    let structure_ent = trigger.0;

    // Closure that creates a new border for a given unit.
    let border = |ent: Entity| -> (SelectBorder, ImageNode) {
        (
            SelectBorder(ent),
            ImageNode {
                image: my_assets.imgs.select_border.clone(),
                ..default()
            },
        )
    };

    cmds.entity(structure_ent).insert(SelectedStructure);
    cmds.spawn(border(structure_ent));
}
