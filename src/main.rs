#[macro_use]
mod DataStructures;
use DataStructures::List::List::*;
use DataStructures::Tree::Tree::*;

fn main() {

	let this : Box<Node<u8>> = Box::new(Node{elem : 3, left : None, right : None});

	let mut _t = Tree::new();
	_t.insert(8);

	println!("{:?}", _t);

	println!("{:?}", *this);
}
