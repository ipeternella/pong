use amethyst::SimpleState;
use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    ecs::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub const ARENA_WIDTH: f32 = 100.0;
pub const ARENA_HEIGHT: f32 = 100.0;

pub const PADDLE_WIDTH: f32 = 4.0;
pub const PADDLE_HEIGHT: f32 = 16.0;

pub struct Pong;

impl SimpleState for Pong {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        let world = _data.world;

        initialize_camera(world);
        initialize_paddles(world);
    }
}

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

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

/// Initializes a camera object at the middle of the (X, Y) axis and in front of the
/// (X, Y) axis at Z = 1 distance.
/// ### (X, Y) axis ranges
/// - X range set: [0, ARENA_WIDTH)
/// - Y range set: [0, ARENA_HEIGHT)
fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();

    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    // world stores all runtime data: entites and components from ECS
    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

/// Creates the left and right paddles at their starting position and attaches them
/// to the World object.
fn initialize_paddles(world: &mut World) {
    let mut left_paddle_transform = Transform::default();
    let mut right_paddle_transform = Transform::default();
    let starting_y = ARENA_HEIGHT / 2.0;

    // coordinate transforms to position the paddles
    left_paddle_transform.set_translation_xyz(0.0, starting_y, 0.0);
    right_paddle_transform.set_translation_xyz(ARENA_WIDTH, starting_y, 0.0);

    // left paddle creation
    world
        .create_entity()
        .with(Paddle::new(Side::Left))
        .with(left_paddle_transform)
        .build();

    // right paddle creation (entity + component<state> in ECS)
    world
        .create_entity()
        .with(Paddle::new(Side::Right))
        .with(right_paddle_transform)
        .build();

    world.register::<Paddle>(); // in order to use Paddle Component on an entity
}
