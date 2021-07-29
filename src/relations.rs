use std::sync::{Arc, RwLock};

// Edge
pub struct Edge {
    node_value_1: String,
    node_value_2: String
}

pub struct EdgeMap {
    edge_collection: Arc<RwLock<Vec<Edge>>> 
}

impl EdgeMap {
    pub fn new() -> Self {
        EdgeMap {edge_collection: Arc::new(RwLock::new(Vec::new()))}
    }
}


// Vertex
pub struct Vertex {
    in_node_value: String,
    out_node_value: String
}

pub struct VertexMap {
    vertex_collection: Arc<RwLock<Vec<Vertex>>> 
}

impl VertexMap {
    pub fn new() -> Self {
        VertexMap {vertex_collection: Arc::new(RwLock::new(Vec::new()))}
    }
}