use std::time::{SystemTime, UNIX_EPOCH};

enum NodeType {
    Message { content: String, timestamp: u64 },
}

struct Node {
    node_type: NodeType,
    idx: u32,
}

enum EdgeType {
    Reply {/* some payload here */},
    Semantic {/* some payload here */},
}

enum Direction {
    In,
    Out,
}

struct Edge {
    to_node_idx: u32,
    edge_type: EdgeType,
    direction: Direction, 
    idx: u32,
    timestamp: u64,
}

impl Edge {
    fn get_age_s(&self) -> u64 {
        let t = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("You time travelled");

        t.as_secs() - self.timestamp
    // also have an way to check ages between the two nodes that point to the edge?
    // no, probably a property of the edge, since it's known at edge creation
}

// default adjacency list structure to think about, borrow checker won't like this
struct Graph<Node> (Vec<Vec<(Node, Edge)>>);



fn main() {
    println!("Hello, world!");
}
