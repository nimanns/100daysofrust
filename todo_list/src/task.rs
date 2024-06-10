#[derive(Debug)]
pub struct Task {
    pub id: usize,
    pub description: String,
}

impl Task {
    pub fn new(id: usize, description: String) -> Task {
        Task { id, description }
    }
}

pub trait Summary {
  fn summarize(&self) -> String;
}

impl Summary for Task {
  fn summarize(&self) -> String {
    format!("Task {}: {}", self.id, self.description)
  }
}