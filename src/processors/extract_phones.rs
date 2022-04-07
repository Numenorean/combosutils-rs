use std::{fs::File, io::BufRead, path::PathBuf, time};

use crate::{
    core::{
        lines_processor::LinesProcessor,
        task::Task,
        utils::{self, open_file_r},
    },
    errors::core_error::CoreError,
};

const MAX_PHONE_LENGTH: usize = 15;
const MIN_PHONE_LENGTH: usize = 8;

pub struct PhonesExtractor {
    targets: Vec<PathBuf>,
    results_path: PathBuf,
    save_period: usize,
    task: Task,
}

impl LinesProcessor for PhonesExtractor {
    fn new(targets: Vec<PathBuf>, results_path: PathBuf, save_period: usize, task: Task) -> Self {
        PhonesExtractor {
            targets,
            results_path,
            save_period,
            task,
        }
    }

    fn process_line(combo: &str) -> Option<String> {
        let (phone, password) = combo.split_once([':', ';'])?;
        if phone.is_empty() || password.is_empty() {
            return None;
        }
        let mut combo = extract_phone(phone)?;
        combo.push(':');
        combo.push_str(password);
        Some(combo)
    }

    fn process(self) -> Result<(), CoreError> {
        println!("Обработка {} файлов", self.targets.len());

        let now = time::Instant::now();

        let mut results: Vec<String> = Vec::with_capacity(self.save_period);

        for (file_num, path) in self.targets.iter().enumerate() {
            let inner_now = time::Instant::now();

            let file = match open_file_r(path) {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("Can't read input file {}. {}", path.display(), err);
                    continue;
                }
            };

            let lines_count = utils::count_lines(file);

            println!(
                "[{}/{}]Файл: {}. Строк: {}",
                file_num + 1,
                self.targets.len(),
                path.display(),
                lines_count
            );

            let file = match open_file_r(path) {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("Can't read input file {}. {}", path.display(), err);
                    continue;
                }
            };

            let reader = utils::reader_from_file(file);

            // TODO: handle files with the same names but in a different dirs
            let results_path =
                utils::build_results_path(path, &self.results_path, self.task.to_suffix());
            let mut results_file: Option<File> = None;

            for (i, combo) in reader.lines().enumerate() {
                let combo = match combo {
                    Ok(combo) => combo,
                    Err(err) => {
                        eprintln!(
                            "Can't read combo on line {} in file {}. {}",
                            i,
                            path.display(),
                            err
                        );
                        continue;
                    }
                };

                let combo = PhonesExtractor::process_line(&combo);
                if results_file.is_none() && combo.is_some() {
                    results_file = Some(utils::open_results_file(&results_path)?);
                }

                if let Some(combo) = combo {
                    results.push(combo);
                }

                if results.len() == self.save_period || lines_count - i == 1 {
                    if let Err(e) = utils::save_results(&mut results, &results_file) {
                        eprintln!("Couldn't write to file: {}", e);
                    }
                }
            }

            println!("Потрачено: {:?}", inner_now.elapsed());
        }

        if self.targets.len() > 1 {
            println!("Потрачено в общем: {:?}", now.elapsed());
        }

        Ok(())
    }
}

fn extract_phone(phone: &str) -> Option<String> {
    if phone.contains('@') {
        return None;
    }

    let has_plus = phone.starts_with('+');

    let extracted_digits = {
        let digits: String = phone.matches(|x| char::is_ascii_digit(&x)).collect();
        match &*digits {
            "" => return None,
            _ => digits,
        }
    };

    if !(MIN_PHONE_LENGTH..=MAX_PHONE_LENGTH).contains(&extracted_digits.len()) {
        return None;
    }

    let mut new_phone = String::with_capacity(MAX_PHONE_LENGTH);

    let is_russian = extracted_digits.len() == 10
        && (extracted_digits.starts_with('9') || extracted_digits.starts_with('7'));
    let is_russian_town = extracted_digits.len() == 11 && extracted_digits.starts_with('8');
    let is_ukrainian = extracted_digits.len() == 10 && extracted_digits.starts_with('0');

    if is_russian {
        new_phone.push('7');
        new_phone.push_str(&extracted_digits);
    } else if is_russian_town {
        new_phone.push('7');
        new_phone.push_str(&extracted_digits[1..]);
    } else if is_ukrainian {
        new_phone.push_str("38");
        new_phone.push_str(&extracted_digits);
    } else {
        new_phone.push_str(&extracted_digits);
    }

    let is_russian = new_phone.starts_with('7') && new_phone.len() == 11;
    let is_ukrainian = new_phone.starts_with("380") && new_phone.len() == 12;
    let is_bel = new_phone.starts_with("375") && new_phone.len() == 12;
    let is_mol = new_phone.starts_with("373") && new_phone.len() == 11;

    if !is_ukrainian && !is_russian && !is_bel && !is_mol && !has_plus {
        return None;
    }

    Some(new_phone)
}
