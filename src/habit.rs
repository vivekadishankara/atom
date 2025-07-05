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
}