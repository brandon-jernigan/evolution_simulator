// Import Rayon for parallel processing
use log::{debug, error, info, trace, warn, LevelFilter};
use rand::Rng;
use rand_distr::{Distribution, Normal};
use rayon::prelude::*;
use std::time::{Duration, SystemTime};

use crate::constants::{ENV_SEED, ENV_STEP, FULLSCREEN, HEIGHT, LOG_LEVEL, WIDTH, NUM_CELLS, FRAME_DUR, COLLIDE_SPRING, COLLIDE_DAMPING, FRICTION_COEFF};
use crate::utils::ui_util::{hsba_to_rgba, UIContext};

pub struct Cell {
    pub id: i32,
    pub parent_id: Option<i32>,
    pub creation_time: std::time::SystemTime,
    pub age: Duration,
    pub alive: bool,
    pub x_pos: f64,
    pub y_pos: f64,
    pub x_vel: f64,
    pub y_vel: f64,
    pub mass: f64,
    pub radius: f64,
    pub inside_color: [u8; 4],
}

impl Cell {
    pub fn new(id: i32) -> Self {
        // Initialize a new cell
        let mut rng = rand::thread_rng();
        let mass: f64 = rng.gen_range(16.0..100.0);
        let radius: f64 = (mass / 3.14).sqrt();
        Self {
            id,
            parent_id: None,
            creation_time: SystemTime::now(),
            age: Duration::new(0, 0),
            alive: true,
            x_pos: rng.gen_range((0.0 + radius)..(WIDTH as f64 - radius)),
            y_pos: rng.gen_range((0.0 + radius)..(HEIGHT as f64 - radius)),
            x_vel: rng.gen_range(-0.5..0.5),
            y_vel: rng.gen_range(-0.5..0.5),
            mass,
            radius,
            inside_color: hsba_to_rgba(rng.gen_range(0.0..180.0), 1.0, rng.gen_range(0.5..0.8), 1.0),
            // Initialize other attributes
        }
    }

    pub fn update(&mut self, gradient: &[Vec<(f64, f64)>]) {
        trace!("cell::update >> Updating cell with id: {}, parent_id: {:?}, creation_time: {}, age: {}, x_pos: {}, y_pos: {}, x_vel: {}, y_vel: {}, mass: {}, radius: {}, inside_color: {}",
            self.id, self.parent_id, self.creation_time.elapsed().unwrap().as_millis(), self.age.as_millis(), self.x_pos, self.y_pos, self.x_vel, self.y_vel, self.mass, self.radius, self.inside_color[0]);
        self.update_age();
        self.handle_boundary_collision();
        self.update_velocity(gradient);
        self.update_position();
    }

    pub fn handle_cell_collision(&mut self, cell2: &mut Cell) {
        let dx = self.x_pos - cell2.x_pos;
        let dy = self.y_pos - cell2.y_pos;
        let dt = FRAME_DUR as f64 / 1000.0;
        let distance_squared = dx * dx + dy * dy;
        
        let min_dist = (self.radius + cell2.radius) as f64;
    
        if distance_squared < min_dist * min_dist {
            let distance = distance_squared.sqrt();
            let overlap = min_dist - distance;

            // normalize dx and dy
            let nx = dx / distance;
            let ny = dy / distance;

            let force = overlap * COLLIDE_SPRING;

            let ax1 = force / self.mass as f64;
            let ay1 = force / self.mass as f64;

            let ax2 = force / cell2.mass as f64;
            let ay2 = force / cell2.mass as f64;

            self.x_vel -= ax1 * nx * dt * COLLIDE_DAMPING;
            self.y_vel -= ay1 * ny * dt * COLLIDE_DAMPING;
            cell2.x_vel += ax2 * nx * dt * COLLIDE_DAMPING;
            cell2.y_vel += ay2 * ny * dt * COLLIDE_DAMPING;
        }
    }

    pub fn update_velocity(&mut self, gradient: &[Vec<(f64, f64)>]) {
        // Assume self.x_pos and self.y_pos are usize for indexing into the gradient
        let (dx, dy) = gradient[self.x_pos.round() as usize][self.y_pos.round() as usize];
    
        // Update velocities
        self.x_vel += dx;
        self.y_vel += dy;
    
    }
    
    pub fn update_position(&mut self) {
        self.x_vel *= (1.0 - FRICTION_COEFF);
        self.y_vel *= (1.0 - FRICTION_COEFF);

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
        if self.x_pos + self.radius >= WIDTH as f64 {
            self.x_pos = WIDTH as f64 - self.radius;
            self.x_vel = -self.x_vel.abs();
        }
        // Left boundary
        if self.x_pos - self.radius <= 0.0 {
            self.x_pos = self.radius;
            self.x_vel = self.x_vel.abs();
        }
        // Bottom boundary
        if self.y_pos + self.radius >= HEIGHT as f64 {
            self.y_pos = HEIGHT as f64 - self.radius;
            self.y_vel = -self.y_vel.abs();
        }
        // Top boundary
        if self.y_pos - self.radius <= 0.0 {
            self.y_pos = self.radius;
            self.y_vel = self.y_vel.abs();
        }
    }
}

// Function to update cells in parallel
pub fn update_cells(cells: &mut [Cell], _terrain: &[Vec<f64>], gradient: &[Vec<(f64, f64)>]) {
    let len = cells.len();
    for i in 0..len {
        for j in (i + 1)..len {
            let (left, right) = cells.split_at_mut(i + 1);
            let cell1 = &mut left[i];
            let cell2 = &mut right[j - i - 1];
            cell1.handle_cell_collision(cell2);        }
    }
    
    cells.iter_mut().for_each(|cell| {
        cell.update(gradient);
    });
}

