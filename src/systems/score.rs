use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, SystemData, WriteStorage},
    shred::{ReadExpect, System},
    ui::UiText,
};

use crate::{
    entities::{Ball, ScoreText, Side},
    settings::{ARENA_HEIGHT, ARENA_WIDTH},
};

#[derive(SystemDesc)]
pub struct ScoreSystem;

impl ScoreSystem {
    // Returns an Option with the paddle side that got defeated. If neither player has
    // scored, than the Option contains None.
    fn has_player_scored(&mut self, ball_x: f32, ball_radius: f32) -> Option<Side> {
        if ball_x <= ball_radius {
            Some(Side::Left)
        } else if ball_x >= ARENA_WIDTH - ball_radius {
            Some(Side::Right)
        } else {
            None
        }
    }
}

/// System responsible for dealing with ball scoring.
impl<'s> System<'s> for ScoreSystem {
    // data changed by the system
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        ReadExpect<'s, ScoreText>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut ball_storage, mut transform_storage, mut ui_text_storage, score_text) = data;

        // join storages: only ball with transforms
        for (ball, transform) in (&mut ball_storage, &mut transform_storage).join() {
            let ball_x = transform.translation().x;
            let ball_radius = ball.radius;

            // centralize the ball and reverse its speed!
            if let Some(side_score) = self.has_player_scored(ball_x, ball_radius) {
                let spawn_x = ARENA_WIDTH / 2.0;
                let spawn_y = ARENA_HEIGHT / 2.0;

                // score handling
                if side_score == Side::Left {
                    if let Some(text) = ui_text_storage.get_mut(score_text.p2_score) {
                        let score: i32 = text.text.parse().unwrap();

                        text.text = (score + 1).to_string();
                    }
                } else {
                    if let Some(text) = ui_text_storage.get_mut(score_text.p1_score) {
                        let score: i32 = text.text.parse().unwrap();

                        text.text = (score + 1).to_string();
                    }
                }

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
        let has_scored_opt = score_system.has_player_scored(ball_x, ball_radius);

        // assert
        assert_eq!(has_scored_opt.is_none(), false); // player has scored
    }

    #[test]
    fn should_assert_ball_has_scored_on_left_edge() {
        // arrange
        let ball_radius = 2.0;
        let ball_x = 2.0;
        let mut score_system = ScoreSystem {};

        // act
        let has_scored_opt = score_system.has_player_scored(ball_x, ball_radius);

        // assert
        assert_eq!(has_scored_opt.is_none(), false); // player has scored
        assert_eq!(has_scored_opt.unwrap(), Side::Left);
    }

    #[test]
    fn should_assert_ball_has_scored_on_right_edge() {
        // arrange
        let ball_radius = 2.0;
        let ball_x = ARENA_WIDTH - ball_radius;
        let mut score_system = ScoreSystem {};

        // act
        let has_scored_opt = score_system.has_player_scored(ball_x, ball_radius);

        // assert
        assert_eq!(has_scored_opt.is_none(), false); // player has scored
        assert_eq!(has_scored_opt.unwrap(), Side::Right);
    }

    #[test]
    fn should_assert_ball_has_not_scored() {
        // arrange
        let ball_radius = 2.0;
        let ball_x = 10.0; // 8 <= ball_x <= 12 (not near the edges)
        let mut score_system = ScoreSystem {};

        // act
        let has_scored_opt = score_system.has_player_scored(ball_x, ball_radius);

        // assert
        assert_eq!(has_scored_opt.is_none(), true);
    }
}
