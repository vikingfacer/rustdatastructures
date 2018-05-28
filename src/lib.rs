

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

    #[test]
    fn front() {
        let mut list = List::new();
        assert_eq!(list.front(), None);
        assert_eq!(list.front_mut(), None);
        list.insert(1); list.insert(2); list.insert(3);

        assert_eq!(list.front(), Some(&3));
        assert_eq!(list.front_mut(), Some(&mut 3));
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.insert(1); list.insert(2); list.insert(3);

        let it = list.into_iter();
        assert_eq!(it.next(), Some(3));
        assert_eq!(it.next(), Some(2));
        assert_eq!(it.next(), Some(1));
    }

}
