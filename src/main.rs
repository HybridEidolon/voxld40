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
extern crate rand;

pub use amethyst::shred as shred;
pub use amethyst::shrev as shrev;
pub use amethyst::ecs as specs;
pub use amethyst::core::math as cgmath;

mod voxel;
mod app;
mod system;
mod log_fps;

use std::time::Duration;

use amethyst::{Application, GameDataBuilder};
use amethyst::core::frame_limiter::FrameRateLimitStrategy;
//use amethyst::renderer::{RenderSystem, Pipeline, Stage, DisplayConfig, DrawShaded, PosNormTex};
use amethyst::renderer::{RenderingBundle, plugins::RenderShaded3D, plugins::RenderSkybox, plugins::RenderToWindow, types::DefaultBackend, bundle::Target};
use amethyst::renderer::palette::rgb::Srgb;
use amethyst::window::DisplayConfig;

fn get_display_config() -> std::io::Result<DisplayConfig> {
    Ok(DisplayConfig {
        title: "voxld".to_string(),
        fullscreen: Option::None,
        dimensions: Some((960, 540)),
        visibility: true,
        ..Default::default()
    })
}

fn run() -> amethyst::Result<()> {
    amethyst::start_logger(amethyst::LoggerConfig::default());

    let display_config = get_display_config().unwrap();

    let game_data = GameDataBuilder::default()
        .with_bundle(voxel::VoxelBundle)?
        .with_bundle(amethyst::core::transform::TransformBundle::new())?
        .with_bundle(amethyst::utils::fps_counter::FpsCounterBundle::default())?
        .with(system::IntervalSystem::wrap(log_fps::LogFps, Duration::from_secs(1)), "debug_log_fps", &[])
        .with(system::ConstantRotationSystem::default(), "constant_rotation_system", &[])
        .with(amethyst::utils::auto_fov::AutoFovSystem::default(), "auto_fov", &[])
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(RenderToWindow::from_config(display_config))
                .with_plugin(RenderShaded3D::default())
                .with_plugin(RenderSkybox::with_colors(
                    Srgb::new(0.82, 0.51, 0.50),
                    Srgb::new(0.18, 0.11, 0.85),
                )),
        )?;


    let mut game = Application::build("", app::PhantomInit)?
        .build(game_data)?;

    game.run();
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Fatal error: {}\n\n {:?}", e, e);
        ::std::process::exit(1);
    }
}
