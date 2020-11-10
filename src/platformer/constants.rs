pub const BACKGROUND: [f32; 4] = [0.328, 0.266, 0.480, 1.0];

pub const PLAYER_FPS: u32 = 24;

pub const ASCEND_TIME: f64 = (1.0 / 60.0) * 12.0; // 12 frames @ 60 FPS
pub const DESCEND_TIME: f64 = (1.0 / 60.0) * 8.0; // 8 frames @ 60 FPS
pub const FLOAT_TIME: f64 = (1.0 / 60.0) * 1.5; // 1.5 frames @ 60 FPS
pub const MAX_ASCEND: f64 = 2.5 * 64.0; // 2.5 tiles
pub const DESCEND_SPEED: f64 = MAX_ASCEND / DESCEND_TIME;
pub const JUMP_COOL_DOWN: f64 = (1.0 / 60.0) * 3.0; // 3 frames @ 60 FPS

pub const SPEED_UP_TIME: f64 = (1.0 / 60.0) * 6.0; // 6 frames @ 60 FPS
pub const MAX_SPEED: f64 = 10.0 * 64.0; // 10 tiles / second
