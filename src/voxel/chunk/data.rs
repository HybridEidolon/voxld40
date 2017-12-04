use super::{Axis, Side};

use specs::{
    Component,
    FlaggedStorage,
    HashMapStorage,
};

pub type Voxel = u8;

// provides for ChunkData to be exactly 1 page (4096 bytes)
pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_SIZE_FLOAT: f32 = CHUNK_SIZE as f32;

#[derive(Clone, Copy, Debug, Default)]
pub struct ChunkData {
    pub data: [[[Voxel; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
}

#[allow(unused)]
fn in_range<V: PartialOrd>(low: V, high: V, value: V) -> bool {
    value >= low && value < high
}

impl ChunkData {
    #[inline]
    pub fn get_voxel(&self, index: (usize, usize, usize)) -> Voxel {
        debug_assert!(in_range(0, 16, index.0));
        debug_assert!(in_range(0, 16, index.1));
        debug_assert!(in_range(0, 16, index.2));
        self.data[index.2][index.1][index.0]
    }

    #[inline]
    pub fn get_voxel_face(&self, index: (usize, usize, usize), _side: Side) -> Voxel {
        // TODO change this to allow for different faces on each side of a voxel...
        self.get_voxel(index)
    }

    #[inline]
    pub fn set_voxel(&mut self, index: (usize, usize, usize), value: Voxel) {
        debug_assert!(in_range(0, 16, index.0));
        debug_assert!(in_range(0, 16, index.1));
        debug_assert!(in_range(0, 16, index.2));
        self.data[index.2][index.1][index.0] = value;
    }

    /// Get an iterator over a cross section of the chunk's data.
    #[allow(unused)]
    pub fn cross_section<'s>(&'s self, axis: Axis, depth: usize) -> CrossSectionIter<'s> {
        debug_assert!(in_range(0, 16, depth));
        CrossSectionIter {
            data: &self.data,
            axis: axis,
            depth: depth,
            row: 0,
        }
    }
}

pub struct CrossSectionIter<'a> {
    data: &'a [[[u8; 16]; 16]; 16],
    axis: Axis,
    depth: usize,
    row: usize,
}

pub struct RowIter<'a> {
    data: &'a [[[u8; 16]; 16]; 16],
    axis: Axis,
    depth: usize,
    row: usize,
    column: usize,
}

impl<'a> Iterator for CrossSectionIter<'a> {
    type Item = RowIter<'a>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= 16 {
            return None;
        }

        self.row += 1;

        Some(RowIter {
            data: self.data,
            axis: self.axis,
            depth: self.depth,
            row: self.row - 1,
            column: 0,
        })
    }
}

impl<'a> Iterator for RowIter<'a> {
    type Item = &'a Voxel;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.column >= 16 {
            return None;
        }

        self.column += 1;

        match self.axis {
            Axis::X => Some(&self.data[self.depth][self.row][self.column - 1]),
            Axis::Y => Some(&self.data[self.row][self.depth][self.column - 1]),
            Axis::Z => Some(&self.data[self.row][self.column - 1][self.depth]),
        }
    }
}

impl Component for ChunkData {
    // the flag is used to reconstruct the mesh
    type Storage = FlaggedStorage<Self, HashMapStorage<Self>>;
}
