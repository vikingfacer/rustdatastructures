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
		self.head.map(|node| {&node.elem})
	}

}
impl<T> Option<T> {
    pub fn as_ref(&self) -> Option<&T>;
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

}