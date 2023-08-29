use std::fs::OpenOptions;
use log::LevelFilter;
use env_logger::Builder;
use chrono::Local;
use std::io::Write;

pub fn init_logging() -> std::io::Result<()> {
    let log_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("logs/simulation.log")?;
    
    let mut builder = Builder::new();
    builder
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Trace) //Error, Warn, Info, Debug, Trace
        .write_style(env_logger::WriteStyle::Always)
        .target(env_logger::Target::Pipe(Box::new(log_file)));

    builder.init();
    Ok(())
}
