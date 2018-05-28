#[macro_use]
mod list;
use list::list::List;

mod Tree;
use Tree::tree::Tree;

fn main() {

	let li : List<u8> = List![1, 2, 3];

	let tr : Tree<u8> = Tree::new();


	println!("{:?}", li);
}
