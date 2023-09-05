use log::{debug, error, info, trace, warn, LevelFilter};

pub const LOG_LEVEL: LevelFilter = LevelFilter::Debug;
pub const WIDTH: u32 = 1920;
pub const HEIGHT: u32 = 1080;
pub const FULLSCREEN: bool = false;
pub const ENV_STEP: bool = false;
pub const ENV_SEED: u32 = 0;
pub const NUM_CELLS: usize = 200;
pub const TARGET_FRAME_RATE: u64 = 120;
pub const FRAME_DUR: u64 = 1_000 / TARGET_FRAME_RATE;
pub const COLLIDE_SPRING: f64 = -10.0;
pub const POST_REPRODUCTION_COLLIDE_SPRING: f64 = -0.2;
pub const FRICTION_COEFF: f64 = 0.01;