//! SARS-CoV-2 3D Knowledge Graph
//! 
//! Comprehensive knowledge graph for tracking viral variants,
//! mutations, and epidemiological data

use petgraph::graph::{DiGraph, NodeIndex};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use anyhow::Result;

pub mod nodes;
pub mod edges;
pub mod query;
pub mod spatial;

pub use nodes::{VirusNode, ProteinNode, MutationNode, VariantNode};
pub use edges::{EdgeType, RelationshipEdge};
pub use query::GraphQuery;

/// Main SARS-CoV-2 knowledge graph structure
pub struct CovidKnowledgeGraph {
    graph: DiGraph<VirusNode, RelationshipEdge>,
    node_index: HashMap<String, NodeIndex>,
    metadata: GraphMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphMetadata {
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub version: String,
    pub data_sources: Vec<String>,
}

impl CovidKnowledgeGraph {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            node_index: HashMap::new(),
            metadata: GraphMetadata {
                created_at: Utc::now(),
                last_updated: Utc::now(),
                version: "0.1.0".to_string(),
                data_sources: vec![
                    "GISAID".to_string(),
                    "NCBI".to_string(),
                    "WHO".to_string(),
                ],
            },
        }
    }

    /// Add a virus node to the graph
    pub fn add_node(&mut self, node: VirusNode) -> NodeIndex {
        let idx = self.graph.add_node(node.clone());
        self.node_index.insert(node.id(), idx);
        idx
    }

    /// Add a relationship edge between nodes
    pub fn add_edge(&mut self, from: NodeIndex, to: NodeIndex, edge: RelationshipEdge) {
        self.graph.add_edge(from, to, edge);
    }

    /// Query the graph
    pub fn query(&self) -> GraphQuery {
        GraphQuery::new(&self.graph)
    }

    /// Load graph from JSON file
    pub fn load(path: &str) -> Result<Self> {
        // Placeholder for actual loading logic
        Ok(Self::new())
    }

    /// Save graph to JSON file
    pub fn save(&self, path: &str) -> Result<()> {
        // Placeholder for actual saving logic
        Ok(())
    }

    /// Get node by ID
    pub fn get_node(&self, id: &str) -> Option<&VirusNode> {
        self.node_index.get(id)
            .and_then(|idx| self.graph.node_weight(*idx))
    }

    /// Get all variants
    pub fn get_variants(&self) -> Vec<&VariantNode> {
        self.graph.node_weights()
            .filter_map(|node| {
                if let VirusNode::Variant(v) = node {
                    Some(v)
                } else {
                    None
                }
            })
            .collect()
    }
}

impl Default for CovidKnowledgeGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_creation() {
        let graph = CovidKnowledgeGraph::new();
        assert_eq!(graph.metadata.version, "0.1.0");
    }

    #[test]
    fn test_add_node() {
        let mut graph = CovidKnowledgeGraph::new();
        let variant = VariantNode {
            id: "test-variant".to_string(),
            name: "Test".to_string(),
            who_label: None,
            pango_lineage: "B.1".to_string(),
            first_detected: Utc::now(),
            geographic_origin: "Test Location".to_string(),
            mutations: vec![],
        };
        
        let idx = graph.add_node(VirusNode::Variant(variant));
        assert!(graph.get_node("test-variant").is_some());
    }
}
