use log::{info, debug};
use environment::Environment;

fn main() {
    env_logger::init();
    info!("Starting simulation");
    let mut env = Environment::new();

    loop {
        // Your simulation loop
        env.update();
    }
}
