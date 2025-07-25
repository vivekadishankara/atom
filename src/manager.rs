use crate::setup::Setup;
use crate::file_names::remove_habit_files;
use crate::todo::Todo;

pub struct Manager;

impl Manager {
    pub fn enact(args: &[String]) {
        if args.len() < 2 {
            let todo = Todo::new();
            todo.list();
            return
        }
        match &args[1][..] {
            "setup" => {
                let mut setup = Setup::from_files();
                if setup.habits_required() {
                    println!("Creating habit files");
                } else {
                    println!("Non-Empty habit files found, reading habits. If you want start anew, run the reset command");
                }
                setup.start();
            }
            "reset" => remove_habit_files(),
            "list" => {
                let todo = Todo::new();
                todo.list();
            },
            "track" => {
                let todo = Todo::new();
                todo.track();
            },
            "streak" => {
                Todo::streak();
            }
            "help" => Self::help(),
            _ => Self::help(),
        };
    }

    fn help() {
        let help_message = "This is the help message";
        println!("{}", help_message);
    }

}
