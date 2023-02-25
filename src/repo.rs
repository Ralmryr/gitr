use crate::Branch;

#[derive(Debug)]
pub struct Repo {
    name: String,
    branch_list: Vec<Branch>,
    current_branch_idx: usize,
    node_counter: usize,
}

impl Repo {
    pub fn new(name: &str) -> Repo {
        Repo {
            name: name.to_string(),
            branch_list: vec![Branch::default()],
            current_branch_idx: 0,
            node_counter: 0,
        }
    }

    pub fn print(&self) {
        println!("{}", self.name);
        println!("---------------------");
        for branch in &self.branch_list {
            branch.print();
        }
    }

    pub fn commit(&mut self, msg: &str) {
        let new_node_id = self.node_counter;
        self.get_current_branch().commit(msg, new_node_id);
        self.node_counter += 1;
    }

    pub fn revert(&mut self) {
        self.get_current_branch().revert();
        self.node_counter += 1;
    }

    pub fn checkout(&mut self, branch_name: &str) {
        match self.get_branch_by_name(branch_name) {
            Some((_, idx)) => {
                self.current_branch_idx = idx;
            }
            None => println!("{} doesn't exist", branch_name)
        };
    }

    pub fn new_branch(&mut self, branch_name: &str) {
        let new_branch = Branch::new(branch_name);

        self.branch_list.push(new_branch);
        self.checkout(branch_name);
    }

    fn get_branch_by_name(&self, branch_name: &str) -> Option<(&Branch, usize)> {
        for (idx, branch) in self.branch_list.iter().enumerate() {
            if branch.name == branch_name {
                return Some((branch, idx));
            }
        }
        None
    }

    fn get_current_branch(&mut self) -> &mut Branch {
        &mut self.branch_list[self.current_branch_idx]
    }
}