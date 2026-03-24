use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use error_macro::graph_error;

#[graph_error]
#[derive(Debug, Clone, Copy)]
pub struct GraphError {
    code: u32,
    name: &'static str,
    message: &'static str,
}

impl GraphError {
    pub const fn new(code: u32, name: &'static str, message: &'static str) -> Self {
        Self { code, name, message }
    }
}

pub static OUT_OF_NODE_STORE: GraphError = GraphError::new(
    401, 
    "Out Of Node Store Error", 
    "Node was not found in node_store. Ensure that nodes exist before adding edges."
);


// playing around with counters to wrap int type for better api semantics
macro_rules! id_type {
    ($name:ident) => {
       #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        struct $name(u32);
        impl $name {
            fn next(&mut self) -> $name {
                self.0 += 1;
                $name(self.0)
            }
        }
    };
}

id_type!(NodeID);
id_type!(EdgeID);

enum NodeType {
    Message { content: String, timestamp: u64 },
}

struct Node {
    node_type: NodeType,
    node_id: NodeID,
}

enum EdgeType {
    Reply {/* some payload here */},
    Semantic {/* some payload here */},
}

struct Edge {
    to_node_idx: NodeID,
    edge_type: EdgeType,
    edge_id: EdgeID,
    timestamp: u64,
}

impl Edge {
    fn get_age_s(&self) -> u64 {
        let t = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("You time travelled");

        t.as_secs() - self.timestamp
    }
    // also have an way to check ages between the two nodes that point to the edge?
    // no, probably a property of the edge, since it's known at edge creation
}

// default adjacency list structure to think about, borrow checker won't like this
// struct Graph<Node> (Vec<Vec<(Node, Edge)>>);
// so we need to backtrace from a given node. that means our data structure should prioritize
// O(1) lookup of all edges from a given node's id. maybe we can also get O(1) lookup of nodes from there
// as well.

enum Direction {
    In,
    Out,
}

struct Graph {
    adjacency: HashMap<NodeID, Vec<(EdgeID, Direction)>>,
    edge_store: HashMap<EdgeID, Edge>,
    edge_counter: EdgeID,
    node_store: HashMap<NodeID, Node>,
    node_counter: NodeID,
}

impl Graph {
    fn add_edge(&mut self, u_id: NodeID, v_id: NodeID, edge_type: EdgeType) -> Result<EdgeID, GraphError> {
        if !self.node_store.contains_key(&u_id) || !self.node_store.contains_key(&v_id) {
            return Err(OUT_OF_NODE_STORE);
        }

        let id = self.edge_counter.next();
        self.edge_store.insert(
            id,
            Edge { 
                to_node_idx: v_id,
                edge_type,
                edge_id: id,
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("You time travelled")
                    .as_secs()
                }
            );

        self.adjacency.entry(u_id)
            .and_modify(|edges: &mut Vec<(EdgeID, Direction)>| 
                edges.push((id, Direction::Out))
            ).or_insert(vec![(id, Direction::Out)]);
        self.adjacency.entry(v_id)
            .and_modify(|edges: &mut Vec<(EdgeID, Direction)>| 
                edges.push((id, Direction::In))
            ).or_insert(vec![(id, Direction::In)]);

        Ok(id)
    }

    fn add_node(&mut self, node_type: NodeType) -> Result<NodeID, GraphError> {
        let id = self.node_counter.next();

        self.node_store.insert(
            id,
            Node {
                node_type,
                node_id: id,
            }
        );

        Ok(id)
    }
}

fn main() {
    println!("Hello, world!");
}
