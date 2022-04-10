use std::{path::PathBuf, time};

use rayon::slice::ParallelSliceMut;

use crate::{
    cmd::Args,
    core::{lines_processor::LinesProcessor, task::Task, utils},
    errors::core_error::CoreError,
};

pub struct DuplicatesRemoverC {
    targets: Vec<PathBuf>,
    results_path: PathBuf,
    task: Task,
}

impl LinesProcessor for DuplicatesRemoverC {
    fn new(args: Args, results_path: PathBuf, _: usize) -> Self {
        DuplicatesRemoverC {
            targets: args.targets,
            results_path,
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

            let mut lines = String::new();

            let mut lines = match utils::read_lines(path, &mut lines) {
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

            println!("Сортировка...");

            lines.par_sort_unstable();

            println!("Удаление дубликатов...");

            lines.dedup();

            let lines_count_after = lines.len();

            println!("Строк после удаления: {}", lines_count_after);
            println!("Сохранение...");

            // TODO: handle files with the same names but in a different dirs
            let results_path =
                utils::build_results_path(path, &self.results_path, self.task.to_suffix());
            let results_file = Some(utils::open_results_file(results_path)?);

            if let Err(e) = utils::save_results(&mut lines, &results_file) {
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
