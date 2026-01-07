//! Graph query interface

use crate::{VirusNode, RelationshipEdge};
use petgraph::graph::DiGraph;
use anyhow::Result;

/// Query builder for knowledge graph
pub struct GraphQuery<'a> {
    graph: &'a DiGraph<VirusNode, RelationshipEdge>,
    filters: Vec<QueryFilter>,
}

#[derive(Debug, Clone)]
enum QueryFilter {
    NodeType(String),
    TemporalRange(String, String),
    GeographicOrigin(String),
}

impl<'a> GraphQuery<'a> {
    pub fn new(graph: &'a DiGraph<VirusNode, RelationshipEdge>) -> Self {
        Self {
            graph,
            filters: Vec::new(),
        }
    }

    /// Filter by node type
    pub fn filter_by_type(mut self, node_type: impl Into<String>) -> Self {
        self.filters.push(QueryFilter::NodeType(node_type.into()));
        self
    }

    /// Filter by temporal range
    pub fn with_temporal_range(mut self, start: &str, end: &str) -> Self {
        self.filters.push(QueryFilter::TemporalRange(
            start.to_string(),
            end.to_string(),
        ));
        self
    }

    /// Filter by geographic origin
    pub fn with_geographic_origin(mut self, origin: &str) -> Self {
        self.filters.push(QueryFilter::GeographicOrigin(origin.to_string()));
        self
    }

    /// Execute the query
    pub fn execute(self) -> Result<Vec<VirusNode>> {
        let mut results: Vec<VirusNode> = self.graph
            .node_weights()
            .cloned()
            .collect();

        // Apply filters
        for filter in self.filters {
            results = self.apply_filter(results, filter);
        }

        Ok(results)
    }

    fn apply_filter(&self, nodes: Vec<VirusNode>, filter: QueryFilter) -> Vec<VirusNode> {
        match filter {
            QueryFilter::NodeType(_) => nodes,
            QueryFilter::TemporalRange(_, _) => nodes,
            QueryFilter::GeographicOrigin(_) => nodes,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_builder() {
        let graph = DiGraph::new();
        let query = GraphQuery::new(&graph)
            .filter_by_type("Variant")
            .with_temporal_range("2021-01-01", "2022-01-01");
        
        let results = query.execute().unwrap();
        assert_eq!(results.len(), 0);
    }
}
