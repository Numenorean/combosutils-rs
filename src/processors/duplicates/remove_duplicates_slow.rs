use std::{io::BufRead, path::PathBuf, sync::Mutex, time};

use rayon::prelude::*;
use rustc_hash::FxHashSet;

use crate::{
    cmd::Args,
    core::{
        lines_processor::LinesProcessor,
        task::Task,
        utils::{self, count_lines, open_file_r},
    },
    errors::core_error::CoreError,
};

pub struct DuplicatesRemoverSlow {
    targets: Vec<PathBuf>,
    results_path: PathBuf,
    save_period: usize,
    task: Task,
}

impl LinesProcessor for DuplicatesRemoverSlow {
    fn new(args: Args, results_path: PathBuf, save_period: usize) -> Self {
        DuplicatesRemoverSlow {
            targets: args.targets,
            results_path,
            save_period,
            task: args.task,
        }
    }

    fn process_line(_: &str) -> Option<String> {
        unreachable!()
    }

    fn process(self) -> Result<(), CoreError> {
        println!("Обработка {} файлов", self.targets.len());

        let now = time::Instant::now();

        for (file_num, path) in self.targets.iter().enumerate() {
            let inner_now = time::Instant::now();

            let file = match open_file_r(path) {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("Can't read input file {}. {}", path.display(), err);
                    continue;
                }
            };

            let lines_count = count_lines(file);

            println!(
                "[{}/{}]Файл: {:?}. Строк: {}",
                file_num + 1,
                self.targets.len(),
                path,
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

            let hashes: Mutex<FxHashSet<u64>> = Mutex::new(FxHashSet::default());

            println!("Сохранение хэшей...");

            reader
                .lines()
                .enumerate()
                .par_bridge()
                .into_par_iter()
                .for_each(|(i, combo)| {
                    let combo = match combo {
                        Ok(combo) => combo,
                        Err(err) => {
                            eprintln!(
                                "Can't read combo on line {} in file {}. {}",
                                i,
                                path.display(),
                                err
                            );
                            return;
                        }
                    };

                    let hash = seahash::hash(combo.as_bytes());
                    hashes.lock().unwrap().insert(hash);
                });

            // TODO: handle files with the same names but in a different dirs
            let results_path =
                utils::build_results_path(path, &self.results_path, self.task.to_suffix());
            let results_file = Some(utils::open_results_file(results_path)?);
            let mut results = Vec::with_capacity(lines_count);

            let mut lines_count_after: usize = 0;

            let file = match open_file_r(path) {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("Can't read input file {}. {}", path.display(), err);
                    continue;
                }
            };

            let reader = utils::reader_from_file(file);

            println!("Удаление дубликатов...");

            for (i, combo) in reader.lines().enumerate() {
                let combo = if let Ok(combo) = combo {
                    combo
                } else {
                    continue;
                };

                let hash = seahash::hash(combo.as_bytes());
                let mut u = hashes.lock().unwrap();
                if u.take(&hash).is_some() {
                    results.push(combo);
                    lines_count_after += 1;
                }

                if results.len() == self.save_period || lines_count - i == 1 {
                    if let Err(e) = utils::save_results(&mut results, &results_file) {
                        eprintln!("Couldn't write to file: {}", e);
                    }
                }
            }

            println!("Строк после удаления: {}", lines_count_after);

            println!(
                "Удалено {} дубликатов за {:?}",
                lines_count - lines_count_after,
                inner_now.elapsed()
            );
        }

        if self.targets.len() > 1 {
            println!("Потрачено в общем: {:?}", now.elapsed());
        }

        Ok(())
    }
}
