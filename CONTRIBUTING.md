# Contributing to SARS-CoV-2 URM Hybrid

Thank you for your interest in contributing! This document provides guidelines for contributing to the project.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/yourusername/sarscov2-urm-hybrid`
3. Create a branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Run tests: `cargo test --all`
6. Commit: `git commit -m "Add your feature"`
7. Push: `git push origin feature/your-feature-name`
8. Open a Pull Request

## Development Setup

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build the project
cargo build

# Run tests
cargo test

# Run examples
cargo run --example complete_workflow
```

## Code Style

- Follow Rust standard formatting: `cargo fmt`
- Run clippy: `cargo clippy`
- Add documentation for public APIs
- Write tests for new features

## Areas for Contribution

### High Priority
- [ ] Additional SARS-CoV-2 variant data integration
- [ ] Enhanced quantum circuit implementations
- [ ] Performance optimizations for large graphs
- [ ] Multilingual test case expansion

### Medium Priority
- [ ] Web API for graph queries
- [ ] Visualization tools
- [ ] Additional protein structure formats
- [ ] Benchmark suite

### Documentation
- [ ] API documentation
- [ ] Tutorial examples
- [ ] Architecture diagrams
- [ ] Performance benchmarks

## Testing

All contributions should include tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_your_feature() {
        // Your test here
    }
}
```

## Pull Request Process

1. Update README.md with details of changes if needed
2. Update CHANGELOG.md
3. Ensure all tests pass
4. Request review from maintainers
5. Address review feedback
6. Merge once approved

## Code of Conduct

- Be respectful and inclusive
- Welcome newcomers
- Focus on constructive feedback
- Maintain professional communication

## Questions?

Open an issue or reach out to maintainers.
