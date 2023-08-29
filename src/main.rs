#![allow(warnings)]
use std::fs::OpenOptions;
use std::io::Write;
mod cell;
mod environment;
mod utils;
use crate::utils::logging::init_logging;
use log::{info, debug};
use env_logger::Builder;
use environment::Environment;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    init_logging()?; // Call init_logging from utils

    info!("================================= New Run =================================");
    debug!("This will not be logged");

    let mut env = Environment::new();

    loop {
        // Your simulation loop
        env.update();
        break;
    }
    Ok(())
}
