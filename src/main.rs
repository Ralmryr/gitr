mod node;
mod repo;
mod branch;

pub use node::Node;
pub use repo::Repo;
pub use branch::Branch;

use std::ops::Drop;

fn main() {
    let mut repo = Repo::new("My new repo");
    repo.commit("My first commit");
    repo.commit("My second commit");
    repo.revert();

    repo.new_branch("Next branch");
    repo.commit("Other branch");
    repo.revert();

    repo.checkout("master");
    repo.commit("Going back to master branch");

    repo.print();

    dbg!(repo);
}
