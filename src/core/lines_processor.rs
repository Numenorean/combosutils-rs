use std::path::PathBuf;

pub trait LinesProcessor {
    fn new(targets: Vec<PathBuf>, results_path: PathBuf, save_period: usize) -> Self;

    fn process_line(line: &str) -> Option<String>;

    fn process(self) -> Result<(), anyhow::Error>;
}
