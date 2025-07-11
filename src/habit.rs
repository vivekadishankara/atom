pub struct Habit {
    pub name: String,
    pub nature: char,
    pub voting_identity: String,
}

impl Habit {
    pub fn new(name: String, nature: char, voting_identity: String) -> Self {
        Habit {
            name,
            nature,
            voting_identity,
        }
    }

    pub fn to_file_line(&self) -> String {
        format!("{},{},{}", self.name, self.nature, self.voting_identity)
    }

    pub fn from_file_line(line: &str) -> Self{
        let habit_components: Vec<&str> = line.split(',').collect();
        let name = habit_components[0].to_string();
        let nature = habit_components[1].chars().next().unwrap();
        let voting_identity = habit_components[2].to_string();

        Self::new(name, nature, voting_identity)
    }
}
