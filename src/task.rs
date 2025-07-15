use std::fs;
use std::io::ErrorKind;
use std::process;
use std::fmt;
use chrono::{Local,NaiveDate};
use colored::*;
use crate::file_names::{TASKS_FILE, TRACKER_FILE};

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

    pub fn read_tasks() -> Vec<Task> {
        let content = match fs::read_to_string(TASKS_FILE) {
            Ok(file_content) => file_content,
            Err(e) => { 
                if e.kind() == ErrorKind::NotFound {
                    println!("Task file does not exist. Please run 'setup' to make one.");
                }
                process::exit(1);
            }
        };
        let mut tasks: Vec<Task> = Vec::new();

        for line in content.lines() {
            let task = Task::new(line.to_string(), false);
            tasks.push(task);
        }

        tasks
    }

    pub fn read_tracker(tasks: &mut Vec<Self>) {
        let content = match fs::read_to_string(TRACKER_FILE) {
            Ok(file_content) => file_content,
            Err(e) => { 
                if e.kind() == ErrorKind::NotFound {
                    println!("Task file does not exist. Please run 'setup' to make one.");
                }
                process::exit(1);
            }
        };

        let lines: Vec<&str> = content.lines().collect();
        if let Some(last_line) = lines.last() {
            let elements: Vec<&str> = last_line.split(',').collect();
            let last_date = elements[0];
            let last_date = NaiveDate::parse_from_str(last_date, "%Y-%m-%d").unwrap();

            let today = Local::now().date_naive();
            if last_date < today {
                return; 
            }
            for (i, ele) in elements.iter().enumerate().skip(1) {
                match *ele {
                    "1" => tasks[i-1].done = true,
                    _ => tasks[i-1].done = false,
                }
            }
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