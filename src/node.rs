#[derive(Default, Debug, Clone)]
pub struct Node {
    message: String,
    id: usize,
}

impl Node {
    pub fn new(message: &str, id: usize) -> Self {
        Node {
            message: message.to_string(),
            id: id,
        }
    }

    pub fn print(&self) {
        println!("Node {} : {}", self.id, self.message);
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn message(&self) -> &String {
        &self.message
    }
}