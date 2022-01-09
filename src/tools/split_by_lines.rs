use std::{fs, io::{BufRead, Read}, path::PathBuf, time};

use crate::core::{
    lines_processor::LinesProcessor,
    task::Task,
    utils::{self, open_file_r},
};

pub struct ByLinesSplitter {
    targets: Vec<PathBuf>,
    results_path: PathBuf,
    save_period: usize,
    task: Task,
    lines_n: usize,
}

impl LinesProcessor for ByLinesSplitter {
    fn new(targets: Vec<PathBuf>, results_path: PathBuf, save_period: usize) -> Self {
        let task = Task::SplitByLines;

        fn get_lines_n() -> usize {
            let static_err = anyhow::anyhow!("Что-то не так с числом");
            let input = utils::user_input("Количество строк в каждом файле: ");

            match input {
                Ok(input) => match input.parse::<usize>() {
                    Ok(n) => n,
                    Err(err) => {
                        println!("{}: {}", static_err, err);
                        get_lines_n()
                    }
                },
                Err(err) => {
                    println!("{}: {}", static_err, err);
                    get_lines_n()
                }
            }
        }

        let lines_n = get_lines_n();

        ByLinesSplitter {
            targets,
            results_path,
            save_period,
            task,
            lines_n,
        }
    }

    fn process_line(_: &str) -> Option<String> {
        unreachable!()
    }

    fn process(self) -> Result<(), anyhow::Error> {
        println!("Обработка {} файлов", self.targets.len());

        let now = time::Instant::now();

        let mut results: Vec<String> = Vec::with_capacity(self.save_period);

        for (file_num, path) in self.targets.iter().enumerate() {
            let inner_now = time::Instant::now();

            let file = match open_file_r(path) {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("Can't read input file {:?}. {}", path, err);
                    continue;
                }
            };

            let lines_count = utils::count_lines(file);

            println!(
                "[{}/{}]Файл: {:?}. Строк: {}",
                file_num + 1,
                self.targets.len(),
                path,
                lines_count
            );

            let file = match fs::OpenOptions::new().read(true).open(path) {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("Can't read input file {:?}. {}", path, err);
                    continue;
                }
            };

            let reader = utils::reader_from_file(file);

            // TODO: handle files with the same names but in a different dirs
            let suffix = self
                .task
                .to_suffix()
                .replace("{num}", &self.lines_n.to_string());
            let mut results_path = utils::build_results_path(path, &self.results_path, &suffix);
            let mut results_file = utils::open_results_file(results_path)?;

            let mut already_written = 0usize;

            for (i, combo) in reader.lines().enumerate() {
                let combo = match combo {
                    Ok(combo) => combo,
                    Err(err) => {
                        eprintln!("Can't read combo on line {} in file {:?}. {}", i, path, err);
                        continue;
                    }
                };

                results.push(combo);

                let next_group = i % self.lines_n == 1 && i > self.lines_n;

                let last_combo = lines_count - i == 1;

                // we have the second branch for saving results which we are not going to save right away
                // so we need to save it only if don't need to switch to the next file
                // in another way we will be just saving results from the second branch hence results files won't contains needed ammount of lines
                let need_save = results.len() == self.save_period && !next_group;

                if need_save || last_combo {
                    if let Err(e) = utils::save_results(&mut results, &mut results_file) {
                        eprintln!("Couldn't write to file: {}", e);
                    }
                    already_written += self.save_period;
                } else if next_group && already_written != self.lines_n {
                    let need_write = self.lines_n - already_written;

                    let mut to_be_written: Vec<&String> = results.iter().take(need_write).collect();

                    if let Err(e) = utils::save_results(&mut to_be_written, &mut results_file) {
                        eprintln!("Couldn't write to file: {}", e);
                    }

                    results.truncate(need_write);
                }

                if next_group {
                    let suffix = self
                        .task
                        .to_suffix()
                        .replace("{num}", &(i - 1 + self.lines_n).to_string());
                    results_path = utils::build_results_path(path, &self.results_path, &suffix);
                    results_file = utils::open_results_file(results_path)?;
                    already_written = 0;
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