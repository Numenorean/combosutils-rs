use std::{
    env::Args,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::anyhow;
use chrono::{DateTime, Local};

use super::{lines_processor::LinesProcessor, task::Task};
use crate::tools::{remove_domain::DomainRemover, remove_duplicates::*};

const SAVE_PERIOD: usize = 1000;
const RESULTS_PATH: &str = "Результаты\\{type}\\{date}\\{time}";

#[derive(Debug)]
pub struct Core {
    results_path: PathBuf,
    targets: Vec<PathBuf>,
    save_period: usize,
    task: Task,
}

impl Core {
    pub fn new(mut args: Args) -> Result<Self, anyhow::Error> {
        let binary_path = PathBuf::from(args.next().ok_or(anyhow!("bad args"))?);

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
                DomainRemover::new(self.targets, results_path, self.save_period).process()
            }

            Task::RemoveDuplicatesM => {
                DuplicatesRemoverM::new(self.targets, results_path, self.save_period).process()
            }

            Task::RemoveDuplicatesC => {
                DuplicatesRemoverC::new(self.targets, results_path, self.save_period).process()
            }
            _ => unreachable!(),
        }?;

        if self.results_path.exists() {
            Core::open_in_explorer(self.results_path);
        }

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
