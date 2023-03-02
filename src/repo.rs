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
            branch_list: vec![Branch::new_master()],
            current_branch_idx: 0,
            node_counter: 0,
        }
    }

    pub fn print(&self) {
        println!("{}", self.name);
        println!("---------------------");
        for branch in &self.branch_list {
            println!("Branch {}", branch.name);
            self.print_branch_history_rec(branch);
            branch.print();
            println!();
        }
    }

    fn print_branch_history_rec(&self, branch: &Branch) {
        match branch.parent_branch_name() {
            Some(parent_name) => {
                let parent_branch = self.get_branch_by_name(parent_name).unwrap().0;
                self.print_branch_history_rec(parent_branch);
                parent_branch.print_slice(branch.parent_node_id.unwrap());
            }
            None => {
                return;
            }
        }
    }

    pub fn commit(&mut self, msg: &str) {
        let new_node_id = self.node_counter;
        self.current_branch_mut().commit(msg, new_node_id);
        self.node_counter += 1;
    }

    pub fn revert(&mut self) {
        self.current_branch_mut().revert();
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
        let current_branch = self.current_branch_mut();
        let new_branch = Branch::new_from_branch(branch_name, current_branch);
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

    fn current_branch_mut(&mut self) -> &mut Branch {
        &mut self.branch_list[self.current_branch_idx]
    }

    fn current_branch(&self) -> &Branch {
        &self.branch_list[self.current_branch_idx]
    }

    pub fn test(&mut self) {
    }
}