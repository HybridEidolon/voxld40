use amethyst::utils::fps_counter::FpsCounter;
use specs::{
    System,
};
use shred::{Fetch, ReadExpect};

pub struct LogFps;

impl<'a> System<'a> for LogFps {
    type SystemData = ReadExpect<'a, FpsCounter>;

    fn run(&mut self, time: Self::SystemData) {
        println!("{}", time.sampled_fps());
    }
}
