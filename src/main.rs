mod cpu_times;
mod mem_info;

struct Args {
    interval_millis: u64,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            interval_millis: 1000,
        }
    }
}

fn error_exit(msg: String) -> ! {
    eprintln!("{}", msg);
    std::process::exit(1);
}

fn args() -> Args {
    enum State {
        Start,
        IntervalMillis,
    }
    let mut args = Args::default();

    let mut state = State::Start;
    for arg in std::env::args().skip(1) {
        match state {
            State::IntervalMillis => {
                args.interval_millis = arg.parse::<u64>().unwrap_or_else(|e| {
                    error_exit(format!("Failed to parse --interval-millis argument: {}", e));
                });
                state = State::Start;
            }
            State::Start => {
                if arg == "--interval-millis" {
                    state = State::IntervalMillis;
                    continue;
                } else {
                    error_exit(format!("Unknown argument: {}", arg));
                }
            }
        }
    }
    args
}

fn main() {
    let args = args();

    let mut prev_cpu_times = cpu_times::snapshot();
    let baseline_mem_info = mem_info::snapshot();

    println!("ms	CPU%	MemAvai	MemFree");
    let start_time = std::time::SystemTime::now();
    loop {
        std::thread::sleep(std::time::Duration::from_millis(args.interval_millis));

        let current_cpu_times = cpu_times::snapshot();
        let cpu_usage_percentage = current_cpu_times.usage_percentage_since(&prev_cpu_times);
        prev_cpu_times = current_cpu_times;

        let mem_info_delta = baseline_mem_info - mem_info::snapshot();

        let elapsed = start_time.elapsed().unwrap().as_millis();
        println!(
            "{elapsed}	{cpu_usage_percentage:.2}	{}	{}",
            mem_info_delta.available_kb, mem_info_delta.free_kb,
        );
    }
}
