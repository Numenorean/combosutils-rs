use std::fmt;

pub enum Task {
    RemoveDomains,
    RemoveDuplicatesM,
    RemoveDuplicatesC,
    SplitByLines,
    Merge,
    Shuffle,
    NotImplemented,
}

impl Task {
    // do we need inline here?
    pub fn to_suffix(&self) -> &str {
        match *self {
            Task::RemoveDomains => "_no_domains",
            Task::RemoveDuplicatesM | Task::RemoveDuplicatesC => "_no_duplicates",
            Task::SplitByLines => "_splitted_{num}",
            Task::Merge => "_merged",
            Task::Shuffle => "_randomized",
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
            Task::Merge => write!(f, "Склеивание"),
            Task::Shuffle => write!(f, "Перемешивание"),
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
            "--merge" => Task::Merge,
            "--shuffle" => Task::Shuffle,
            _ => Task::NotImplemented,
        }
    }
}
