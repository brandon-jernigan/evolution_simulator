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
use sdl2::audio::{AudioCallback, AudioSpecDesired};

// Internal module imports
mod cell;
mod constants;
mod environment;
mod utils;

// Functions from your internal modules
use crate::utils::log_util::init_logging;
use crate::utils::ui_util::{handle_events, init_sdl, render_current_state, capture_png, generate_loud_tone}; // Add this line
use environment::Environment;

use constants::{ENV_SEED, ENV_STEP, FULLSCREEN, HEIGHT, LOG_LEVEL, WIDTH, FRAME_DUR, TARGET_FRAME_RATE, NUM_CELLS, STEPS_PER_RENDER};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();
    let mut loop_step: i64 = 0;
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
    let mut should_render = true;

    debug!("main >> init_sdl");
    let (mut ui_context, width, height) = init_sdl()?;
    let desired_spec = AudioSpecDesired {
        freq: Some(44100),  // or 48000, or another standard rate
        channels: Some(1),  // mono output
        samples: None       // default sample size
    };
    let device = ui_context.audio_subsystem.open_queue::<f32, _>(None, &desired_spec)?;
    device.resume();

    debug!("main >> Environment::new. env_seed: {}", env_seed);
    let mut env = Environment::new(width, height, env_seed, loop_step);

    debug!("main >> Starting main loop");

    loop {
        let loop_start_time = SystemTime::now();
        loop_step += 1;
        let (new_should_render, should_exit) = handle_events(&mut ui_context.event_pump, should_render);
        should_render = new_should_render;
        if should_exit {
            debug!("main >> Escape pressed or window closed, exiting");
            break;
        }

        if should_render && loop_step % STEPS_PER_RENDER == 0 {
            debug!("main >> render_current_state");
            render_current_state(&mut env, &mut ui_context.canvas)?;
            let filename = format!("/media/volume/sdb/evolution_simulator/frames/frame_{:06}.png", loop_step / STEPS_PER_RENDER);
            capture_png(&ui_context.canvas, &filename).unwrap_or_else(|e| {
                error!("Failed to capture PNG: {}", e);
            });

        } else {
            let canvas = &mut ui_context.canvas;

            canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
            canvas.draw_line((10, 10), (10, 30)).unwrap();
            canvas.draw_line((20, 10), (20, 30)).unwrap();
            canvas.present();
        }

        debug!("main >> env.update()");
        let mut amplitude_sequence = env.update(loop_step);
        //amplitude_sequence = generate_loud_tone();
        for (i, item) in amplitude_sequence.iter().enumerate() {
            println!("t:{} A:{}", i, item);
        }
        device.queue(&amplitude_sequence);

        debug!("main >> env.update_terrain");
        if ENV_STEP {
            env.update_terrain(height, height, env_seed, loop_step);
        }
        let elapsed_time = loop_start_time.elapsed()
            .expect("Time went backwards")
            .as_millis() as u64;

        if elapsed_time < FRAME_DUR {
            sleep(Duration::from_millis(FRAME_DUR - elapsed_time));
        }
        println!("loop_step: {} elapsed_time: {}ms fps: {}", loop_step, elapsed_time, 1000.0 / elapsed_time as f64)
    }

    debug!("main >> Exiting main loop");
    Ok(())
}
