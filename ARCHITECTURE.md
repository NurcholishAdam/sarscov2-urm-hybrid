# Architecture Documentation

## System Overview

The SARS-CoV-2 URM Hybrid system combines Universal Reasoning Model (URM) recurrent loops with a comprehensive SARS-CoV-2 knowledge graph, enhanced by LIMIT-GRAPH quantum reasoning capabilities.

## Component Architecture

### 1. URM Core (`urm-core`)

**Purpose**: Implements the ConvisulGLU decoder with dual recurrent loops

**Key Components**:
- `ConvisulGLUDecoder`: Main decoder with gated linear units
- `Loop1`: Initial token-level processing
- `Loop2`: Context-aware refinement
- `MultilingualProcessor`: Cross-lingual alignment

**Data Flow**:
```
Input Prompt → Tokenization → Loop1 → Loop2 → Test Cases → Output
```

### 2. SARS-CoV-2 Graph (`sarscov2-graph`)

**Purpose**: 3D knowledge graph for viral tracking

**Node Types**:
- `VariantNode`: Viral variants (Alpha, Delta, Omicron, etc.)
- `ProteinNode`: Protein structures with 3D coordinates
- `MutationNode`: Specific mutations with impact assessment
- `SequenceNode`: Genomic sequences

**Edge Types**:
- HasMutation
- DerivedFrom
- BindsTo
- TemporalSuccession
- SpatialProximity

### 3. LIMIT-GRAPH Integration (`limit-integration`)

**Purpose**: Connect to LIMIT-GRAPH platform for quantum reasoning

**Features**:
- Graph query interface
- Provenance tracking
- Durable memory storage

### 4. Confucius Agent (`confucius-agent`)

**Purpose**: Intelligent code generation and hypothesis evaluation

**Workflow**:
1. Receive hypothesis from URM
2. Generate code implementation
3. Evaluate against test cases
4. Provide feedback for refinement

### 5. Quantum Reasoning (`quantum-reasoning`)

**Purpose**: Quantum-enhanced reasoning capabilities

**Components**:
- `QuantumCircuit`: Quantum state manipulation
- `QuantumWalk`: Graph traversal with quantum speedup

## Workflow Integration

### Complete Pipeline

```
┌─────────────────┐
│ Multilingual    │
│ Prompt          │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ ConvisulGLU     │
│ Loop 1 & 2      │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Test Case       │
│ Generation      │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ SARS-CoV-2      │
│ Graph Query     │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Confucius       │
│ Code Agent      │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Quantum         │
│ Evaluation      │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Provenance      │
│ Tracking        │
└─────────────────┘
```

## Data Models

### URMOutput
```rust
struct URMOutput {
    final_representation: Vec<f32>,
    intermediate_states: Vec<RecurrentState>,
    test_cases: Vec<TestCase>,
    confidence: f32,
}
```

### VirusNode
```rust
enum VirusNode {
    Variant(VariantNode),
    Protein(ProteinNode),
    Mutation(MutationNode),
    Sequence(SequenceNode),
}
```

## Performance Considerations

- **Parallel Processing**: Use Rayon for parallel graph operations
- **Memory Efficiency**: Stream large datasets instead of loading all at once
- **Caching**: Cache frequently accessed graph nodes
- **Quantum Optimization**: Use quantum circuits for complex graph queries

## Security & Privacy

- All patient data is anonymized
- Genomic sequences follow GISAID data sharing protocols
- API keys stored in environment variables
- Provenance tracking for audit trails
