use super::{ChunkData, ChunkIndex};

use std::borrow::Cow;

use amethyst::assets::{AssetStorage, Loader, Handle};
use amethyst::renderer::{Material, MaterialDefaults, Texture};
use amethyst::renderer::types::TextureData;
use amethyst::renderer::rendy::texture::{TextureBuilder};
use amethyst::renderer::rendy::texture::image::TextureKind;
use amethyst::renderer::rendy::texture::pixel::Rgba8Uint;
use amethyst::renderer::loaders::load_from_srgb;
use amethyst::renderer::palette::rgb::Srgb;
use specs::{
    System,
    Entities,
    ReadStorage,
    WriteStorage,
    Join,
};
use shred::{Fetch, ReadExpect};

#[derive(Clone, Debug, Default)]
pub struct ChunkMaterialSystem {
    white_texture: Option<Handle<Texture>>,
    white_material: Option<Handle<Material>>,
}

impl<'a> System<'a> for ChunkMaterialSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, MaterialDefaults>,
        ReadExpect<'a, AssetStorage<Texture>>,
        ReadExpect<'a, AssetStorage<Material>>,
        ReadExpect<'a, Loader>,
        ReadStorage<'a, ChunkData>,
        ReadStorage<'a, ChunkIndex>,
        WriteStorage<'a, Handle<Material>>,
    );

    fn run(&mut self, (
        entities,
        material_defaults,
        texture_storage,
        material_storage,
        loader,
        chunk_datas,
        chunk_indices,
        mut materials,
    ): Self::SystemData) {
        if self.white_texture.is_none() {
            let handle = loader.load_from_data(
                TextureData(
                    load_from_srgb(Srgb::new(1., 1., 1.))
                ),
                (),
                &texture_storage,
            );
            self.white_texture = Some(handle);
        }
        let white_texture = self.white_texture.clone().unwrap();
        if self.white_material.is_none() {
            let handle = loader.load_from_data(
                Material {
                    albedo: white_texture.clone(),
                    ..material_defaults.0.clone()
                },
                (),
                &material_storage
            );
            self.white_material = Some(handle);
        }
        let white_material = self.white_material.clone().unwrap();

        for (entity, _, _chunk_index, _) in (&*entities, &chunk_datas, &chunk_indices, !materials.mask().clone()).join() {
            materials.insert(entity, white_material.clone());
        }
    }
}
