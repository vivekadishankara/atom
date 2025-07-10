use std::fs;

pub const CURRENT_HABITS_FILE: &str = "current_habits.csv";
pub const DESIRED_HABITS_FILE: &str = "desired_habits.csv";


pub fn remove_habit_files() {
    for file in [CURRENT_HABITS_FILE, DESIRED_HABITS_FILE] {
        let _ = fs::remove_file(file);
    }
}