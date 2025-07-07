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
        format!("\"{}\",{},{},", self.name, self.nature, self.voting_identity)
    }
}