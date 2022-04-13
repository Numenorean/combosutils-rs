use std::{
    ffi::OsString,
    io::BufRead,
    path::PathBuf,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Mutex,
    },
    time,
};

use rayon::prelude::*;

use crate::{
    cmd::Args,
    core::{
        lines_processor::LinesProcessor,
        task::Task,
        utils::{self, open_file_r, NoHashSet},
    },
    errors::core_error::CoreError,
};

pub struct Comparer {
    targets: Vec<PathBuf>,
    results_path: PathBuf,
    compare_with: Vec<PathBuf>,
    compare_name: OsString,
    save_period: usize,
    task: Task,
}

impl LinesProcessor for Comparer {
    fn new(args: Args, results_path: PathBuf, save_period: usize) -> Self {
        let compare_with = if let Some(compare_with) = args.compare_with {
            compare_with
        } else {
            unimplemented!()
        };

        let compare_name = compare_with.file_stem().unwrap_or_default().to_owned();

        let compare_with_dir = match utils::list_dir(compare_with) {
            Ok(dir) => dir,
            Err(error) => {
                panic!("Can't list directory files: {}", error)
            }
        };

        Comparer {
            targets: args.targets,
            results_path,
            compare_with: compare_with_dir,
            compare_name,
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

            let reader = utils::reader_from_file(file);

            println!("[{}/{}]Файл: {:?}", file_num + 1, self.targets.len(), path,);

            println!("Сохранение хэшей строк основного файла...");

            let main_lines: Mutex<NoHashSet> = Mutex::new(NoHashSet::default());
            let lines_count = AtomicUsize::new(0);

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
                    main_lines.lock().unwrap().insert(hash);
                    lines_count.fetch_add(1, Ordering::Relaxed);
                });

            let lines_count = lines_count.load(Ordering::Relaxed);

            let total_lines = AtomicUsize::new(0);

            println!("Сравнение...");
            self.compare_with
                .iter()
                .enumerate()
                .par_bridge()
                .into_par_iter()
                .for_each(|(file_num, compare_path)| {
                    let file = match open_file_r(compare_path) {
                        Ok(file) => file,
                        Err(err) => {
                            eprintln!("Can't read input file {}. {}", compare_path.display(), err);
                            return;
                        }
                    };

                    let reader = utils::reader_from_file(file);

                    let lines: NoHashSet = reader
                        .lines()
                        .enumerate()
                        .filter_map(|(line_num, combo)| {
                            let combo = match combo {
                                Ok(combo) => combo,
                                Err(err) => {
                                    eprintln!(
                                        "Can't read combo on line {} in file {}. {}",
                                        line_num,
                                        compare_path.display(),
                                        err
                                    );
                                    return None;
                                }
                            };

                            total_lines.fetch_add(1, Ordering::Relaxed);

                            let hash = seahash::hash(combo.as_bytes());
                            Some(hash)
                        })
                        .collect();

                    let mut mx = main_lines.lock().unwrap();

                    println!(
                        "[{}/{}]Сравнение с: {:?}",
                        file_num + 1,
                        self.compare_with.len(),
                        compare_path,
                    );

                    mx.drain_filter(|x| lines.contains(x));
                });

            let main_lines = main_lines.into_inner().unwrap();

            println!("Сохранение...");
            if !main_lines.is_empty() {
                let file = match open_file_r(path) {
                    Ok(file) => file,
                    Err(err) => {
                        eprintln!("Can't read input file {}. {}", path.display(), err);
                        continue;
                    }
                };

                let reader = utils::reader_from_file(file);

                let suffix = self
                    .task
                    .to_suffix()
                    .replace("{file}", self.compare_name.to_str().unwrap_or_default());
                let results_path = utils::build_results_path(path, &self.results_path, &suffix);
                let results_file = Some(utils::open_results_file(results_path)?);

                let mut results = Vec::with_capacity(lines_count);

                reader
                    .lines()
                    .enumerate()
                    .filter_map(|(i, combo)| {
                        if let Ok(combo) = combo {
                            let hash = seahash::hash(combo.as_bytes());
                            Some((i, combo, hash))
                        } else {
                            None
                        }
                    })
                    .for_each(|(i, combo, hash)| {
                        if main_lines.contains(&hash) {
                            results.push(combo);
                        }

                        if results.len() == self.save_period || lines_count - i == 1 {
                            if let Err(e) = utils::save_results(&mut results, &results_file) {
                                eprintln!("Couldn't write to file: {}", e);
                            }
                        }
                    });
            }

            println!(
                "Итого {}/{} уникальных строк за {:?}\nФайлов для сравнения: {}. Всего строк: {}",
                main_lines.len(),
                lines_count,
                inner_now.elapsed(),
                self.compare_with.len(),
                total_lines.load(Ordering::Relaxed),
            );
        }

        if self.targets.len() > 1 {
            println!("Потрачено в общем: {:?}", now.elapsed());
        }

        Ok(())
    }
}
