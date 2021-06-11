use amethyst::{
    assets::Handle,
    core::Transform,
    ecs::{Component, DenseVecStorage},
    prelude::{Builder, WorldExt},
    renderer::{SpriteRender, SpriteSheet},
    shred::World,
};

use crate::settings::{ARENA_HEIGHT, ARENA_WIDTH, BALL_RADIUS, BALL_VELOCITY_X, BALL_VELOCITY_Y};

/// Ball entity definition.
#[derive(Debug)]
pub struct Ball {
    pub velocity: [f32; 2],
    pub radius: f32,
}

impl Ball {
    // associated function
    fn new(velocity: [f32; 2], radius: f32) -> Self {
        Ball {
            velocity: velocity,
            radius: radius,
        }
    }
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

pub fn initialize_ball(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut position = Transform::default();

    // ball starts in the center of the screen
    position.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 0.0);

    let ball = Ball::new([BALL_VELOCITY_X, BALL_VELOCITY_Y], BALL_RADIUS);

    let ball_sprite_render = SpriteRender::new(sprite_sheet_handle, 1);

    // creates a new entity and associates the following components to it:
    // 1. Ball's state (struct)
    // 2. Ball's transform (position, rotation, scale vectors)
    // 3. Ball's sprite renderer from a sprite sheet
    world
        .create_entity()
        .with(ball)
        .with(position)
        .with(ball_sprite_render)
        .build();
}
