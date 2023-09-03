// Suppress all warnings (not recommended in production code)
#![allow(warnings)]

// Standard library imports
use std::fs::OpenOptions;
use std::io::Write;
use std::thread::sleep;
use std::time::{Duration, Instant, SystemTime};

// External crate imports
use env_logger::Builder;
use log::{debug, error, info, trace, warn, LevelFilter};
use rand::Rng;

// Internal module imports
mod cell;
mod constants;
mod environment;
mod utils;

// Functions from your internal modules
use crate::utils::log_util::init_logging;
use crate::utils::ui_util::{check_escape_pressed, init_sdl, render_current_state}; // Add this line
use environment::Environment;

use constants::{ENV_SEED, ENV_STEP, FULLSCREEN, HEIGHT, LOG_LEVEL, WIDTH};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();
    let mut step = 0;
    init_logging(start_time, LOG_LEVEL)?;
    info!(
        "main >>  WIDTH: {}, HEIGHT: {}, FULLSCREEN: {}, ENV_STEP: {}, ENV_SEED: {}",
        WIDTH, HEIGHT, FULLSCREEN, ENV_STEP, ENV_SEED
    );
    let env_seed = if ENV_SEED == 0 {
        let mut rng = rand::thread_rng();
        rng.gen()
    } else {
        ENV_SEED
    };

    debug!("main >> init_sdl");
    let (mut ui_context, width, height) = init_sdl()?;
    debug!("main >> Environment::new. env_seed: {}", env_seed);
    let mut env = Environment::new(width, height, env_seed, step);
    debug!("main >> Starting main loop");

    loop {
        if ENV_STEP {
            step += 1;
        }
        if check_escape_pressed(&mut ui_context.event_pump)? {
            debug!("main >> Escape pressed, exiting");
            break;
        }

        debug!("main >> render_current_state");
        render_current_state(&mut env, &mut ui_context.canvas)?;
        debug!("main >> env.update_terrain");
        if ENV_STEP {
            env.update_terrain(width, height, env_seed, step);
            step += 1;
        }
        //sleep(Duration::from_millis(1000));
    }

    debug!("main >> Exiting main loop");
    Ok(())
}
