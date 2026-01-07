# SARS-CoV-2 Knowledge Graph

A comprehensive 3D knowledge graph for tracking SARS-CoV-2 variants, mutations, and protein structures.

## Features

- **Variant Tracking**: Track viral variants with WHO labels and Pango lineages
- **3D Protein Structures**: Store and query protein structures with spatial coordinates
- **Mutation Analysis**: Track mutations with impact assessment
- **Temporal Tracking**: Monitor variant evolution over time
- **Spatial Operations**: Calculate distances, RMSD, and find neighbors in 3D space

## Usage

```rust
use sarscov2_graph::{CovidKnowledgeGraph, VariantNode, VirusNode};
use chrono::Utc;

// Create a new graph
let mut graph = CovidKnowledgeGraph::new();

// Add a variant
let omicron = VariantNode {
    id: "omicron-ba1".to_string(),
    name: "Omicron".to_string(),
    who_label: Some("Omicron".to_string()),
    pango_lineage: "BA.1".to_string(),
    first_detected: Utc::now(),
    geographic_origin: "South Africa".to_string(),
    mutations: vec!["S:N501Y".to_string()],
};

graph.add_node(VirusNode::Variant(omicron));

// Query variants
let variants = graph.get_variants();
println!("Found {} variants", variants.len());
```

## Node Types

- **VariantNode**: Viral variants (Alpha, Delta, Omicron, etc.)
- **ProteinNode**: Protein structures with 3D coordinates
- **MutationNode**: Specific mutations with impact assessment
- **SequenceNode**: Genomic sequences

## Edge Types

- **HasMutation**: Variant has specific mutation
- **DerivedFrom**: Variant derived from another
- **BindsTo**: Protein binding relationships
- **TemporalSuccession**: Time-based relationships
- **SpatialProximity**: 3D spatial relationships

## Spatial Operations

```rust
use sarscov2_graph::spatial::{distance, center_of_mass, rmsd};
use sarscov2_graph::nodes::Coordinate3D;

let p1 = Coordinate3D::new(0.0, 0.0, 0.0);
let p2 = Coordinate3D::new(1.0, 0.0, 0.0);

let dist = distance(&p1, &p2);
println!("Distance: {}", dist);
```

## Data Sources

- GISAID: Viral genome sequences
- NCBI: Protein structures
- WHO: Variant classifications
