use std::hash::Hash;

#[derive(Debug, PartialEq)]
pub struct Node<T: Eq + Hash + Clone> {
    pub id: T,
}

impl<T: Eq + Hash + Clone> Node<T> {
    pub fn new(id: T) -> Node<T> {
        Node { id: id }
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
