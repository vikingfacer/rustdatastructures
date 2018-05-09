
mod list;
use list::list::List;

fn main() {

	let mut _ll : List<u8> = List::new();

	_ll.insert(8);
	println!("{:?}", _ll.front());

	_ll.insert(10);

	let this = match _ll.pop(){
		Some(n) => n,
		None => 0
	};
	println!("{:?}",this );


}

