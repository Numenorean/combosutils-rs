use std::{path::PathBuf, time};

use memmap::MmapOptions;

use crate::core::{
    lines_processor::LinesProcessor,
    task::Task,
    utils::{self, open_file_r},
};

use rand::{prelude::SliceRandom, thread_rng};

pub struct Shuffler {
    targets: Vec<PathBuf>,
    results_path: PathBuf,
    save_period: usize,
    task: Task,
}

#[derive(Debug)]
struct ComboOffset {
    start: usize,
    end: usize,
}

impl LinesProcessor for Shuffler {
    fn new(targets: Vec<PathBuf>, results_path: PathBuf, save_period: usize, task: Task) -> Self {
        Shuffler {
            targets,
            results_path,
            save_period,
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

            // TODO: handle files with the same names but in a different dirs
            let results_path =
                utils::build_results_path(path, &self.results_path, self.task.to_suffix());
            let results_file = Some(utils::open_results_file(results_path)?);

            let mmap = unsafe { MmapOptions::new().map(&file)? };

            let mut start = 0usize;

            let mut lines_offsets: Vec<ComboOffset> = Vec::with_capacity(lines_count);

            println!("Поиск сдвигов...");

            for (end, &char) in mmap.iter().enumerate() {
                if char == b'\n' || mmap.len() - end == 1 {
                    lines_offsets.push(ComboOffset { start, end });
                    start = end + 1;
                }
            }

            let mut rng = thread_rng();

            println!("Перемешивание...");

            lines_offsets.shuffle(&mut rng);

            let mut results: Vec<String> = Vec::with_capacity(self.save_period);

            println!("Сохранение результатов...");

            for (i, offset) in lines_offsets.iter().enumerate() {
                let data = &mmap[offset.start..=offset.end];
                let combo = String::from_utf8_lossy(data).trim_end().to_owned();

                results.push(combo);

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
