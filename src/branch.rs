use crate::Node;

#[derive(Debug)]
pub struct Branch {
    pub name: String,
    node_list: Vec<Node>,
    parent_branch_name: Option<String>,
    pub parent_node_id: Option<usize>,
}

impl Branch {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            node_list: Vec::new(),
            parent_branch_name: Some(String::new()),
            parent_node_id: Some(0),
        }
    }

    pub fn new_master() -> Self {
        Self {
            name: "master".to_string(),
            node_list: Vec::new(),
            parent_branch_name: None,
            parent_node_id: None
        }
    }

    pub fn new_from_branch(name: &str, parent_branch: &Branch) -> Self {
        let parent_node_id = parent_branch.last_node_id();

        Self {
            name: name.to_string(),
            node_list: Vec::new(),
            parent_branch_name: Some(parent_branch.name.clone()),
            parent_node_id: Some(parent_node_id),
        }
    }

    pub fn copy_history_from(&mut self, copy_branch: &Branch) {
        self.node_list = copy_branch.node_list.clone();
    }

    pub fn print(&self) {
        for node in &self.node_list {
            node.print();
        }
    }

    pub fn print_slice(&self, node_id: usize) {
        // TODO remove unwrap
        let node_idx = self.node_list.iter().position(|node| node.id() == node_id).unwrap();
        for node in &self.node_list[..node_idx+1] {
            node.print();
        }
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

    pub fn slice(&self, node_id: usize) -> &[Node] {
        // TODO remove unwrap()
        let node_index = self.node_list.iter().position(|node| node.id() == node_id).unwrap();
        &self.node_list.as_slice()[..node_index]
    }

    pub fn last_node_id(&self) -> usize {
        self.node_list.last().unwrap().id()
    }

    pub fn parent_branch_name(&self) -> Option<&String> {
        match self.parent_branch_name {
            Some(ref name) => Some(name),
            None => None,
        }
    }
}