// Import Rayon for parallel processing
use log::{debug, error, info, trace, warn, LevelFilter};
use rand::Rng;
use rand_distr::num_traits::float;
use rand_distr::{Distribution, Normal};
use rayon::prelude::*;
use std::time::{Duration, SystemTime};

use crate::constants::{ENV_SEED, ENV_STEP, FULLSCREEN, HEIGHT, LOG_LEVEL, WIDTH, NUM_CELLS, TARGET_FRAME_RATE, FRAME_DUR, COLLIDE_SPRING, POST_REPRODUCTION_COLLIDE_SPRING, FRICTION_COEFF, PI};
use crate::utils::ui_util::{hsva_to_rgba, rgba_to_hsva, UIContext, render_terrain};
use crate::utils::math_util::{velocity_to_polar, polar_to_velocity, gradient_along_heading, gradient_perpendicular_heading, generate_non_zero_integer, generate_random_position};

pub struct Cell {
    pub id: i64,
    pub parent_id: i64,
    pub creation_step: i64,
    pub age: i64,
    pub alive: bool,
    pub reproducing: bool,
    pub reproduce_now: bool,
    pub last_reproduction_age: i64,
    pub x_pos: f64,
    pub y_pos: f64,
    pub x_vel: f64,
    pub y_vel: f64,
    pub x_acc: f64,
    pub y_acc: f64,
    pub heading: f64,
    pub speed: f64,
    pub mass: f64,
    pub radius: f64,
    pub health: f64,
    pub health_capacity: f64,
    pub health_restore_rate: f64,
    pub health_decay_rate: f64,
    pub energy: f64,
    pub energy_capacity: f64,
    pub energy_decay_rate: f64,
    pub light_exposure: f64,
    pub light_consumtion_efficiency: f64,
    pub reproduction_cost: f64,
    pub reproduction_progress: f64,
    pub membrane_color: [u8; 4],
    pub inside_color: [u8; 4],
    pub nucleus_color: [u8; 4],
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
        let mut mass: f64 = rng.gen_range(81.0..256.0);
        let mut radius: f64 = (mass / PI).sqrt();
        let x_vel: f64 = rng.gen_range(-0.5..0.5);
        let y_vel: f64 = rng.gen_range(-0.5..0.5);
        let mut membrane_color = hsva_to_rgba(rng.gen_range(0.0..1.0), 1.0, 1.0, 1.0);
        let mut inside_color = hsva_to_rgba(rng.gen_range(0.0..1.0), 1.0, 1.0, 1.0);
        let mut nucleus_color = hsva_to_rgba(rng.gen_range(0.0..1.0), 1.0, 1.0, 1.0);
        let light_consumtion_efficiency = mass / 2000.0;
        let reproduction_progress = rng.gen_range(0.0..0.5);
        if id == 1 {
            membrane_color = hsva_to_rgba(0.0, 0.0, 1.0, 1.0);
            inside_color = hsva_to_rgba(0.0, 0.0, 1.0, 1.0);
            mass = 256.0;
            radius = (mass / PI).sqrt();
        } 
        let (heading, speed) = velocity_to_polar(x_vel, y_vel);
        Self {
            id,
            parent_id: -1,
            creation_step: loop_step,
            age: 0,
            alive: true,
            reproducing: false,
            reproduce_now: false,
            last_reproduction_age: 0,
            x_pos: rng.gen_range((0.0 + radius)..(WIDTH as f64 - radius)),
            y_pos: rng.gen_range((0.0 + radius)..(HEIGHT as f64 - radius)),
            x_vel,
            y_vel,
            x_acc: 0.0,
            y_acc: 0.0,
            heading,
            speed,
            mass,
            radius,
            health: 100.0,
            health_capacity: 100.0,
            health_restore_rate: 0.02,
            health_decay_rate: 0.01,
            energy: 100.0,
            energy_capacity: 100.0,
            energy_decay_rate: 0.01,
            light_consumtion_efficiency,
            light_exposure: 0.0,
            reproduction_cost: mass,
            reproduction_progress,
            membrane_color,
            inside_color,
            nucleus_color,
            gravity_gradient_along_heading: 0.0,
            gravity_gradient_perpendicular_heading: 0.0,
        }
    }

    pub fn new_from_reproduction(id: i64, parent_id: i64, creation_step: i64, mass: f64, x_pos: f64, y_pos: f64, x_vel: f64, y_vel: f64, membrane_color: [u8; 4], inside_color: [u8; 4], nucleus_color: [u8; 4], reproductive_cost: f64) -> Self {
        let mut rng = rand::thread_rng();
        let color_mutate_magnitude = 0.04;
        let mut radius: f64 = (mass / PI).sqrt();
        let [r, g, b, a] = membrane_color;
        let (mut h, s, v, a) = rgba_to_hsva(r, g, b, a);
        h += rng.gen_range(-color_mutate_magnitude..color_mutate_magnitude);
        let mut membrane_color = hsva_to_rgba(h, s, v, a);
        let [r, g, b, a] = inside_color;
        let (mut h, s, v, a) = rgba_to_hsva(r, g, b, a);
        h += rng.gen_range(-color_mutate_magnitude..color_mutate_magnitude);
        let mut inside_color = hsva_to_rgba(h, s, v, a);
        let [r, g, b, a] = nucleus_color;
        let (mut h, s, v, a) = rgba_to_hsva(r, g, b, a);
        h += rng.gen_range(-color_mutate_magnitude..color_mutate_magnitude);
        let mut nucleus_color = hsva_to_rgba(h, s, v, a);
        let (heading, speed) = velocity_to_polar(x_vel, y_vel);
        let mut new_reproductive_cost = 0.0;
        while new_reproductive_cost < 36.0 {
            new_reproductive_cost = reproductive_cost + rng.gen_range(-2.0..2.0);
        }
        let light_consumtion_efficiency = mass / 2000.0;
        Self {
            id,
            parent_id,
            creation_step,
            age: 0,
            alive: true,
            reproducing: false,
            reproduce_now: false,
            last_reproduction_age: 0,
            x_pos,
            y_pos,
            x_vel,
            y_vel,
            x_acc: 0.0,
            y_acc: 0.0,
            heading,
            speed,
            mass,
            radius,
            health: 100.0,
            health_capacity: 100.0,
            health_restore_rate: 0.2,
            health_decay_rate: 0.1,
            energy: 100.0,
            energy_capacity: 100.0,
            energy_decay_rate: 0.01,
            light_consumtion_efficiency: light_consumtion_efficiency,
            light_exposure: 0.0,
            reproduction_cost: new_reproductive_cost,
            reproduction_progress: 0.0,
            membrane_color,
            inside_color,
            nucleus_color,
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
        self.update_and_check_reproduction();
        self.update_health();
        self.update_energy();
        if self.id == 1 {
            //self.print_cell_properties();
        }
        self.update_light_exposure_sense(terrain);
    }
    pub fn cell_freq(&mut self) -> f32{
        let base_frequency = 440.0;
        let freq = base_frequency * self.mass;
        return freq as f32;
    }

    pub fn update_and_check_reproduction(&mut self){
        let mut rng = rand::thread_rng();
        if self.energy >= self.energy_capacity * 0.1 {
            self.reproducing = true;
        } else {
            self.reproducing = false;
        }
        
        if self.reproducing {
            let reproduction_variation = rng.gen_range(0.5..1.5);
            self.energy -= self.reproduction_cost * 0.02 * reproduction_variation;
            self.reproduction_progress += 0.02 * reproduction_variation;
            self.mass += self.reproduction_cost * 0.02 * reproduction_variation;
            self.radius = (self.mass / PI).sqrt();
            self.light_consumtion_efficiency = self.mass / 2000.0;
        }
        if self.reproduction_progress >= 1.0 {
            self.reproduction_progress = 0.0;
            self.reproducing = false;
            self.reproduce_now = true;
            self.last_reproduction_age = self.age;
        }

    }
    pub fn update_light_exposure_sense (&mut self, terrain: &[Vec<(f64)>]) {
        self.light_exposure = terrain[self.x_pos.round() as usize][self.y_pos.round() as usize];
    }

    pub fn update_energy(&mut self) {
        self.energy -= self.energy_decay_rate * self.mass;
        self.energy -= f64::min(self.health_restore_rate * self.health_capacity, self.health_capacity - self.health);
        self.energy += self.light_exposure * self.light_consumtion_efficiency * 100.0;

        if self.energy <= 0.0 {
            self.energy = 0.0;
        } else if self.energy >= self.energy_capacity {
            self.energy = self.energy_capacity;
        }
            
        
    }

    pub fn update_health(&mut self) {

        self.health -= f64::min(self.health_decay_rate * self.health_capacity, self.health);
        self.health += f64::min(f64::min(self.health_restore_rate * self.health_capacity, self.health_capacity - self.health), self.energy);

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
        println!("  Position: ({:.3}, {:.3})", self.x_pos, self.y_pos);
        println!("  Velocity: ({:.3}, {:.3})", self.x_vel, self.y_vel);
        println!("  Heading: {:.3}", self.heading);
        println!("  Speed: {:.6}", self.speed);
        println!("  Mass: {:.1}", self.mass);
        println!("  Radius: {:.1}", self.radius);
        println!("  Health: {:.1}/{:.1}", self.health, self.health_capacity);
        println!("  Health Decay Rate: {:.3}", self.health_decay_rate);
        println!("  Energy: {:.1}/{:.1}", self.energy, self.energy_capacity);
        println!("  Energy Decay Rate: {:.3}", self.energy_decay_rate);
        println!("  Light Exposure: {:.3}", self.light_exposure);
        println!("  Light Consumption Efficiency: {:.3}", self.light_consumtion_efficiency);
        println!("  Membrane Color: {:?}", self.energy_decay_rate);
        println!("  Inside Color: {:?}", self.inside_color);
        println!("  Gravity Gradient Along Heading: {:.6}", self.gravity_gradient_along_heading);
        println!("  Gravity Gradient Perpendicular Heading: {:.6}", self.gravity_gradient_perpendicular_heading);
        println!("  Reproducing: {}", self.reproducing);
        println!("  Reproduce Now: {}", self.reproduce_now);
        println!("  Reproduction Cost: {:.1}", self.reproduction_cost);
        println!("  Reproduction Progress: {:.3}", self.reproduction_progress);
        println!();  
    }

    pub fn update_gravity_gradient_sense(&mut self, gradient: &[Vec<(f64, f64)>]) {
        let (g_x, g_y) = gradient[self.x_pos.round() as usize][self.y_pos.round() as usize];
        let gradient_along = gradient_along_heading((g_x, g_y), self.heading);
        let gradient_perpendicular = gradient_perpendicular_heading((g_x, g_y), self.heading);
        self.gravity_gradient_along_heading = gradient_along;
        self.gravity_gradient_perpendicular_heading = gradient_perpendicular;
    }

    pub fn handle_cell_collision(&mut self, cell2: &mut Cell, terrain: &[Vec<(f64)>]) {
        let dx = self.x_pos - cell2.x_pos;
        let dy = self.y_pos - cell2.y_pos;
        let area_overlap: f64;
        let mut distance_squared = dx * dx + dy * dy;
        if distance_squared == 0.0 {
            distance_squared = 0.1;
        }
        
        let min_dist = (self.radius + cell2.radius) as f64;
        if distance_squared < min_dist * min_dist {
            let distance = distance_squared.sqrt();
            let overlap = min_dist - distance;
            let small_rad = f64::min(self.radius, cell2.radius);
            let big_rad = f64::max(self.radius, cell2.radius);

            if (distance > big_rad - small_rad) && (distance < big_rad + small_rad) {
                area_overlap = PI * (small_rad * (big_rad + small_rad - distance)/(2.0*small_rad)).powf(2.0);
            } else if distance <= big_rad - small_rad{
                area_overlap = PI * small_rad * small_rad;
            } else {
                area_overlap = 0.0;
            }

            let self_percent_overlap = area_overlap / (PI * self.radius * self.radius);
            let cell2_percent_overlap = area_overlap / (PI * cell2.radius * cell2.radius);

            self.light_exposure = self.light_exposure * (1.0 - self_percent_overlap);
            cell2.light_exposure = cell2.light_exposure * (1.0 - self_percent_overlap);

            // normalize dx and dy
            let nx = dx / distance;
            let ny = dy / distance;

            let mut force: f64;
            
            if (self.age - self.last_reproduction_age <= 30) && (cell2.age - cell2.last_reproduction_age <= 30) && ((self.id == cell2.parent_id) || (self.parent_id == cell2.id)) {
                force = overlap * POST_REPRODUCTION_COLLIDE_SPRING;
            } else {
                force = overlap * COLLIDE_SPRING;
            }

            let ax1 = force / self.mass as f64;
            let ay1 = force / self.mass as f64;

            let ax2 = force / cell2.mass as f64;
            let ay2 = force / cell2.mass as f64;

            // self.x_acc -= ax1 * nx;
            // self.y_acc -= ax1 * nx;
            // cell2.x_acc += ax2 * nx;
            // cell2.y_acc += ay2 * ny;

            self.x_vel -= ax1 * nx;
            self.y_vel -= ay1 * ny;
            cell2.x_vel += ax2 * nx;
            cell2.y_vel += ay2 * ny;
        }
    }

    pub fn update_velocity(&mut self, gradient: &[Vec<(f64, f64)>]) {
        // Assume self.x_pos and self.y_pos are usize for indexing into the gradient
        let (dx, dy) = gradient[self.x_pos.round() as usize][self.y_pos.round() as usize];

        //self.x_acc += dx;
        //self.y_acc += dy;
    
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
pub fn update_cells(cells: &mut Vec<Cell>, terrain: &[Vec<f64>], gradient: &[Vec<(f64, f64)>], loop_step: i64) -> Vec<f32> {
    let mut num_cells_updated = 0;
    // let sample_rate = 44100;
    // let samples_per_frame = sample_rate / TARGET_FRAME_RATE;
    // let mut amplitude_sequence = vec![0.0; samples_per_frame as usize];
    // let mut amplitude_mult = 0.001;

    let amplitude_sequence =reproduce_now(cells, loop_step);
    remove_dead_cells(cells);
    let len = cells.len();
    for i in 0..len {
        for j in (i + 1)..len {
            let (left, right) = cells.split_at_mut(i + 1);
            let cell1 = &mut left[i];
            let cell2 = &mut right[j - i - 1];
            cell1.handle_cell_collision(cell2, terrain);        }
    }
    for cell in cells.iter_mut() {
        cell.update(terrain, gradient, loop_step);
        // if cell.id == 1 {
                
        //     for i in 0..samples_per_frame {
        //         let t = i as f32 / sample_rate as f32;  // time in seconds
        //         let sample_value = amplitude_mult * f32::sin(2.0 * std::f32::consts::PI * cell.cell_freq() * t);
        //         amplitude_sequence[i as usize] += sample_value.abs();
        //     }
        // } else {
        //     for i in 0..samples_per_frame {
        //         let t = i as f32 / sample_rate as f32;  // time in seconds
        //         let sample_value = amplitude_mult * f32::sin(2.0 * std::f32::consts::PI * cell.cell_freq() * t);
        //         amplitude_sequence[i as usize] += sample_value.abs();
        //     }
        // }
        // cell.x_acc = 0.0;
        // cell.y_acc = 0.0;
        num_cells_updated += 1;
    }    
    println!("Number of cells updated: {}", num_cells_updated);
    println!();
    //normalize_amplitude(&mut amplitude_sequence);
    return amplitude_sequence;
}

pub fn reproduce_now(cells: &mut Vec<Cell>, loop_step: i64) -> Vec<f32> {
    let sample_rate = 44100;
    let samples_per_frame = sample_rate / TARGET_FRAME_RATE;
    let mut amplitude_sequence = vec![0.0; samples_per_frame as usize];
    let mut amplitude_mult = 0.001;

    let mut rng = rand::thread_rng();  // Not used yet
    let mut max_id = 0;
    let mut cells_to_add: Vec<Cell> = Vec::new();

    for cell in cells.iter_mut() {
        if cell.id > max_id {
            max_id = cell.id;
        }
    }
    
    for cell in cells.iter_mut() {
        if cell.reproduce_now {
            let child_mass = cell.mass/2.0;
            cell.mass = cell.mass/2.0;
            let (x_offset, y_offset) = generate_random_position(&mut rng, cell.radius/2.0, cell.radius/2.0);
            let child_x_pos = cell.x_pos + x_offset;
            let child_y_pos = cell.y_pos + y_offset;
            let child_cell = Cell::new_from_reproduction(max_id + 1 as i64, cell.id, loop_step, child_mass, child_x_pos, child_y_pos, cell.x_vel, cell.y_vel, cell.membrane_color, cell.inside_color, cell.nucleus_color, cell.reproduction_cost);
            //child_cell.print_cell_properties();
            cells_to_add.push(child_cell);
            max_id += 1;
            // Reset the flag
            if cell.id == 1 {
                
                for i in 0..samples_per_frame {
                    let t = i as f32 / sample_rate as f32;  // time in seconds
                    let sample_value = amplitude_mult * f32::sin(2.0 * std::f32::consts::PI * cell.cell_freq() * t);
                    amplitude_sequence[i as usize] += sample_value.abs();
                }
            } else {
                for i in 0..samples_per_frame {
                    let t = i as f32 / sample_rate as f32;  // time in seconds
                    let frequency = cell.cell_freq();  // get the frequency from the cell
                    let sample_value = if (frequency * t) % 1.0 < 0.5 {
                        amplitude_mult
                    } else {
                        -amplitude_mult
                    };
                    amplitude_sequence[i as usize] += sample_value;
                }
            }
            cell.x_acc = 0.0;
            cell.y_acc = 0.0;
            cell.reproduce_now = false;
        }
    }
    
    // Append the new cells to the original list
    cells.append(&mut cells_to_add);

    //normalize_amplitude(&mut amplitude_sequence);
    return amplitude_sequence;
}

pub fn remove_dead_cells(cells: &mut Vec<Cell>) {
    let initial_len = cells.len();  // Debug print
    cells.retain(|cell| {
        let keep = cell.alive;
        if !keep {
            //println!("Initial number of cells: {}, Removing dead cell with ID: {}",initial_len, cell.id);  // Debug print
        }
        keep
    });
    //println!("Initial number of cells: {}, Remaining cells: {}", initial_len, cells.len());  // Debug print
}

pub fn normalize_amplitude(sequence: &mut Vec<f32>) {
    let min_val = sequence.iter().cloned().fold(f32::MAX, f32::min);
    let max_val = sequence.iter().cloned().fold(f32::MIN, f32::max);
    
    // Avoid division by zero
    if max_val - min_val != 0.0 {
        let range = max_val - min_val;
        let mid = (max_val + min_val) / 2.0;
        
        // Normalize and shift each value
        for val in sequence.iter_mut() {
            *val = 2.0 * (*val - mid) / range;
        }
    }
}