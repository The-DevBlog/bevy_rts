use bevy::prelude::*;

pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_unit);
    }
}

#[derive(Component)]
pub struct Speed(f32);

#[derive(Component)]
pub struct Unit;

fn spawn_unit(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    cmds.spawn((
        PbrBundle {
            mesh: meshes.add(Capsule3d::new(0.5, 1.0)),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            material: materials.add(Color::DARK_GRAY),
            ..default()
        },
        Name::new("Unit"),
        Speed(10.0),
        Unit,
    ));
}
