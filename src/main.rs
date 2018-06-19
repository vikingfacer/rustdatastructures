#[macro_use]
mod DataStructures;
use DataStructures::List::List::*;
use DataStructures::Tree::Tree::*;

fn main() {

	let mut t1 : Tree<u8> = Tree::new();

	t1.insert(9);
	t1.insert(1);
	t1.insert(0);
	t1.insert(10);



	print!("{:?}",t1 );

}
