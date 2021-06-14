mod ball;
mod camera;
mod paddle;
mod score;

// exposes Ball struct
pub use ball::{initialize_ball, Ball};
pub use camera::initialize_camera;
pub use paddle::{initialize_paddles, Paddle, Side};
pub use score::{intialize_scoreboard, ScoreText};
