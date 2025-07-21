use std::io;
use std::io::Write;
use chrono::{Duration, Local, NaiveDate};
use std::fs;
use std::io::ErrorKind;
use std::process;
use crate::file_names::{TASKS_FILE, TRACKER_FILE};
use crate::task::Task;

pub struct Todo {
    tasks: Vec<Task>,
}

impl Todo {
    pub fn new() -> Self {
        let mut tasks = Self::read_tasks();
        Self::populate_done(&mut tasks);
        Self {
            tasks,
        }
    }

    pub fn list(&self) {
        for (i, a_task) in self.tasks.iter().enumerate() {
            println!("{}. {}", i + 1, a_task);
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

    pub fn read_tracker() -> String {
        let content = match fs::read_to_string(TRACKER_FILE) {
            Ok(file_content) => file_content,
            Err(e) => { 
                if e.kind() == ErrorKind::NotFound {
                    println!("Tracker file does not exist. Please run 'setup' to make one.");
                }
                process::exit(1);
            }
        };
        content
    }

    pub fn populate_done(tasks: &mut Vec<Task>) {
        let content = Self::read_tracker();
        if let Some(last_line) = content.lines().last() {
            let elements: Vec<&str> = last_line.split(',').collect();
            let last_date = elements[0];
            if let Ok(last_date) = NaiveDate::parse_from_str(last_date, "%Y-%m-%d") {
                let today = Local::now().date_naive();
                if last_date < today {
                    return; 
                }
                for (task, ele) in tasks.iter_mut().zip(elements.iter().skip(1)) {
                    task.done = *ele == "1";
                }
            }
        }
    }

    pub fn ask_completion(&self) -> String {
        let mut completions = String::new();
        println!("Did you complete these tasks:");

        for a_task in self.tasks.iter() {
            completions.push(',');
            print!("{} [y/N]", a_task.name.clone());

            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let ch = input.trim().chars().next();
            if let Some(response) = ch {
                match response {
                    'Y' | 'y' => completions.push('1'),
                    _ => completions.push('0'),
                };
            } else {
                completions.push('0');
            }
        }
        completions
    }

    pub fn track(&self) {
        let content = Self::read_tracker();
        let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

        let date_str = lines
            .last()
            .and_then(|line| line.split(',').next())
            .unwrap()
            .to_string();

        let mut last_date = match NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
            Ok(date) => date + Duration::days(1),
            Err(_) => Local::now().date_naive(),
        };

        let today = Local::now().date_naive();
        let tomorrow = today + Duration::days(1);

        while last_date != tomorrow {
            println!("For date: {}", last_date);
            let completions = self.ask_completion();
            let last_line = format!("{}{}", last_date, completions);
            lines.push(last_line);
            last_date += Duration::days(1);
        }
        let mut updated_content = lines.join("\n");
        updated_content.push('\n');
        fs::write(TRACKER_FILE, updated_content).expect("Failed to write file");
    }
}
