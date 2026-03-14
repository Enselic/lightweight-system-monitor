pub(crate) fn mem_available_snapshot() -> i64 {
    let file = std::fs::File::open("/proc/meminfo").unwrap_or_else(|e| {
        super::error_exit(&format!("Failed to open /proc/meminfo: {}", e));
    });

    let reader = std::io::BufReader::new(file);
    for line in std::io::BufRead::lines(reader).map_while(Result::ok) {
        if line.starts_with("MemAvailable:") {
            return line
                .split_whitespace()
                .nth(1)
                .unwrap_or_else(|| super::error_exit("MemAvailable missing kB value"))
                .parse::<i64>()
                .unwrap_or_else(|_| super::error_exit("MemAvailable kB failed to parse"));
        }
    }

    super::error_exit("Failed to find MemAvailable in /proc/meminfo");
}
