use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;

use crate::edge::Edge;
use crate::errors::GraphError;
use crate::node::Node;

#[derive(Debug, PartialEq)]
pub struct Graph<T: Eq + Hash + Clone + Display> {
    nodes: HashMap<T, Node<T>>,
    edges: HashMap<(T, T), Vec<Edge<T>>>,
}

impl<T: Eq + Hash + Clone + Display> Graph<T> {
    pub fn new() -> Graph<T> {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node<T>) {
        self.nodes.insert(node.id.clone(), node);
    }

    pub fn add_edge(&mut self, edge: Edge<T>) {
        // Add corresponding nodes if they don't exist
        self.nodes
            .entry(edge.u.clone())
            .or_insert(Node::new(edge.u.clone()));

        self.nodes
            .entry(edge.v.clone())
            .or_insert(Node::new(edge.v.clone()));

        // add to successors Map
        self.nodes.entry(edge.u.clone()).and_modify(|e| {
            e.successors.insert(edge.v.clone());
        });
        // add to predecessors Map
        self.nodes.entry(edge.v.clone()).and_modify(|e| {
            e.predecessors.insert(edge.u.clone());
        });

        // add edge
        let edges = self
            .edges
            .entry((edge.u.clone(), edge.v.clone()))
            .or_insert(Vec::<Edge<T>>::new());
        edges.push(edge);
    }

    pub fn get_successors(&self, node_id: &T) -> Result<HashSet<T>, GraphError> {
        match self.nodes.get(&node_id) {
            Some(n) => Ok(n.successors.clone()),
            None => Err(GraphError::new("Node not found")),
        }
    }

    pub fn get_predecessors(&self, node_id: &T) -> Result<HashSet<T>, GraphError> {
        match self.nodes.get(&node_id) {
            Some(n) => Ok(n.predecessors.clone()),
            None => Err(GraphError::new("Node not_found")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_empty_graph() {
        let graph = Graph::<i32>::new();
        assert_eq!(graph.nodes.is_empty(), true)
    }

    #[test]
    fn add_node() {
        let mut graph = Graph::<&str>::new();
        let node = Node::new("a");
        graph.add_node(node);
        assert_eq!(graph.nodes["a"].id, "a");
    }

    #[test]
    fn add_edges() {
        let mut graph = Graph::<char>::new();
        let edge = Edge::new('a', 'b');
        graph.add_edge(edge);
        assert_eq!(graph.edges[&('a', 'b')][0].u, 'a');
        assert_eq!(graph.nodes[&'a'].id, 'a');
        assert_eq!(graph.nodes[&'b'].id, 'b');
    }

    // #[test]
    // fn remove_edge() {
    //     let mut graph = Graph::<char>::new();
    //     let edge = Edge::new('a', 'b');
    //     graph.add_edge(edge);
    //     let edge = Edge::new('b', 'c');
    //     graph.add_edge(edge);
    //     graph.edges
    //     graph.remove_edge(id);
    // }

    #[test]
    fn get_empty_successors() {
        let mut graph = Graph::<char>::new();
        graph.add_node(Node::new('a'));
        assert_eq!(graph.get_successors(&'a').unwrap().len(), 0);
    }

    #[test]
    fn get_single_successor() {
        let mut graph = Graph::<char>::new();
        graph.add_edge(Edge::new('a', 'b'));
        let successors = graph.get_successors(&'a').unwrap();
        assert_eq!(successors.len(), 1);
        assert_eq!(successors.contains(&'b'), true);
    }

    #[test]
    fn get_predecessors() {
        let mut graph = Graph::<char>::new();
        graph.add_edge(Edge::new('a', 'b'));
        graph.add_edge(Edge::new('c', 'b'));
        let predecessors = graph.get_predecessors(&'b').unwrap();
        assert_eq!(predecessors.len(), 2);
        assert_eq!(predecessors.contains(&'a'), true);
        assert_eq!(predecessors.contains(&'c'), true);
    }
}
