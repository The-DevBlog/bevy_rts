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
    Red,
    Green,
    Blue,
    Black,
    White,
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
            StructureType::Red => {
                let size = Vec3::new(25.0, 25.0, 25.0);
                (
                    Mesh3d(meshes.add(Cuboid::from_size(size))),
                    MeshMaterial3d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
                    pf_comps::RtsObjSize(size.xz()),
                )
            }
            StructureType::Green => {
                let size = Vec3::new(25.0, 25.0, 25.0);
                (
                    Mesh3d(meshes.add(Cuboid::from_size(size))),
                    MeshMaterial3d(materials.add(Color::srgb(0.0, 1.0, 0.0))),
                    pf_comps::RtsObjSize(size.xz()),
                )
            }
            StructureType::Blue => {
                let size = Vec3::new(25.0, 25.0, 25.0);
                (
                    Mesh3d(meshes.add(Cuboid::from_size(size))),
                    MeshMaterial3d(materials.add(Color::srgb(0.0, 0.0, 1.0))),
                    pf_comps::RtsObjSize(size.xz()),
                )
            }
            StructureType::Black => {
                let size = Vec3::new(25.0, 25.0, 25.0);
                (
                    Mesh3d(meshes.add(Cuboid::from_size(size))),
                    MeshMaterial3d(materials.add(Color::srgb(0.0, 0.0, 0.0))),
                    pf_comps::RtsObjSize(size.xz()),
                )
            }
            StructureType::White => {
                let size = Vec3::new(25.0, 25.0, 25.0);
                (
                    Mesh3d(meshes.add(Cuboid::from_size(size))),
                    MeshMaterial3d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
                    pf_comps::RtsObjSize(size.xz()),
                )
            }
        }
    }
}

#[derive(Component)]
pub struct Unit;

#[derive(Component)]
pub struct BuildStructurePlaceholder;
