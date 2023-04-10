use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug, PartialEq)]
pub struct Node<T: Eq + Hash + Clone> {
    pub id: T,
    pub predecessors: HashSet<T>,
    pub successors: HashSet<T>,
}

impl<T: Eq + Hash + Clone> Node<T> {
    pub fn new(id: T) -> Node<T> {
        Node {
            id,
            predecessors: HashSet::<T>::new(),
            successors: HashSet::<T>::new(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RocksNode {
    pub id: String,
    pub predecessors: HashSet<String>,
    pub successors: HashSet<String>,
}

impl RocksNode {
    pub fn new(id: String) -> RocksNode {
        RocksNode {
            id,
            predecessors: HashSet::<String>::new(),
            successors: HashSet::<String>::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_node() {
        let new_node = Node::new("abc");
        assert_eq!(new_node.id, "abc");
    }

    #[test]
    fn create_rocks_node() {
        let new_node = RocksNode::new(String::from("abc"));
        assert_eq!(new_node.id, "abc");
    }
}
