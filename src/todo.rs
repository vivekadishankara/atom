
use crate::task::Task;

pub struct Todo {
    tasks: Vec<Task>,
}

impl Todo {
    pub fn new() -> Self {
        let mut tasks = Task::read_tasks();
        Task::read_tracker(&mut tasks);
        Self {
            tasks,
        }
    }

    pub fn list(&self) {
        for (i, a_task) in self.tasks.iter().enumerate() {
            println!("{}. {}", i + 1, a_task);
        }
    }
}
