use std::thread;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use clap::{App, Arg};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn stress_cpu(terminate: Arc<AtomicBool>) {
    while !terminate.load(Ordering::Relaxed) { }
}

fn main() {
    let matches = App::new("CPU Stress Test")
        .arg(Arg::with_name("threads")
        .short("t")
        .long("threads")
        .value_name("THREADS")
        .help("Number of threads to create")
        .required(true)
        .takes_value(true))
        .arg(Arg::with_name("duration")
        .short("d")
        .long("duration")
        .value_name("DURATION")
        .help("Duration of the test in seconds")
        .required(true)
        .takes_value(true))
        .version(VERSION)
        .get_matches();

    let mut num_threads: u32 = 1;
    if let Ok(num) = matches.value_of("threads").unwrap().parse::<u32>() {
        num_threads = num
    }

    let mut duration: u64 = 0;
    if let Ok(dur) = matches.value_of("duration").unwrap().parse::<u64>() {
        duration = dur
    }

    let terminate = Arc::new(AtomicBool::new(false));
    let mut threads = Vec::new();

    for _ in 0..num_threads {
        let terminate_clone = terminate.clone();
        let handle = thread::spawn(move || {
            stress_cpu(terminate_clone);
        });
        threads.push(handle);
    }

    thread::sleep(Duration::from_secs(duration));

    terminate.store(true, Ordering::Relaxed);

    for handle in threads {
        handle.join().unwrap();
    }
}
