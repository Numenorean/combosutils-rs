use std::path::PathBuf;

use crate::{cmd::Args, errors::core_error::CoreError};

pub trait LinesProcessor {
    fn new(args: Args, results_path: PathBuf, save_period: usize) -> Self;

    fn process_line(line: &str) -> Option<String>;

    fn process(self) -> Result<(), CoreError>;
}
