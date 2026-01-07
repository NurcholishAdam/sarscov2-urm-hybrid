//! Node types for SARS-CoV-2 knowledge graph

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Base virus node in the knowledge graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VirusNode {
    Variant(VariantNode),
    Protein(ProteinNode),
    Mutation(MutationNode),
    Sequence(SequenceNode),
}

impl VirusNode {
    pub fn id(&self) -> String {
        match self {
            VirusNode::Variant(v) => v.id.clone(),
            VirusNode::Protein(p) => p.id.clone(),
            VirusNode::Mutation(m) => m.id.clone(),
            VirusNode::Sequence(s) => s.id.clone(),
        }
    }
}

/// SARS-CoV-2 variant node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantNode {
    pub id: String,
    pub name: String,
    pub who_label: Option<String>,
    pub pango_lineage: String,
    pub first_detected: DateTime<Utc>,
    pub geographic_origin: String,
    pub mutations: Vec<String>,
}

/// Protein structure node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProteinNode {
    pub id: String,
    pub name: String,
    pub protein_type: ProteinType,
    pub sequence: String,
    pub structure_3d: Option<Vec<Coordinate3D>>,
    pub pdb_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProteinType {
    Spike,
    Nucleocapsid,
    Membrane,
    Envelope,
    Other(String),
}

/// Mutation node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutationNode {
    pub id: String,
    pub position: usize,
    pub original_aa: char,
    pub mutated_aa: char,
    pub protein: String,
    pub impact: MutationImpact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MutationImpact {
    High,
    Medium,
    Low,
    Unknown,
}

/// Genomic sequence node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequenceNode {
    pub id: String,
    pub accession: String,
    pub sequence: String,
    pub length: usize,
    pub collection_date: DateTime<Utc>,
}

/// 3D coordinate for protein structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coordinate3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Coordinate3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn distance_to(&self, other: &Coordinate3D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}
