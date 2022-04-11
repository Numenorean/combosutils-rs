use std::{io::BufRead, path::PathBuf, time, hash::BuildHasherDefault, collections::HashMap};

use nohash_hasher::NoHashHasher;

use crate::{
    cmd::Args,
    core::{
        lines_processor::LinesProcessor,
        task::Task,
        utils::{self, count_lines, open_file_r},
    },
    errors::core_error::CoreError,
};
use rayon::prelude::*;
use std::sync::Mutex;

pub struct DuplicatesExtractor {
    targets: Vec<PathBuf>,
    results_path: PathBuf,
    save_period: usize,
    task: Task,
}

impl LinesProcessor for DuplicatesExtractor {
    fn new(args: Args, results_path: PathBuf, save_period: usize) -> Self {
        DuplicatesExtractor {
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

            println!("Запись хэшей строк...");

            type NoHashMap = HashMap<u64, usize, BuildHasherDefault<NoHashHasher<u64>>>;

            let fx: Mutex<NoHashMap> = Mutex::new(NoHashMap::default());
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
                    *fx.lock().unwrap().entry(hash).or_insert(0) += 1;
                });

            // TODO: handle files with the same names but in a different dirs
            let results_path =
                utils::build_results_path(path, &self.results_path, self.task.to_suffix());
            let results_file = Some(utils::open_results_file(results_path)?);
            let mut results = Vec::with_capacity(lines_count);

            let file = match open_file_r(path) {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("Can't read input file {}. {}", path.display(), err);
                    continue;
                }
            };

            let reader = utils::reader_from_file(file);
            let mut lines_count_after: usize = 0;

            println!("Сохранение результатов...");

            for (i, combo) in reader.lines().enumerate() {
                let combo = if let Ok(combo) = combo {
                    combo
                } else {
                    continue;
                };

                let hash = seahash::hash(combo.as_bytes());
                let mut u = fx.lock().unwrap();
                if let Some(&v) = u.get(&hash) && v > 1 {
                    results.push(combo);
                    lines_count_after += 1;
                    u.remove(&hash);
                }

                if results.len() == self.save_period || lines_count - i == 1 {
                    if let Err(e) = utils::save_results(&mut results, &results_file) {
                        eprintln!("Couldn't write to file: {}", e);
                    }
                }
            }

            println!("Найдено дубликатов: {}", lines_count_after);
            println!("Потрачено: {:?}", inner_now.elapsed());
        }

        if self.targets.len() > 1 {
            println!("Потрачено в общем: {:?}", now.elapsed());
        }

        Ok(())
    }
}
