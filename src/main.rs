use amethyst::{
    core::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

mod pong;
mod systems;

fn main() -> amethyst::Result<()> {
    // logger setup
    amethyst::start_logger(Default::default());

    // paths
    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets");

    let display_config_path = app_root.join("config").join("display.ron");
    let key_bindings_path = app_root.join("config").join("key_bindings.ron");

    // input handler: parameter type determines how the axes/actions are read -> here strings are used!
    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(key_bindings_path)?;

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
        .with_bundle(TransformBundle::new())? // bundle for tracking entities positions
        .with_bundle(input_bundle)? // bundle for reading inputs
        .with(systems::PaddleSystem, "paddle_system", &["input_system"]); // system (not bundle) -> input sys must run b4!

    // game application
    let mut game = Application::new(assets_dir, pong::Pong, game_data)?;

    game.run();

    Ok(())
}
