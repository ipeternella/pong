use amethyst::{
    core::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

mod pong;

fn main() -> amethyst::Result<()> {
    // logger setup
    amethyst::start_logger(Default::default());

    // config paths
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");

    let assets_dir = app_root.join("assets");

    // repo with game logic
    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?; // bundle for tracking entities positions

    // game application
    let mut game = Application::new(assets_dir, pong::Pong, game_data)?;

    game.run();

    Ok(())
}
