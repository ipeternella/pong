use amethyst::{
    assets::Handle,
    core::Transform,
    ecs::{Component, DenseVecStorage},
    prelude::{Builder, WorldExt},
    renderer::{SpriteRender, SpriteSheet},
    shred::World,
};

use crate::settings::{ARENA_HEIGHT, ARENA_WIDTH, PADDLE_HEIGHT, PADDLE_WIDTH};

/// Enum used to identify the left/right paddles.
#[derive(PartialEq, Eq, Debug)]
pub enum Side {
    Left,
    Right,
}

/// Paddle component struct.
#[derive(Debug)]
pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Paddle {
    fn new(side: Side) -> Paddle {
        Paddle {
            side,
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
        }
    }
}

// This adds the 'Component' behavior to the Paddle
impl Component for Paddle {
    type Storage = DenseVecStorage<Self>; // Component<Paddle>
}

/// Creates the left and right paddles at their starting position and attaches them
/// to the World object.
pub fn initialize_paddles(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut left_paddle_transform = Transform::default();
    let mut right_paddle_transform = Transform::default();
    let starting_y = ARENA_HEIGHT / 2.0;

    // SpriteRender is a component that is a 'slice' of a spritesheet
    let paddle_left = SpriteRender::new(sprite_sheet_handle, 0); // paddle is the first sprite
    let paddle_right = paddle_left.clone();

    // coordinate transforms to position the paddles
    left_paddle_transform.set_translation_xyz(PADDLE_WIDTH * 0.5, starting_y, 0.0);
    right_paddle_transform.set_translation_xyz(ARENA_WIDTH - PADDLE_WIDTH * 0.5, starting_y, 0.0);

    // left paddle creation
    world
        .create_entity()
        .with(Paddle::new(Side::Left)) // component used in ReadStorage
        .with(paddle_left)
        .with(left_paddle_transform)
        .build();

    // right paddle creation (entity + component<state> in ECS)
    world
        .create_entity()
        .with(Paddle::new(Side::Right))
        .with(paddle_right) // sprite renderer
        .with(right_paddle_transform)
        .build();
}
