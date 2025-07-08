use crate::setup::Setup;

pub struct Manager {
    pub setup: Setup,
}

impl Manager {
    pub fn new() -> Self {
        let mut setup = Setup::new();

        Self {
            setup,
        }
    }

    pub fn enact(&mut self, args: &[String]) {
        if args.len() < 2 {
            Self::help();
            return
        }
        match &args[1][..] {
            "setup" => self.setup.start(),
            "help" => Self::help(),
            _ => Self::help(),
        }
    }

    fn help() {
        let help_message = "This is the help message";
        println!("{}", help_message);
    }

}