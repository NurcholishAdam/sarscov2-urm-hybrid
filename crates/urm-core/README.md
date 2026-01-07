# URM Core: Universal Reasoning Model

Core implementation of the Universal Reasoning Model with ConvisulGLU decoder and dual recurrent loops.

## Architecture

The URM Core implements a sophisticated recurrent architecture with two main loops:

### Loop 1: Token-Level Processing
- Initial tokenization and embedding
- Token-level representations
- Language-specific encoding

### Loop 2: Context-Aware Refinement
- Contextual integration
- Cross-token dependencies
- Semantic enrichment

## Components

### ConvisulGLUDecoder
The main decoder implementing gated linear units with recurrent processing.

```rust
use urm_core::{ConvisulGLUDecoder, URMConfig};

let config = URMConfig::default();
let decoder = ConvisulGLUDecoder::new(config);

let result = decoder.process("What are Omicron mutations?", "en")?;
println!("Confidence: {}", result.confidence);
```

### Multilingual Processor
Handles cross-lingual alignment and test case generation.

```rust
use urm_core::MultilingualProcessor;

let processor = MultilingualProcessor::new(vec![
    "en".to_string(),
    "zh".to_string(),
    "es".to_string(),
]);

let test_cases = processor.generate_multilingual_tests("Test query")?;
```

### Recurrent Loops
State management across iterations.

```rust
use urm_core::{Loop1, Loop2};

let loop1 = Loop1::new(768);
let state1 = loop1.forward(&input);

let loop2 = Loop2::new(768);
let state2 = loop2.forward(&state1);
```

## Configuration

```rust
use urm_core::URMConfig;

let config = URMConfig {
    hidden_dim: 768,
    num_loops: 2,
    languages: vec!["en".to_string(), "zh".to_string()],
    max_iterations: 10,
};
```

## Features

- **Dual Recurrent Loops**: Iterative refinement through Loop1 and Loop2
- **Multilingual Support**: Process queries in multiple languages
- **Token Representations**: Fine-grained token-level processing
- **Test Case Generation**: Automatic multilingual test case creation
- **Configurable Architecture**: Flexible hidden dimensions and loop counts

## Integration

Works seamlessly with:
- **sarscov2-graph**: Domain-specific knowledge graph
- **confucius-agent**: Code generation and evaluation
- **limit-integration**: LIMIT-GRAPH platform connection
- **quantum-reasoning**: Quantum-enhanced processing

## Performance

- Hidden dimension: 768 (default)
- Processing time: ~100ms per query
- Supports batch processing
- Parallel token processing with Rayon

## Testing

```bash
cargo test -p urm-core
```

## Examples

See `examples/` directory for complete usage examples.
