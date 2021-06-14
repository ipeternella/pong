use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    core::Transform,
    ecs::{Join, ReadStorage, WriteStorage},
    shred::{Read, ReadExpect, System},
};

use crate::{
    audio::{play_bounce_sfx, Sounds},
    entities::{Ball, Paddle, Side},
    settings::ARENA_HEIGHT,
};

pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        WriteStorage<'s, Ball>, // mutable: gonna update it's velocity vector
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            transform_storage,
            paddle_storage,
            mut ball_storage,
            audio_storage,
            sounds,
            audio_output,
        ) = data;

        // ball and its transforms
        for (ball, transform) in (&mut ball_storage, &transform_storage).join() {
            let ball_x = transform.translation().x;
            let ball_y = transform.translation().y;
            let ball_velocity_y = ball.velocity[1];

            if has_top_or_bottom_collision(ball_y, ball_velocity_y, ball.radius) {
                ball.velocity[1] *= -1.0;
            }

            // traverses all paddles/transforms to check for collisions
            for (paddle, transform) in (&paddle_storage, &transform_storage).join() {
                let paddle_x = transform.translation().x;
                let paddle_y = transform.translation().y;

                if has_paddle_collision(
                    ball_x,
                    ball_y,
                    paddle_x,
                    paddle_y,
                    paddle.width,
                    paddle.height,
                ) {
                    if (paddle.side == Side::Left && ball.velocity[0] < 0.0)
                        || (paddle.side == Side::Right && ball.velocity[0] > 0.0)
                    {
                        play_bounce_sfx(&sounds, &audio_storage, audio_output.as_deref());
                        ball.velocity[0] *= -1.0;
                    }
                }
            }
        }
    }
}

fn has_top_or_bottom_collision(ball_y: f32, ball_velocity_y: f32, ball_radius: f32) -> bool {
    (ball_y < ball_radius && ball_velocity_y < 0.0)
        || (ball_y > ARENA_HEIGHT - ball_radius && ball_velocity_y > 0.0)
}

fn has_paddle_collision(
    ball_x: f32,
    ball_y: f32,
    paddle_x: f32,
    paddle_y: f32,
    paddle_width: f32,
    paddle_height: f32,
) -> bool {
    let paddle_top = paddle_y + paddle_height / 2.0;
    let paddle_bottom = paddle_y - paddle_height / 2.0;

    let paddle_right = paddle_x + paddle_width / 2.0;
    let paddle_left = paddle_x - paddle_width / 2.0;

    (ball_x >= paddle_left && ball_x <= paddle_right)
        && (ball_y >= paddle_bottom && ball_y <= paddle_top)
}

// built-in rust attribute: compiled only when in "test" mode!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_assert_paddle_collision() {
        // arrange - ball within the paddle ranges
        let paddle_width = 4.0;
        let paddle_height = 10.0;

        let ball_x = 18.0;
        let ball_y = 25.0;

        let paddle_x = 20.0; //  18 <= paddle_range_x <= 22
        let paddle_y = 20.0; //  15 <= paddle_range_y <= 25

        // act
        let has_collision = has_paddle_collision(
            ball_x,
            ball_y,
            paddle_x,
            paddle_y,
            paddle_width,
            paddle_height,
        );

        // assert
        assert_eq!(true, has_collision);
    }

    #[test]
    fn should_assert_paddle_collision_on_edge() {
        // arrange - ball within the paddle ranges
        let paddle_width = 4.0;
        let paddle_height = 10.0;

        let ball_x = 22.0;
        let ball_y = 25.0;

        let paddle_x = 20.0; //  18 <= paddle_range_x <= 22
        let paddle_y = 20.0; //  15 <= paddle_range_y <= 25

        // act
        let has_collision = has_paddle_collision(
            ball_x,
            ball_y,
            paddle_x,
            paddle_y,
            paddle_width,
            paddle_height,
        );

        // assert
        assert_eq!(true, has_collision);
    }

    #[test]
    fn should_assert_no_paddle_collision() {
        // arrange - ball outside the paddle ranges
        let paddle_width = 4.0;
        let paddle_height = 10.0;

        let ball_x = 2.0;
        let ball_y = 2.0;

        let paddle_x = 20.0; //  18 <= paddle_range_x <= 22
        let paddle_y = 20.0; //  15 <= paddle_range_y <= 25

        // act
        let has_collision = has_paddle_collision(
            ball_x,
            ball_y,
            paddle_x,
            paddle_y,
            paddle_width,
            paddle_height,
        );

        // assert
        assert_eq!(false, has_collision);
    }
}
