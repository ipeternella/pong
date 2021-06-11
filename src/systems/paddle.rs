use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::{
    entities::{Paddle, Side},
    settings::{ARENA_HEIGHT, PADDLE_HEIGHT},
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_update_paddle_movement_with_bottom_clamping() {
        // arrange
        let paddle_system = PaddleSystem {};
        let mut paddle_transform = Transform::default();
        let paddle_y = 5.0;
        let mv_amount = 2.0;
        let expected_y = 8.0; // 5.0 + 1.2 * 2.0 = 7.4, which is less than MIN == 8.0 => so, bottom clamped!

        paddle_transform.set_translation_xyz(1.0, paddle_y, 0.0);

        // act
        paddle_system.update_position(&mut paddle_transform, mv_amount);

        // assert
        assert_eq!(expected_y, paddle_transform.translation().y);
    }

    #[test]
    fn should_update_paddle_movement_with_top_clamping() {
        // arrange
        let paddle_system = PaddleSystem {};
        let mut paddle_transform = Transform::default();
        let paddle_y = 94.0;
        let mv_amount = 2.0;
        let expected_y = 92.0; // 94.0 + 1.2 * 2.0 = 96.4, which is more than MAX == 92.0 => so, top clamped!

        paddle_transform.set_translation_xyz(1.0, paddle_y, 0.0);

        // act
        paddle_system.update_position(&mut paddle_transform, mv_amount);

        // assert
        assert_eq!(expected_y, paddle_transform.translation().y);
    }

    #[test]
    fn should_update_paddle_movement_without_clamping() {
        // arrange
        let paddle_system = PaddleSystem {};
        let mut paddle_transform = Transform::default();
        let paddle_y = 40.0;
        let mv_amount = 2.0;
        let expected_y = 42.4; // 40.0 + 1.2 * 2.0 = 42.4 => 8.0 < 42.4 < 92.0 : no clamping!

        paddle_transform.set_translation_xyz(1.0, paddle_y, 0.0);

        // act
        paddle_system.update_position(&mut paddle_transform, mv_amount);

        // assert
        assert_eq!(expected_y, paddle_transform.translation().y);
    }
}
