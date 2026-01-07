use urm_core::{ConvisulGLUDecoder, URMConfig};
use sarscov2_graph::CovidKnowledgeGraph;
use anyhow::Result;

#[test]
fn test_complete_workflow() -> Result<()> {
    let config = URMConfig::default();
    let decoder = ConvisulGLUDecoder::new(config);
    let graph = CovidKnowledgeGraph::new();
    
    let result = decoder.process("Test query", "en")?;
    
    assert!(result.confidence > 0.0);
    assert!(!result.test_cases.is_empty());
    
    Ok(())
}

#[test]
fn test_multilingual_support() -> Result<()> {
    let config = URMConfig::default();
    let decoder = ConvisulGLUDecoder::new(config);
    
    for lang in &["en", "zh", "es"] {
        let result = decoder.process("Test", lang)?;
        assert!(result.confidence > 0.0);
    }
    
    Ok(())
}
