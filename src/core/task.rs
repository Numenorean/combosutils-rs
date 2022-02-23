use std::fmt;

#[derive(Clone, Copy)]
pub enum Task {
    RemoveDomains,
    RemoveDuplicatesM,
    RemoveDuplicatesC,
    SplitByLines,
    SplitByParts,
    Merge,
    Shuffle,
    ExtractLogins,
    ExtractPasswords,
    ExtractPhones,
    NotImplemented,
}

impl Task {
    // do we need inline here?
    pub fn to_suffix(self) -> &'static str {
        match self {
            Task::RemoveDomains => "_no_domains",
            Task::RemoveDuplicatesM | Task::RemoveDuplicatesC => "_no_duplicates",
            Task::SplitByLines | Task::SplitByParts => "_splitted_{num}",
            Task::Merge => "_merged",
            Task::Shuffle => "_randomized",
            Task::ExtractLogins => "_logins",
            Task::ExtractPasswords => "_passwords",
            Task::ExtractPhones => "_phones",
            _ => unreachable!(),
        }
    }
}

impl fmt::Debug for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Task::RemoveDomains => write!(f, "Удаление доменов"),
            Task::RemoveDuplicatesM | Task::RemoveDuplicatesC => write!(f, "Удаление дубликатов"),
            Task::SplitByLines => write!(f, "Разделение по количеству строк"),
            Task::SplitByParts => write!(f, "Разделение по частям"),
            Task::Merge => write!(f, "Склеивание"),
            Task::Shuffle => write!(f, "Перемешивание"),
            Task::ExtractLogins => write!(f, "Получение логинов"),
            Task::ExtractPasswords => write!(f, "Получение паролей"),
            Task::ExtractPhones => write!(f, "Нормализация телефонов"),
            _ => write!(f, "Такого пока нет"),
        }
    }
}

impl From<&str> for Task {
    fn from(s: &str) -> Task {
        match s {
            "--remove-domains" => Task::RemoveDomains,
            "--remove-duplicates-m" => Task::RemoveDuplicatesM,
            "--remove-duplicates-c" => Task::RemoveDuplicatesC,
            "--split-by-lines" => Task::SplitByLines,
            "--split-by-parts" => Task::SplitByParts,
            "--merge" => Task::Merge,
            "--shuffle" => Task::Shuffle,
            "--extract-logins" => Task::ExtractLogins,
            "--extract-passwords" => Task::ExtractPasswords,
            "--extract-phones" => Task::ExtractPhones,
            _ => Task::NotImplemented,
        }
    }
}
