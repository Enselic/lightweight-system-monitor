#[derive(argh::FromArgs)]
/// Lightweight Linux system monitor for CPU and RAM.
pub struct Args {
    #[argh(option, default = "1000")]
    /// interval between samples in milliseconds.
    pub interval_millis: u64,

    #[argh(option, default = "0")]
    /// optional amount of kB to subtract from `/proc/meminfo` `MemAvailable`
    /// before reporting as `Avail_kB`.
    pub mem_available_baseline_kb: i64,
}

mod cpu_times;
mod mem_info;

fn main() {
    let args: Args = argh::from_env();

    let mut prev_cpu_times = cpu_times::snapshot();

    println!("seconds\tCPU%\tAvail_kB");
    let start_time = std::time::SystemTime::now();
    loop {
        // Sleep
        std::thread::sleep(std::time::Duration::from_millis(args.interval_millis));

        // Sample CPU
        let current_cpu_times = cpu_times::snapshot();
        let cpu_usage_percentage = current_cpu_times.usage_percentage_since(&prev_cpu_times);
        prev_cpu_times = current_cpu_times;

        // Sample Memory
        let available_kb = mem_info::mem_available_snapshot() - args.mem_available_baseline_kb;

        // Print
        let elapsed = start_time.elapsed().unwrap().as_secs_f64();
        println!("{elapsed:.1}\t{cpu_usage_percentage:.1}\t{available_kb}");
    }
}

fn error_exit(msg: &str) -> ! {
    eprintln!("{}", msg);
    std::process::exit(1);
}
