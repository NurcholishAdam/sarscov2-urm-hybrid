# Deployment Guide

## Prerequisites

- Rust 1.70+
- 8GB RAM minimum
- 50GB disk space for data
- API keys for LIMIT-GRAPH (optional)

## Installation

### From Source

```bash
git clone https://github.com/yourusername/sarscov2-urm-hybrid
cd sarscov2-urm-hybrid
cargo build --release
```

### Using Docker

```bash
docker build -t sarscov2-urm-hybrid .
docker run -p 8080:8080 sarscov2-urm-hybrid
```

## Configuration

Create `.env` file:

```env
LIMIT_GRAPH_ENDPOINT=https://api.limit-graph.io
LIMIT_GRAPH_API_KEY=your_api_key_here
GROQ_API_KEY=your_groq_key
LOG_LEVEL=info
```

## Running

### CLI Mode

```bash
cargo run --release -- --query "What mutations are in Omicron?"
```

### Server Mode

```bash
cargo run --release --bin server
```

## Data Setup

Download SARS-CoV-2 data:

```bash
./scripts/download_data.sh
```

## Monitoring

- Logs: `./logs/app.log`
- Metrics: `http://localhost:9090/metrics`
- Health: `http://localhost:8080/health`

## Scaling

For production deployments:

1. Use multiple worker threads
2. Enable caching layer
3. Set up load balancer
4. Configure database replication

## Troubleshooting

### Out of Memory
- Reduce batch size
- Enable streaming mode
- Increase system RAM

### Slow Queries
- Check graph indices
- Enable query caching
- Optimize graph structure
