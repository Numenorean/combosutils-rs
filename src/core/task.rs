use std::fmt;

pub enum Task {
    RemoveDomains,
    BadTask,
}

impl fmt::Debug for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Task::RemoveDomains => write!(f, "Удаление доменов"),
            _ => write!(f, "Такого пока нет"),
        }
    }
}

impl From<&str> for Task {
    fn from(s: &str) -> Task {
        match s {
            "/RemoveDomains" => Task::RemoveDomains,
            _ => Task::BadTask,
        }
    }
}
