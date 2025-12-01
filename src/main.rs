use clap::{Parser};
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::{Instant, Duration};
use num_cpus;

#[derive(Parser)]
#[command(disable_version_flag = true, disable_help_flag = true, name = "timeRs", about = "timeRs - A python program timer written in rust.")]
struct Cli {
    #[arg(short = 't', long, default_value = "1")]
    threads: u8,

    #[arg(short = 'r', long, default_value = "1")]
    reruns: usize,

    #[arg(short = 'v', long)]
    version: bool,

    #[arg(short = 'h', long)]
    help: bool,

    #[arg(value_name = "PYTHON_FILE")]
    file: Option<String>,

}

fn print_help() {
    println!(
        r#"timeRs 0.2.0 - A python program timer written in rust.

Usage: timeRs [OPTIONS] <PYTHON_FILE>

Arguments:
  <PYTHON_FILE>    The path to the Python script you wish to time.

Options:
  -t, --threads <THREADS>    Sets the number of concurrent threads to use for testing. [default: 1]
  -r, --reruns <RERUNS>      Sets the number of times to re-run the python file per thread. [default: 1]
  -h, --help                 Print help and exit.
  -v, --version              Print version and exit."#

    )
}
fn execute(file: &Path) -> Duration {
    let python_bin = if cfg!(target_os = "windows") { "python" } else { "python3" };
    let start = Instant::now();

    let _ = Command::new(python_bin)
        .arg(file)
        .output()
        .expect("failed to execute process");

    start.elapsed()
}

fn main() -> std::io::Result<()> {
    if std::env::args().len() == 1 {
        print_help();
        return Ok(());
    }

    let cli = Cli::parse();

    if cli.help {
        print_help();
        return Ok(());
    }

    if cli.version {
        println!("timeRs 0.1.0");
        return Ok(());
    }

     match cli.file {
        Some(ref s) => if Path::new(s).exists() {} else {
            eprintln!("Error: file specified does not exist");
            return Ok(());
        },
        None => {
            eprintln!("Error: No python file provided. See 'timeRs -h' for more information.");
            return Ok(());
        }
    };

    let cpus = num_cpus::get() as u8;

    let all_results: Vec<Vec<Duration>> = thread::scope(|s| {
        let mut handles = vec![];
        let max_threads = if cli.threads < cpus { cli.threads } else { cpus };
        for _ in 1..=max_threads {
            let program = Path::new(cli.file.as_ref().unwrap());

            let handle = s.spawn(move || {
                let mut thread_runs = Vec::with_capacity(cli.reruns);

                for _ in 1..=cli.reruns {
                    let duration = execute(program);
                    thread_runs.push(duration);
                }
                thread_runs
            });
            handles.push(handle);
        }

        // Join threads and collect results
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    });

    let flat_times: Vec<Duration> = all_results.into_iter().flatten().collect();
    let average = flat_times.iter().sum::<Duration>() / flat_times.len() as u32;
    println!("Average time taken to run: {:?} ms", average.as_secs_f64() * 1000.0);
    Ok(())

}
