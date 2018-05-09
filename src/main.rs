use std::mem;

mod list;
use list::list::List;

fn main() {

	let mut _ll : List<u8> = List::new();

	_ll.insert(8);
	_ll.insert(10);

	let this = match _ll.pop(){
		Some(n) => n,
		None => 0
	};
	println!("{:?}",this );


}



// impl <'a> LinkedList<'a>{
// 	pub fn new() -> LinkedList<'a>{
// 		LinkedList{
// 		head : None
// 		}
// 	}
// 	pub fn insert(mut self, n : u8){
// 		let mut nu = Node::new(n);
// 		nu.next =self.head;
// 		self.head = Some(&nu);
// 	}
// }
