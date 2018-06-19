
pub mod Tree{
    use std::cmp::Ordering;
    use std::mem;

#[derive(Debug)]
pub enum Tree<T :Ord> {
    Node{
        elem: T,
        left:  Box<Tree<T>>,
        right:  Box<Tree<T>>},
    Empty
}

// enum ArgumentsForOverload<T :Ord> {
//     NoArgs,
//     OneArg,
// }

impl<T :Ord> Tree<T>{
    pub fn new() -> Tree<T>{
        Tree::Empty
    }

    fn newFromItem(item : T) -> Self{
        Tree::Node{ elem : item, left : Box::new(Tree::Empty), right : Box::new(Tree::Empty)}
    }

    // this should be a result
    pub fn insert(&mut self, item : T) -> bool{
        match self {
            &mut Tree::Node{ref elem,ref mut left,ref mut right} =>{
                match item.cmp(elem) {
                    Ordering::Less => {left.insert(item);
                                            true},
                    Ordering::Equal=> false,
                    Ordering::Greater=>{right.insert(item);
                                            true},
                }
            },
            &mut Tree::Empty=> {mem::replace(self, Tree::newFromItem(item));
                                true}
        }
    }
    // travers would be nice

}
}
