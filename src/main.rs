use std::io::{self, Write};
use std::io::Read;
use std::ops::Sub;
use std::sync::mpsc::RecvTimeoutError;
use std::sync::{RwLock, Arc, mpsc};
use std::thread;
use std::time::{Duration, Instant};

use clap::Parser;

const VERSION: &str = "2.0.0";

/// Print out (to stderr) the transfer speed and total throughput of stdin
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Print version number
    #[clap(short, long, value_parser)]
    version: bool,
    
    /// Forward stdin to stdout
    #[clap(short, long, value_parser)]
    forward_stdin: bool,
    
    /// Time interval in seconds to print speed
    #[clap(short, long, value_parser, default_value_t = 1.0)]
    time_interval: f64,

    /// Number of bytes to read from stdin at once in the read loop
    #[clap(short, long, value_parser, default_value_t = 4096)]
    block_size: usize,

    /// Do not print any intermediate stats
    #[clap(short, long, value_parser)]
    quiet: bool,
}

fn main() {
    let args = Args::parse();

    if args.version {
        eprintln!("Version: {}", VERSION);
        return
    }

    let total_bytes_read = Arc::new(RwLock::new(0usize));

    let time_interval = Duration::from_millis((args.time_interval * 1000f64) as u64);

    let rx = create_read_thread(args.block_size, args.forward_stdin, total_bytes_read.clone());
    print_stats_loop(time_interval, args.quiet, rx, total_bytes_read.clone());
}

fn print_stats_loop(time_interval: Duration, quiet: bool, read_thread: mpsc::Receiver<()>, bytes_read: Arc<RwLock<usize>>) {
    let ctrlc_bytes_read = bytes_read.clone();
    let ctrlc_time_start = Instant::now();
    ctrlc::set_handler(move || {
        let final_time = Instant::now() - ctrlc_time_start;
        let total_bytes = { *ctrlc_bytes_read.read().unwrap() };
        print_final_stats(final_time.as_secs_f64(), total_bytes);
        std::process::exit(0);
    }).unwrap();

    let mut prev_bytes = 0usize;
    let start_time = Instant::now();

    while is_read_thread_alive(&read_thread, time_interval) {
        // Don't print stats if we want the output quiet
        if quiet {
            continue;
        }

        let extra_time = Instant::now();

        let new_bytes = {
            let current_bytes = bytes_read.read().unwrap();
            let res = current_bytes.sub(prev_bytes);
            prev_bytes = *current_bytes;
            res
        };

        // Add on any additional time we spent waiting for the read lock
        let dt = time_interval + (Instant::now() - extra_time);

        // Calculate bytes per second
        let bytes_per_second = ((new_bytes as f64) / dt.as_secs_f64()) as usize;
        eprintln!("{}/s", get_human_readable_bytes(bytes_per_second));
    }

    let final_time = Instant::now() - start_time;
    
    let total_bytes = { *bytes_read.read().unwrap() };
    print_final_stats(final_time.as_secs_f64(), total_bytes);
}

fn print_final_stats(final_time: f64, total_bytes: usize) {
    let average_speed = ((total_bytes as f64) / final_time) as usize;

    // Print final stats
    eprint!(
        "\nFinished!\nTotal data: {}\nAverage speed: {}/s\n", 
        get_human_readable_bytes(total_bytes),
        get_human_readable_bytes(average_speed),
    );
}

fn create_read_thread(block_size: usize, forward: bool, bytes_read: Arc<RwLock<usize>>) -> mpsc::Receiver<()> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        // Move tx in so it closes when this thread dies
        let _tx = tx;
        let mut stdin = io::stdin();
        let mut stdout = forward.then(|| io::stdout());
        // Create read buffer
        let mut buf = vec![0; block_size];

        loop {
            // Attempt to read data. Blocks until data is read into buffer
            let res = stdin.read(&mut buf);
            
    
            // Get how many bytes were read
            let bytes = match res {
                Ok(0) => break, // stdin closed
                Ok(n) => n,
                _ => break, // Unknown error
            };

            
            {
                // Update bytes read counter
                let mut bytes_lock = bytes_read.write().unwrap();
                *bytes_lock += bytes;
            }

            if let Some(stdout) = &mut stdout {
                stdout.write(&buf[..bytes]).unwrap();
            }
        }
    });

    rx
}

fn is_read_thread_alive(read_thread: &mpsc::Receiver<()>, timeout: Duration) -> bool { 
    let res = read_thread.recv_timeout(timeout);
    match res {
        Err(RecvTimeoutError::Timeout) => true,
        _ => false,
    }
}

fn get_human_readable_bytes(bytes: usize) -> String {
    let mut bytes = bytes as f64;
    let suffixes = ["bytes", "KiB", "MiB", "GiB", "TiB", "PiB"];

    for suffix in suffixes {
        if bytes < 1024f64 {
            return format!("{:.2} {}", bytes, suffix);
        }
        bytes /= 1024f64;
    }

    return format!("{:.2} {}", bytes * 1024f64, suffixes.last().unwrap());
}
