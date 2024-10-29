use bevy::prelude::*;

use crate::{components::*, COLOR_PATH_FINDING};

pub struct PathFindingPlugin;

impl Plugin for PathFindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_line_to_destination);
    }
}

fn draw_line_to_destination(
    unit_q: Query<(&Destination, &Transform), With<Friendly>>,
    mut gizmos: Gizmos,
) {
    for (destination, transform) in unit_q.iter() {
        if let Some(destination) = destination.0 {
            let unit_pos = transform.translation;
            gizmos.line(unit_pos, destination, COLOR_PATH_FINDING);
        }
    }
}
