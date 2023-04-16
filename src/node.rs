use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub predecessors: HashSet<String>,
    pub successors: HashSet<String>,
}

impl Node {
    pub fn new(id: String) -> Node {
        Node {
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
        let new_node = Node::new(String::from("abc"));
        assert_eq!(new_node.id, "abc");
    }

}
