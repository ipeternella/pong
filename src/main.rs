mod entities;
mod settings;
mod sprite_sheet;
mod startup;
mod state;
mod systems;

use amethyst::{prelude::*, utils::application_root_dir};
use startup::{build_game_config, setup_logger};

fn main() -> amethyst::Result<()> {
    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets");
    let initial_state = state::Pong::default();

    setup_logger();

    let game_config = build_game_config(&app_root)?;
    let mut game = Application::new(assets_dir, initial_state, game_config)?;

    // main game loop
    game.run();

    Ok(())
}
