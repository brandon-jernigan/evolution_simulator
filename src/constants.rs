use log::{debug, error, info, trace, warn, LevelFilter};

pub const LOG_LEVEL: LevelFilter = LevelFilter::Debug;
pub const WIDTH: u32 = 1280*2;
pub const HEIGHT: u32 = 720*2;
pub const FULLSCREEN: bool = false;
pub const ENV_STEP: bool = false;
pub const ENV_SEED: u32 = 0;
pub const NUM_CELLS: usize = 200;
pub const TARGET_FRAME_RATE: u64 = 120;
pub const FRAME_DUR: u64 = 1_000 / TARGET_FRAME_RATE;
pub const COLLIDE_SPRING: f64 = -7.5;
pub const POST_REPRODUCTION_COLLIDE_SPRING: f64 = -0.4;
pub const FRICTION_COEFF: f64 = 0.075;
pub const STEPS_PER_RENDER: i64 = 1;
