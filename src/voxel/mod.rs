pub mod chunk;
pub mod bundle;
pub mod world_slice;

pub use self::chunk::{
    ChunkIndex,
    ChunkIndexPositionSystem,
    ClearIndexFlagSystem,
    Face,
    Side,
    Axis,
};
pub use self::chunk::data::{
    ChunkData,
    Voxel,
    CHUNK_SIZE,
};
pub use self::chunk::mesh::{
    ChunkQuads,
    MeshFaceSystem,
};
pub use self::chunk::material::ChunkMaterialSystem;
pub use self::bundle::VoxelBundle;
pub use self::world_slice::*;

use fnv::FnvHashMap;

use specs::{
    Entity,
    Entities,
    EntitiesRes,
    System,
    ReadStorage,
    FetchMut,
    Join,
};

/// A resource that keeps track of the entities representing chunks.
pub struct VoxelWorld {
    index_entity: FnvHashMap<(i32, i32, i32), Entity>,
}

impl VoxelWorld {
    pub fn new() -> Self {
        VoxelWorld {
            index_entity: FnvHashMap::default(),
        }
    }

    fn insert(&mut self, index: (i32, i32, i32), entity: Entity) {
        self.index_entity.insert(index, entity);
    }

    #[allow(unused)]
    pub fn get_entity(&self, index: (i32, i32, i32)) -> Option<Entity> {
        self.index_entity.get(&index).map(|e| *e)
    }

    fn clear_dead(&mut self, entities: &EntitiesRes) {
        self.index_entity.retain(|_, e| {
            entities.is_alive(*e)
        });
    }
}

/// Updates VoxelWorld's data to represent the current scene
pub struct Bookkeeper;

impl<'a> System<'a> for Bookkeeper {
    type SystemData = (
        FetchMut<'a, VoxelWorld>,
        Entities<'a>,
        ReadStorage<'a, ChunkIndex>,
    );

    fn run(&mut self, (mut voxel_world, entities, chunk_indices): Self::SystemData) {
        voxel_world.clear_dead(&entities);
        for (entity, chunk_index) in (&*entities, &chunk_indices).join() {
            voxel_world.insert((chunk_index.x, chunk_index.y, chunk_index.z), entity);
        }
    }
}
