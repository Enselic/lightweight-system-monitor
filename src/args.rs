pub struct Args {
    pub interval_millis: u64,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            interval_millis: 1000,
        }
    }
}

pub fn parse() -> Args {
    enum State {
        ExpectingArg,
        ExpectingIntervalMillisValue,
    }
    let mut args = Args::default();

    let mut state = State::ExpectingArg;
    for arg in std::env::args().skip(1) {
        match state {
            State::ExpectingIntervalMillisValue => {
                args.interval_millis = arg.parse::<u64>().unwrap_or_else(|_| {
                    super::error_exit(&format!(
                        "Failed to parse --interval-millis argument `{}`",
                        arg
                    ));
                });
                state = State::ExpectingArg;
            }
            State::ExpectingArg => {
                if arg == "--interval-millis" {
                    state = State::ExpectingIntervalMillisValue;
                    continue;
                } else {
                    super::error_exit(&format!("Unknown argument `{}`", arg));
                }
            }
        }
    }
    args
}
