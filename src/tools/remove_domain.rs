use std::{fs, io::BufRead, path::PathBuf, time};

use crate::core::lines_processor::LinesProcessor;

pub struct DomainRemover {
    targets: Vec<PathBuf>,
    results_path: PathBuf,
    save_period: usize,
}

impl LinesProcessor for DomainRemover {
    fn new(targets: Vec<PathBuf>, results_path: PathBuf, save_period: usize) -> Self {
        DomainRemover {
            targets,
            results_path,
            save_period,
        }
    }

    fn process_line(line: &str) -> Option<String> {
        remove_domain(line)
    }

    fn process(self) -> Result<(), anyhow::Error> {
        println!("Обработка {} файлов", self.targets.len());

        let now = time::Instant::now();

        let mut results: Vec<String> = Vec::with_capacity(self.save_period);

        for (file_num, path) in self.targets.iter().enumerate() {
            println!("[{}/{}]Файл: {:?}", file_num + 1, self.targets.len(), path);
            let inner_now = time::Instant::now();

            let file = match fs::OpenOptions::new().read(true).open(&path) {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("Can't read input file {:?}. {}", path, err);
                    continue;
                }
            };

            let lines_count = DomainRemover::count_lines(file);

            // TODO: handle files with the same names but in a different dirs
            let results_path =
                DomainRemover::build_results_path(path, &self.results_path, "_no_domains");
            let mut results_file = DomainRemover::open_results_file(results_path)?;

            let file = match fs::OpenOptions::new().read(true).open(path) {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("Can't read input file {:?}. {}", path, err);
                    continue;
                }
            };

            let reader = DomainRemover::reader_from_file(file);

            for (i, combo) in reader.lines().enumerate() {
                let combo = match combo {
                    Ok(combo) => combo,
                    Err(err) => {
                        eprintln!("Can't read combo on line {} in file {:?}. {}", i, path, err);
                        continue;
                    }
                };

                let combo = DomainRemover::process_line(&combo);
                if let Some(combo) = combo {
                    results.push(combo);
                }

                if results.len() == self.save_period || lines_count - i <= self.save_period {
                    if let Err(e) = DomainRemover::save_results(&mut results, &mut results_file) {
                        eprintln!("Couldn't write to file: {}", e);
                    }
                    results.clear();
                }
            }

            println!("Потрачено: {:?}", inner_now.elapsed());
        }

        println!("Потрачено в общем: {:?}", now.elapsed());

        Ok(())
    }
}

fn remove_domain(combo: &str) -> Option<String> {
    let (email, password) = combo.split_once(&[':', ';'][..])?;
    if email.is_empty() || password.is_empty() {
        return None;
    }

    email.split('@').next().map(|username| {
        let mut result = String::from(username);
        result.push(':');
        result.push_str(password);
        result
    })
}
