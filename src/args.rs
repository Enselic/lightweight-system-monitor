pub struct Args {
    pub interval_millis: u64,
    pub aux_data_point_path: Option<String>,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            interval_millis: 1000,
            aux_data_point_path: None,
        }
    }
}

pub fn parse() -> Args {
    enum State {
        ExpectingArg,
        ExpectingIntervalMillisValue,
        ExpectingAuxDataPointPathValue,
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
            State::ExpectingAuxDataPointPathValue => {
                args.aux_data_point_path = Some(arg);
                state = State::ExpectingArg;
            }
            State::ExpectingArg => {
                if arg == "--interval-millis" {
                    state = State::ExpectingIntervalMillisValue;
                    continue;
                } else if arg == "--aux-data-point-path" {
                    state = State::ExpectingAuxDataPointPathValue;
                    continue;
                } else {
                    super::error_exit(&format!("Unknown argument `{}`", arg));
                }
            }
        }
    }
    args
}
