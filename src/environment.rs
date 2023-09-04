use crate::cell::{update_cells, Cell};
use log::{debug, error, info, trace, warn, LevelFilter};
use noise::{NoiseFn, Perlin, Seedable};
use rand::Rng;

use crate::constants::{ENV_SEED, ENV_STEP, FULLSCREEN, HEIGHT, LOG_LEVEL, WIDTH, NUM_CELLS};

pub struct Environment {
    pub cells: Vec<Cell>,
    pub terrain: Vec<Vec<f64>>,
    pub gradient: Vec<Vec<(f64, f64)>>,
}

impl Environment {
    pub fn new(width: u32, height: u32, env_seed: u32, loop_step: i64) -> Self {
        
        let terrain: Vec<Vec<f64>> = generate_terrain(width as usize, height as usize, env_seed, loop_step);
        let gradient: Vec<Vec<(f64, f64)>> = calculate_gradient(&terrain);
        let mut cells: Vec<Cell> = Vec::with_capacity(NUM_CELLS);
        for ii in 0..NUM_CELLS {
            cells.push(Cell::new(ii as i64, loop_step));
        }        Self { cells, terrain, gradient}
    }

    pub fn update(&mut self, loop_step: i64) {
        update_cells(&mut self.cells, &self.terrain, &self.gradient, loop_step);
        // Code to update terrain if needed
    }
    pub fn update_terrain(&mut self, width: u32, height: u32, env_seed: u32, loop_step: i64) {
        self.terrain = generate_terrain(width as usize, height as usize, env_seed, loop_step);
    }
}


fn generate_terrain(width: usize, height: usize, env_seed: u32, loop_step: i64) -> Vec<Vec<f64>> {
    let perlin = Perlin::new(env_seed);
    let step_rate: f64 = 2.0;
    let texture_frequency: f64 = 0.010; // Adjust this for smoother, wider valleys and ranges
    let octaves: i32 = 4;
    let persistence: f64 = 0.4; // Adjust this for smoother transitions
    let lacunarity: f64 = 2.3; // Controls frequency increment between octaves

    let valley_floor: f64 = -0.35; // This is the floor level for the valleys, adjust as needed
    let smoothing_factor: f64 = 0.05; // This adjusts how quickly the value approaches the floor
    let min_value: f64 = valley_floor + (-1.0 - valley_floor) * smoothing_factor * 1.1;

    let ridge_frequency: f64 = 0.004; // Frequency for the ridge or chasm lines
    let ridge_multiplier: f64 = 0.5; // How much the ridges or chasms will influence the terrain

    let mut terrain: Vec<Vec<f64>> = vec![vec![0.0; height]; width];
    trace!("Environment::generate_terrain >> Generating terrain");
    for x in 0..width {
        for y in 0..height {
            let mut amplitude: f64 = 1.0;
            let mut frequency = texture_frequency;
            let mut total: f64 = 0.0;
            let mut fbm_max_value = 0.0;
            trace!("Environment::generate_terrain >> making octaves");
            // Multi-octave Perlin noise (Fractal Brownian Motion)
            for _ in 0..octaves {
                total += perlin.get([
                    x as f64 * frequency,
                    y as f64 * frequency,
                    loop_step as f64 * step_rate as f64 * frequency,
                ]) * amplitude;
                fbm_max_value += amplitude;
                amplitude *= persistence;
                frequency *= lacunarity;
            }
            trace!("Environment::generate_terrain >> applying modifiers");
            total /= fbm_max_value;

            total += ridge_multiplier
                * perlin.get([(x as f64) * ridge_frequency, (y as f64) * ridge_frequency]);

            if total < valley_floor {
                total = valley_floor + (total - valley_floor) * smoothing_factor;
            }
            trace!("Environment::generate_terrain >> normalizing and storing");

            terrain[x][y] = (total - min_value) / (1.0 - min_value);
        }
    }

    terrain
}

fn calculate_gradient(terrain: &Vec<Vec<f64>>) -> Vec<Vec<(f64, f64)>> {
    let width = terrain.len();
    let height = terrain[0].len();
    let mut gradient = vec![vec![(0.0, 0.0); height]; width];

    for x in 1..(width - 1) {
        for y in 1..(height - 1) {
            let dx = (terrain[x + 1][y] - terrain[x - 1][y]) / 2.0; // Change in x-direction
            let dy = (terrain[x][y + 1] - terrain[x][y - 1]) / 2.0; // Change in y-direction
            gradient[x][y] = (-1.0 * dx, -1.0 * dy);
        }
    }

    // Handle edge cases (you could handle these more carefully if needed)
    for x in 0..width {
        gradient[x][0] = gradient[x][1];
        gradient[x][height - 1] = gradient[x][height - 2];
    }
    for y in 0..height {
        gradient[0][y] = gradient[1][y];
        gradient[width - 1][y] = gradient[width - 2][y];
    }

    gradient
}

