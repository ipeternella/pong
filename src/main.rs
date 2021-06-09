mod entities;
mod pong;
mod sprite_sheet;
mod startup;
mod systems;

use amethyst::{prelude::*, utils::application_root_dir};
use startup::{build_game_config, setup_logger};

fn main() -> amethyst::Result<()> {
    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets");

    // logger setup
    setup_logger();

    // game config and game instance building
    let game_config = build_game_config(&app_root)?;
    let mut game = Application::new(assets_dir, pong::Pong::default(), game_config)?;

    // run game loop
    game.run();

    Ok(())
}
