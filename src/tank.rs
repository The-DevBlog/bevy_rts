use bevy::math::f32;
use bevy::time::common_conditions::once_after_delay;
use std::f32::consts::PI;
use std::time::Duration;

// use crate::asset_manager::audio::MyAudio;
use crate::asset_manager::models::MyModels;
use crate::units::components::*;
use crate::*;

pub const BORDER_SIZE: Vec2 = Vec2::new(50.0, 50.0);

pub struct TankPlugin;

impl Plugin for TankPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Startup, spawn_tank);
        app.add_systems(
            Update,
            (spawn_tanks.run_if(once_after_delay(Duration::from_secs(1))),).chain(),
        );
    }
}

pub fn spawn_tank(
    mut cmds: Commands,
    my_models: Res<MyModels>,
    // audio: Res<bevy_kira_audio::Audio>,
    // my_audio: Res<MyAudio>,
) {
    // Define the 180 degree rotation about the Y axis.
    let tank_rotation = Quat::from_rotation_y(PI);

    // GEN I: Create a transform with both translation and the rotation.
    let transform = Transform {
        translation: Vec3::new(-100.0, 2.0, 0.0),
        rotation: tank_rotation,
        scale: Vec3::ONE,
    };
    cmds.spawn(UnitType::TankGen1.build(transform, &my_models));

    // GEN II: Another tank with the same rotation.
    let transform = Transform {
        translation: Vec3::new(-25.0, 2.0, 0.0),
        rotation: tank_rotation,
        scale: Vec3::ONE,
    };
    cmds.spawn(UnitType::TankGen2.build(transform, &my_models));

    // GEN II: And one more tank with the rotation.
    let transform = Transform {
        translation: Vec3::new(0.0, 2.0, 0.0),
        rotation: tank_rotation,
        scale: Vec3::ONE,
    };
    cmds.spawn(UnitType::TankGen2.build(transform, &my_models));
}

pub fn spawn_tanks(
    mut cmds: Commands,
    my_models: Res<MyModels>,
    // audio: Res<bevy_kira_audio::Audio>,
    // my_audio: Res<MyAudio>,
) {
    let initial_pos_left = Vec3::new(-200.0, 0.0, 0.0);
    let initial_pos_right = Vec3::new(200.0, 0.0, 0.0);
    let offset = Vec3::new(30.0, 0.0, 30.0);
    let grid_size = (TANK_COUNT as f32).sqrt().ceil() as usize;

    // Create tank on the left side facing right
    let _create_left_tank = |row: usize, col: usize| {
        let pos = initial_pos_left + Vec3::new(offset.x * row as f32, 2.0, offset.z * col as f32);
        let tank_rotation = Quat::from_rotation_y(-PI * 0.5);
        let mut transform = Transform::from_translation(pos);
        transform.rotation = tank_rotation;
        UnitType::TankGen1.build(transform, &my_models)
    };

    // Create tank on the right side facing left
    let create_right_tank = |row: usize, col: usize| {
        let pos = initial_pos_right + Vec3::new(-offset.x * row as f32, 2.0, offset.z * col as f32);
        let tank_rotation = Quat::from_rotation_y(PI * 0.5);
        let mut transform = Transform::from_translation(pos);
        transform.rotation = tank_rotation;
        UnitType::TankGen1.build(transform, &my_models)
    };

    // Spawn Left Group (facing right)
    let mut count = 0;
    for _row in 0..grid_size {
        for _col in 0..grid_size {
            if count >= TANK_COUNT {
                break;
            }
            cmds.spawn(_create_left_tank(_row, _col));
            count += 1;
        }
    }

    // Spawn Right Group (facing left)
    let mut count = 0;
    for row in 0..grid_size {
        for col in 0..grid_size {
            if count >= TANK_COUNT {
                break;
            }
            cmds.spawn(create_right_tank(row, col));
            count += 1;
        }
    }
}
