mod ball;
mod camera;
mod paddle;

// exposes Ball struct
pub use self::ball::{initialize_ball, Ball};
pub use self::camera::initialize_camera;
pub use self::paddle::{initialize_paddles, Paddle, Side, PADDLE_HEIGHT, PADDLE_WIDTH};
