//! SARS-CoV-2 Variant Tracking Example

use sarscov2_graph::{CovidKnowledgeGraph, VariantNode};
use chrono::Utc;
use anyhow::Result;

fn main() -> Result<()> {
    println!("🦠 SARS-CoV-2 Variant Tracking");
    println!("==============================\n");

    let mut graph = CovidKnowledgeGraph::new();

    // Add Omicron variant
    let omicron = VariantNode {
        id: "omicron-ba1".to_string(),
        name: "Omicron".to_string(),
        who_label: Some("Omicron".to_string()),
        pango_lineage: "BA.1".to_string(),
        first_detected: Utc::now(),
        geographic_origin: "South Africa".to_string(),
        mutations: vec![
            "S:N501Y".to_string(),
            "S:E484A".to_string(),
            "S:K417N".to_string(),
        ],
    };

    let node_idx = graph.add_node(
        sarscov2_graph::VirusNode::Variant(omicron)
    );
    
    println!("✓ Added Omicron variant to graph");
    println!("  Node index: {:?}", node_idx);
    println!("  Mutations tracked: 3");

    Ok(())
}
