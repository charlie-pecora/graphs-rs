use std::hash::Hash;

#[derive(Debug, PartialEq, Clone)]
pub struct Edge<T: Eq + Hash + Clone> {
    pub u: T,
    pub v: T,
}

impl<T: Eq + Hash + Clone> Edge<T> {
    pub fn new(u: T, v: T) -> Edge<T> {
        Edge { u: u, v: v }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_edge() {
        let new_edge = Edge::new("a", "b");
        assert_eq!(new_edge.u, "a");
        assert_eq!(new_edge.v, "b");
    }
}
