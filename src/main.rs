mod node;
mod repo;
mod branch;
mod branch_iterator;

pub use node::Node;
pub use repo::Repo;
pub use branch::Branch;
pub use branch_iterator::BranchContainer;

fn main() {
    let mut repo = Repo::new("My new repo");
    repo.commit("My first commit");
    repo.commit("My second commit");
    repo.revert();

    repo.new_branch("Next branch");
    repo.commit("Other branch");
    repo.revert();

    repo.new_branch("Next Next branch");
    repo.commit("Other other branch");
    repo.revert();

    repo.checkout("master");
    repo.commit("Going back to master branch");

    repo.checkout("Next branch");
    repo.new_branch("New branch from next branch");
    repo.commit("Commit from new branch");
    repo.revert();

    repo.test();

    repo.print();

    //dbg!(repo);
}
