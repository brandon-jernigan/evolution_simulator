use std::fs::OpenOptions;
use log::LevelFilter;
use env_logger::Builder;
use chrono::Local;
use std::io::Write;
use std::time::Instant;

pub fn init_logging(start_time: Instant) -> std::io::Result<()> {
    let log_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("logs/simulation.log")?;
    
    let mut builder = Builder::new();
    builder
        .format(move |buf, record| {
            let elapsed = start_time.elapsed().as_secs_f64(); // Elapsed time in seconds
            writeln!(
                buf,
                "{} [{:.3}s] [{}] - {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                elapsed,
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Trace)  // Info Debug Warn Error Trace
        .write_style(env_logger::WriteStyle::Always)
        .target(env_logger::Target::Pipe(Box::new(log_file)));

    builder.init();
    Ok(())
}
