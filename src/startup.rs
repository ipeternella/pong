use std::path::PathBuf;

use amethyst::{
    audio::AudioBundle,
    core::TransformBundle,
    input::{InputBundle, StringBindings},
    renderer::{types::DefaultBackend, RenderFlat2D, RenderToWindow, RenderingBundle},
    start_logger,
    ui::{RenderUi, UiBundle},
    GameDataBuilder,
};

use crate::systems;

/// Builds the game configuration by pluging bundles and systems to it.
pub fn build_game_config(
    app_root: &PathBuf,
) -> amethyst::Result<GameDataBuilder<'static, 'static>> {
    // paths based on app root path
    let display_config_path = app_root.join("src").join("settings").join("display.ron");
    let key_bindings_path = app_root
        .join("src")
        .join("settings")
        .join("key_bindings.ron");

    // input handler: parameter type determines how the axes/actions are read
    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(key_bindings_path)?;

    // game configuration building: use bundles for rendering, for transforms, inputs, plug systems, etc.
    let game_config = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()), // We need UI elements to be rendered: text, scores, etc.
        )?
        .with_bundle(TransformBundle::new())? // bundle for tracking entities positions
        .with_bundle(input_bundle)? // bundle for reading inputs
        .with_bundle(UiBundle::<StringBindings>::new())? // UiBundle MUST match InputHandler type!
        .with_bundle(AudioBundle::default())? // audio and sfx ECS components
        .with(systems::PaddleSystem, "paddle_system", &["input_system"])
        .with(systems::BallSystem, "ball_system", &[])
        .with(systems::ScoreSystem, "score_system", &["ball_system"])
        .with(
            systems::CollisionSystem,
            "collision_system",
            &["paddle_system", "ball_system"],
        );

    Ok(game_config)
}

/// Configures Amethyst's logger.
pub fn setup_logger() {
    // builds loger with default values
    start_logger(Default::default());
}
