use std::ops::Deref;
use super::{
    ChunkData,
    VoxelWorld,
    Voxel,
    CHUNK_SIZE,
    Side,
};
use specs::{
    Storage,
    Fetch,
    MaskedStorage,
};

pub struct WorldSlice<'a, 'b: 'a, T: 'b> {
    chunk_datas: &'a Storage<'b, ChunkData, T>,
    voxel_world: &'a Fetch<'b, VoxelWorld>,

    /// chunk index representing the origin for this slice
    origin: (i32, i32, i32),

    /// the Moore neighborhood of chunk datas for immediate access
    moore_neighborhood: [[[Option<&'a ChunkData>; 3]; 3]; 3],
}

impl<'a, 'b: 'a, T: 'b> WorldSlice<'a, 'b, T>
where T: Deref<Target=MaskedStorage<ChunkData>> {
    pub fn new(chunk_datas: &'a Storage<'b, ChunkData, T>,
               voxel_world: &'a Fetch<'b, VoxelWorld>,
               origin: (i32, i32, i32)) -> WorldSlice<'a, 'b, T>
    {
        // Produce the Moore neighborhood of the origin chunk
        let mut moore_neighborhood: [[[Option<&'a ChunkData>; 3]; 3]; 3] = [[[None; 3]; 3]; 3];

        let (origin_x, origin_y, origin_z) = origin;

        for x in 0..3 {
            for y in 0..3 {
                for z in 0..3 {
                    let chunk_data = voxel_world.get_entity((origin_x + (x as i32 - 1), origin_y + (y as i32 - 1), origin_z + (z as i32 - 1)))
                        .and_then(|entity| {
                            chunk_datas.get(entity)
                        });
                    moore_neighborhood[x][y][z] = chunk_data;
                }
            }
        }

        WorldSlice {
            chunk_datas,
            voxel_world,
            origin,
            moore_neighborhood,
        }
    }

    #[inline(always)]
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

        let chunk_data: Option<&ChunkData> = if self.chunk_is_in_moore(origin_offset) {
            let (o_x, o_y, o_z) = origin_offset;
            self.moore_neighborhood[(o_x + 1) as usize][(o_y + 1) as usize][(o_z + 1) as usize]
        } else {
            self.voxel_world.get_entity(global_index)
            .and_then(|entity| {
                self.chunk_datas.get(entity)
            })
        };
        chunk_data.map(|&cd| cd.get_voxel(chunk_index))
    }

    #[inline(always)]
    pub fn get_voxel_face(&self, index: (i32, i32, i32), _side: Side) -> Option<Voxel> {
        self.get_voxel(index)
    }

    #[inline(always)]
    fn chunk_is_in_moore(&self, index: (i32, i32, i32)) -> bool {
        let (x, y, z) = index;
        (x >= -1 && x <= 1) && (y >= -1 && y <= 1) && (z >= -1 && z <= 1)
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
