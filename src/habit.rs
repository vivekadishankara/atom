pub struct Habit {
    name: String,
    nature: char,
    voting_identity: String,
}

impl Habit {
    pub fn new(name: String, nature: char, voting_identity: String) -> Self {
        Habit {
            name,
            nature,
            voting_identity,
        }
    }
}