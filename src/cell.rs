// Import Rayon for parallel processing
use log::debug; // Make sure to import debug
use rand::Rng;
use rand_distr::{Distribution, Normal};
use rayon::prelude::*;
use std::time::{Duration, SystemTime};

use crate::constants::{ENV_SEED, ENV_STEP, FULLSCREEN, HEIGHT, LOG_LEVEL, WIDTH};
use crate::utils::ui_util::{hsba_to_rgba, UIContext};

pub struct Cell {
    pub id: i32,
    pub parent_id: Option<i32>,
    pub creation_time: std::time::SystemTime,
    pub age: Duration,
    pub alive: bool,
    pub x_pos: i32,
    pub y_pos: i32,
    pub x_vel: i32,
    pub y_vel: i32,
    pub mass: f32,
    pub radius: i32,
    pub inside_color: [u8; 4],
}

impl Cell {
    pub fn new(id: i32) -> Self {
        // Initialize a new cell
        let mut rng = rand::thread_rng();
        let mass = rng.gen_range(100.0..10000.0);
        let radius = (mass / 3.14_f32).sqrt().round() as i32;
        Self {
            id,
            parent_id: None,
            creation_time: SystemTime::now(),
            age: Duration::new(0, 0),
            alive: true,
            x_pos: rng.gen_range(0..WIDTH as i32),
            y_pos: rng.gen_range(0..HEIGHT as i32),
            x_vel: rng.gen_range(-2..2),
            y_vel: rng.gen_range(-2..2),
            mass,
            radius,
            inside_color: hsba_to_rgba(rng.gen_range(0.0..360.0), 1.0, 1.0, 1.0),
            // Initialize other attributes
        }
    }

    pub fn update(&mut self) {
        debug!("Updating cell with ID: {}", self.id);
        self.update_age();
        self.handle_boundary_collision();
        self.update_position();
    }

    pub fn update_position(&mut self) {
        self.x_pos = self.x_pos + self.x_vel;
        self.y_pos = self.y_pos + self.y_vel;
    }

    pub fn update_age(&mut self) -> Result<(), &'static str> {
        match SystemTime::now().duration_since(self.creation_time) {
            Ok(duration) => {
                self.age = duration;
                Ok(())
            }
            Err(_) => Err("System time is earlier than creation time; could not update age."),
        }
    }

    pub fn handle_boundary_collision(&mut self) {
        // Right boundary
        if self.x_pos + self.radius >= WIDTH as i32 {
            self.x_vel = -self.x_vel.abs();
        }
        // Left boundary
        if self.x_pos - self.radius <= 0 {
            self.x_vel = self.x_vel.abs();
        }
        // Bottom boundary
        if self.y_pos + self.radius >= HEIGHT as i32 {
            self.y_vel = -self.y_vel.abs();
        }
        // Top boundary
        if self.y_pos - self.radius <= 0 {
            self.y_vel = self.y_vel.abs();
        }
    }
}

// Function to update cells in parallel
pub fn update_cells(cells: &mut [Cell]) {
    cells.par_iter_mut().for_each(|cell| {
        cell.update();
    });
}
