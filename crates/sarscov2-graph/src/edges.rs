//! Edge types for knowledge graph relationships

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Relationship edge between nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipEdge {
    pub edge_type: EdgeType,
    pub weight: f32,
    pub created_at: DateTime<Utc>,
    pub metadata: EdgeMetadata,
}

impl RelationshipEdge {
    pub fn new(edge_type: EdgeType, weight: f32) -> Self {
        Self {
            edge_type,
            weight,
            created_at: Utc::now(),
            metadata: EdgeMetadata {
                confidence: 1.0,
                source: "manual".to_string(),
                evidence: vec![],
            },
        }
    }
}

/// Types of relationships in the graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EdgeType {
    HasMutation,
    DerivedFrom,
    BindsTo,
    InhibitedBy,
    EnhancedBy,
    CoOccursWith,
    TemporalSuccession,
    SpatialProximity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeMetadata {
    pub confidence: f32,
    pub source: String,
    pub evidence: Vec<String>,
}
