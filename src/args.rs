#[derive(argh::FromArgs)]
/// Sample CPU and memory usage at fixed intervals.
pub struct Args {
    #[argh(option, default = "1000")]
    /// interval between samples in milliseconds.
    pub interval_millis: u64,

    #[argh(option)]
    /// optional file path for an auxiliary data point displayed as the final column.
    pub aux_data_point_path: Option<String>,
}

pub fn parse() -> Args {
    argh::from_env()
}
