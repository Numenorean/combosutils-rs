use std::ffi::OsString;
use std::io::Write;
use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

pub trait LinesProcessor {
    fn new(targets: Vec<PathBuf>, results_path: PathBuf, save_period: usize) -> Self;

    fn process_line(line: &str) -> Option<String>;

    fn process(self) -> Result<(), anyhow::Error>;

    fn save_results(results: &mut Vec<String>, file: &mut File) -> Result<(), anyhow::Error> {
        writeln!(file, "{}", results.join("\r\n"))?;
        results.clear();
        //file.flush()?;
        Ok(())
    }

    fn open_results_file<P: AsRef<Path>>(path: P) -> Result<File, anyhow::Error> {
        let path = path.as_ref();

        if let Some(path) = path.parent() {
            std::fs::create_dir_all(path)?;
        }

        let file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .append(true)
            .open(path)?;
        Ok(file)
    }

    fn count_lines(file: File) -> usize {
        let buffer = BufReader::new(file);

        let mut combos_count = 0usize;
        buffer.lines().for_each(|_| combos_count += 1);
        combos_count
    }

    fn build_result_filename<P: AsRef<Path>>(path: P, postfix: &str) -> OsString {
        let path = path.as_ref();
        let mut new_file_name = path.file_stem().unwrap().to_owned();
        new_file_name.push(postfix);
        new_file_name.push(".");
        new_file_name.push(path.extension().unwrap_or_default());
        new_file_name
    }
}