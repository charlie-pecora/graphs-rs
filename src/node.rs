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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_node() {
        let new_node = Node::new("abc");
        assert_eq!(new_node.id, "abc");
    }
}
