//! Settings variables used throughout the project.

// Gameplay
pub const BALL_SPAWN_DELAY: f32 = 2.0;

// Audio - sfx
pub const BOUNCE_SFX: &str = "audio/sfx/bounce.ogg";
pub const SCORE_SFX: &str = "audio/sfx/score.ogg";

// Audio - tracks
pub const MUSIC_TRACKS: &[&str] = &["audio/tracks/track-01.ogg", "audio/tracks/track-02.ogg"];

// Arena (game window) settings
pub const ARENA_WIDTH: f32 = 100.0;
pub const ARENA_HEIGHT: f32 = 100.0;

// Ball definitions
pub const BALL_VELOCITY_X: f32 = 75.0;
pub const BALL_VELOCITY_Y: f32 = 50.0;
pub const BALL_RADIUS: f32 = 2.0;

// Paddle definitions
pub const PADDLE_WIDTH: f32 = 4.0;
pub const PADDLE_HEIGHT: f32 = 16.0;

// Assets paths
pub const SQUARE_FONT_PATH: &str = "fonts/square.ttf";
pub const SPRITE_SHEET_PATH: &str = "textures/spritesheet.png";
pub const SPRITE_SHEET_ATLAS_PATH: &str = "textures/spritesheet.ron";
