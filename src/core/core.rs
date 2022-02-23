use std::{
    env::Args,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::anyhow;
use chrono::{DateTime, Local};

use super::{lines_processor::LinesProcessor, task::Task};
use crate::tools::{
    extract_logins_passwords::PartExtractor, extract_phones::PhonesExtractor, merge::Merger,
    remove_domain::DomainRemover, duplicates::*, shuffle::Shuffler,
    split_by_lines::ByLinesSplitter, split_by_parts::ByPartsSplitter,
};

const SAVE_PERIOD: usize = 1000;
const RESULTS_PATH: &str = "Результаты\\{type}\\{date}\\{time}";

pub struct Core {
    results_path: PathBuf,
    targets: Vec<PathBuf>,
    save_period: usize,
    task: Task,
}

impl Core {
    pub fn new(mut args: Args) -> Result<Self, anyhow::Error> {
        let binary_path = args.next().ok_or(anyhow!("bad args"))?;
        let binary_path = PathBuf::from(binary_path);

        let task = match args.next().ok_or(anyhow!("bad args"))?.as_str().into() {
            Task::NotImplemented => return Err(anyhow!("{:?}", Task::NotImplemented)),
            task => task,
        };

        let base_path = binary_path.parent().ok_or(anyhow!("bad args"))?;
        let results_path = Core::format_results_path(base_path, &task);

        let targets: Vec<PathBuf> = args.map(PathBuf::from).collect();

        if targets.is_empty() {
            return Err(anyhow!("targets are not specified"));
        }

        let save_period = SAVE_PERIOD;

        Ok(Core {
            results_path,
            targets,
            save_period,
            task,
        })
    }

    pub fn process(self) -> Result<(), anyhow::Error> {
        let results_path = self.results_path.clone();
        match self.task {
            Task::RemoveDomains => {
                DomainRemover::new(self.targets, results_path, self.save_period, self.task)
                    .process()
            }

            Task::RemoveDuplicatesM => {
                DuplicatesRemoverM::new(self.targets, results_path, self.save_period, self.task)
                    .process()
            }

            Task::RemoveDuplicatesC => {
                DuplicatesRemoverC::new(self.targets, results_path, self.save_period, self.task)
                    .process()
            }

            Task::SplitByLines => {
                ByLinesSplitter::new(self.targets, results_path, self.save_period, self.task)
                    .process()
            }

            Task::SplitByParts => {
                ByPartsSplitter::new(self.targets, results_path, self.save_period, self.task)
                    .process()
            }

            Task::Merge => {
                Merger::new(self.targets, results_path, self.save_period, self.task).process()
            }

            Task::Shuffle => {
                Shuffler::new(self.targets, results_path, self.save_period, self.task).process()
            }

            Task::ExtractLogins | Task::ExtractPasswords => {
                PartExtractor::new(self.targets, results_path, self.save_period, self.task)
                    .process()
            }

            Task::ExtractPhones => {
                PhonesExtractor::new(self.targets, results_path, self.save_period, self.task)
                    .process()
            }
            _ => unreachable!(),
        }?;

        if !self.results_path.exists() {
            return Err(anyhow!("Результатов нет"));
        }

        Core::open_in_explorer(self.results_path);

        Ok(())
    }

    fn open_in_explorer(path: PathBuf) {
        Command::new("explorer").arg(path).spawn().unwrap();
    }

    fn format_results_path(base_path: &Path, task: &Task) -> PathBuf {
        let date: DateTime<Local> = Local::now();
        let result_path = RESULTS_PATH
            .replace("{date}", &date.format("%d.%m.%Y").to_string())
            .replace("{type}", format!("{:?}", task).as_str())
            .replace("{time}", &date.format("%H_%M_%S").to_string());
        base_path.join(result_path)
    }
}
