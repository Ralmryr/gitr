mod node;
mod repo;
mod branch;

pub use node::Node;
pub use repo::Repo;
pub use branch::Branch;

fn main() {
    let mut repo = Repo::new("My new repo");
    repo.commit("My first commit");
    repo.commit("My second commit");
    repo.revert();

    repo.branch("Next branch");
    repo.commit("Other branch");
    repo.revert();

    repo.print();
}
