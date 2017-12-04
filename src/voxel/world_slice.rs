use super::{
    ChunkData,
    VoxelWorld,
    Voxel,
    CHUNK_SIZE,
};
use specs::{
    ReadStorage,
    Fetch,
};

pub struct WorldSlice<'a, 'b: 'a> {
    chunk_datas: &'a ReadStorage<'b, ChunkData>,
    voxel_world: &'a Fetch<'b, VoxelWorld>,

    /// chunk index representing the origin for this slice
    origin: (i32, i32, i32),
}

impl<'a, 'b: 'a> WorldSlice<'a, 'b> {
    pub fn new(chunk_datas: &'b ReadStorage<'a, ChunkData>,
               voxel_world: &'b Fetch<'a, VoxelWorld>,
               origin: (i32, i32, i32)) -> WorldSlice<'a, 'b>
    {
        WorldSlice {
            chunk_datas,
            voxel_world,
            origin,
        }
    }

    pub fn get_voxel(&self, index: (i32, i32, i32)) -> Option<Voxel> {
        let origin_offset = (
            adjust_origin_offset(index.0) / CHUNK_SIZE as i32,
            adjust_origin_offset(index.1) / CHUNK_SIZE as i32,
            adjust_origin_offset(index.2) / CHUNK_SIZE as i32,
        );
        let global_index = (origin_offset.0 + self.origin.0, origin_offset.1 + self.origin.1, origin_offset.2 + self.origin.2);
        let chunk_index = (
            adjust(index.0) % CHUNK_SIZE,
            adjust(index.1) % CHUNK_SIZE,
            adjust(index.2) % CHUNK_SIZE,
        );

        self.voxel_world.get_entity(global_index)
        .and_then(|entity| {
            self.chunk_datas.get(entity)
        })
        .map(|&chunk_data| {
            chunk_data.get_voxel(chunk_index)
        })
    }
}

#[inline(always)]
fn adjust(v: i32) -> usize {
    if v < 0 { (v + CHUNK_SIZE as i32) as usize } else { v as usize }
}

#[inline(always)]
fn adjust_origin_offset(v: i32) -> i32 {
    if v < 0 { v - CHUNK_SIZE as i32 } else { v }
}
