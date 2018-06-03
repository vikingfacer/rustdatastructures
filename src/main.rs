#[macro_use]
mod DataStructures;
use DataStructures::List::List::*;
use DataStructures::Tree::Tree::*;

fn main() {

	let li : List<u8> =  List![1, 2, 3];

	let tr : Tree<u8> = Tree::new();


	println!("{:?}", li);
}
