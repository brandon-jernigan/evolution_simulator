use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::time::Instant;

const LOG_LEVEL: LevelFilter = LevelFilter::Debug;

pub fn init_logging(start_time: Instant, log_level: LevelFilter) -> std::io::Result<()> {
    let log_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("logs/simulation.log")?;

    let last_instant = Arc::new(Mutex::new(start_time));

    let mut builder = Builder::new();
    builder
        .format(move |buf, record| {
            let now = Instant::now();
            let elapsed = start_time.elapsed().as_secs_f64();

            let mut last_inst = last_instant.lock().unwrap();
            let delta = now.duration_since(*last_inst).as_secs_f64();

            // Update last_instant with the current time
            *last_inst = now;

            let elapsed_whole: u64 = elapsed.trunc() as u64;
            let elapsed_frac = elapsed.fract();

            let delta_whole: u64 = delta.trunc() as u64;
            let delta_frac = delta.fract();

            writeln!(
                buf,
                "{} [{:04}.{:0>6}s | Δ{:02}.{:0>6}s] [{}] - {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                elapsed_whole,
                format!("{:.*}", 6, elapsed_frac)
                    .strip_prefix("0.")
                    .unwrap_or("0"),
                delta_whole,
                format!("{:.*}", 6, delta_frac)
                    .strip_prefix("0.")
                    .unwrap_or("0"),
                record.level(),
                record.args()
            )
        })
        .filter(None, log_level)
        .write_style(env_logger::WriteStyle::Always)
        .target(env_logger::Target::Pipe(Box::new(log_file)));

    builder.init();
    Ok(())
}

fn main() -> std::io::Result<()> {
    init_logging(Instant::now(), LOG_LEVEL)?;
    // Rest of your code
    Ok(())
}
