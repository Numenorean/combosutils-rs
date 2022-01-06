use std::io::Write;
use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use encoding_rs::WINDOWS_1252;
use encoding_rs_io::{DecodeReaderBytes, DecodeReaderBytesBuilder};

pub trait LinesProcessor {
    fn new(targets: Vec<PathBuf>, results_path: PathBuf, save_period: usize) -> Self;

    fn process_line(line: &str) -> Option<String>;

    fn process(self) -> Result<(), anyhow::Error>;

    fn save_results<T>(results: &mut Vec<T>, file: &mut File) -> Result<(), anyhow::Error>
    where
        T: AsRef<str>,
    {
        let results_str: Vec<&str> = results.iter().map(|v| v.as_ref()).collect();
        writeln!(file, "{}", results_str.join("\r\n"))?;
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

    fn build_results_path<P: AsRef<Path>>(file_path: P, results_path: P, suffix: &str) -> PathBuf {
        let file_path = file_path.as_ref();
        let mut new_file_name = file_path.file_stem().unwrap().to_owned();
        new_file_name.push(suffix);
        new_file_name.push(".");
        new_file_name.push(file_path.extension().unwrap_or_default());
        results_path.as_ref().join(new_file_name)
    }

    fn reader_from_file(file: File) -> BufReader<DecodeReaderBytes<File, Vec<u8>>> {
        BufReader::new(
            DecodeReaderBytesBuilder::new()
                .encoding(Some(WINDOWS_1252))
                .build(file),
        )
    }
}
