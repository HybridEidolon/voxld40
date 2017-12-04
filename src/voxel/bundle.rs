use amethyst::core::bundle::ECSBundle;
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

impl<'a, 'b> ECSBundle<'a, 'b> for VoxelBundle {
    fn build(
        self,
        world: &mut World,
        dispatcher: DispatcherBuilder<'a, 'b>,
    ) -> ::amethyst::core::Result<DispatcherBuilder<'a, 'b>> {
        world.register::<ChunkData>();
        world.register::<ChunkIndex>();
        world.register::<ChunkQuads>();
        world.add_resource(VoxelWorld::new());

        Ok(
            dispatcher
                .add(Bookkeeper, "voxel_world_bookkeeper", &[])
                .add(ChunkIndexPositionSystem, "chunk_index_position_system", &[])
                .add(MeshFaceSystem::default(), "chunk_mesh_face_system", &[])
                .add(ChunkMaterialSystem::default(), "chunk_material_system", &[])
        )
    }
}
