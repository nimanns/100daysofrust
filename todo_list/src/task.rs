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
