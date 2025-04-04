use std::time::Duration;

use bevy::{animation::transition, prelude::*};

use crate::{structures::components::PrimaryStructure, units::events::BuildVehicleEv};

pub struct AnimtationsPlugin;

impl Plugin for AnimtationsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, build_animation_graph)
            .add_observer(garage_door_animation);
    }
}

#[derive(Resource)]
pub struct Animations {
    pub animations: Vec<AnimationNodeIndex>,
    pub graph: Handle<AnimationGraph>,
}

fn build_animation_graph(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    // Build the animation graph
    let mut graph = AnimationGraph::new();
    let animations = graph
        .add_clips(
            [GltfAssetLabel::Animation(0)
                .from_asset("models/structures/vehicle_depot/vehicle_depot.gltf")]
            .into_iter()
            .map(|path| asset_server.load(path)),
            1.0,
            graph.root,
        )
        .collect();

    // Insert a resource with the current scene information
    let graph = graphs.add(graph);
    cmds.insert_resource(Animations {
        animations,
        graph: graph.clone(),
    });
}

fn garage_door_animation(
    _trigger: Trigger<BuildVehicleEv>,
    mut cmds: Commands,
    animations: Res<Animations>,
    q_vehicle_depot: Query<Entity, With<PrimaryStructure>>,
    q_children: Query<&Children>,
    mut players: Query<(Entity, &mut AnimationPlayer)>,
) {
    let Ok(vehicle_depot_ent) = q_vehicle_depot.get_single() else {
        return;
    };

    for child_ent in q_children.iter_descendants(vehicle_depot_ent) {
        if let Ok((ent, mut animation_player)) = players.get_mut(child_ent) {
            let mut transitions = AnimationTransitions::new();
            let animation = animations.animations[0];

            transitions.play(&mut animation_player, animation, Duration::ZERO);

            cmds.entity(ent)
                .insert(AnimationGraphHandle(animations.graph.clone()))
                .insert(transitions);
        }
    }
}
