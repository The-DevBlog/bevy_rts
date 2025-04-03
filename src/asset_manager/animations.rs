use std::time::Duration;

use bevy::prelude::*;

pub struct AnimtationsPlugin;

impl Plugin for AnimtationsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, build_animation_graph)
            .add_systems(Update, setup_scene_once_loaded);
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
            // [GltfAssetLabel::Animation(0).from_asset("cube.gltf")]
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

fn setup_scene_once_loaded(
    mut cmds: Commands,
    animations: Res<Animations>,
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
) {
    for (entity, mut player) in players.iter_mut() {
        let mut transitions = AnimationTransitions::new();
        let animation = animations.animations[0];

        transitions
            .play(&mut player, animation, Duration::ZERO)
            .repeat();

        cmds.entity(entity)
            .insert(AnimationGraphHandle(animations.graph.clone()))
            .insert(transitions);
    }
}
