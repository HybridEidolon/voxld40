use specs::{
    Component,
    System,
    HashMapStorage,
    ReadStorage,
    WriteStorage,
    Join,
};
use shred::{Fetch, ReadExpect};
//use cgmath::{vec3, Deg};
//use amethyst::core::math::Vector3;
use amethyst::core::{Transform, Time};

#[derive(Clone, Copy, Debug, Default)]
pub struct ConstantRotation;

impl Component for ConstantRotation {
    type Storage = HashMapStorage<Self>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ConstantRotationSystem;

impl<'a> System<'a> for ConstantRotationSystem {
    type SystemData = (
        ReadExpect<'a, Time>,
        ReadStorage<'a, ConstantRotation>,
        WriteStorage<'a, Transform>,
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
                local_transform.prepend_rotation_y_axis(0.174533 * delta_time);
            });
    }
}
