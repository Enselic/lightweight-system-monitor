#[derive(argh::FromArgs)]
/// Sample CPU and memory usage at fixed intervals.
pub struct Args {
    #[argh(option, default = "1000")]
    /// interval between samples in milliseconds.
    pub interval_millis: u64,

    #[argh(option)]
    /// optional path to a file from which to read poll values.
    pub polled_path: Option<String>,

    #[argh(option)]
    /// optional name for the poll values.
    pub polled_title: Option<String>,
}

impl Args {
    pub fn polled_title_or_empty(&self) -> String {
        format!(
            "\t{}",
            match self.polled_path {
                Some(_) => self.polled_title.as_deref().unwrap_or("Polled"),
                None => "",
            }
        )
    }
}

pub fn parse() -> Args {
    argh::from_env()
}
