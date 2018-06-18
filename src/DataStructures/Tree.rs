
pub mod Tree{
    use std::cmp::Ordering;
    use std::ops::Deref;

#[derive(Debug)]
pub struct Tree<T> {
    root : Option<Box<Node<T>>>
}

#[derive(Debug)]
pub struct Node<T>{
    pub elem: T,
    pub left:  Option<Box<Node<T>>>,
    pub right:  Option<Box<Node<T>>>
}

impl<T> Node<T>{
    fn new(item : T) -> Self{
        Node{
            elem : item,
            left : None,
            right: None
        }
    }
    fn insert(&mut self, item : T) -> bool{
        match self.elem.partial_cmp(item) {
            Ordering::Less   => {
                match self.left {
                    None => {self.left = Some(Box::new(Node::new(item)));
                                true}
                    Some(ref n) => false
                }
            }
            Ordering::Equal  => false,
            Ordering::Greater=> false,
        }
    }

}

impl<T> Tree<T> {
    pub fn new() -> Self{
        Tree{ root : None}
    }

    pub fn insert(&mut self, item : T) -> bool{
        match self.root {
            None => self.root = Some(Box::new(Node::new(item))),
            Some( ref n) => unimplemented!(), // Some should call the insert function for the node
        }
        true
    }

}
//
// impl<T> Deref for Link<T> {
//     fn deref(&self) -> Option<&Node<T>>{
//         self.as_ref().map(|Node| &**Node)
//     }
// }

impl<T> PartialEq for Node<T> where T : Eq{
    fn eq(&self, other : &Node<T>) -> bool{
        self.elem == other.elem
    }
}

// this could be cool to implemented recursively so that the total left or total right is given
impl<T> PartialOrd for Node<T> where T : Ord{
    fn partial_cmp(&self, other: &Node<T>) -> Option<Ordering>{
        self.elem.partial_cmp(&other.elem)
    }
}

}
