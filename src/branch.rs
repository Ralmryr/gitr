use crate::Node;

pub struct Branch {
    pub name: String,
    node_list: Vec<Node>,
    node_counter: i32,
}

impl Branch {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            node_list: Vec::new(),
            node_counter: 0,
        }
    }

    pub fn default() -> Self {
        Self {
            name: "master".to_string(),
            node_list: Vec::new(),
            node_counter: 0,
        }
    }

    pub fn print(&self) {
        println!("Branch {}", self.name);
        for node in &self.node_list {
            node.print();
        }
        println!();
    }

    pub fn add_node(&mut self, node: Node) {
        self.node_list.push(node);
    }

    pub fn commit(&mut self, message: &str) {
        self.node_list.push(Node::new(message, self.node_counter));
        self.node_counter += 1;
    }

    pub fn revert(&mut self) {
        if let Some(last_commit) = self.node_list.last() {
            let commit_msg = last_commit.message();
            let revert_msg = format!("REVERT {}", commit_msg);
            self.commit(revert_msg.as_str());
        }
    }
}