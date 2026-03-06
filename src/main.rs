#[derive(argh::FromArgs)]
/// Lightweight Linux system monitor for CPU and RAM.
pub struct Args {
    #[argh(option, default = "1000")]
    /// interval between samples in milliseconds.
    pub interval_millis: u64,
}

mod cpu_times;
mod mem_info;

fn main() {
    let args: Args = argh::from_env();

    let mut prev_cpu_times = cpu_times::snapshot();
    let baseline_mem_info = mem_info::snapshot();

    println!("seconds\tCPU%\tΔRAM kB");
    let start_time = std::time::SystemTime::now();
    loop {
        // Sleep
        std::thread::sleep(std::time::Duration::from_millis(args.interval_millis));

        // Sample CPU
        let current_cpu_times = cpu_times::snapshot();
        let cpu_usage_percentage = current_cpu_times.usage_percentage_since(&prev_cpu_times);
        prev_cpu_times = current_cpu_times;

        // Sample Memory
        let mem_info_delta = mem_info::snapshot() - baseline_mem_info;
        let available_kb = mem_info_delta.available_kb;

        // Print
        let elapsed = start_time.elapsed().unwrap().as_secs_f64();
        println!("{elapsed:.1}\t{cpu_usage_percentage:.1}\t{available_kb}");
    }
}

fn error_exit(msg: &str) -> ! {
    eprintln!("{}", msg);
    std::process::exit(1);
}
