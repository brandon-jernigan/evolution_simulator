use std::f64::consts::PI;
use rand::Rng;

pub fn velocity_to_polar(x_vel: f64, y_vel: f64) -> (f64, f64) {
    let speed = (x_vel.powi(2) + y_vel.powi(2)).sqrt();
    let mut heading = x_vel.atan2(y_vel) / PI;
    heading = (heading + 1.0) % 1.0;
    (heading, speed)
}

pub fn polar_to_velocity(heading: f64, speed: f64) -> (f64, f64) {
    let x_vel = speed * heading.cos();
    let y_vel = speed * heading.sin();
    (x_vel, y_vel)
}

pub fn gradient_along_heading(gradient: (f64, f64), heading: f64) -> f64 {
    let (g_x, g_y) = gradient;
    let gradient_along = g_x * heading.cos() + g_y * heading.sin();

    gradient_along
}

pub fn gradient_perpendicular_heading(gradient: (f64, f64), heading: f64) -> f64 {
    let (g_x, g_y) = gradient;
    let gradient_perpendicular = -g_x * heading.sin() + g_y * heading.cos();

    gradient_perpendicular
}
pub fn generate_non_zero_integer<R: Rng + ?Sized>(rng: &mut R, min: i64, max: i64) -> i64 {
    let mut result: i64;
    loop {
        result = rng.gen_range(min..=max);
        if result != 0 {
            break;
        }
    }
    result
}

pub fn generate_random_position(rng: &mut impl Rng, min_distance: f64, max_distance: f64) -> (f64, f64) {
    let angle = rng.gen_range(0.0..(2.0 * std::f64::consts::PI));
    
    let distance = if (min_distance - max_distance).abs() < f64::EPSILON {
        min_distance // or max_distance, they are equal
    } else {
        rng.gen_range(min_distance..max_distance)
    };
    
    let x_offset = distance * angle.cos();
    let y_offset = distance * angle.sin();
    
    (x_offset, y_offset)
}
