extern crate itertools;
// extern crate shred;
// extern crate specs;
// extern crate nalgebra;
extern crate rayon;
extern crate amethyst;
extern crate genmesh;
extern crate hibitset;
extern crate noise;
extern crate fnv;

pub use amethyst::shred as shred;
pub use amethyst::shrev as shrev;
pub use amethyst::ecs as specs;
pub use amethyst::core::cgmath as cgmath;

mod voxel;
mod app;
mod system;
mod log_fps;

use std::time::Duration;

use amethyst::Application;
use amethyst::core::frame_limiter::FrameRateLimitStrategy;
use amethyst::renderer::{RenderSystem, Pipeline, Stage, DisplayConfig, DrawShaded, PosNormTex};

fn get_display_config() -> std::io::Result<DisplayConfig> {
    Ok(DisplayConfig {
        title: "voxld".to_string(),
        fullscreen: false,
        dimensions: Some((960, 540)),
        vsync: false,
        multisampling: 1,
        visibility: true,
        ..Default::default()
    })
}

fn run() -> amethyst::Result<()> {
    let display_config = get_display_config().unwrap();

    let game = Application::build("", app::PhantomInit)?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            300,
        )
        // .with_frame_limit(
        //     FrameRateLimitStrategy::Unlimited,
        //     0,
        // )
        .with_bundle(voxel::VoxelBundle)?
        .with_bundle(amethyst::core::transform::TransformBundle::new())?
        .with_bundle(amethyst::utils::fps_counter::FPSCounterBundle::default())?
        .with(system::IntervalSystem::wrap(log_fps::LogFps, Duration::from_secs(1)), "debug_log_fps", &[])
        .register::<system::ConstantRotation>()
        .with(system::ConstantRotationSystem::default(), "constant_rotation_system", &[])
        .with_bundle(amethyst::renderer::RenderBundle::new())?;
    
    let pipe = {
        Pipeline::build().with_stage(
            Stage::with_backbuffer()
                .clear_target([0.01, 0.02, 0.04, 1.0], 1.0)
                .with_pass(DrawShaded::<PosNormTex>::default())
        )
    };

    Ok(
        game
            .with_local(RenderSystem::build(pipe, Some(display_config))?)
            .build()?
            .run()
    )
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Fatal error: {}\n\n {:?}", e, e);
        ::std::process::exit(1);
    }
}
