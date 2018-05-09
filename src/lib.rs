pub mod list;

#[cfg(test)]
mod tests {
	use list::list::List;
	#[test]
	fn pop_insert() {

		let mut l1 : List<u8> = List::new();
		let mut l2 : List<u8> = List::new();

		l2.insert(1);
		assert_ne!( l1.pop(), l2.pop());

		l2.insert(1);
		assert_eq!( 1, l2.pop().unwrap());
		assert_eq!( None, l2.pop());
	}

}
