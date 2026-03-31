use std::collections::{HashMap, VecDeque};
use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt;

#[derive(Debug)]
enum GraphError {
    OutOfNodeStore,
}

impl fmt::Display for GraphError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GraphError::OutOfNodeStore => {
                write!(f, "Node was not found in node store. 
                    Ensure that nodes exist before referencing by id.")
            }
        }
    }
}

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
    source_node: NodeID,
    dest_node: NodeID,
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
// so we need to backtrace from a given node. that means our data structure should 
// prioritize O(1) lookup of all edges from a given node's id. maybe we can also 
// get O(1) lookup of nodes from there as well.

#[derive(Debug, Clone, Copy, PartialEq)]
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
    fn add_edge(
        &mut self, 
        u_id: NodeID, 
        v_id: NodeID, 
        edge_type: EdgeType
    ) -> Result<EdgeID, GraphError> {

        if !self.node_store.contains_key(&u_id) || !self.node_store.contains_key(&v_id) {
            return Err(GraphError::OutOfNodeStore);
        }

        let id = self.edge_counter.next();
        self.edge_store.insert(
            id,
            Edge { 
                source_node: u_id,
                dest_node: v_id,
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

    fn find_neighbours(
        &self, 
        node: NodeID, 
        direction: Direction
    ) -> Result<impl Iterator<Item = NodeID>, GraphError> {

        if !self.node_store.contains_key(&node) {
            return Err(GraphError::OutOfNodeStore);
        }

        let nodes = self.adjacency[&node].iter()
            .filter(move |&edge| edge.1 == direction)
            .map(move |(edge, _)|{
                debug_assert!(self.edge_store.contains_key(edge));
                let Edge{ source_node, dest_node, .. } = self.edge_store[edge];
                match direction {
                    Direction::In => source_node,
                    Direction::Out => dest_node
                }});

        Ok(nodes)
    }

    fn pathed_bfs(
        &self, 
        source: NodeID, 
        target: NodeID, 
        direction: Direction
    ) -> Result<Option<Vec<NodeID>>, GraphError> {

        if !self.node_store.contains_key(&source) || !self.node_store.contains_key(&target) {
            return Err(GraphError::OutOfNodeStore);
        }

        let mut q = VecDeque::<NodeID>::new();
        q.push_back(source);
        let mut visited_parents = HashMap::<NodeID, NodeID>::new();

        while let Some(curr) = q.pop_front() {
            if curr == target {
                return Ok(Some(reconstruct_path(visited_parents, source, target)));             
            }
            // handle the error properly later
            self.find_neighbours(curr, direction)?.for_each(|neighbour| {
                if !visited_parents.contains_key(&neighbour) {
                    visited_parents.insert(neighbour, curr);
                    q.push_front(neighbour);
                }
            });
        }

        return Ok(None)
    }
}

fn reconstruct_path(parents_map: HashMap<NodeID, NodeID>, source: NodeID, target: NodeID) -> Vec<NodeID> {
    let mut path: Vec<NodeID> = vec![target];
    // path is initialized and only grows. unwrap cannot panic.
    while source != *path.last().unwrap() {
       path.push(parents_map[path.last().unwrap()]);
    }
    path.reverse();

    path
}

fn main() {
    println!("Hello, world!");
}
