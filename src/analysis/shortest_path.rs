use std::collections::{HashSet, VecDeque};
use std::fmt::Display;
use std::hash::Hash;

use crate::errors;
use crate::graph::Graph;

pub fn shortest_path_bfs<T: Eq + Hash + Clone + Display>(
    graph: &Graph<T>,
    start: &T,
    stop: &T,
) -> Result<Vec<T>, errors::GraphError> {
    let mut visited_nodes = HashSet::<T>::new();
    let mut queue = VecDeque::new();
    let mut start_path = Vec::new();
    start_path.push(start.clone());
    queue.push_back((start.clone(), start_path));
    loop {
        let (successors, current_path) = match queue.pop_front() {
            Some((current_node, current_path)) => {
                visited_nodes.insert(current_node.clone());
                (graph.get_successors(&current_node)?, current_path)
            }
            None => {
                break Result::Err(errors::GraphError::new(
                    format!(
                        "start '{}' and stop '{}' nodes are not connected",
                        start, stop
                    )
                    .as_str(),
                ))
            }
        };
        let mut matched_path: Option<Vec<T>> = None;
        for successor in successors.iter() {
            let mut successor_path = current_path.clone();
            successor_path.push(successor.clone());
            if successor == stop {
                matched_path = Some(successor_path.clone());
            } else if visited_nodes.contains(&successor) == false {
                queue.push_back((successor.clone(), successor_path))
            }
        }
        match matched_path {
            Some(path) => break Ok(path),
            None => continue,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Edge;
    #[test]
    fn find_shortest_path_does_not_exist() {
        let mut graph = Graph::<char>::new();
        graph.add_edge(Edge::new('a', 'b'));
        let shortest_path = shortest_path_bfs(&graph, &'b', &'a');
        assert_eq!(
            shortest_path,
            Err(errors::GraphError::new(
                "start 'b' and stop 'a' nodes are not connected"
            ))
        );
    }

    #[test]
    fn find_direct_shortest_path() {
        let mut graph = Graph::<char>::new();
        graph.add_edge(Edge::new('a', 'b'));
        let shortest_path = shortest_path_bfs(&graph, &'a', &'b').unwrap();
        assert_eq!(shortest_path.len(), 2);
        assert_eq!(shortest_path, vec!('a', 'b'))
    }

    #[test]
    fn find_shortest_path_with_alternative() {
        let mut graph = Graph::<char>::new();
        graph.add_edge(Edge::new('a', 'b'));
        graph.add_edge(Edge::new('a', 'c'));
        graph.add_edge(Edge::new('b', 'c'));
        let shortest_path = shortest_path_bfs(&graph, &'a', &'c').unwrap();
        assert_eq!(shortest_path.len(), 2);
        assert_eq!(shortest_path, vec!('a', 'c'))
    }

    #[test]
    fn find_multi_hop_shortest_path() {
        let mut graph = Graph::<char>::new();
        graph.add_edge(Edge::new('a', 'b'));
        graph.add_edge(Edge::new('b', 'c'));
        let shortest_path = shortest_path_bfs(&graph, &'a', &'c').unwrap();
        assert_eq!(shortest_path.len(), 3);
        assert_eq!(shortest_path, vec!('a', 'b', 'c'))
    }
}
