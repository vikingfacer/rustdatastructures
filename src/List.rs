
#[macro_use]
pub mod list{
	use std::mem;


#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

// #[derive(Debug)]
// enum Link<T> {
//     Empty,
//     More(Box<Node<T>>),
// }

type Link<T> = Option<Box<Node<T>>>;

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T:'a> {
	next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T: 'a> {
    next: Option<&'a mut Node<T>>,
}

#[derive(Debug)]
struct Node<T>{
    elem: T,
    next: Link<T>,
}

impl <T> List<T> {
	pub fn new() -> Self {
		List{
			head : None
		}
	}

	pub fn insert(&mut self, n : T){
		let nu = Box::new(Node{elem : n,
				next : mem::replace(&mut self.head, None)});
		self.head = Some(nu);
	}

	pub fn pop(&mut self) -> Option<T>{
        self.head.take().map(|node| {
            let node = *node;
            self.head = node.next;
            node.elem
        })
	}

	pub fn front(&self) -> Option<&T>{
		self.head.as_ref().map(|node| {&node.elem})
	}

	pub fn front_mut(&mut self) ->Option<&mut T>{
		self.head.as_mut().map(|node| {
			&mut node.elem
		})
	}

	pub fn into_iter(self) -> IntoIter<T> {
		IntoIter(self)
	}

	pub fn iter(&self) -> Iter<T>{
        Iter{ next: self.head.as_ref().map(|node| &**node) }
	}

	pub fn mut_iter(&mut self) -> IterMut<T>{
		IterMut{ next: self.head.as_mut().map(|node| &mut ** node)}
	}
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, None);
        // `while let` == "do this thing until this pattern doesn't match"
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}
#[macro_export]
macro_rules! List {
	($($element:expr),*)=> {
		{
			let mut l  = List::new();
			$(l.insert($element);)*
		l}
	};
}
}
