

pub mod tree{

pub struct Tree<T> {
    root : Node<T>
}

#[derive(Debug)]
struct Node<T>{
    elem: T,
    left: Link<T>,
    right: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;

impl<T> Tree<T> {
    pub fn new() {
        unimplemented!()
    }

}

}
