use std::borrow::BorrowMut;
use std::io;
use std::io::Read;
use std::io::Write;
use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use encoding_rs::WINDOWS_1252;
use encoding_rs_io::{DecodeReaderBytes, DecodeReaderBytesBuilder};

pub fn save_results<T>(results: &mut Vec<T>, file: &Option<File>) -> io::Result<()>
where
    T: AsRef<str>,
{
    if file.is_none() {
        return Ok(());
    }
    let mut file = file.as_ref().unwrap();
    let results_str: Vec<&str> = results.iter().map(|v| v.as_ref()).collect();
    let mut results_str = results_str.join("\n");
    results_str.push('\n');
    let encoded = WINDOWS_1252.encode(results_str.as_str());
    file.borrow_mut().write_all(&encoded.0)?;
    results.clear();
    Ok(())
}

pub fn open_results_file<P: AsRef<Path>>(path: P) -> io::Result<File> {
    let path = path.as_ref();

    if !path.exists() {
        if let Some(path) = path.parent() {
            std::fs::create_dir_all(path)?;
        }
    }

    OpenOptions::new()
        .create_new(true)
        .write(true)
        .append(true)
        .open(path)
}

pub fn count_lines(file: File) -> usize {
    let buffer = BufReader::new(file);

    let mut combos_count = 0usize;
    buffer.lines().for_each(|_| combos_count += 1);
    combos_count
}

pub fn build_results_path<P: AsRef<Path>>(file_path: P, results_path: P, suffix: &str) -> PathBuf {
    let file_path = file_path.as_ref();
    let mut new_file_name = file_path.file_stem().unwrap().to_owned();
    new_file_name.push(suffix);
    new_file_name.push(".");
    new_file_name.push(file_path.extension().unwrap_or_default());
    results_path.as_ref().join(new_file_name)
}

pub fn reader_from_file(file: File) -> BufReader<DecodeReaderBytes<File, Vec<u8>>> {
    BufReader::new(
        DecodeReaderBytesBuilder::new()
            .encoding(Some(WINDOWS_1252))
            .build(file),
    )
}

pub fn read_lines<'a>(path: &Path, buffer: &'a mut String) -> io::Result<Vec<&'a str>> {
    let file = open_file_r(path)?;
    let mut reader = reader_from_file(file);

    reader.read_to_string(buffer)?;
    let lines: Vec<&str> = buffer.split('\n').map(|v| v.trim_end()).collect();

    Ok(lines)
}

pub fn open_file_r(path: &Path) -> io::Result<File> {
    let file = OpenOptions::new().read(true).open(path)?;
    Ok(file)
}

// TODO: Make inline
pub fn save_results_hashset<'a>(
    results: impl Iterator<Item = &'a str>,
    file: &mut File,
) -> io::Result<()> {
    let results_str = join(results, "\n");
    let encoded = WINDOWS_1252.encode(results_str.as_str());
    file.write_all(&encoded.0)?;
    Ok(())
}

fn join<'a>(mut iter: impl Iterator<Item = &'a str>, joiner: &str) -> String {
    let mut joined = String::new();

    if let Some(item) = iter.next() {
        joined.push_str(item);
    }

    for item in iter {
        joined.push_str(joiner);
        joined.push_str(item);
    }

    joined
}

pub fn user_input(input: &str) -> io::Result<String> {
    print!("{}", input);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
