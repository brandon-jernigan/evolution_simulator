extern crate sdl2; // SDL2 library

use crate::environment::Environment;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::thread::sleep;
use std::time::Duration;

pub struct UIContext {
    pub sdl_context: sdl2::Sdl,
    pub event_pump: sdl2::EventPump,
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

pub fn init_sdl(
    width: u32,
    height: u32,
    fullscreen: bool,
) -> Result<(UIContext, u32, u32), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let mut window = video_subsystem
        .window("SDL2 Window", width, height)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let (actual_width, actual_height) = if fullscreen {
        window
            .set_fullscreen(sdl2::video::FullscreenType::Desktop)
            .unwrap();
        let display_index = window.display_index().unwrap();
        let display_mode = video_subsystem.current_display_mode(display_index).unwrap();
        (display_mode.w as u32, display_mode.h as u32)
    } else {
        (width, height)
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
    env: &Environment,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    render_terrain(env, canvas)?;

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
