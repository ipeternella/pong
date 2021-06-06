use amethyst::{prelude::*, SimpleState};

use crate::{
    entities::{initialize_ball, initialize_camera, initialize_paddles, Ball, Paddle},
    sprite_sheet::load_sprite_sheet,
};

pub const ARENA_WIDTH: f32 = 100.0;
pub const ARENA_HEIGHT: f32 = 100.0;

/// Represents the Pong game state.

#[derive(Default)]
pub struct Pong {
    ball_spawn_timer: Option<f32>,
}

impl SimpleState for Pong {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        let world = _data.world;
        let sprite_sheet_handle = load_sprite_sheet(world);

        world.register::<Paddle>(); // in order to use Paddle Component on an entity
        world.register::<Ball>(); // in order to use the Ball Component on an entity

        // entities and their componenets initialization
        initialize_camera(world);
        initialize_ball(world, sprite_sheet_handle.clone()); // handler is consumed per entitty
        initialize_paddles(world, sprite_sheet_handle);
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::None
    }
}
