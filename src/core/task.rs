use std::{fmt, str::FromStr};

use clap::{ArgEnum, PossibleValue};

#[derive(Debug, Clone, Copy, ArgEnum)]
pub enum Task {
    RemoveDomains,
    RemoveDuplicatesFast,
    RemoveDuplicatesSlow,
    ExtractDuplicates,
    SplitByLines,
    SplitByParts,
    Merge,
    Shuffle,
    ExtractLogins,
    ExtractPasswords,
    ExtractPhones,
    Compare,
}

impl Task {
    // do we need inline here?
    pub fn to_suffix(self) -> &'static str {
        match self {
            Task::RemoveDomains => "_no_domains",
            Task::RemoveDuplicatesFast | Task::RemoveDuplicatesSlow => "_no_duplicates",
            Task::SplitByLines | Task::SplitByParts => "_splitted_{num}",
            Task::Merge => "_merged",
            Task::Shuffle => "_randomized",
            Task::ExtractLogins => "_logins",
            Task::ExtractPasswords => "_passwords",
            Task::ExtractPhones => "_phones",
            Task::ExtractDuplicates => "_duplicates",
            Task::Compare => "_uniq_{file}",
        }
    }

    pub fn possible_values() -> impl Iterator<Item = PossibleValue<'static>> {
        Task::value_variants()
            .iter()
            .filter_map(ArgEnum::to_possible_value)
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Task::RemoveDomains => write!(f, "Удаление доменов"),
            Task::RemoveDuplicatesFast | Task::RemoveDuplicatesSlow => {
                write!(f, "Удаление дубликатов")
            }
            Task::SplitByLines => write!(f, "Разделение по количеству строк"),
            Task::SplitByParts => write!(f, "Разделение по частям"),
            Task::Merge => write!(f, "Склеивание"),
            Task::Shuffle => write!(f, "Перемешивание"),
            Task::ExtractLogins => write!(f, "Получение логинов"),
            Task::ExtractPasswords => write!(f, "Получение паролей"),
            Task::ExtractPhones => write!(f, "Нормализация телефонов"),
            Task::ExtractDuplicates => write!(f, "Дубликаты"),
            Task::Compare => write!(f, "Сравнение"),
        }
    }
}

impl FromStr for Task {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let task = match s {
            "remove-domains" => Task::RemoveDomains,
            "remove-duplicates-fast" => Task::RemoveDuplicatesFast,
            "remove-duplicates-slow" => Task::RemoveDuplicatesSlow,
            "split-by-lines" => Task::SplitByLines,
            "split-by-parts" => Task::SplitByParts,
            "merge" => Task::Merge,
            "shuffle" => Task::Shuffle,
            "extract-logins" => Task::ExtractLogins,
            "extract-passwords" => Task::ExtractPasswords,
            "extract-phones" => Task::ExtractPhones,
            "extract-duplicates" => Task::ExtractDuplicates,
            "compare" => Task::Compare,
            _ => return Err("Такого пока нет".to_owned()),
        };

        Ok(task)
    }
}
