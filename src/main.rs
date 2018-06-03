#[macro_use]
mod DataStructures;
use DataStructures::List::List::*;
use DataStructures::Tree::Tree::*;

fn main() {

	let this : Box<Node<u8>> = Box::new(Node{elem : 3, left : None, right : None});



	println!("{:?}", *this);
}
