#[derive(Default, Debug)]
pub struct Node {
    message: String,
    id: i32,
}

impl Node {
    pub fn new(message: &str, id: i32) -> Self {
        Node {
            message: message.to_string(),
            id: id,
        }
    }

    pub fn print(&self) {
        println!("Node {} : {}", self.id, self.message);
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn message(&self) -> &String {
        &self.message
    }
}