use std::io;
use std::io::Write;
use chrono::{Duration, Local, NaiveDate};
use std::fs::{self, OpenOptions};
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
                } else {
                    println!("There was an error opening the task file.");
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
                } else {
                    println!("There was error opening the tracker file");
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
        let last_line = content.lines().rev().next();

        let last_date = if let Some(line) = last_line {
            let date_str = line.split(',').next().unwrap_or("");
            NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap_or(Local::now().date_naive())
        } else {
            Local::now().date_naive()
        };
        let mut new_date = last_date + Duration::days(1);

        let tomorrow = Local::now().date_naive() + Duration::days(1);

        if new_date == tomorrow {
            println!("Tasks for the day have been tracked");
            return;
        }

        let mut file = OpenOptions::new()
            .append(true)
            .open(TRACKER_FILE)
            .unwrap();

        while new_date != tomorrow {
            println!("For date: {}", new_date);
            let completions = self.ask_completion();
            write!(file, "{}{}\n", new_date, completions).expect("Failed to write to file");
            new_date += Duration::days(1);
        }
    }

    pub fn streak() {
        let tasks = Self::read_tasks();
        let tracker_content = Self::read_tracker();
        let mut task_streaks: Vec<String> = vec![String::new(); tasks.len()];

        for line in tracker_content.lines().skip(1) {
            let elements: Vec<&str> = line.split(',').collect();
            for (a_streak, ele) in task_streaks.iter_mut().zip(elements.iter().skip(1)) {
                match *ele {
                    "1" => a_streak.push('_'),
                    _ => a_streak.push(' '),
                }
            }
        }

        for (a_task, a_streak) in tasks.iter().zip(task_streaks) {
            let formatted = format!("{:<30}{}", a_task.name, a_streak);
            println!("{}", formatted);
        }
    }
}
