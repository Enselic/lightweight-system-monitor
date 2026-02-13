/// Field docs taken from `man proc_stat`.
#[derive(Clone, Copy, Debug)]
pub(crate) struct CpuTimes {
    /// (1) Time spent in user mode.
    pub(crate) user: u64,
    /// (2) Time spent in user mode with low priority (nice).
    pub(crate) nice: u64,
    /// (3) Time spent in system mode.
    pub(crate) system: u64,
    /// (4) Time spent in the idle task.
    pub(crate) idle: u64,
    /// (5) Time waiting for I/O to complete.
    pub(crate) iowait: u64,
    /// (6) Time servicing interrupts.
    pub(crate) irq: u64,
    /// (7) Time servicing softirqs.
    pub(crate) softirq: u64,
    /// (8) Stolen time, which is the time spent in other operating systems when running in a virtualized environment
    pub(crate) steal: u64,
    /// (9) Time spent running a virtual CPU for guest operating systems under the control of the Linux kernel.
    pub(crate) guest: u64,
    /// (10) Time spent running a niced guest (virtual CPU for guest operating systems under the control of the Linux kernel).
    pub(crate) guest_nice: u64,
}

impl std::ops::Sub for CpuTimes {
    type Output = CpuTimes;

    fn sub(self, rhs: CpuTimes) -> Self::Output {
        CpuTimes {
            user: self.user.saturating_sub(rhs.user),
            nice: self.nice.saturating_sub(rhs.nice),
            system: self.system.saturating_sub(rhs.system),
            idle: self.idle.saturating_sub(rhs.idle),
            iowait: self.iowait.saturating_sub(rhs.iowait),
            irq: self.irq.saturating_sub(rhs.irq),
            softirq: self.softirq.saturating_sub(rhs.softirq),
            steal: self.steal.saturating_sub(rhs.steal),
            guest: self.guest.saturating_sub(rhs.guest),
            guest_nice: self.guest_nice.saturating_sub(rhs.guest_nice),
        }
    }
}

impl CpuTimes {
    pub fn usage_percentage_since(&self, prev: &CpuTimes) -> u64 {
        let delta = *self - *prev;

        let total_cpu_time = delta.user
            + delta.nice
            + delta.system
            + delta.idle
            + delta.iowait
            + delta.irq
            + delta.softirq
            + delta.steal
            + delta.guest
            + delta.guest_nice;

        let idle_cpu_time = delta.idle + delta.iowait + delta.guest + delta.guest_nice;

        let used_cpu_time = total_cpu_time - idle_cpu_time;

        used_cpu_time * 100 / total_cpu_time
    }
}

pub(crate) fn snapshot() -> CpuTimes {
    let stat = std::fs::read_to_string("/proc/stat").unwrap();
    let cpu_times = stat.lines().next().unwrap();

    let mut parts = cpu_times.split_whitespace();
    assert_eq!(parts.next().unwrap(), "cpu");

    let mut parts = parts.map(|s| s.parse::<u64>().unwrap()).fuse();
    CpuTimes {
        user: parts.next().unwrap_or_default(),
        nice: parts.next().unwrap_or_default(),
        system: parts.next().unwrap_or_default(),
        idle: parts.next().unwrap_or_default(),
        iowait: parts.next().unwrap_or_default(),
        irq: parts.next().unwrap_or_default(),
        softirq: parts.next().unwrap_or_default(),
        steal: parts.next().unwrap_or_default(),
        guest: parts.next().unwrap_or_default(),
        guest_nice: parts.next().unwrap_or_default(),
    }
}
