use std::{io::BufRead, path::PathBuf, time};

use crate::core::{
    lines_processor::LinesProcessor,
    task::Task,
    utils::{self, open_file_r},
};

pub struct ByPartsSplitter {
    targets: Vec<PathBuf>,
    results_path: PathBuf,
    save_period: usize,
    task: Task,
    parts_n: usize,
}

impl LinesProcessor for ByPartsSplitter {
    fn new(targets: Vec<PathBuf>, results_path: PathBuf, save_period: usize, task: Task) -> Self {
        fn get_parts_n() -> usize {
            let static_err = "Что-то не так с числом";
            let input = utils::user_input("Количество частей: ");

            match input {
                Ok(input) => match input.parse::<usize>() {
                    Ok(n) => {
                        if n == 0 {
                            println!("{}: Не может быть 0", static_err);
                            return get_parts_n();
                        }
                        n
                    }
                    Err(err) => {
                        println!("{}: {}", static_err, err);
                        get_parts_n()
                    }
                },
                Err(err) => {
                    println!("{}: {}", static_err, err);
                    get_parts_n()
                }
            }
        }

        let parts_n = get_parts_n();

        ByPartsSplitter {
            targets,
            results_path,
            save_period,
            task,
            parts_n
        }
    }

    fn process_line(_: &str) -> Option<String> {
        unreachable!()
    }

    fn process(mut self) -> Result<(), anyhow::Error> {
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

            let mut lines_n = lines_count/self.parts_n;

            if lines_n == 0 {
                lines_n = lines_count;
            }

            if self.save_period > lines_n {
                self.save_period = lines_n;
            }

            let file = match open_file_r(path) {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("Can't read input file {}. {}", path.display(), err);
                    continue;
                }
            };

            let reader = utils::reader_from_file(file);

            let mut part = 1;

            // TODO: handle files with the same names but in a different dirs
            let suffix = self.task.to_suffix().replace("{num}", &part.to_string());
            let mut self_results_path = self.results_path.clone();
            self_results_path.push(path.file_name().unwrap_or_default());

            let mut results_path = utils::build_results_path(path, &self_results_path, &suffix);
            let mut results_file = Some(utils::open_results_file(results_path)?);

            let mut already_written = 0usize;

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

                results.push(combo);

                let next_group = i % lines_n == 1 && i > lines_n;

                let last_combo = lines_count - i == 1;

                // we have the second branch for saving results which we are not going to save right away
                // so we need to save it only if don't need to switch to the next file
                // in another way we will be just saving results from the second branch hence results files won't contains needed ammount of lines
                // UPD: we also need to do lines_n - already_written >= self.save_period, because otherwise it write unnecessary data before next_group will be true
                let need_save = results.len() == self.save_period
                    && !next_group
                    && lines_n - already_written >= self.save_period;

                if need_save || last_combo {
                    if let Err(e) = utils::save_results(&mut results, &results_file) {
                        eprintln!("Couldn't write to file: {}", e);
                    }
                    already_written += self.save_period;
                } else if next_group && already_written != lines_n {
                    let need_write = lines_n - already_written;

                    let mut to_be_written: Vec<&String> = results.iter().take(need_write).collect();

                    if let Err(e) = utils::save_results(&mut to_be_written, &results_file) {
                        eprintln!("Couldn't write to file: {}", e);
                    }

                    results.truncate(need_write);
                }

                if next_group {
                    part+=1;
                    let suffix = self
                        .task
                        .to_suffix()
                        .replace("{num}", &part.to_string());
                    results_path = utils::build_results_path(path, &self_results_path, &suffix);
                    results_file = Some(utils::open_results_file(results_path)?);
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
