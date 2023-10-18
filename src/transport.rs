use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct TransportNetwork {
    graph: TransportNetworkGraph,
}

#[derive(Deserialize)]
struct TransportNetworkGraph {
    nodes: HashMap<String, TransportNetworkNode>,
    edges: Vec<TransportNetworkEdge>,
}

#[derive(Deserialize)]
struct TransportNetworkNode {
    label: String,
}

#[derive(Deserialize)]
struct TransportNetworkEdge {
    source: String,
    target: String,
    relation: String,
    directed: bool,
    metadata: TransportNetworkEdgeMetadata,
}

#[derive(Deserialize)]
struct TransportNetworkEdgeMetadata {
    distance: u32,
}
