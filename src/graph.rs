use std::collections::{HashMap, HashSet};

use crate::edge::Edge;
use crate::errors::GraphError;
use crate::node::Node;

#[derive(Debug, PartialEq)]
pub struct Graph {
    nodes: HashMap<String, Node>,
    edges: HashMap<(String, String), Vec<Edge>>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id.clone(), node);
    }

    // Adds an edge, with all corresponding nodes
    // Adds successors and predecessors as needed
    pub fn add_edge(&mut self, edge: Edge) {
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
            .or_insert(Vec::<Edge<String>>::new());
        edges.push(edge);
    }

    pub fn get_successors(&self, node_id: &String) -> Result<HashSet<String>, GraphError> {
        match self.nodes.get(&node_id) {
            Some(n) => Ok(n.successors.clone()),
            None => Err(GraphError::new("Node not found")),
        }
    }

    pub fn get_predecessors(&self, node_id: &String) -> Result<HashSet<String>, GraphError> {
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
        let graph = Graph::new();
        assert_eq!(graph.nodes.is_empty(), true)
    }

    #[test]
    fn add_node() {
        let mut graph = Graph::<&str>::new();
        let node = Node::new(String::from("a"));
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
        let successors = graph.get_successors(&'a'.to_string()).unwrap();
        assert_eq!(successors.len(), 1);
        assert_eq!(successors.contains(&'b'.to_string()), true);
    }

    #[test]
    fn get_predecessors() {
        let mut graph = Graph::new();
        graph.add_edge(Edge::new('a'.to_string(), 'b'.to_string()));
        graph.add_edge(Edge::new('c'.to_string(), 'b'.to_string()));
        let predecessors = graph.get_predecessors(&'b'.to_string()).unwrap();
        assert_eq!(predecessors.len(), 2);
        assert_eq!(predecessors.contains(&'a'.to_string()), true);
        assert_eq!(predecessors.contains(&'c'.to_string()), true);
    }
}
