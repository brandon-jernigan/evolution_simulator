extern crate sdl2;
extern crate rand;
extern crate rand_distr;
extern crate rayon;
extern crate num_format;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{Instant, Duration, SystemTime};
use std::thread::sleep;
use rand::Rng;
use rand_distr::{Distribution, Normal};
//use rayon::prelude::*;
use std::io::{self, Write};
use sdl2::gfx::primitives::DrawRenderer;
use num_format::{Locale, ToFormattedString};


const MULTITHREAD: bool = false;
const DEBUG_MODE: bool = false;
const DEBUG_SLEEP_DUR: f64 = 0.0;
const DEBUG_MAX_ITER: usize = 1000;
//const CHUNK_SIZE: usize = 10;
const WIN_X: u32 = 1600;
const WIN_Y: u32 = 1600;
const TARGET_FRAME_RATE: u64 = 60;
const FRAME_DUR: u64 = 1_000 / TARGET_FRAME_RATE; // in milliseconds

const PART_NUM: usize = 20;
const RED_MEAN_MASS: f64 = 500.0;
const RED_STDEV_MASS: f64 = 400.0;
const GREEN_MEAN_MASS: f64 = 250.0;
const GREEN_STDEV_MASS: f64 = 200.0;
const RED_MEAN_VEL: f64 = 0.0;
const RED_STDEV_VEL: f64 = 4.0;
const COLLIDE_SPRING: f64 = -10000.0; // Adjust this constant according to your needs
const FRICTION_COEFF: f64 = 0.999999; // Choose a value close to but less than 1.0

const EDGE_BUFFER: u32 = 8 as u32; // This sets the buffer zone width in pixels.
const TOP_BUFFER: u32 = 8;




#[derive(Clone, Debug)]
struct Particle {
    part_type: String,
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    mass: f64,
    rad: i16,
    color: [f32; 4], // R,G,B,A
    energy: f64, // Add an energy meter
    creation_time: Instant, // Time when the particle was created
}

fn init_particles() -> Vec<Particle> {
    let mut rng = rand::thread_rng();

    let vel_normal = Normal::new(RED_MEAN_VEL, RED_STDEV_VEL).unwrap();
    let mass_normal = Normal::new(RED_MEAN_MASS, RED_STDEV_MASS).unwrap(); 

    (0..PART_NUM)
        .map(|_| {
            let mut mass = mass_normal.sample(&mut rng); // Generate random mass
            if mass <= 36.0 {
                mass = 36.0;
            }
            let rad = (mass as f64).sqrt() as i16; // Set radius based on mass
            let energy = mass;
            Particle {
                part_type: String::from("red"),
                x: rng.gen_range((EDGE_BUFFER ) as f64..(WIN_X - EDGE_BUFFER) as f64),
                y: rng.gen_range((EDGE_BUFFER + TOP_BUFFER) as f64..(WIN_Y - EDGE_BUFFER) as f64),
                vx: vel_normal.sample(&mut rng),
                vy: vel_normal.sample(&mut rng),
                mass,
                rad,
                color: [1.0,0.0,0.0,1.0], //red
                energy,
                creation_time: Instant::now(),

            }            
        })
        .collect()
}

struct NeuralNetwork {
    input_layer: [f64; 17],
    hidden_layer: [f64; 8],
    output_layer: [f64; 2],
    weights_ih: [[f64; 8]; 17],  // weights from input to hidden layer
    weights_ho: [[f64; 2]; 8],  // weights from hidden to output layer
}

fn update_neural_inputs(particles: &mut [Particle]) {
    for particle in particles.iter_mut() {
        // Initialize/reset input_layer to zeros
        //particle.neural_network.input_layer = [0.0; 17];
        
        // Loop through all particles to populate proximity detectors
        // for other_particle in particles.iter_mut() {
            // if particle == other_particle {
            //     continue;
            // }
            // Calculate distance, angle, and other metrics between particle and other_particle
            // Update the appropriate channels in particle.neural_network.input_layer
            // let dx = particle.x - other_particle.x;
            // let dy = particle.y - other_particle.y;
            // let distance_squared = dx * dx + dy * dy;
            // let distance = distance_squared.sqrt();


        // }

        // Add self velocity magnitude to the last neuron
        //let self_velocity_magnitude = (particle.velocity[0].powi(2) + particle.velocity[1].powi(2)).sqrt();
        //particle.neural_network.input_layer[20] = self_velocity_magnitude;
    }
}

fn handle_collision(p1: &mut Particle, p2: &mut Particle, i: usize, j: usize) -> Vec<usize> {
    let mut indices_to_remove = Vec::new();
    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;
    let dt = FRAME_DUR as f64 / 1000.0;
    let distance_squared = dx * dx + dy * dy;
    
    let min_dist = (p1.rad + p2.rad) as f64;

    if distance_squared < min_dist * min_dist {

        let distance = distance_squared.sqrt();
        let overlap = min_dist - distance;

        // normalize dx and dy
        let nx = dx / distance;
        let ny = dy / distance;

        let force = overlap * COLLIDE_SPRING;

        let ax1 = force / p1.mass as f64;
        let ay1 = force / p1.mass as f64;

        let ax2 = force / p2.mass as f64;
        let ay2 = force / p2.mass as f64;

        p1.vx -= ax1 * nx * dt;
        p1.vy -= ay1 * ny * dt;
        p2.vx += ax2 * nx * dt;
        p2.vy += ay2 * ny * dt;

        if p1.part_type == "green" && p2.part_type == "red" {
            p2.energy = p2.energy + p1.energy;
            p2.mass = p2.energy;
            p2.rad = (p2.mass as f64).sqrt() as i16;
            indices_to_remove.push(i);
        } else if p1.part_type == "red" && p2.part_type == "green" {
            p1.energy = p1.energy + p2.energy;
            p1.mass = p1.energy;
            p1.rad = (p1.mass as f64).sqrt() as i16;
            indices_to_remove.push(j);
        } 
    }
    indices_to_remove
}


fn update_particle(p: &mut Particle) {
    handle_boundary(p);
    p.vx *= FRICTION_COEFF;
    p.vy *= FRICTION_COEFF;
    
    p.x += p.vx;
    p.y += p.vy;

    

    if DEBUG_MODE {
        sleep(Duration::from_secs_f64(DEBUG_SLEEP_DUR));
    }
}


fn handle_boundary(p: &mut Particle) {
    if p.x <= EDGE_BUFFER as f64 {
        p.x = EDGE_BUFFER as f64 + 2.0;
        p.vx = -p.vx;
    }
    else if p.x >= (WIN_X - EDGE_BUFFER) as f64 {
        p.x = (WIN_X - EDGE_BUFFER) as f64 - 2.0;
        p.vx = -p.vx;
    }
    if p.y <= (EDGE_BUFFER + TOP_BUFFER) as f64 {
        p.y = (EDGE_BUFFER + TOP_BUFFER) as f64 + 2.0;
        p.vy = -p.vy;
    } else if p.y >= (WIN_Y - EDGE_BUFFER) as f64 {
        p.y = (WIN_Y - EDGE_BUFFER) as f64 - 2.0;
        p.vy = -p.vy;
    }
}

use rayon::prelude::*;

fn update_particles(particles: &mut Vec<Particle>) ->  u64 {
    let mut indices_to_remove = Vec::new();
    let now = Instant::now(); // Current time
    let mut total_kinetic_energy = 0.0;

    particles.iter_mut().for_each(|particle| {
        update_particle(particle);
        total_kinetic_energy += 0.5 * particle.mass as f64 * (particle.vx.powi(2) + particle.vy.powi(2));
    });

    for i in 0..particles.len() {
        for j in i + 1..particles.len() {
            let (left, right) = particles.split_at_mut(j);
            let mut removal_indices = handle_collision(&mut left[i], &mut right[0], i, j);
            indices_to_remove.append(&mut removal_indices);
        }
    }

    // Clone the indices_to_remove before removing particles
    let mut indices_to_remove_clone = indices_to_remove.clone();
    
    // Remove particles based on indices
    indices_to_remove_clone.sort();
    indices_to_remove_clone.dedup();
    indices_to_remove_clone.reverse();
    for index in indices_to_remove_clone {
        particles.remove(index);
    }

    total_kinetic_energy as u64
}


fn draw_particles(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, particles: &[Particle]) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for p in particles {
        let center_x = p.x.round() as i16;
        let center_y = p.y.round() as i16;
        
        // Draw outer circle
        canvas.filled_circle(center_x, center_y, p.rad, f32_color_to_sdl_color([1.0, 1.0, 1.0, 1.0]))?;
        
        // Draw mid-circle
        canvas.filled_circle(center_x, center_y, p.rad - 2, f32_color_to_sdl_color(p.color))?;

        // Calculate the offset based on the velocity vector
        let velocity_magnitude = (p.vx.powi(2) + p.vy.powi(2)).sqrt();
        if velocity_magnitude != 0.0 {
            let offset_x = (p.vx / velocity_magnitude * (p.rad as f64 / 4.0)).round() as i16;
            let offset_y = (p.vy / velocity_magnitude * (p.rad as f64 / 4.0)).round() as i16;

            // Draw inner circle
            canvas.filled_circle(center_x + offset_x, center_y + offset_y, (p.rad as f64 * (2.0/3.0)).round() as i16, f32_color_to_sdl_color([0.0, 0.0, 1.0, 1.0]))?;
        } else {
            // Draw inner circle
            canvas.filled_circle(center_x, center_y, (p.rad as f64 * (2.0/3.0)).round() as i16, f32_color_to_sdl_color([0.0, 0.0, 1.0, 1.0]))?;}
    }
    Ok(())
}




fn handle_events(event_pump: &mut sdl2::EventPump) -> bool {
    for event in event_pump.poll_iter() {
        if DEBUG_MODE {
            println!("Received event: {:?}", event);
        }
        match event {
            Event::Quit { .. } |
            Event::KeyUp { keycode: Some(Keycode::Escape), .. } => {
                if DEBUG_MODE {
                    println!("Exit condition met. Exiting.");
                }
                return false;
            },
            _ => {}
        }
    }
    true
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

fn main() -> Result<(), String> {
    if DEBUG_MODE {
        println!("Initialization starting...");
    }
    let init_start_time = SystemTime::now();
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("Particles", WIN_X, WIN_Y)
        .position_centered()
        .build()
        .expect("Could not initialize video subsystem");
    let mut canvas = window.into_canvas().build().expect("Could not make a canvas");
    let mut event_pump = sdl_context.event_pump()?;
    while event_pump.poll_iter().next().is_some() {}

    let mut particles = init_particles();
    let init_dur_secs = match init_start_time.elapsed() {
        Ok(duration) => duration.as_secs_f64(),
        Err(_) => 0.0, // or handle the error in some other way
    };
    let post_init_dur = Instant::now();

    let num_threads = if MULTITHREAD {
        rayon::current_num_threads()
    } else {
        1
    };
    
    if DEBUG_MODE {
        println!("Initialization complete.");
    }

    let mut last_particle_addition_time = Instant::now();
    let mut step: usize = 0;
    loop {
        if DEBUG_MODE {
            println!("Starting step {}", step);
        }
        let loop_start_time = SystemTime::now();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        let mut rng = rand::thread_rng();

        if last_particle_addition_time.elapsed() > Duration::from_millis(rng.gen_range(100..2000)) {
            last_particle_addition_time = Instant::now();
            let mut rng = rand::thread_rng();
            let mass_normal = Normal::new(GREEN_MEAN_MASS, GREEN_STDEV_MASS).unwrap();
            let mut mass = mass_normal.sample(&mut rng); // Generate random mass
            if mass <= 36.0 {
                mass = 36.0;
            }

            let rad = (mass as f64).sqrt() as i16; // Set radius based on mass
            let energy = mass;
            let new_particle = Particle {
                part_type: String::from("green"),
                x: rng.gen_range((EDGE_BUFFER ) as f64..(WIN_X - EDGE_BUFFER) as f64),
                y: rng.gen_range((EDGE_BUFFER + TOP_BUFFER) as f64..(WIN_Y - EDGE_BUFFER) as f64),
                rad: rad,
                vx: 0.0,
                vy: 0.0,
                color: [0.0, 1.0, 0.0, 1.0], // Green
                energy: energy,
                creation_time: Instant::now(),
                mass: mass
            };
            particles.push(new_particle);
        }

        let mut total_kinetic_energy = update_particles(&mut particles);


        draw_particles(&mut canvas, &particles)?;

        let elapsed_time = loop_start_time.elapsed()
            .expect("Time went backwards")
            .as_millis() as u64;

        if elapsed_time < FRAME_DUR {
            sleep(Duration::from_millis(FRAME_DUR - elapsed_time));
        }

        let fps = 1000.0 / loop_start_time.elapsed()
            .expect("Time went backwards")
            .as_millis() as f64;

        let post_init_dur_secs = post_init_dur.elapsed().as_secs_f64();


        canvas.present();

        print!("\rThreads: {}, Init Dur: {:.2}s, Step: {}, Post-Init Dur: {:.2}s, FPS: {:.2}, Particles: {}, Tot Kinetic Energy: {}", 
        num_threads, init_dur_secs, step, post_init_dur_secs, fps, particles.len(), total_kinetic_energy.to_formatted_string(&Locale::en));
        io::stdout().flush().unwrap();

        if !handle_events(&mut event_pump) {
            break;
        }

        step += 1;
        if DEBUG_MODE && step >= DEBUG_MAX_ITER {
            println!("Max iterations reached in debug mode. Exiting.");
            break;
        }
    }
    if DEBUG_MODE {
        println!("Exiting program.");
    }
    println!();
    sleep(Duration::from_millis(300));
    Ok(())
}