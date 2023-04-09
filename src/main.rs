use graphs::{analysis, Edge, Graph};
// use serde_json::to_string_pretty;

fn main() {
    let mut graph = Graph::<i32>::new();
    for node_id in 0..10000 {
        let edge = Edge::new(node_id, node_id + 2);
        graph.add_edge(edge);
    }
    // println!("{:?}", graph);
    let shortest_path = analysis::shortest_path_bfs(&graph, &0, &1000).unwrap();
    println!("{:?}", shortest_path);
}
