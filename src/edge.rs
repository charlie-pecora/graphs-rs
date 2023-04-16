use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub id: Uuid,
    pub u: String,
    pub v: String,
}

impl Edge {
    pub fn new(u: String, v: String) -> Edge {
        Edge {
            id: Uuid::new_v4(),
            u,
            v,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_edge() {
        let new_edge = Edge::new(String::from("a"), String::from("b"));
        assert_eq!(new_edge.u, "a");
        assert_eq!(new_edge.v, "b");
    }
}
