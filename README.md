# SARS-CoV-2 URM Hybrid v2.4.2: Universal Reasoning Model with Knowledge Graph

[![Version](https://img.shields.io/badge/version-2.4.2-blue.svg)](https://github.com/yourusername/sarscov2-urm-hybrid)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![LIMIT-GRAPH](https://img.shields.io/badge/LIMIT--GRAPH-v2.4.2-purple.svg)](https://github.com/yourusername/quantum-limit-graph-v2.4.0)

A cutting-edge hybrid architecture combining Universal Reasoning Model (URM) recurrent loops with LIMIT-GRAPH v2.4.2 platform for SARS-CoV-2 knowledge graph tracking and quantum-enhanced reasoning.

## 🏗️ Architecture Overview

This system integrates six core components in a unified workflow:

1. **URM Recurrent Loops** - ConvisulGLU decoder with dual-loop architecture (Loop1 + Loop2)
2. **SARS-CoV-2 3D Knowledge Graph** - Comprehensive viral variant tracking with spatial-temporal analysis
3. **LIMIT-GRAPH Platform v2.4.2** - Quantum-enhanced reasoning and provenance tracking
4. **Confucius Code Agent** - Intelligent hypothesis generation and code evaluation
5. **Durable Memory System** - Persistent provenance tracking with full lineage
6. **Quantum Principled HLP** - Hybrid learning paradigm with quantum circuits

**Version**: 2.4.2  
**Release Date**: January 7, 2026  
**Compatibility**: LIMIT-GRAPH v2.4.2, Rust 2021 Edition

## 🔄 Workflow Components

### Phase 1: Multilingual Prompt Processing
- ConvisulGLU decoder loops
- Token representations (Loop 1 & Loop 2)
- Multilingual test case generation

### Phase 2: Knowledge Graph Integration
- SARS-CoV-2 node and edge management
- 3D spatial relationship modeling
- Temporal evolution tracking

### Phase 3: Reasoning & Code Generation
- Confucius Code Agent for hypothesis generation
- Module 1: Retrieval from durable memory
- Module 2: Correction and alignment
- Module 3: Cross-lingual alignment

### Phase 4: Evaluation & Generalization
- Quantum-principled evaluation
- Provenance tracking
- Generalization across domains

## 📦 Crate Structure

```
sarscov2-urm-hybrid/
├── crates/
│   ├── urm-core/           # Core URM recurrent loop implementation
│   ├── sarscov2-graph/     # SARS-CoV-2 knowledge graph ✅
│   ├── limit-integration/  # LIMIT-GRAPH platform integration
│   ├── confucius-agent/    # Code generation agent ✅
│   └── quantum-reasoning/  # Quantum-enhanced reasoning
├── examples/               # Usage examples
├── tests/                  # Integration tests
├── data/                   # SARS-CoV-2 data ✅
└── docs/                   # Documentation
```

## 🚀 Quick Start

```bash
# Clone the repository
git clone https://github.com/yourusername/sarscov2-urm-hybrid
cd sarscov2-urm-hybrid

# Build all crates
cargo build --release

# Run the complete demo
cargo run --example complete_workflow

# Run SARS-CoV-2 specific demo
cargo run --example sarscov2_tracking

# Interactive mode
cargo run --release -- --interactive
```

## 🧪 Examples

### Basic URM Loop
```rust
use urm_core::ConvisulGLUDecoder;
use sarscov2_graph::CovidKnowledgeGraph;

let decoder = ConvisulGLUDecoder::new(Default::default());
let graph = CovidKnowledgeGraph::new();

let result = decoder.process(
    "What are the spike protein mutations?",
    "en"
)?;
```

### Knowledge Graph Query
```rust
use sarscov2_graph::{CovidKnowledgeGraph, VariantNode};

let mut graph = CovidKnowledgeGraph::new();
let variants = graph.get_variants();

for variant in variants {
    println!("{}: {}", variant.name, variant.pango_lineage);
}
```

### Spatial Analysis
```rust
use sarscov2_graph::spatial::{distance, center_of_mass};
use sarscov2_graph::nodes::Coordinate3D;

let p1 = Coordinate3D::new(0.0, 0.0, 0.0);
let p2 = Coordinate3D::new(1.0, 0.0, 0.0);
let dist = distance(&p1, &p2);
```

## 🔬 Key Features

- **Multilingual Support**: Process queries in multiple languages
- **Recurrent Reasoning**: ConvisulGLU loops for iterative refinement
- **3D Knowledge Graph**: Spatial-temporal SARS-CoV-2 tracking
- **Quantum Enhancement**: Principled hybrid learning paradigm
- **Provenance Tracking**: Full lineage of reasoning steps
- **Code Generation**: Confucius agent for hypothesis testing

## 📊 Data Sources

This system integrates data from:
- [GISAID](https://www.gisaid.org/) - Viral genome sequences
- [NCBI](https://www.ncbi.nlm.nih.gov/) - Protein structures
- [WHO](https://www.who.int/) - Epidemiological data
- Custom 3D knowledge graph from [SARS-CoV-2-3D-Knowledge-Graph](https://github.com/NurcholishAdam/SARS-CoV-2-3D-Knowledge-Graph-1)

## 🧪 Testing

```bash
# Run all tests
cargo test --all

# Run specific crate tests
cargo test -p sarscov2-graph

# Run benchmarks
cargo bench
```

## 📚 Documentation

- [Architecture Guide](ARCHITECTURE.md)
- [Deployment Guide](DEPLOYMENT.md)
- [Contributing Guidelines](CONTRIBUTING.md)
- [API Documentation](https://docs.rs/sarscov2-urm-hybrid)

## 🤝 Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📄 License

This project is licensed under MIT License. See [LICENSE](LICENSE) for details.

## 🔗 Related Projects

- [LIMIT-GRAPH](https://github.com/yourusername/quantum-limit-graph-v2.4.0)
- [SARS-CoV-2 3D Knowledge Graph](https://github.com/NurcholishAdam/SARS-CoV-2-3D-Knowledge-Graph-1)

## 📚 Citation

If you use this work, please cite:

```bibtex
@software{sarscov2_urm_hybrid,
  title={SARS-CoV-2 URM Hybrid: Universal Reasoning Model with Knowledge Graph},
  author={Your Name},
  year={2026},
  url={https://github.com/yourusername/sarscov2-urm-hybrid}
}
```

## 🙏 Acknowledgments

- GISAID for viral sequence data
- NCBI for protein structures
- WHO for epidemiological guidance
- LIMIT-GRAPH platform contributors
