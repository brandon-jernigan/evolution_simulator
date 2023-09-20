extern crate sdl2; // SDL2 library

use crate::environment::Environment;
use sdl2::event::Event;
use sdl2::EventPump;
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
        .window("🧬 Evolution Simulator", WIDTH, HEIGHT)
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
            //let color_value = (rescaled_val * color_mult) as u8; // Scale the 0-1 value to 0-255
            let [r, g, b, a] = hsva_to_rgba(197.0/360.0, 0.5, rescaled_val as f32, 1.0);


            canvas.set_draw_color(Color::RGB(r, g, b));

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

            let [mut r_mem, mut g_mem, mut b_mem, mut a_mem] = rbga_cell_lighting(cell, terrain, "membrane");

            let [mut r_in, mut b_in, mut g_in, mut a_in] = rbga_cell_lighting(cell, terrain, "inside");

            let [mut r_nuc, mut b_nuc, mut g_nuc, mut a_nuc] = rbga_cell_lighting(cell, terrain, "nucleus");
            if cell.id == 1 {
                r_in = 255;
                b_in = 255;
                g_in = 255;
                a_in = 255;
            }

            // Draw outer circle
            canvas.filled_circle(center_x, center_y, radius, (r_mem, g_mem, b_mem, a_mem))?;

            // Draw mid-circle
            canvas.filled_circle(center_x, center_y, radius - 2, Color::RGBA(r_in, b_in, g_in, a_in),)?;

            // Calculate the offset based on the velocity vector
            let velocity_magnitude = (cell.x_vel.powi(2) + cell.y_vel.powi(2)).sqrt();
            if velocity_magnitude != 0.0 {
                let offset_x = (cell.x_vel / velocity_magnitude * (radius as f64 / 4.0)).round() as i16;
                let offset_y = (cell.y_vel / velocity_magnitude * (radius as f64 / 4.0)).round() as i16;

                canvas.filled_circle(center_x + offset_x, center_y + offset_y, (radius as f64 * (2.0/5.0)).round() as i16, (r_nuc, b_nuc, g_nuc, a_nuc))?;
            } else {
                canvas.filled_circle(center_x, center_y, (radius as f64 * (2.0/5.0)).round() as i16, (r_nuc, b_nuc, g_nuc, a_nuc))?;
            }
        }
    }
    Ok(())
}

pub fn rbga_cell_lighting(cell: &Cell,terrain: &[Vec<f64>], color_type: &str) -> [u8; 4] {
    let lowest_cell_brightness = 0.2;
    let (mut h, mut s, mut v, mut a) = (0.0, 0.0, 0.0, 0.0);
    if color_type == "inside"{
        (h, s, v, a) = rgba_to_hsva(
            cell.inside_color[0],
            cell.inside_color[1],
            cell.inside_color[2],
            cell.inside_color[3]
        );
    } else if color_type == "membrane"{
        (h, s, v, a) = rgba_to_hsva(
            cell.membrane_color[0],
            cell.membrane_color[1],
            cell.membrane_color[2],
            cell.membrane_color[3]
        );
    } else if color_type == "nucleus"{
        (h, s, v, a) = rgba_to_hsva(
            cell.nucleus_color[0],
            cell.nucleus_color[1],
            cell.nucleus_color[2],
            cell.nucleus_color[3]
        );
    } else {
        panic!("Invalid color_type: {}", color_type);
    }

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


pub fn handle_events(event_pump: &mut EventPump, should_render: bool) -> (bool, bool) {
    let mut new_should_render = should_render;
    let mut should_exit = false;

    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(sdl2::keyboard::Keycode::Escape),
                ..
            } => {
                // Handle exit logic
                should_exit = true;
            }
            Event::MouseButtonDown { .. } => {
                // Toggle rendering when mouse is clicked
                new_should_render = !new_should_render;
            }
            _ => {}
        }
    }
    (new_should_render, should_exit)
}

pub fn hsva_to_rgba(h: f32, s: f32, v: f32, a: f32) -> [u8; 4] {
    let normalized_h = (h % 1.0 + 1.0) % 1.0;
    let c = v * s;
    let x = c * (1.0 - ((normalized_h * 6.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = if normalized_h >= 0.0 && normalized_h < 1.0/6.0 {
        (c, x, 0.0)
    } else if normalized_h >= 1.0/6.0 && normalized_h < 2.0/6.0 {
        (x, c, 0.0)
    } else if normalized_h >= 2.0/6.0 && normalized_h < 3.0/6.0 {
        (0.0, c, x)
    } else if normalized_h >= 3.0/6.0 && normalized_h < 4.0/6.0 {
        (0.0, x, c)
    } else if normalized_h >= 4.0/6.0 && normalized_h < 5.0/6.0 {
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

pub fn rgba_to_hsva(r: u8, g: u8, b: u8, a: u8) -> (f32, f32, f32, f32) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;
    
    let max = r.max(g.max(b));
    let min = r.min(g.min(b));
    let delta = max - min;

    let h = if delta == 0.0 {
        0.0
    } else if max == r {
        ((1.0/6.0 * ((g - b) / delta) + 1.0) % 1.0)
    } else if max == g {
        (1.0/6.0 * ((b - r) / delta) + 1.0/3.0)
    } else {
        (1.0/6.0 * ((r - g) / delta) + 2.0/3.0)
    };

    let s = if max == 0.0 { 0.0 } else { delta / max };
    let v = max;
    let a = a as f32 / 255.0;

    (h, s, v, a)
}
