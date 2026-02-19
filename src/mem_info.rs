#[derive(Default, Clone, Copy)]
pub(crate) struct MemInfo {
    pub(crate) available_kb: i64,
}

impl std::ops::Sub for MemInfo {
    type Output = MemInfo;

    fn sub(self, rhs: MemInfo) -> Self::Output {
        MemInfo {
            available_kb: self.available_kb - rhs.available_kb,
        }
    }
}

pub(crate) fn snapshot() -> MemInfo {
    let file = std::fs::File::open("/proc/meminfo").unwrap_or_else(|e| {
        super::error_exit(&format!("Failed to open /proc/meminfo: {}", e));
    });
    let reader = std::io::BufReader::new(file);

    let mut mem_info = MemInfo::default();

    for line in std::io::BufRead::lines(reader).map_while(Result::ok) {
        if line.starts_with("MemAvailable:") {
            mem_info.available_kb = line
                .split_whitespace()
                .nth(1)
                .unwrap_or("0")
                .parse::<i64>()
                .unwrap_or(0);
        }
    }
    mem_info
}
