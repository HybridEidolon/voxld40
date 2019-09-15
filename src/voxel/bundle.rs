use amethyst::core::bundle::SystemBundle;
use amethyst::Error as AmethystError;
use specs::{
    World,
    DispatcherBuilder,
};

use super::{
    ChunkData,
    ChunkIndex,
    ChunkQuads,
    VoxelWorld,
    ChunkIndexPositionSystem,
    // ClearIndexFlagSystem,
    Bookkeeper,
    MeshFaceSystem,
    ChunkMaterialSystem,
};

pub struct VoxelBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for VoxelBundle {
    fn build(
        self,
        dispatcher: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), AmethystError> {
//        world.register::<ChunkData>();
//        world.register::<ChunkIndex>();
//        world.register::<ChunkQuads>();
        dispatcher.add(Bookkeeper, "voxel_world_bookkeeper", &[]);
        dispatcher.add(ChunkIndexPositionSystem, "chunk_index_position_system", &[]);
        dispatcher.add(MeshFaceSystem::default(), "chunk_mesh_face_system", &[]);
        dispatcher.add(ChunkMaterialSystem::default(), "chunk_material_system", &[]);
        Ok(())
    }
}
