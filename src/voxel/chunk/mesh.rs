//! The meshing algorithm used here is derived from
//! https://0fps.net/2012/06/30/meshing-in-a-minecraft-game/

use super::{ChunkData, ChunkIndex, Axis, Side, Face};
use super::data::{CHUNK_SIZE, Voxel};
use super::super::world_slice::WorldSlice;
use super::super::VoxelWorld;

use std::ops::Deref;

use amethyst::assets::{AssetStorage, Loader};
use amethyst::renderer::{Mesh, MeshHandle};
use specs::{
    Entities,
    System,
    ReadStorage,
    WriteStorage,
    MaskedStorage,
    Fetch,
    Component,
    HashMapStorage,
    Join,
    ParJoin,
};
use rayon::prelude::*;
use cgmath::Vector3;

#[derive(Clone, Debug, Default)]
pub struct ChunkQuads {
    // (position relative to side, size, side)
    // quads: Vec<(Vector3<f32>, Vector2<f32>, Side)>,
    quads: Vec<([Vector3<f32>; 4], Side)>,
}

impl Component for ChunkQuads {
    type Storage = HashMapStorage<Self>;
}

/// Meshes chunks from ChunkData into collections.
/// Starts by turning contiguous faces into polygons and then
/// triangulating those into meshes.
pub struct MeshFaceSystem {
    _unused: (),
}

impl Default for MeshFaceSystem {
    fn default() -> Self {
        MeshFaceSystem {
            _unused: (),
        }
    }
}

impl<'a> System<'a> for MeshFaceSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, ChunkIndex>,
        WriteStorage<'a, ChunkData>,
        WriteStorage<'a, ChunkQuads>,
        WriteStorage<'a, MeshHandle>,
        Fetch<'a, Loader>,
        Fetch<'a, AssetStorage<Mesh>>,
        Fetch<'a, VoxelWorld>,
    );

    fn run(
        &mut self,
        (
            entities,
            chunk_indices,
            mut chunk_datas,
            mut chunk_meshes,
            mut mesh_handles,
            loader,
            mesh_storage,
            voxel_world,
        ): Self::SystemData
    ) {
        use amethyst::renderer::{MeshData, PosNormTex};

        // create the world slice handle

        // {
        //     let chunk_data = &chunk_datas;
        //     let world = &voxel_world;
        //     ((&chunk_datas).open().1.open().0, &mut chunk_meshes)
        //         .par_join()
        //         .for_each(|(_, data, quads)| {
        //             let world_slice = WorldSlice::new(
        //                 chunk_data,
        //                 world,
        //                 (0, 0, 0),
        //             );
        //             quads_from_data(data, quads)
        //         });
        // }
        {
            let chunk_datas = &chunk_datas;
            let world = &voxel_world;
            ((chunk_datas).open().1.open().0, &mut chunk_meshes, &chunk_indices)
                .par_join()
                .for_each(|(_, chunk_mesh, index)| {
                    let world_slice = WorldSlice::new(
                        chunk_datas,
                        world,
                        (*index).into(),
                    );
                    quads_from_data(&world_slice, chunk_mesh)
                });
        }
        
        // load the meshes into the asset registry
        for (entity, chunk_quads, _) in (&*entities, &chunk_meshes, (&chunk_datas).open().1.open().0).join() {
            // No reusing indices, I suppose.
            use genmesh::*;

            if chunk_quads.quads.is_empty() {
                // no mesh! it's all air
                mesh_handles.remove(entity);
                continue;
            }

            let verts: Vec<PosNormTex> = chunk_quads.quads.iter()
                .map(|q| {
                    let normal: [f32; 3] = match q.1 {
                        Side::Bottom => [0., -1., 0.],
                        Side::Top => [0., 1., 0.],
                        Side::East => [1., 0., 0.],
                        Side::West => [-1., 0., 0.],
                        Side::North => [0., 0., 1.],
                        Side::South => [0., 0., -1.],
                    };
                    // let tangent: [f32; 3] = match q.1 {
                    //     Side::Bottom => [-1., 0., 0.],
                    //     Side::Top => [1., 0., 0.],
                    //     Side::East => [0., 1., 0.],
                    //     Side::West => [0., -1., 0.],
                    //     Side::North => [0., 1., 0.],
                    //     Side::South => [0., -1., 0.],
                    // };

                    Quad::new(
                        PosNormTex {
                            position: q.0[0].into(),
                            normal,
                            // tangent,
                            tex_coord: [0., 0.],
                        },
                        PosNormTex {
                            position: q.0[1].into(),
                            normal,
                            // tangent,
                            tex_coord: [0., 1.],
                        },
                        PosNormTex {
                            position: q.0[2].into(),
                            normal,
                            // tangent,
                            tex_coord: [1., 1.],
                        },
                        PosNormTex {
                            position: q.0[3].into(),
                            normal,
                            // tangent,
                            tex_coord: [1., 0.],
                        },
                    )
                })
                .triangulate()
                .vertices()
                .collect();

            let mesh_handle = loader.load_from_data(
                MeshData::PosNormTex(verts),
                (),
                &*mesh_storage
            );

            mesh_handles.insert(entity, mesh_handle);
        }

        // all meshed up, unset dirty bits
        (&mut chunk_datas).open().1.clear_flags();
    }
}

static FACES: &'static [Face] = &[Face::Front, Face::Back];
static AXES: &'static [Axis] = &[Axis::X, Axis::Y, Axis::Z];

/// Given an axis, row, column and depth, give the x, y, z index into the chunk array.
#[inline(always)]
fn get_rcd_xyz<N>(axis: Axis, row: N, col: N, depth: N) -> (N, N, N) {
    use self::Axis::*;

    match axis {
        X => (depth, row, col),
        Y => (col, depth, row),
        Z => (col, row, depth),
    }
}

#[inline(always)]
fn get_rcd_xyz_array<N>(axis: Axis, row: N, col: N, depth: N) -> [N; 3] {
    use self::Axis::*;

    match axis {
        X => [depth, row, col],
        Y => [col, depth, row],
        Z => [col, row, depth],
    }
}

fn quads_from_data<'a, 'b: 'a, T>(data: &WorldSlice<'a, 'b, T>, mesh: &mut ChunkQuads)
where T: Deref<Target=MaskedStorage<ChunkData>>
{
    mesh.quads.clear();

    for face in FACES.into_iter() {
        for axis in AXES.into_iter() {
            let side: Side = (*axis, *face).into();

            for depth in (-1)..(CHUNK_SIZE as isize) {
                let mut slice: [[Option<Voxel>; CHUNK_SIZE]; CHUNK_SIZE] = [[None; CHUNK_SIZE]; CHUNK_SIZE];

                // set the culled slice
                for r in 0..CHUNK_SIZE {
                    for c in 0..CHUNK_SIZE {
                        // let face_1 = if depth >= 0 && depth < CHUNK_SIZE as isize {
                        //     Some(data.get_voxel_face(get_rcd_xyz(*axis, r as i32, c as i32, depth as usize), side))
                        // } else {
                        //     None
                        // };
                        let face_1 = data.get_voxel_face(get_rcd_xyz(*axis, r as i32, c as i32, depth as i32), side);


                        // let face_2 = if depth >= -1 && depth < CHUNK_SIZE as isize - 1 {
                        //     Some(data.get_voxel_face(get_rcd_xyz(*axis, r, c, (depth + 1) as usize), side))
                        // } else {
                        //     None
                        // };
                        let face_2 = data.get_voxel_face(get_rcd_xyz(*axis, r as i32, c as i32, (depth + 1) as i32), side);

                        slice[r][c] = if face_1.is_some() && face_2.is_some() && face_1 == face_2 {
                            None
                        } else {
                            if *face == Face::Back { face_2 } else { face_1 }
                        };
                    }
                }

                // the part where we use the slice to produce quads
                for r in 0..CHUNK_SIZE {
                    let mut c = 0;
                    while c < CHUNK_SIZE {
                        if slice[r][c].is_some() {
                            let starting_voxel = slice[r][c];

                            // Find the span on the row (at least 1)
                            let width: usize = slice[r]
                                .iter()
                                .enumerate()
                                .skip(1 + c)
                                .skip_while(|&(_, voxel)| { *voxel == starting_voxel })
                                .next()
                                .map(|(w, _)| { w })
                                .unwrap_or(CHUNK_SIZE) - c;
                            
                            // How far down does this span go? (It's at least 1)
                            let height: usize = slice
                                .iter()
                                .enumerate()
                                .skip(1 + r)
                                .skip_while(|&(_, row)| {
                                    for c2 in (c)..(c + width) {
                                        if row[c2] != starting_voxel {
                                            return false
                                        }
                                    }
                                    true
                                })
                                .next()
                                .map(|(h, _)| { h })
                                .unwrap_or(CHUNK_SIZE) - r;
                            
                            // Make a quad
                            if is_opaque(starting_voxel.unwrap()) {
                                let mut verts: [Vector3<f32>; 4] = 
                                    [
                                        get_rcd_xyz_array(*axis, (r) as f32, (c) as f32, depth as f32 + 1.).into(),
                                        get_rcd_xyz_array(*axis, (r) as f32, (c + width) as f32, depth as f32 + 1.).into(),
                                        get_rcd_xyz_array(*axis, (r + height) as f32, (c + width) as f32, depth as f32 + 1.).into(),
                                        get_rcd_xyz_array(*axis, (r + height) as f32, (c) as f32, depth as f32 + 1.).into(),
                                    ];
                                match side {
                                    Side::East | Side::Top | Side::South => {
                                        verts.reverse();
                                    },
                                    _ => {}
                                }
                                
                                mesh.quads.push((
                                    verts,
                                    side
                                ));
                            }

                            // clear out the mask for the range
                            for w in 0..(width) { for h in 0..(height) { slice[r + h][c + w] = None; } }

                            // Increment c by the width we jumped
                            c += width;
                            debug_assert!(c <= CHUNK_SIZE); // should not have created a quad past the chunk size
                        } else {
                            c += 1;
                        }
                    }
                }
            }
        }
    }
}

#[inline(always)]
fn is_opaque(v: Voxel) -> bool {
    v != 0
}
