extern crate sdl2; // SDL2 library

use crate::environment::Environment;
use sdl2::event::Event;
use sdl2::render::Canvas;
use sdl2::video::Window;
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
    render_cells(&env.cells, &env.terrain, canvas)?;

    canvas.present();
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    Ok(())
}

pub fn render_terrain(
    env: &Environment,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
) -> Result<(), String> {
    let min_bright_val = 0.0;
    let max_bright_val = 0.9;
    let color_mult = 255.0;

    for x in 0..env.terrain.len() {
        for y in 0..env.terrain[x].len() {
            let val = env.terrain[x][y];
            let rescaled_val = min_bright_val + ((val - 0.0) / (1.0 - 0.0)) * (max_bright_val - min_bright_val);
            let color_value = (rescaled_val * color_mult) as u8; // Scale the 0-1 value to 0-255

            canvas.set_draw_color(Color::RGB(color_value, color_value, color_value));

            // Draw a single pixel at (x, y)
            canvas.draw_point(Point::new(x as i32, y as i32))?;
        }
    }
    Ok(())
}

pub fn render_cells(
    cells: &[Cell],
    terrain: &[Vec<f64>],
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
) -> Result<(), String> {
    for cell in cells.iter() {
        if cell.alive {
            let center_x = cell.x_pos as i16;
            let center_y = cell.y_pos as i16;
            let radius = cell.radius as i16;

            let [mut r_new, mut b_new, mut g_new, mut a_new] = rbga_cell_lighting(cell, terrain);
            if cell.id == 1 {
                r_new = 255;
                b_new = 255;
                g_new = 255;
                a_new = 255;
            }

            // Draw outer circle
            //canvas.filled_circle(center_x, center_y, radius, f32_color_to_sdl_color([1.0, 1.0, 1.0, 1.0]))?;

            // Draw mid-circle
            canvas.filled_circle(center_x, center_y, radius - 1, Color::RGBA(r_new, b_new, g_new, a_new),)?;

            // Calculate the offset based on the velocity vector
            let velocity_magnitude = (cell.x_vel.powi(2) + cell.y_vel.powi(2)).sqrt();
            if velocity_magnitude != 0.0 {
                let offset_x = (cell.x_vel / velocity_magnitude * (radius as f64 / 4.0)).round() as i16;
                let offset_y = (cell.y_vel / velocity_magnitude * (radius as f64 / 4.0)).round() as i16;

                // Draw inner circle
                canvas.filled_circle(center_x + offset_x, center_y + offset_y, (radius as f64 * (1.0/3.0)).round() as i16, f32_color_to_sdl_color([0.0, 0.0, 0.0, 1.0]))?;
            } else {
                // Draw inner circle
                canvas.filled_circle(center_x, center_y, (radius as f64 * (1.0/3.0)).round() as i16, f32_color_to_sdl_color([0.0, 0.0, 0.0, 1.0]))?;
            }
        }
    }
    Ok(())
}

pub fn rbga_cell_lighting(cell: &Cell,terrain: &[Vec<f64>]) -> [u8; 4] {
    let lowest_cell_brightness = 0.33;
    let (mut h, mut s, mut b, mut a) = rgba_to_hsba(
        cell.inside_color[0],
        cell.inside_color[1],
        cell.inside_color[2],
        cell.inside_color[3]
    );

    let terrain_val = terrain[cell.x_pos.round() as usize][cell.y_pos.round() as usize];
    let v_new = lowest_cell_brightness + (1.0 - lowest_cell_brightness) * terrain_val as f32;
    let [r_new, b_new, g_new, a_new] = hsva_to_rgba(h, s, v_new, a);

    [r_new, b_new, g_new, a_new]

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

pub fn hsva_to_rgba(h: f32, s: f32, v: f32, a: f32) -> [u8; 4] {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

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

pub fn rgba_to_hsba(r: u8, g: u8, b: u8, a: u8) -> (f32, f32, f32, f32) {
    // Convert r, g, b to [0, 1] range
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;
    
    // Find max, min, and their difference
    let max = r.max(g.max(b));
    let min = r.min(g.min(b));
    let delta = max - min;
    
    // Calculate Hue
    let h = if delta == 0.0 {
        0.0
    } else if max == r {
        ((60.0 * ((g - b) / delta) + 360.0) % 360.0)
    } else if max == g {
        (60.0 * ((b - r) / delta) + 120.0)
    } else {
        (60.0 * ((r - g) / delta) + 240.0)
    };

    // Calculate Saturation
    let s = if max == 0.0 {
        0.0
    } else {
        delta / max
    };

    // Calculate Brightness
    let b = max;
    
    // Alpha
    let a = a as f32 / 255.0;

    (h, s, b, a)
}