extern crate sdl2; // SDL2 library

use crate::environment::Environment;
use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::thread::sleep;
use std::time::Duration;

use crate::cell::Cell;
use crate::constants::{ENV_SEED, ENV_STEP, FULLSCREEN, HEIGHT, LOG_LEVEL, WIDTH};

pub struct UIContext {
    pub sdl_context: sdl2::Sdl,
    pub event_pump: sdl2::EventPump,
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

pub fn init_sdl() -> Result<(UIContext, u32, u32), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let mut window = video_subsystem
        .window("SDL2 Window", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let (actual_width, actual_height) = if FULLSCREEN {
        window
            .set_fullscreen(sdl2::video::FullscreenType::Desktop)
            .unwrap();
        let display_index = window.display_index().unwrap();
        let display_mode = video_subsystem.current_display_mode(display_index).unwrap();
        (display_mode.w as u32, display_mode.h as u32)
    } else {
        (WIDTH, HEIGHT)
    };

    let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let event_pump = sdl_context.event_pump()?;

    Ok((
        UIContext {
            sdl_context,
            event_pump,
            canvas,
        },
        actual_width,
        actual_height,
    ))
}

pub fn render_current_state(
    env: &mut Environment,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    render_terrain(env, canvas)?;
    env.update();
    render_cells(&env.cells, canvas)?;

    canvas.present();
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    Ok(())
}

pub fn render_terrain(
    env: &Environment,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
) -> Result<(), String> {
    let color_mult = 255.0 * (0.5);
    for x in 0..env.terrain.len() {
        for y in 0..env.terrain[x].len() {
            let val = env.terrain[x][y];
            let color_value = (val * color_mult) as u8; // Scale the 0-1 value to 0-255

            canvas.set_draw_color(Color::RGB(color_value, color_value, color_value));

            // Draw a single pixel at (x, y)
            canvas.draw_point(Point::new(x as i32, y as i32))?;
        }
    }
    Ok(())
}

pub fn render_cells(
    cells: &[Cell],
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
) -> Result<(), String> {
    for cell in cells.iter() {
        let [r, g, b, a] = cell.inside_color;
        canvas.set_draw_color(Color::RGBA(r, g, b, a));

        canvas.filled_circle(
            cell.x_pos as i16,
            cell.y_pos as i16,
            cell.radius as i16,
            Color::RGBA(r, g, b, a),
        )?;
    }
    Ok(())
}

fn f32_color_to_sdl_color(color: [f32; 4]) -> sdl2::pixels::Color {
    let (r, g, b, _) = (
        (color[0] * 255.0) as u8,
        (color[1] * 255.0) as u8,
        (color[2] * 255.0) as u8,
        (color[3] * 255.0) as u8,
    );
    sdl2::pixels::Color::RGB(r, g, b)
}

pub fn check_escape_pressed(event_pump: &mut sdl2::EventPump) -> Result<bool, String> {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyUp {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                sleep(Duration::from_millis(500));
                return Ok(true);
            }
            _ => {}
        }
    }
    Ok(false)
}

pub fn hsba_to_rgba(h: f32, s: f32, b: f32, a: f32) -> [u8; 4] {
    let c = b * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = b - c;

    let (r, g, b) = if h >= 0.0 && h < 60.0 {
        (c, x, 0.0)
    } else if h >= 60.0 && h < 120.0 {
        (x, c, 0.0)
    } else if h >= 120.0 && h < 180.0 {
        (0.0, c, x)
    } else if h >= 180.0 && h < 240.0 {
        (0.0, x, c)
    } else if h >= 240.0 && h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    [
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
        (a * 255.0) as u8,
    ]
}
