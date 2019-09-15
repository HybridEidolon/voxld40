pub mod data;
pub mod mesh;
pub mod material;

use self::data::ChunkData;

use specs::{
    Component,
    FlaggedStorage,
    HashMapStorage,
    System,
    ReadStorage,
    WriteStorage,
    Join,
};
use amethyst::core::Transform;
use cgmath::Vector3;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ChunkIndex {
    pub x: i32,
    pub y: i32,
    pub z: i32
}

impl Component for ChunkIndex {
    type Storage = FlaggedStorage<Self, HashMapStorage<Self>>;
}

impl From<(i32, i32, i32)> for ChunkIndex {
    fn from(value: (i32, i32, i32)) -> ChunkIndex {
        ChunkIndex {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}

impl Into<(i32, i32, i32)> for ChunkIndex {
    fn into(self) -> (i32, i32, i32) {
        (self.x, self.y, self.z)
    }
}

pub struct ChunkIndexPositionSystem;

impl<'a> System<'a> for ChunkIndexPositionSystem {
    type SystemData = (
        ReadStorage<'a, ChunkIndex>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, (chunk_idxs, mut locals): Self::SystemData) {
        for (chunk_index, mut local) in (&chunk_idxs, &mut locals).join() {
            local.set_translation(Vector3::new(
                data::CHUNK_SIZE_FLOAT * chunk_index.x as f32,
                data::CHUNK_SIZE_FLOAT * chunk_index.y as f32,
                data::CHUNK_SIZE_FLOAT * chunk_index.z as f32,
            ));
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Face {
    Front,
    Back,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    North,
    South,
    East,
    West,
    Top,
    Bottom,
}

impl From<(Axis, Face)> for Side {
    fn from(coord: (Axis, Face)) -> Side {
        use self::Axis::*;
        use self::Side::*;
        use self::Face::*;

        match coord {
            (X, Front) => East,
            (X, Back) => West,
            (Y, Front) => Top,
            (Y, Back) => Bottom,
            (Z, Front) => North,
            (Z, Back) => South,
        }
    }
}

impl From<(Face, Axis)> for Side {
    fn from((face, axis): (Face, Axis)) -> Side {
        From::from((axis, face))
    }
}

impl From<Side> for (Axis, Face) {
    fn from(side: Side) -> (Axis, Face) {
        use self::Axis::*;
        use self::Side::*;
        use self::Face::*;

        match side {
            East => (X, Front),
            West => (X, Back),
            Top => (Y, Front),
            Bottom => (Y, Back),
            North => (Z, Front),
            South => (Z, Back),
        }
    }
}

//#[allow(unused)]
//pub struct ClearIndexFlagSystem;
//
//impl<'a> System<'a> for ClearIndexFlagSystem {
//    type SystemData = WriteStorage<'a, ChunkIndex>;
//
//    fn run(&mut self, mut chunk_indices: Self::SystemData) {
//        (&mut chunk_indices).open().1.clear_flags();
//    }
//}
