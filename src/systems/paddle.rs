use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::{
    entities::{Paddle, Side, PADDLE_HEIGHT},
    pong::ARENA_HEIGHT,
};

#[derive(SystemDesc)]
pub struct PaddleSystem;

impl PaddleSystem {
    /// Updates the paddle's position (translation's y position) by adding a movement amount to it.
    fn update_position(&self, paddle_transform: &mut Transform, mv_amount: f32) {
        let current_y = paddle_transform.translation().y;
        let scaled_mv_amount = 1.2 * mv_amount;

        // updates y between a given range
        let updated_y = (current_y + scaled_mv_amount).clamp(
            0.0 + PADDLE_HEIGHT * 0.5,
            ARENA_HEIGHT - PADDLE_HEIGHT * 0.5,
        );

        paddle_transform.set_translation_y(updated_y);
    }
}

// implementation of System interface for PaddleSystem
impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>, // mutates the transforms
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut transform_storage, paddle_storage, input) = data;

        for (paddle, transform) in (&paddle_storage, &mut transform_storage).join() {
            // axis_value returns the axis input value or None
            let captured_input = match paddle.side {
                Side::Left => input.axis_value("left_paddle"),
                Side::Right => input.axis_value("right_paddle"),
            };

            if let Some(mv_amount) = captured_input {
                self.update_position(transform, mv_amount);
            }
        }
    }
}
