use amethyst::{assets::Handle, core::Time, prelude::*, renderer::SpriteSheet, SimpleState};

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
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

impl SimpleState for Pong {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        let world = _data.world;

        self.sprite_sheet_handle.replace(load_sprite_sheet(world)); // adds sprite sheet handle state
        self.ball_spawn_timer.replace(1.0); // adds ball time spawner

        world.register::<Paddle>(); // in order to use Paddle Component on an entity
        world.register::<Ball>(); // in order to use the Ball Component on an entity

        // entities and their componenets initialization
        initialize_camera(world);
        initialize_paddles(world, self.sprite_sheet_handle.clone().unwrap());
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        // take the value from the Option, leaving None in its place
        if let Some(mut ball_timer) = self.ball_spawn_timer.take() {
            // time/fetch struct must be deallocated by going out of scope to avoid problems
            {
                let time = _data.world.fetch::<Time>();
                ball_timer -= time.delta_seconds(); // seconds passed after last frame (spf)
            }

            if ball_timer < 0.0 {
                let sprite_sheet_handle = self.sprite_sheet_handle.clone().unwrap(); // clone and unwraps Option handle

                initialize_ball(_data.world, sprite_sheet_handle); // handler is consumed per entitty
            } else {
                self.ball_spawn_timer.replace(ball_timer);
            }
        }

        Trans::None
    }
}
