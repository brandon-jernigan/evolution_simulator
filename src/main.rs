// Suppress all warnings (not recommended in production code)
#![allow(warnings)]

// Standard library imports
use std::fs::OpenOptions;
use std::io::Write;
use std::thread::sleep;
use std::time::{Duration, Instant, SystemTime};

// External crate imports
use env_logger::Builder;
use log::{debug, info};
use rand::Rng;

// Internal module imports
mod cell;
mod environment;
mod utils;

// Functions from your internal modules
use crate::utils::log_util::init_logging;
use crate::utils::ui_util::{check_escape_pressed, init_sdl, render_current_state}; // Add this line
use environment::Environment;

const FULLSCREEN: bool = true;
const ENV_STEP: bool = true;
const ENV_SEED: u32 = 0;
const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();
    let mut step = 0;
    init_logging(start_time)?;
    info!("================================= New Run =================================");
    debug!("Debug message");
    let env_seed = if ENV_SEED == 0 {
        let mut rng = rand::thread_rng();
        rng.gen()
    } else {
        ENV_SEED
    };

    let (mut ui_context, width, height) = init_sdl(WIDTH, HEIGHT, FULLSCREEN)?;
    let mut env = Environment::new(width, height, env_seed, step);

    loop {
        if ENV_STEP {
            step += 1;
        }
        if check_escape_pressed(&mut ui_context.event_pump)? {
            break;
        }

        render_current_state(&mut env, &mut ui_context.canvas)?;
        env.update_terrain(width, height, env_seed, step);
        //sleep(Duration::from_millis(1000));
    }
    Ok(())
}
