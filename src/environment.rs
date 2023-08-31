use crate::cell::{update_cells, Cell};
use noise::{NoiseFn, Perlin, Seedable};
use rand::Rng;

pub struct Environment {
    pub cells: Vec<Cell>,
    pub terrain: Vec<Vec<f64>>,
}

impl Environment {
    pub fn new(width: u32, height: u32, env_seed: u32, step: u32) -> Self {
        let cells = vec![Cell::new(1), Cell::new(2)];
        let terrain = generate_terrain(width as usize, height as usize, env_seed, step);
        Self { cells, terrain }
    }

    pub fn update(&mut self) {
        update_cells(&mut self.cells);
        // Code to update terrain if needed
    }
    pub fn update_terrain(&mut self, width: u32, height: u32, env_seed: u32, step: u32) {
        self.terrain = generate_terrain(width as usize, height as usize, env_seed, step);
    }
}

fn generate_terrain(width: usize, height: usize, env_seed: u32, step: u32) -> Vec<Vec<f64>> {
    let mut min_value = f64::INFINITY;
    let mut new_max_value = f64::NEG_INFINITY;

    let perlin = Perlin::new(env_seed);
    let step_rate = 2.0;
    let base_frequency = 0.010; // Adjust this for smoother, wider valleys and ranges
    let octaves = 6;
    let persistence = 0.4; // Adjust this for smoother transitions
    let lacunarity = 2.0; // Controls frequency increment between octaves

    let valley_floor = -0.4; // This is the floor level for the valleys, adjust as needed
    let smoothing_factor = 0.15; // This adjusts how quickly the value approaches the floor

    let power_exponent = 2.0; // Increase to make valleys deeper and peaks more prominent
    let ridge_frequency = 0.004; // Frequency for the ridge or chasm lines
    let ridge_multiplier = 0.5; // How much the ridges or chasms will influence the terrain

    let mut terrain = vec![vec![0.0; height]; width];

    for x in 0..width {
        for y in 0..height {
            let mut amplitude = 1.0;
            let mut frequency = base_frequency;
            let mut total = 0.0;
            let mut fbm_max_value = 0.0;

            // Multi-octave Perlin noise (Fractal Brownian Motion)
            for _ in 0..octaves {
                total += perlin.get([
                    x as f64 * frequency,
                    y as f64 * frequency,
                    step as f64 * step_rate as f64 * frequency,
                ]) * amplitude;
                fbm_max_value += amplitude;
                amplitude *= persistence;
                frequency *= lacunarity;
            }

            total /= fbm_max_value;

            total += ridge_multiplier
                * perlin.get([(x as f64) * ridge_frequency, (y as f64) * ridge_frequency]);

            terrain[x][y] = (terrain[x][y] as f64).powf(power_exponent);

            if total < valley_floor {
                total = valley_floor + (total - valley_floor) * smoothing_factor;
            }

            if total < min_value {
                min_value = total;
            }

            if total > new_max_value {
                new_max_value = total;
            }

            terrain[x][y] = total;
        }
    }

    for x in 0..width {
        for y in 0..height {
            terrain[x][y] = (terrain[x][y] - min_value) / (new_max_value - min_value);
        }
    }

    terrain
}

fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}
