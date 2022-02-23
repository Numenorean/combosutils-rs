use std::{path::PathBuf, time};

use rustc_hash::FxHashSet;

use crate::core::{lines_processor::LinesProcessor, task::Task, utils};

pub struct DuplicatesRemoverM {
    targets: Vec<PathBuf>,
    results_path: PathBuf,
    task: Task,
}

impl LinesProcessor for DuplicatesRemoverM {
    fn new(targets: Vec<PathBuf>, results_path: PathBuf, _: usize, task: Task) -> Self {
        DuplicatesRemoverM {
            targets,
            results_path,
            task,
        }
    }

    fn process_line(_: &str) -> Option<String> {
        unreachable!()
    }

    fn process(self) -> Result<(), anyhow::Error> {
        println!("Обработка {} файлов", self.targets.len());

        let now = time::Instant::now();

        for (file_num, path) in self.targets.iter().enumerate() {
            let inner_now = time::Instant::now();

            let mut lines = String::new();

            let lines = match utils::read_lines(path, &mut lines) {
                Err(err) => {
                    eprintln!("Can't read input file {:?}. {}", path, err);
                    continue;
                }
                Ok(lines) => lines,
            };

            let lines_count = lines.len();
            println!(
                "[{}/{}]Файл: {:?}. Строк: {}",
                file_num + 1,
                self.targets.len(),
                path,
                lines_count
            );

            println!("Удаление дубликатов...");

            let lines: FxHashSet<&str> = FxHashSet::from_iter(lines);

            let lines_count_after = lines.len();

            println!("Строк после удаления: {}", lines_count_after);
            println!("Сохранение...");

            // TODO: handle files with the same names but in a different dirs
            let results_path =
                utils::build_results_path(path, &self.results_path, self.task.to_suffix());
            let mut results_file = utils::open_results_file(results_path)?;

            if let Err(e) =
                utils::save_results_hashset(&mut lines.iter().copied(), &mut results_file)
            {
                eprintln!("Couldn't save results to file: {}", e);
                continue;
            }

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
