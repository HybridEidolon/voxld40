use amethyst::utils::fps_counter::FPSCounter;
use specs::{
    System,
    Fetch,
};

pub struct LogFps;

impl<'a> System<'a> for LogFps {
    type SystemData = Fetch<'a, FPSCounter>;

    fn run(&mut self, time: Self::SystemData) {
        println!("{}", time.sampled_fps());
    }
}
