use bevy::prelude::*;
use bevy_rts_pathfinding::components::{self as pf_comps};

#[derive(Component)]
pub struct CmdInterfaceCtr;

#[derive(Component)]
pub struct MiniMapCtr;

#[derive(Component)]
pub struct IconsCtr;

#[derive(Component)]
pub struct BuildColumnsCtr;

#[derive(Component)]
#[require(Button)]
pub struct BuildActionBtn;

#[derive(Component, Clone, Copy)]
pub struct Structure(pub StructureType);

#[derive(Clone, Copy)]
pub enum StructureType {
    Turret,
    Barracks,
    VehicleDepot,
    Black,
    White,
}

impl StructureType {
    pub fn to_string(&self) -> &str {
        match self {
            StructureType::Turret => "Turret",
            StructureType::Barracks => "Barracks",
            StructureType::VehicleDepot => "Vehicle Depot",
            StructureType::Black => "Black",
            StructureType::White => "White",
        }
    }
}

impl Structure {
    pub fn build(
        &self,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) -> (
        Mesh3d,
        MeshMaterial3d<StandardMaterial>,
        pf_comps::RtsObjSize,
    ) {
        match self.0 {
            StructureType::Turret => {
                let size = Vec3::new(10.0, 7.5, 10.0);
                (
                    Mesh3d(meshes.add(Cuboid::from_size(size))),
                    MeshMaterial3d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
                    pf_comps::RtsObjSize(size),
                )
            }
            StructureType::Barracks => {
                let size = Vec3::new(25.0, 17.5, 25.0);
                (
                    Mesh3d(meshes.add(Cuboid::from_size(size))),
                    MeshMaterial3d(materials.add(Color::srgb(0.0, 1.0, 0.0))),
                    pf_comps::RtsObjSize(size),
                )
            }
            StructureType::VehicleDepot => {
                let size = Vec3::new(30.0, 25.0, 40.0);
                (
                    Mesh3d(meshes.add(Cuboid::from_size(size))),
                    MeshMaterial3d(materials.add(Color::srgb(0.0, 0.0, 1.0))),
                    pf_comps::RtsObjSize(size),
                )
            }
            StructureType::Black => {
                let size = Vec3::new(25.0, 25.0, 25.0);
                (
                    Mesh3d(meshes.add(Cuboid::from_size(size))),
                    MeshMaterial3d(materials.add(Color::srgb(0.0, 0.0, 0.0))),
                    pf_comps::RtsObjSize(size),
                )
            }
            StructureType::White => {
                let size = Vec3::new(25.0, 25.0, 25.0);
                (
                    Mesh3d(meshes.add(Cuboid::from_size(size))),
                    MeshMaterial3d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
                    pf_comps::RtsObjSize(size),
                )
            }
        }
    }
}

#[derive(Component)]
pub struct Unit;

#[derive(Component)]
pub struct BuildStructurePlaceholder;
