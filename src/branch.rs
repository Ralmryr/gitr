use crate::Node;

#[derive(Debug)]
pub struct Branch {
    pub name: String,
    node_list: Vec<Node>,
    node_counter: usize,
    parent_branch_name: String,
    parent_node_id: usize,
}

impl Branch {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            node_list: Vec::new(),
            node_counter: 0,
            parent_branch_name: String::new(),
            parent_node_id: 0,
        }
    }

    pub fn default() -> Self {
        Self {
            name: "master".to_string(),
            node_list: Vec::new(),
            node_counter: 0,
            parent_branch_name: String::new(),
            parent_node_id: 0,
        }
    }

    pub fn copy_history_from(&mut self, copy_branch: &Branch) {
        self.node_list = copy_branch.node_list.clone();
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

    pub fn commit(&mut self, message: &str, node_id: usize) {
        self.node_list.push(Node::new(message, node_id));
    }

    // TODO Add option to revert a specific commit
    pub fn revert(&mut self) {
        if let Some(last_commit) = self.node_list.last() {
            let commit_msg = last_commit.message();
            let revert_msg = format!("REVERT {}", commit_msg);
            self.commit(revert_msg.as_str(), last_commit.id()+1);
        }
    }
}