use std::fmt;
use colored::*;

pub struct Task {
    pub name: String,
    pub done: bool,
}

impl Task {
    pub fn new(name: String, done: bool) -> Self {
        Self {
            name,
            done,
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let task_line: String;
        if self.done {
            task_line = self.name.strikethrough().to_string();
        } else {
            task_line = self.name.clone();
        }
        write!(f, "{}", task_line)
    }
}
