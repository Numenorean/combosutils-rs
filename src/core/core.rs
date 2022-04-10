use std::{
    path::{Path, PathBuf},
    process::Command,
};

use chrono::{DateTime, Local};

use super::{lines_processor::LinesProcessor, task::Task};
use crate::{
    cmd::Args,
    errors::core_error::CoreError,
    processors::{
        duplicates::*, extract_logins_passwords::PartExtractor, extract_phones::PhonesExtractor,
        merge::Merger, remove_domain::DomainRemover, shuffle::Shuffler,
        split_by_lines::ByLinesSplitter, split_by_parts::ByPartsSplitter,
    },
};

const SAVE_PERIOD: usize = 1000;
const RESULTS_PATH: &str = "Результаты\\{type}\\{date}\\{time}";

pub struct Core {
    args: Args,
    results_path: PathBuf,
    save_period: usize,
}

impl Core {
    pub fn new(args: Args) -> Result<Self, CoreError> {
        let base_path = args.binary_path.parent().ok_or(CoreError::UnexpectedArgs)?;
        let results_path = Core::format_results_path(base_path, &args.task);

        let save_period = SAVE_PERIOD;

        Ok(Core {
            args,
            results_path,
            save_period,
        })
    }

    pub fn process(self) -> Result<(), CoreError> {
        let results_path = self.results_path.clone();
        match self.args.task {
            Task::RemoveDomains => {
                DomainRemover::new(self.args, results_path, self.save_period).process()
            }

            Task::RemoveDuplicatesM => {
                DuplicatesRemoverM::new(self.args, results_path, self.save_period).process()
            }

            Task::RemoveDuplicatesC => {
                DuplicatesRemoverC::new(self.args, results_path, self.save_period).process()
            }

            Task::SplitByLines => {
                ByLinesSplitter::new(self.args, results_path, self.save_period).process()
            }

            Task::SplitByParts => {
                ByPartsSplitter::new(self.args, results_path, self.save_period).process()
            }

            Task::Merge => Merger::new(self.args, results_path, self.save_period).process(),

            Task::Shuffle => Shuffler::new(self.args, results_path, self.save_period).process(),

            Task::ExtractLogins | Task::ExtractPasswords => {
                PartExtractor::new(self.args, results_path, self.save_period).process()
            }

            Task::ExtractPhones => {
                PhonesExtractor::new(self.args, results_path, self.save_period).process()
            }
        }?;

        if !self.results_path.exists() {
            return Err(CoreError::NoResults);
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
            .replace("{type}", format!("{}", task).as_str())
            .replace("{time}", &date.format("%H_%M_%S").to_string());
        base_path.join(result_path)
    }
}
