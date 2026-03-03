mod args;
mod cpu_times;
mod mem_info;

fn main() {
    let args = args::parse();

    let mut prev_cpu_times = cpu_times::snapshot();
    let baseline_mem_info = mem_info::snapshot();

    println!("s\tCPU%\tMemAvai\tAux");
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
        let available = mem_info_delta.available_kb;

        // Sample Aux Data Point
        let aux = get_aux(&args.aux_data_point_path);

        // Print
        let elapsed = start_time.elapsed().unwrap().as_secs_f64();
        println!("{elapsed:.2}\t{cpu_usage_percentage:.1}\t{available}{aux}");
    }
}

fn get_aux(aux_data_point_path: &Option<String>) -> String {
    if let Some(path) = aux_data_point_path {
        match std::fs::read_to_string(path) {
            Ok(content) => format!("\t{}", content.trim()),
            Err(e) => format!("\tError reading aux data point: {}", e),
        }
    } else {
        String::new()
    }
}

fn error_exit(msg: &str) -> ! {
    eprintln!("{}", msg);
    std::process::exit(1);
}
