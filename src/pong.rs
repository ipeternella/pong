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
        let sprite_sheet_handle = load_sprite_sheet(world);

        world.register::<Paddle>(); // in order to use Paddle Component on an entity

        initialize_camera(world);
        initialize_paddles(world, sprite_sheet_handle);
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
fn initialize_paddles(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
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
        .with(Paddle::new(Side::Left))
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

/// Reads a spritesheet png file and its related ron file in order to parse the spritesheet
/// at the appropriate positions in order to generate a handle to a well-formatted SpriteSheet
/// struct which contains the sprites at the right sizes/positions.
fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    // the handle points to the location where the asset was loaded: faster and reduces mem usage!
    let texture_handle = {
        // loader is not ECS: it's a resource that belongs to the ECS World
        let loader = world.read_resource::<Loader>(); // added by the core application

        // texture storage is not ECS: it's a resource where the loader puts Textures loaded from spritesheet, etc.
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();

        loader.load(
            "textures/spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();

    loader.load(
        "textures/spritesheet.ron",        // ron file: sprites' positions
        SpriteSheetFormat(texture_handle), // handle to the loaded spritesheet png file
        (),
        &sprite_sheet_store, // storage used to store spritesheet data (extract pics from the png file)
    )
}
