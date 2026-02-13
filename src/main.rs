mod args;
mod cpu_times;
mod mem_info;

fn main() {
    let args = args::parse();

    let mut prev_cpu_times = cpu_times::snapshot();
    let baseline_mem_info = mem_info::snapshot();

    println!("ms	CPU%	Avai kB	Free kB");
    let start_time = std::time::SystemTime::now();
    loop {
        std::thread::sleep(std::time::Duration::from_millis(args.interval_millis));

        let current_cpu_times = cpu_times::snapshot();
        let cpu_usage_percentage = current_cpu_times.usage_percentage_since(&prev_cpu_times);
        prev_cpu_times = current_cpu_times;

        let mem_info_delta = mem_info::snapshot() - baseline_mem_info;

        let elapsed = start_time.elapsed().unwrap().as_millis();
        println!(
            "{elapsed}	{cpu_usage_percentage:.2}	{}	{}",
            mem_info_delta.available_kb, mem_info_delta.free_kb,
        );
    }
}

fn error_exit(msg: &str) -> ! {
    eprintln!("{}", msg);
    std::process::exit(1);
}
