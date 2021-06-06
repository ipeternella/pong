use amethyst::{
    core::{Time, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

use crate::components::Ball;

// SystemDesc requires System Trait implementation
#[derive(SystemDesc)]
pub struct BallSystem;

impl<'s> System<'s> for BallSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Ball>,
        Read<'s, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut transform_storage, ball_storage, time) = data;

        // traverses the storages to get components (state) to add behavior to it!
        for (transform, ball) in (&mut transform_storage, &ball_storage).join() {
            // increment the ball's velocity to each translation (position) vector
            transform.prepend_translation_x(ball.velocity[0] * time.delta_seconds());
            transform.prepend_translation_x(ball.velocity[1] * time.delta_seconds());
        }
    }
}
