use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, SystemData, WriteStorage},
    shred::System,
};

use crate::{
    entities::Ball,
    pong::{ARENA_HEIGHT, ARENA_WIDTH},
};

#[derive(SystemDesc)]
pub struct ScoreSystem;

impl ScoreSystem {
    fn has_player_scored(&mut self, ball_x: f32, ball_radius: f32) -> bool {
        (ball_x <= ball_radius) || (ball_x >= ARENA_WIDTH - ball_radius)
    }
}

/// System responsible for dealing with ball scoring.
impl<'s> System<'s> for ScoreSystem {
    type SystemData = (WriteStorage<'s, Ball>, WriteStorage<'s, Transform>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut ball_storage, mut transform_storage) = data;

        // join storages: only ball with transforms
        for (ball, transform) in (&mut ball_storage, &mut transform_storage).join() {
            let ball_x = transform.translation().x;
            let ball_radius = ball.radius;

            // centralize the ball and reverse its speed!
            if self.has_player_scored(ball_x, ball_radius) {
                let spawn_x = ARENA_WIDTH / 2.0;
                let spawn_y = ARENA_HEIGHT / 2.0;

                transform.set_translation_xyz(spawn_x, spawn_y, 0.0);
                ball.velocity[0] *= -1.0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_assert_ball_has_scored() {
        // arrange
        let ball_x = 0.0;
        let ball_radius = 2.0;
        let mut score_system = ScoreSystem {};

        // act
        let has_score = score_system.has_player_scored(ball_x, ball_radius);

        // assert
        assert_eq!(has_score, true);
    }

    #[test]
    fn should_assert_ball_has_scored_on_left_edge() {
        // arrange
        let ball_radius = 2.0;
        let ball_x = 2.0;
        let mut score_system = ScoreSystem {};

        // act
        let has_score = score_system.has_player_scored(ball_x, ball_radius);

        // assert
        assert_eq!(has_score, true);
    }

    #[test]
    fn should_assert_ball_has_scored_on_right_edge() {
        // arrange
        let ball_radius = 2.0;
        let ball_x = ARENA_WIDTH - ball_radius;
        let mut score_system = ScoreSystem {};

        // act
        let has_score = score_system.has_player_scored(ball_x, ball_radius);

        // assert
        assert_eq!(has_score, true);
    }

    #[test]
    fn should_assert_ball_has_not_scored() {
        // arrange
        let ball_radius = 2.0;
        let ball_x = 10.0; // 8 <= ball_x <= 12 (not near the edges)
        let mut score_system = ScoreSystem {};

        // act
        let has_score = score_system.has_player_scored(ball_x, ball_radius);

        // assert
        assert_eq!(has_score, false);
    }
}
