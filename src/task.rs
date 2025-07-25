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
        if self.done {
            write!(f, "{}", self.name.strikethrough())
        } else {
            write!(f, "{}", self.name)
        }
    }
}
