use rocksdb;
use serde_json;

use crate::node::RocksNode;
use crate::edge::RocksEdge;
use crate::errors::GraphError;

pub struct RocksDBGraph {
    db: rocksdb::TransactionDB,
}

impl RocksDBGraph {
    pub fn new(path: &str) -> RocksDBGraph {
        let db = rocksdb::TransactionDB::open_default(path).unwrap();
        RocksDBGraph { db }
    }

    pub fn add_node(&mut self, node: RocksNode) {
        self.db
            .put(&node.id, serde_json::to_string(&node).unwrap())
            .unwrap();
    }

    pub fn get_node(&mut self, node_id: &str) -> Option<RocksNode> {
        match self.db.get(&node_id) {
            Ok(Some(s)) => {
                let node = serde_json::from_slice::<RocksNode>(&s);
                match node {
                    Ok(n) => Some(n),
                    Err(_) => None,
                }
            }
            Ok(None) => None,
            Err(_) => None,
        }
    }

    pub fn update_node(&mut self, node_id: &str, update: &dyn Fn(RocksNode) -> RocksNode) -> Result<RocksNode, GraphError> {
        let txn = self.db.transaction();
        match txn.get(&node_id) {
            Ok(Some(s)) => {
                let node = serde_json::from_slice::<RocksNode>(&s);
                match node {
                    Ok(n) => {
                        let new_node = update(n);
                        match txn.put(
                            &node_id,
                            serde_json::to_string(&new_node).unwrap(),
                        ) {
                            Ok(_) => Ok(new_node),
                            Err(_) => Err(GraphError::new("C")),
                        }
                    },
                    Err(_) => Err(GraphError::new("Issue deserializing key")),
                }
            }
            Ok(None) => Err(GraphError::new("No such node")),
            Err(_) => Err(GraphError::new("Error getting node")),
        }
    }

    pub fn add_edge(&mut self, edge: RocksEdge) -> Result<(), GraphError> {
        match self.db.put(
            format!("{}{}_{}", "edge_", edge.u, edge.v),
            serde_json::to_string(&edge).unwrap(),
        ) {
            Ok(_) => {
                self.update_node(
                    &edge.u,
                    &|mut node: RocksNode| -> RocksNode {
                        node.successors.insert(edge.v.clone());
                        node
                    }
                )?;
                self.update_node(
                    &edge.v,
                    &|mut node: RocksNode| -> RocksNode {
                        node.successors.insert(edge.u.clone());
                        node
                    }
                )?;
                Ok(())
            },
            Err(_) => Err(GraphError::new("Unable to add edge")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn create_empty_graph() {
        let path = format!("test.{}.db", Uuid::new_v4());
        let _graph = RocksDBGraph::new(&path);
        let _ = rocksdb::DB::destroy(&rocksdb::Options::default(), path);
    }

    #[test]
    fn add_node() {
        let test_node_name = String::from("test");
        let path = format!("test.{}.db", Uuid::new_v4());
        let mut _graph = RocksDBGraph::new(&path);
        _graph.add_node(RocksNode::new(test_node_name.clone()));
        let node = _graph.get_node(test_node_name.as_str()).unwrap();
        assert_eq!(node.id, test_node_name.as_str());
        let _ = rocksdb::DB::destroy(&rocksdb::Options::default(), path);
    }

    #[test]
    fn update_node() {
        let test_node_name = String::from("test");
        let update_fn = |mut node: RocksNode| -> RocksNode {
            node.successors.insert(String::from("other"));
            node
        };
        let path = format!("test.{}.db", Uuid::new_v4());
        let mut _graph = RocksDBGraph::new(&path);
        _graph.add_node(RocksNode::new(test_node_name.clone()));
        _graph.update_node(test_node_name.as_str(), &update_fn).unwrap();
        let node = _graph.get_node(test_node_name.as_str()).unwrap();
        assert_eq!(node.id, test_node_name.as_str());
        let _ = rocksdb::DB::destroy(&rocksdb::Options::default(), path);
    }
}
