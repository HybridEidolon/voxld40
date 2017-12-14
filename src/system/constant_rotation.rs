use specs::{
    Component,
    System,
    HashMapStorage,
    ReadStorage,
    WriteStorage,
    Fetch,
    Join,
};
use cgmath::{vec3, Deg};
use amethyst::core::{LocalTransform, Time};

#[derive(Clone, Copy, Debug, Default)]
pub struct ConstantRotation;

impl Component for ConstantRotation {
    type Storage = HashMapStorage<Self>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ConstantRotationSystem;

impl<'a> System<'a> for ConstantRotationSystem {
    type SystemData = (
        Fetch<'a, Time>,
        ReadStorage<'a, ConstantRotation>,
        WriteStorage<'a, LocalTransform>,
    );

    fn run(&mut self, (
        time,
        constant_rotations,
        mut local_transforms,
    ): Self::SystemData) {
        let delta_time = time.delta_seconds();

        (&constant_rotations, &mut local_transforms)
        .join()
        .for_each(|(_constant_rotation, local_transform)| {
            local_transform.rotate_local(vec3(0., 1., 0.), Deg(10. * delta_time));
        });
    }
}
