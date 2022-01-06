use std::{fs, path::PathBuf, time};

use crate::core::{lines_processor::LinesProcessor, task::Task};

pub struct DuplicatesRemoverM {
    targets: Vec<PathBuf>,
    results_path: PathBuf,
    task: Task,
}

impl LinesProcessor for DuplicatesRemoverM {
    fn new(targets: Vec<PathBuf>, results_path: PathBuf, _: usize) -> Self {
        let task = Task::RemoveDuplicatesM;
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
            println!("[{}/{}]Файл: {:?}", file_num + 1, self.targets.len(), path);
            let inner_now = time::Instant::now();

            // TODO: handle files with the same names but in a different dirs
            let results_path = DuplicatesRemoverM::build_results_path(
                path,
                &self.results_path,
                self.task.to_suffix(),
            );
            let mut results_file = DuplicatesRemoverM::open_results_file(results_path)?;

            /*let file = match fs::OpenOptions::new().read(true).open(path) {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("Can't read input file {:?}. {}", path, err);
                    continue;
                }
            };*/

            let lines = match fs::read_to_string(path) {
                Ok(data) => data,
                Err(err) => {
                    eprintln!("Can't read input file {:?}. {}", path, err);
                    continue;
                }
            };
            let mut lines: Vec<&str> = lines.split('\n').map(|v| v.trim_end()).collect();

            /*let reader = DuplicatesRemoverM::reader_from_file(file);
            let mut lines: Vec<String> = Vec::new();

            for (i, combo) in reader.lines().enumerate() {
                let combo = match combo {
                    Ok(combo) => combo,
                    Err(err) => {
                        eprintln!("Can't read combo on line {} in file {:?}. {}", i, path, err);
                        continue;
                    }
                };

                lines.push(combo)
            }*/

            let lines_count = lines.len();

            println!("Строк: {}", lines_count);

            println!("Сортировка...");

            lines.sort_unstable();

            println!("Удаление дубликатов...");

            lines.dedup();

            let lines_count_after = lines.len();

            println!("Строк после удаления: {}", lines_count);

            if let Err(e) = DuplicatesRemoverM::save_results(&mut lines, &mut results_file) {
                eprintln!("Couldn't write to file: {}", e);
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
