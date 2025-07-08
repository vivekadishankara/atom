use std::io;
use std::io::Write;
use std::collections::HashMap;
use std::fs;

use crate::habit::Habit;
use crate::file_names::habit_setup_file;

pub struct Setup {
    current_habits: Vec<Habit>,
    desired_habits: Vec<Habit>,
}

impl Setup {
    pub fn new() -> Self {
        Self {
            current_habits: Vec::new(),
            desired_habits: Vec::new(),
        }
    }

    pub fn start(&mut self) {
        self.ask_current_habits();
        self.calculate_current_matrix();
        self.ask_desired_habits();
        self.calculate_desired_matrix();
        self.write_habits_to_file();
    }

    fn ask_current_habits(&mut self) {
        Self::ask_habits("current", &mut self.current_habits);
    }

    fn ask_desired_habits(&mut self) {
        Self::ask_habits("desired", &mut self.desired_habits);
    }

    fn ask_habits(habit_stage: &str, habits: &mut Vec<Habit>) {
        println!("Enter details of your {} habits", habit_stage);

        loop {
            let habit = Self::ask_habit_name();
            if habit.trim().is_empty() { break };
            let nature = Self::ask_habit_nature();
            println!("A 'voting identity' for a habit is one for which you vote every time you carry out the habit");
            let voting_identity = Self::ask_voting_identity();

            let current_habit = Habit::new(habit, nature, voting_identity);

            habits.push(current_habit)
        }
    }

    fn ask_habit_name() -> String {
        print!("Enter the habit: ");

        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");

        input.trim().to_string()
    }

    fn ask_habit_nature() -> char {
        loop {
            print!("Is it a good(+), bad(-) or neutral(=) habit: ");

            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read");
            let ch = input.trim().chars().next();

            if let Some(c) = ch {
                if ['+', '-', '='].contains(&c) { return c };
            }
            println!("Please enter a valid character");
        }
    }

    fn ask_voting_identity() -> String {
        // TODO: autocomplete can be added using rustyline
        loop {
            print!("What is the voting identity for this habit: ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read");
            
            let input = input.trim().to_string();
            if !input.is_empty() {
                return input;
            }
            println!("Please enter a non-empty string");
        }
    }

    fn build_habit_identity_matrix(habits: &Vec<Habit>) -> HashMap<String, i32> {
        let mut identity_map = HashMap::new();
        for a_habit in habits.iter() {
            *identity_map.entry(a_habit.voting_identity.clone()).or_insert(0) += 1;
        }
        identity_map
    }

    fn calculate_current_matrix(&self) {
        let habit_identity_matrix = Self::build_habit_identity_matrix(&self.current_habits);
        println!("Here is your habit identity matrix, telling you how many of your habits are voting for your identities");
        println!("{:?}", &habit_identity_matrix);
    }

    fn calculate_desired_matrix(&self) {
        let habit_identity_matrix = Self::build_habit_identity_matrix(&self.desired_habits);
        println!("Here is your habit identity matrix for your desired habits");
        println!("{:?}", &habit_identity_matrix);
    }

    fn write_habits_to_file(&self) {
        let mut content = String::from("current habits,");
        for a_habit in self.current_habits.iter() {
            content.push('\n');
            let habit_line = a_habit.to_file_line();
            content.push_str(&habit_line);
        }
        content.push_str("\ndesired habits,");
        for a_habit in self.desired_habits.iter() {
            content.push('\n');
            let habit_line = a_habit.to_file_line();
            content.push_str(&habit_line);
        }
        content.push('\n');
        fs::write(habit_setup_file, content).expect("Uable to write file");
    }
}