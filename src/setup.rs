use crate::habit::Habit;
use std::io;
use std::io::Write;

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
        
    }

    fn ask_current_habits(&mut self) {
        println!("Enter details of your current habits");

        loop {
            let habit = Self::ask_habit_name();
            if habit.trim().is_empty() { break };
            let nature = Self::ask_habit_nature();
            println!("A 'voting identity' for a habit is one for which you vote every time you carry out the habit");
            let voting_identity = Self::ask_voting_identity();

            let current_habit = Habit::new(habit, nature, voting_identity);

            self.current_habits.push(current_habit)
        }
    }

    fn ask_habit_name() -> String {
        print!("Enter the habit: ");

        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");

        input
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
        loop {
            print!("What is the voting identity for this habit: ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read");

            if !input.trim().is_empty() {
                return input;
            }
            println!("Please enter a non-empty string");
        }
    }
}