use graphs::{Edge, Graph, Node};
// use serde_json::to_string_pretty;

fn main() {
    let mut graph = Graph::<char>::new();
    for node_id in 'a'..'z' {
        let node = Node::new(node_id);
        graph.add_node(node);
    }
    for (u, v) in vec![
        ('a', 'b'),
        ('b', 'c'),
        ('c', 'd'),
        ('d', 'e'),
        ('e', 'f'),
        ('f', 'g'),
        ('g', 'h'),
        ('h', 'i'),
        ('i', 'j'),
        ('j', 'k'),
        ('k', 'l'),
        ('l', 'm'),
        ('m', 'n'),
        ('n', 'o'),
        ('o', 'p'),
        ('p', 'q'),
        ('q', 'r'),
        ('r', 's'),
        ('s', 't'),
        ('t', 'u'),
        ('u', 'v'),
        ('v', 'w'),
        ('w', 'x'),
        ('x', 'y'),
        ('y', 'z'),
        ('c', 'z'),
    ]
    .iter()
    {
        let edge = Edge::new(u.clone(), v.clone());
        graph.add_edge(edge);
    }
    // println!("{:?}", graph);
    let shortest_path = graph.find_shortest_path(&'a', &'z');
    match shortest_path {
        Ok(path) => println!("{:?}", path),
        Err(error) => println!("Couldn't find shortest path: {}", error.message),
    };
}
