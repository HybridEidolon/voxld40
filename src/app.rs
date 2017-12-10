use system::ConstantRotation;

use std::collections::HashMap;

use amethyst::State;
use amethyst::core::{LocalTransform, Transform, Parent};
use amethyst::renderer::{Camera, DirectionalLight, PointLight, Light, Rgba};
use amethyst::core::orientation::Orientation;
use specs::World;
use cgmath::{vec3, Deg};
use cgmath::prelude::*;
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

use voxel::{ChunkIndex, ChunkData, ChunkQuads};

/// Initial state
pub struct PhantomInit;

impl State for PhantomInit {
    fn on_start(&mut self, world: &mut World) {
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

        const SIZE_X: i32 = 8;
        const SIZE_Z: i32 = 8;

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
                world.create_entity()
                    .with::<ChunkIndex>((x, 0, z).into())
                    .with(datas[&x][&z])
                    .with(LocalTransform::default())
                    .with(Transform::default())
                    .with(ChunkQuads::default())
                    .build();
            }
        }

        // let mut chunk_data = ChunkData::default();
        // for x in 0..16 {
        //     for y in 0..1 {
        //         for z in 0..16 {
        //             chunk_data.set_voxel((x, y, z), 1);
        //         }
        //     }
        // }
        // for x in 4..12 {
        //     for z in 4..8 {
        //         chunk_data.set_voxel((x, 1, z), 1);
        //     }
        // }
        // for x in 6..10 {
        //     for z in 6..10 {
        //         chunk_data.set_voxel((x, 2, z), 1);
        //     }
        // }
        // for x in 0..16 {
        //     chunk_data.set_voxel((0, x, 0), 1);
        // }
        // for x in 0..16 {
        //     chunk_data.set_voxel((15, x, 15), 1);
        // }
        // for x in 0..16 {
        //     chunk_data.set_voxel((0, x, 15), 1);
        // }
        // for x in 0..16 {
        //     chunk_data.set_voxel((15, x, 0), 1);
        // }

        // chunk_data.set_voxel((8, 8, 8), 1);

        // world.create_entity()
        //     .with::<ChunkIndex>((0, 0, 0).into())
        //     .with(chunk_data)
        //     .with(LocalTransform::default())
        //     .with(Transform::default())
        //     .with(ChunkQuads::default())
        //     .build();
        
        // for x in (-8)..(8) {
        //     for z in (-8)..(8) {
        //         world.create_entity()
        //             .with::<ChunkIndex>((x, 0, z).into())
        //             .with(chunk_data)
        //             .with(LocalTransform::default())
        //             .with(Transform::default())
        //             .with(ChunkQuads::default())
        //             .build();
        //     }
        // }

        
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
                color: Rgba(0.05, 0.05, 0.1, 1.0),
                direction: vec3(-1.0, -1.0, -1.1).normalize().into(),
                ..Default::default()
            }))
            .build();
        
        world.create_entity()
            .with({
                LocalTransform::default()
            })
            .with(Transform::default())
            .with(Light::Point(PointLight {
                center: [-4.0, 32.0, -4.0],
                color: Rgba(1.0, 1.0, 0.0, 1.0),
                intensity: 300.,
                radius: 8.,
                ..Default::default()
            }))
            .build();
    }
}
