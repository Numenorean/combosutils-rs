use std::fmt;

pub enum Task {
    RemoveDomains,
    RemoveDuplicatesM,
    NotImplemented,
}

impl Task {
    // do we need inline here?
    pub fn to_suffix(&self) -> &'static str {
        match *self {
            Task::RemoveDomains => "_no_domains",
            Task::RemoveDuplicatesM => "_no_duplicates",
            _ => unreachable!(),
        }
    }
}

impl fmt::Debug for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Task::RemoveDomains => write!(f, "Удаление доменов"),
            Task::RemoveDuplicatesM => write!(f, "Удаление дубликатов"),
            _ => write!(f, "Такого пока нет"),
        }
    }
}

impl From<&str> for Task {
    fn from(s: &str) -> Task {
        match s {
            "--remove-domains" => Task::RemoveDomains,
            "--remove-duplicates-m" => Task::RemoveDuplicatesM,
            _ => Task::NotImplemented,
        }
    }
}

#[macro_export]
macro_rules! task_to_suffix {
    ( $e:expr ) => {
        match $e {
            Task::RemoveDomains => "_no_domains",
            Task::RemoveDuplicatesM => "_no_duplicates",
            _ => unreachable!(),
        }
    };
}
