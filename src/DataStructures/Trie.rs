// the trie data type is an adt
// each node will have a vector of nodes
// each node has a key the key is a letter
// each node can have one value but nodes are not garanteed a value it can be none
// the Trie is accessed by a string
//  each node holds a character
//  at the end of the string the last character holds the value
//  so Trie {"word" : 9}
//  the Trie 'w'(node) -> 'o'(node) -> 'r'(node) -> 'd'(node) => 9
//  also if {'wo' : 1} Then
//  'w'(node) -> 'o'(node) => 1

pub mod Trie {

use std::vec;
#[derive(Debug)]
pub enum Trie<T: Ord>{
    Empty,
    Node {links : Vec<Node>, key: char, value : T}
}

impl<T: Ord> Trie<T>{
        pub fn new() -> Self{
            Trie::Empty
        }

        pub fn insert(k : &str, v : T) -> bool{
            false
        }
}
}
