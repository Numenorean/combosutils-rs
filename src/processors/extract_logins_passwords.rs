use std::{io::BufRead, path::PathBuf, time};

use crate::{
    core::{
        lines_processor::LinesProcessor,
        task::Task,
        utils::{self, open_file_r},
    },
    errors::core_error::CoreError,
};

pub struct PartExtractor {
    targets: Vec<PathBuf>,
    results_path: PathBuf,
    save_period: usize,
    task: Task,
}

impl LinesProcessor for PartExtractor {
    fn new(targets: Vec<PathBuf>, results_path: PathBuf, save_period: usize, task: Task) -> Self {
        PartExtractor {
            targets,
            results_path,
            save_period,
            task,
        }
    }

    fn process_line(_: &str) -> Option<String> {
        unreachable!()
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
            let results_file = Some(utils::open_results_file(results_path)?);

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

                let part = extract(&combo, self.task);
                if let Some(combo) = part {
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

fn extract(combo: &str, task: Task) -> Option<String> {
    let (email, password) = combo.split_once([':', ';'])?;
    if email.is_empty() || password.is_empty() {
        return None;
    }

    match task {
        Task::ExtractLogins => Some(email.to_owned()),
        Task::ExtractPasswords => Some(password.to_owned()),
        _ => unreachable!(),
    }
}
