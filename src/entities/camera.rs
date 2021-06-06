use amethyst::{
    core::Transform,
    prelude::{Builder, WorldExt},
    renderer::Camera,
    shred::World,
};

use crate::pong::{ARENA_HEIGHT, ARENA_WIDTH};

/// Initializes a camera object at the middle of the (X, Y) axis and in front of the
/// (X, Y) axis at Z = 1 distance.
/// ### (X, Y) axis ranges
/// - X range set: [0, ARENA_WIDTH)
/// - Y range set: [0, ARENA_HEIGHT)
pub fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();

    // (x, y, z = 1.0)
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    // world stores all runtime data: entites and components from ECS
    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}
