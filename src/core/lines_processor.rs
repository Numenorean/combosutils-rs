use std::path::PathBuf;

use super::task::Task;

pub trait LinesProcessor {
    fn new(targets: Vec<PathBuf>, results_path: PathBuf, save_period: usize, task: Task) -> Self;

    fn process_line(line: &str) -> Option<String>;

    fn process(self) -> Result<(), anyhow::Error>;
}
