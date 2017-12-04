use system::ConstantRotation;

use amethyst::State;
use amethyst::core::{LocalTransform, Transform, Parent};
use amethyst::renderer::{Camera, DirectionalLight, Light, Rgba};
use amethyst::core::orientation::Orientation;
use specs::World;
use cgmath::{vec3, Deg};
use cgmath::prelude::*;

use voxel::{ChunkIndex, ChunkData, ChunkQuads};

/// Initial state
pub struct PhantomInit;

impl State for PhantomInit {
    fn on_start(&mut self, world: &mut World) {
        let mut chunk_data = ChunkData::default();
        for x in 0..16 {
            for y in 0..1 {
                for z in 0..16 {
                    chunk_data.set_voxel((x, y, z), 1);
                }
            }
        }
        for x in 4..12 {
            for z in 4..8 {
                chunk_data.set_voxel((x, 1, z), 1);
            }
        }
        for x in 6..10 {
            for z in 6..10 {
                chunk_data.set_voxel((x, 2, z), 1);
            }
        }
        for x in 0..16 {
            chunk_data.set_voxel((0, x, 0), 1);
        }
        for x in 0..16 {
            chunk_data.set_voxel((15, x, 15), 1);
        }
        for x in 0..16 {
            chunk_data.set_voxel((0, x, 15), 1);
        }
        for x in 0..16 {
            chunk_data.set_voxel((15, x, 0), 1);
        }

        chunk_data.set_voxel((8, 8, 8), 1);

        world.create_entity()
            .with::<ChunkIndex>((0, 0, 0).into())
            .with(chunk_data)
            .with(LocalTransform::default())
            .with(Transform::default())
            .with(ChunkQuads::default())
            .build();
        
        for x in (-8)..(8) {
            for z in (-8)..(8) {
                world.create_entity()
                    .with::<ChunkIndex>((x, 0, z).into())
                    .with(chunk_data)
                    .with(LocalTransform::default())
                    .with(Transform::default())
                    .with(ChunkQuads::default())
                    .build();
            }
        }

        
        let camera_target = world.create_entity()
            .with({
                LocalTransform::default()
                    .set_position(vec3(8.0, 0.0, 8.0))
                    .clone()
            })
            .with(Transform::default())
            .with(ConstantRotation)
            .build();

        world.create_entity()
            .with({
                LocalTransform::default()
                    .set_position([0.0, 15.0, 15.0].into())
                    // .pitch_local(&Orientation::default(), Deg(30.))
                    .roll_local(&Orientation::default(), Deg(-30.))
                    .clone()
            })
            .with(Transform::default())
            .with(Parent { entity: camera_target })
            .with::<Camera>(Camera::standard_3d(16., 9.))
            .build();
        
        // add a directional light for clarity

        world.create_entity()
            .with({
                LocalTransform::default()
            })
            .with(Transform::default())
            .with(Light::Directional(DirectionalLight {
                color: Rgba::white(),
                direction: vec3(-1.0, -1.0, -1.1).normalize().into(),
                ..Default::default()
            }))
            .build();
    }
}
