// Import Rayon for parallel processing
use log::{debug, error, info, trace, warn, LevelFilter};
use rand::Rng;
use rand_distr::{Distribution, Normal};
use rayon::prelude::*;
use std::time::{Duration, SystemTime};

use crate::constants::{ENV_SEED, ENV_STEP, FULLSCREEN, HEIGHT, LOG_LEVEL, WIDTH, NUM_CELLS, FRAME_DUR, COLLIDE_SPRING, FRICTION_COEFF};
use crate::utils::ui_util::{hsva_to_rgba, rgba_to_hsba, UIContext, render_terrain};
use crate::utils::math_util::{velocity_to_polar, polar_to_velocity, gradient_along_heading, gradient_perpendicular_heading};

pub struct Cell {
    pub id: i64,
    pub parent_id: Option<i64>,
    pub creation_step: i64,
    pub age: i64,
    pub alive: bool,
    pub reproducing: bool,
    pub reproduce_now: bool,
    pub x_pos: f64,
    pub y_pos: f64,
    pub x_vel: f64,
    pub y_vel: f64,
    pub heading: f64,
    pub speed: f64,
    pub mass: f64,
    pub radius: f64,
    pub health: f64,
    pub health_capacity: f64,
    pub wasting_rate: f64,
    pub energy: f64,
    pub energy_capacity: f64,
    pub metabolic_rate: f64,
    pub light_exposure: f64,
    pub light_consumtion_efficiency: f64,
    pub reproduction_cost: f64,
    pub reproduction_progress: f64,
    pub membrane_color: [u8; 4],
    pub inside_color: [u8; 4],
    pub gravity_gradient_along_heading: f64,
    pub gravity_gradient_perpendicular_heading: f64,


    // pub decayed: bool,
    // pub max_speed: f64,
    // pub max_acceleration: f64,
    // pub max_turn_rate: f64,
    // pub max_turn_acceleration: f64,
    // pub light_consumtion_capacity: f64,
    // pub reproductive_capacity: i64,






}

impl Cell {
    pub fn new(id: i64, loop_step: i64) -> Self {
        // Initialize a new cell
        let mut rng = rand::thread_rng();
        let mut mass: f64 = rng.gen_range(9.0..81.0);
        let mut radius: f64 = (mass / 3.1415).sqrt();
        let x_vel: f64 = rng.gen_range(-0.5..0.5);
        let y_vel: f64 = rng.gen_range(-0.5..0.5);
        let mut membrane_color = hsva_to_rgba(rng.gen_range(0.0..360.0), 1.0, 1.0, 1.0);
        let mut inside_color = hsva_to_rgba(rng.gen_range(0.0..360.0), 1.0, 1.0, 1.0);
        if id == 1 {
            membrane_color = hsva_to_rgba(0.0, 0.0, 1.0, 1.0);
            inside_color = hsva_to_rgba(0.0, 0.0, 1.0, 1.0);
            mass = 100.0;
            radius = (mass / 3.1415).sqrt();
        } 
        let (heading, speed) = velocity_to_polar(x_vel, y_vel);
        Self {
            id,
            parent_id: None,
            creation_step: loop_step,
            age: 0,
            alive: true,
            reproducing: false,
            reproduce_now: false,
            x_pos: rng.gen_range((0.0 + radius)..(WIDTH as f64 - radius)),
            y_pos: rng.gen_range((0.0 + radius)..(HEIGHT as f64 - radius)),
            x_vel,
            y_vel,
            heading,
            speed,
            mass,
            radius,
            health: 100.0,
            health_capacity: 100.0,
            wasting_rate: 0.1,
            energy: 100.0,
            energy_capacity: 100.0,
            metabolic_rate: 0.01,
            light_consumtion_efficiency: 1.0,
            light_exposure: 0.0,
            reproduction_cost: 50.0,
            reproduction_progress: 0.0,
            membrane_color,
            inside_color,
            gravity_gradient_along_heading: 0.0,
            gravity_gradient_perpendicular_heading: 0.0,
        }
    }

    pub fn new_from_reproduction(id: i64, parent_id: Option<i64>, creation_step: i64, mass: f64, x_pos: f64, y_pos: f64, x_vel: f64, y_vel: f64, membrane_color: [u8; 4], inside_color: [u8; 4]) -> Self {
        let mut rng = rand::thread_rng();
        let mut radius: f64 = (mass / 3.1415).sqrt();
        let [r, g, b, a] = membrane_color;
        let (mut h, s, v, a) = rgba_to_hsba(r, g, b, a);
        h += rng.gen_range(-0.01..0.01);
        let mut membrane_color = hsva_to_rgba(h, s, v, a);
        let [r, g, b, a] = inside_color;
        let (mut h, s, v, a) = rgba_to_hsba(r, g, b, a);
        h += rng.gen_range(-0.01..0.01);
        let mut inside_color = hsva_to_rgba(h, s, v, a);
        let (heading, speed) = velocity_to_polar(x_vel, y_vel);
        Self {
            id,
            parent_id,
            creation_step,
            age: 0,
            alive: true,
            reproducing: false,
            reproduce_now: false,
            x_pos,
            y_pos,
            x_vel,
            y_vel,
            heading,
            speed,
            mass,
            radius,
            health: 100.0,
            health_capacity: 100.0,
            wasting_rate: 0.1,
            energy: 100.0,
            energy_capacity: 100.0,
            metabolic_rate: 0.01,
            light_consumtion_efficiency: 1.0,
            light_exposure: 0.0,
            reproduction_cost: 50.0,
            reproduction_progress: 0.0,
            membrane_color,
            inside_color,
            gravity_gradient_along_heading: 0.0,
            gravity_gradient_perpendicular_heading: 0.0,
        }
    }

    pub fn update(&mut self, terrain: &[Vec<(f64)>], gradient: &[Vec<(f64, f64)>], loop_step: i64) {
        trace!("cell::update >> Updating cell with id: {}, parent_id: {:?}, creation_time: {}, age: {}, x_pos: {}, y_pos: {}, x_vel: {}, y_vel: {}, mass: {}, radius: {}, inside_color: {}",
            self.id, self.parent_id, self.creation_step, self.age, self.x_pos, self.y_pos, self.x_vel, self.y_vel, self.mass, self.radius, self.inside_color[0]);
        self.update_age(loop_step);
        self.update_velocity(gradient);
        self.update_position();
        self.handle_boundary_collision();
        self.update_gravity_gradient_sense(gradient);
        self.update_light_exposure_sense(terrain);
        self.update_energy();
        self.update_health();
        self.update_and_check_reproduction();
        if self.id == 1 {
            self.print_cell_properties();
        }
    }

    pub fn update_and_check_reproduction(&mut self){
        if self.energy >= self.reproduction_cost * 1.5 {
            self.reproducing = true;
        } 
        
        if self.reproducing {
            self.energy -= self.reproduction_cost * 0.01;
            self.reproduction_progress += 0.01;
        }
        if self.reproduction_progress >= 1.0 {
            self.reproduction_progress = 0.0;
            self.reproducing = false;
            self.reproduce_now = true;
        }

    }
    pub fn update_light_exposure_sense (&mut self, terrain: &[Vec<(f64)>]) {
        self.light_exposure = terrain[self.x_pos.round() as usize][self.y_pos.round() as usize];
    }

    pub fn update_energy(&mut self) {
        self.energy -= self.metabolic_rate * self.mass;
        self.energy += self.light_exposure * self.light_consumtion_efficiency * 100.0;
        if self.energy <= 0.0 {
            self.energy = 0.0;
        } else if self.energy >= self.energy_capacity {
            self.energy = self.energy_capacity;
        }
            
        
    }

    pub fn update_health(&mut self) {
        if self.energy <= 0.33 * self.energy_capacity {
            self.health -= self.energy_capacity/self.energy * self.wasting_rate;
            if self.energy <= 0.1 * self.energy_capacity {
                self.health -= 10.0 * self.wasting_rate;
            } 
        }
        if self.energy >= 0.66 * self.energy_capacity {
            self.health += self.energy/self.energy_capacity * self.wasting_rate;
        } 
        if self.health <= 0.0 {
            self.health = 0.0;
            self.alive = false;
        } else if self.health >= self.health_capacity {
            self.health = self.health_capacity;
        }
    }
    pub fn print_cell_properties(&self) {
        println!("Cell Properties for ID {}:", self.id);
        println!("  Parent ID: {:?}", self.parent_id);
        println!("  Creation Step: {}", self.creation_step);
        println!("  Age: {}", self.age);
        println!("  Alive: {}", self.alive);
        println!("  Position: ({:.6}, {:.6})", self.x_pos, self.y_pos);
        println!("  Velocity: ({:.6}, {:.6})", self.x_vel, self.y_vel);
        println!("  Heading: {:.6}", self.heading);
        println!("  Speed: {:.6}", self.speed);
        println!("  Mass: {:.6}", self.mass);
        println!("  Radius: {:.6}", self.radius);
        println!("  Health: {:.6}/{:.6}", self.health, self.health_capacity);
        println!("  Wasting Rate: {:.6}", self.wasting_rate);
        println!("  Energy: {:.6}/{:.6}", self.energy, self.energy_capacity);
        println!("  Metabolic Rate: {:.6}", self.metabolic_rate);
        println!("  Light Exposure: {:.6}", self.light_exposure);
        println!("  Light Consumption Efficiency: {:.6}", self.light_consumtion_efficiency);
        println!("  Membrane Color: {:?}", self.membrane_color);
        println!("  Inside Color: {:?}", self.inside_color);
        println!("  Gravity Gradient Along Heading: {:.6}", self.gravity_gradient_along_heading);
        println!("  Gravity Gradient Perpendicular Heading: {:.6}", self.gravity_gradient_perpendicular_heading);
        println!("  Reproducing: {}", self.reproducing);
        println!("  Reproduce Now: {}", self.reproduce_now);
        println!("  Reproduction Cost: {:.6}", self.reproduction_cost);
        println!("  Reproduction Progress: {:.6}", self.reproduction_progress);
        println!();  
    }

    pub fn update_gravity_gradient_sense(&mut self, gradient: &[Vec<(f64, f64)>]) {
        let (g_x, g_y) = gradient[self.x_pos.round() as usize][self.y_pos.round() as usize];
        let gradient_along = gradient_along_heading((g_x, g_y), self.heading);
        let gradient_perpendicular = gradient_perpendicular_heading((g_x, g_y), self.heading);
        self.gravity_gradient_along_heading = gradient_along;
        self.gravity_gradient_perpendicular_heading = gradient_perpendicular;
    }

    pub fn handle_cell_collision(&mut self, cell2: &mut Cell) {
        let dx = self.x_pos - cell2.x_pos;
        let dy = self.y_pos - cell2.y_pos;
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

            self.x_vel -= ax1 * nx;
            self.y_vel -= ay1 * ny;
            cell2.x_vel += ax2 * nx;
            cell2.y_vel += ay2 * ny;
        }
    }

    pub fn update_velocity(&mut self, gradient: &[Vec<(f64, f64)>]) {
        // Assume self.x_pos and self.y_pos are usize for indexing into the gradient
        let (dx, dy) = gradient[self.x_pos.round() as usize][self.y_pos.round() as usize];
    
        // Update velocities
        self.x_vel += dx;
        self.y_vel += dy;

        (self.heading, self.speed) = velocity_to_polar(self.x_vel, self.y_vel);
    
    }
    
    pub fn update_position(&mut self) {
        let mut rng = rand::thread_rng();
        let brownian_motion = 0.01;

        self.x_vel *= (1.0 - FRICTION_COEFF);
        self.y_vel *= (1.0 - FRICTION_COEFF);

        self.x_vel += rng.gen_range(-brownian_motion..brownian_motion);
        self.x_vel += rng.gen_range(-brownian_motion..brownian_motion);

        self.x_pos = self.x_pos + self.x_vel;
        self.y_pos = self.y_pos + self.y_vel;
    }
    

    

    pub fn update_age(&mut self, loop_step: i64) {
        self.age = loop_step - self.creation_step;
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
pub fn update_cells(cells: &mut Vec<Cell>, terrain: &[Vec<f64>], gradient: &[Vec<(f64, f64)>], loop_step: i64) {
    let mut num_cells_updated = 0;
    let len = cells.len();
    for i in 0..len {
        for j in (i + 1)..len {
            let (left, right) = cells.split_at_mut(i + 1);
            let cell1 = &mut left[i];
            let cell2 = &mut right[j - i - 1];
            cell1.handle_cell_collision(cell2);        }
    }
    
    cells.iter_mut().for_each(|cell| {
        cell.update(terrain, gradient, loop_step);
        num_cells_updated += 1;
    });
    println!("Number of cells updated: {}", num_cells_updated);
    println!();
}

pub fn reproduce_now(cells: &mut Vec<Cell>, id: i64, loop_step: i64) {
    let mut rng = rand::thread_rng();
    let mut new_cells: Vec<Cell> = Vec::new();
    let mut max_id = 0;
    let mut indices_to_add = Vec::new();
    for (index, cell) in cells.iter_mut().enumerate() {
        if cell.id > max_id {
            max_id = cell.id;
        }
        if cell.reproduce_now {
            indices_to_add.push(index);
            cell.reproduce_now = false;
        }
    }
    for index in indices_to_add {
        let cell = &cells[index];
        if cell.reproduce_now {
            let mut new_cell = Cell::new(id, loop_step);
            new_cell.parent_id = Some(cell.id as i64);
            new_cells.push(new_cell);
            let child_mass = cell.mass;
            cells.push(Cell::new_from_reproduction(max_id + 1 as i64, cell.parent_id, loop_step, child_mass, cell.x_pos, cell.y_pos, cell.x_vel, cell.y_vel, cell.membrane_color, cell.inside_color));
            max_id += 1;
        }
    }
}
