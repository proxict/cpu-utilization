use clap::Parser;
use cpu::utilization::Utilization;
use duration_string::DurationString;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

mod cpu;

/// A small utility for reporting CPU utilization.
#[derive(Parser, Debug)]
struct Args {
    /// Interval of CPU utilization reporting (format "[0-9]+(ns|us|ms|[smhdwy])").
    /// Defaults to 1s.
    #[arg(
        short = 'i',
        long = "interval",
        value_name = "INTERVAL",
        value_parser = |arg: &str| -> Result<Duration, String> {
            Ok(DurationString::from_string(String::from(arg))?.into())
        },
    )]
    interval: Option<Duration>,

    /// Report utilization of each CPU core separately
    #[arg(short = 'c', long = "per-core")]
    per_core: bool,
}

fn main() -> Result<(), cpu::ParseError> {
    let args = Args::parse();
    let interval = args.interval.unwrap_or(Duration::from_secs(1));

    let mut u = Utilization::new()?;

    let quit = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&quit))
        .expect("Failed to register SIGTERM handler");
    signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&quit))
        .expect("Failed to register SIGINT handler");

    while !quit.load(Ordering::Relaxed) {
        u.update()?;
        match args.per_core {
            true => {
                let mut delim = "";
                for load in u.iter() {
                    print!("{delim}{load}");
                    delim = ";";
                }
                println!();
            }
            false => println!("{}", u.get_average_load()?),
        }
        std::thread::sleep(interval);
    }
    println!();
    Ok(())
}
