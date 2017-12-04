use super::{ChunkData, ChunkIndex};

use amethyst::assets::{AssetStorage, Loader};
use amethyst::renderer::{Material, MaterialDefaults, Texture, TextureData, TextureHandle, TextureMetadata};
use specs::{
    System,
    Entities,
    Fetch,
    ReadStorage,
    WriteStorage,
    Join,
};

#[derive(Clone, Debug, Default)]
pub struct ChunkMaterialSystem {
    white_texture: Option<TextureHandle>,
}

impl<'a> System<'a> for ChunkMaterialSystem {
    type SystemData = (
        Entities<'a>,
        Fetch<'a, MaterialDefaults>,
        Fetch<'a, AssetStorage<Texture>>,
        Fetch<'a, Loader>,
        ReadStorage<'a, ChunkData>,
        ReadStorage<'a, ChunkIndex>,
        WriteStorage<'a, Material>,
    );

    fn run(&mut self, (
        entities,
        material_defaults,
        texture_storage,
        loader,
        chunk_datas,
        chunk_indices,
        mut materials,
    ): Self::SystemData) {
        if self.white_texture.is_none() {
            let handle = loader.load_from_data(
                TextureData::Rgba([1., 1., 1., 1.], TextureMetadata::default()),
                (),
                &texture_storage,
            );
            self.white_texture = Some(handle);
        }
        let white_texture = self.white_texture.clone().unwrap();

        for (entity, _, _chunk_index, _) in (&*entities, &chunk_datas, &chunk_indices, &!materials.open().0.clone()).join() {
            materials.insert(entity, Material {
                albedo: white_texture.clone(),
                ..material_defaults.0.clone()
            });
        }
    }
}
