use std::fs;
use std::io::ErrorKind;
use std::process;
use crate::file_names::TASKS_FILE;

pub struct Todo {
    tasks: Vec<String>,
}

impl Todo {
    pub fn new() -> Self {
        let content = match fs::read_to_string(TASKS_FILE) {
            Ok(file_content) => file_content,
            Err(e) => { 
                if e.kind() == ErrorKind::NotFound {
                    println!("Task file does not exist. Please run 'setup' to make one.");
                }
                process::exit(1);
            }
        };
        let mut tasks: Vec<String> = Vec::new();

        for line in content.lines() {
            tasks.push(line.to_string());
        }

        Self {
            tasks,
        }
    }

    pub fn list(&self) {
        for (i, a_task) in self.tasks.iter().enumerate() {
            println!("{}. {}", i + 1, a_task);
        }
    }
}
