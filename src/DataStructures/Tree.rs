

pub mod Tree{

    use std::ops::Deref;

pub struct Tree<T> {
    root : Link<T>
}

#[derive(Debug)]
pub struct Node<T>{
    pub elem: T,
    pub left: Link<T>,
    pub right: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;

impl<T> Tree<T> {
    pub fn new() -> Self{
        Tree{ root : None}
    }

    pub fn insert(&self, item : T) -> bool{
        if self.root == None {

        }
        true
    }

}

impl<T> Deref for Link<T> {
    fn deref(&self) -> Option<&Node<T>>{
        self.as_ref().map(|Node| &**Node)
    }
}

impl<T> PartialEq for Link<T>{
    fn eq(&self, other : &Link<T>) -> bool{
        Some(Some(*self)) == Some(Some(*other))
    }
}

}
