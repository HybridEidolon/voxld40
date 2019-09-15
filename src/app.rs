use system::ConstantRotation;

use std::collections::HashMap;

use amethyst::{SimpleState, StateData, GameData};
use amethyst::core::{Transform, Parent};
//use amethyst::renderer::palette;
use amethyst::renderer::light::{DirectionalLight, PointLight, Light};
use amethyst::renderer::camera::{Camera, Projection};
use amethyst::renderer::palette::rgb::Srgb;
use amethyst::utils::auto_fov::AutoFov;
//use amethyst::core::orientation::Orientation;
use specs::{World, Builder};
//use cgmath::{vec3, Deg};
//use cgmath::prelude::*;
use noise::{
    NoiseModule,
    Add,
    Multiply,
    Constant,
    Perlin,
    ScalePoint,
    Seedable,
};
use rayon::prelude::*;
use rand;

use voxel::{ChunkIndex, ChunkData, ChunkQuads, VoxelWorld};

/// Initial state
pub struct PhantomInit;

impl SimpleState for PhantomInit {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        // make some noise...
        let noise = Multiply::new(
            Constant::new(8.),
            Add::new(
                Constant::new(0.5),
                Multiply::new(
                    Constant::new(0.5),
                    ScalePoint::new(Perlin::new().set_seed(rand::random())).set_scale(0.5),
                )
            )
        );

        data.world.add_resource(VoxelWorld::new());

        const SIZE_X: i32 = 12;
        const SIZE_Z: i32 = 12;

        let datas: HashMap<i32, HashMap<i32, ChunkData>> = ((-SIZE_X)..(SIZE_X)).into_par_iter().map(|chunk_x| {
            (chunk_x, ((-SIZE_Z)..(SIZE_Z)).into_par_iter().map(|chunk_z| {
                let mut chunk_data = ChunkData::default();
                for x in 0..16 {
                    for z in 0..16 {
                        let height = (noise.get([
                            ((x as i32 + chunk_x*16) as f32)/16.,
                            ((z as i32 + chunk_z*16) as f32)/16.,
                        ])).round() as i64;
                        if height < 0 {
                            panic!("height < 0");
                        }
                        if height >= 16 {
                            println!("{}", height);
                            panic!();
                        }
                        for y in 0..height {
                            chunk_data.set_voxel((x, y as usize, z), 1);
                        }
                    }
                }
                (chunk_z, chunk_data)
            }).collect::<HashMap<_, _>>())
        }).collect();

        for x in (-SIZE_X)..(SIZE_X) {
            for z in (-SIZE_Z)..(SIZE_Z) {
                data.world.create_entity()
                    .with::<ChunkIndex>((x, 0, z).into())
                    .with(datas[&x][&z])
                    .with(Transform::default())
                    .with(ChunkQuads::default())
                    .build();
            }
        }

//         let mut chunk_data = ChunkData::default();
//         for x in 0..16 {
//             for y in 0..1 {
//                 for z in 0..16 {
//                     chunk_data.set_voxel((x, y, z), 1);
//                 }
//             }
//         }
//         for x in 4..12 {
//             for z in 4..8 {
//                 chunk_data.set_voxel((x, 1, z), 1);
//             }
//         }
//         for x in 6..10 {
//             for z in 6..10 {
//                 chunk_data.set_voxel((x, 2, z), 1);
//             }
//         }
//         for x in 0..16 {
//             chunk_data.set_voxel((0, x, 0), 1);
//         }
//         for x in 0..16 {
//             chunk_data.set_voxel((15, x, 15), 1);
//         }
//         for x in 0..16 {
//             chunk_data.set_voxel((0, x, 15), 1);
//         }
//         for x in 0..16 {
//             chunk_data.set_voxel((15, x, 0), 1);
//         }
//
//         chunk_data.set_voxel((8, 8, 8), 1);
//
//         data.world.create_entity()
//             .with::<ChunkIndex>((0, 0, 0).into())
//             .with(chunk_data)
//             .with(Transform::default())
//             .with(ChunkQuads::default())
//             .build();
//
//         for x in (-8)..(8) {
//             for z in (-8)..(8) {
//                 data.world.create_entity()
//                     .with::<ChunkIndex>((x, 0, z).into())
//                     .with(chunk_data)
//                     .with(Transform::default())
//                     .with(ChunkQuads::default())
//                     .build();
//             }
//         }

        
        let camera_target = data.world.create_entity()
            .with({
                Transform::default()
                    .set_translation_xyz(8., 8., 0.)
                    .clone()
            })
            .with(ConstantRotation)
            .build();

        data.world.create_entity()
            .with({
                Transform::default()
                    .set_translation_xyz(0.0, 20.0, 15.0)
                    // .pitch_local(&Orientation::default(), Deg(30.))
                    .prepend_rotation_x_axis(-std::f32::consts::FRAC_PI_6)
                    .clone()
            })
            .with(Parent { entity: camera_target })
            .with::<Camera>(Camera::from(Projection::perspective(
                1.0,
                std::f32::consts::FRAC_PI_3,
                0.1,
                1000.0
            )))
            .with({
                let mut f = AutoFov::default();
                f
            })
            .build();

        // add a directional light for clarity

        data.world.create_entity()
            .with(Transform::default())
            .with(Light::Directional(DirectionalLight {
                color: Srgb::new(0.05, 0.05, 0.1),
                direction: [-1.0, -1.0, -1.1].into(),
                ..Default::default()
            }))
            .build();

        data.world.create_entity()
            .with({
                Transform::default()
                    .set_translation_xyz(-4.0, 32.0, -4.0)
                    .clone()
            })
            .with(Light::Point(PointLight {
                color: Srgb::new(1.0, 1.0, 0.0),
                intensity: 300.,
                radius: 8.,
                ..Default::default()
            }))
            .build();
    }
}
