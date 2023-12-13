use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub name: String,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    pub tasks: Vec<Task>,
}

impl State {
    pub fn new() -> Self {
        State { tasks: vec![] }
    }

    pub fn add(&mut self, name: String) {
        self.tasks.push(Task {
            name,
            completed: false,
        });
    }

    pub fn remove(&mut self, index: usize) {
        self.tasks.remove(index);
    }

    pub fn toggle(&mut self, index: usize) {
        self.tasks[index].completed = !self.tasks[index].completed;
    }
}