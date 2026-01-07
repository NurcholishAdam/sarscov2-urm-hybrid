use urm_core::{ConvisulGLUDecoder, URMConfig, MultilingualProcessor};
use anyhow::Result;

#[test]
fn test_decoder_initialization() {
    let config = URMConfig::default();
    let decoder = ConvisulGLUDecoder::new(config);
    assert_eq!(decoder.config.hidden_dim, 768);
}

#[test]
fn test_process_english() -> Result<()> {
    let config = URMConfig::default();
    let decoder = ConvisulGLUDecoder::new(config);
    
    let result = decoder.process("Test query", "en")?;
    
    assert!(result.confidence > 0.0);
    assert!(!result.intermediate_states.is_empty());
    assert!(!result.test_cases.is_empty());
    
    Ok(())
}

#[test]
fn test_multilingual_support() -> Result<()> {
    let config = URMConfig::default();
    let decoder = ConvisulGLUDecoder::new(config);
    
    for lang in &["en", "zh", "es", "fr", "de"] {
        let result = decoder.process("Test", lang)?;
        assert!(result.confidence > 0.0);
    }
    
    Ok(())
}

#[test]
fn test_multilingual_processor() -> Result<()> {
    let processor = MultilingualProcessor::new(vec![
        "en".to_string(),
        "zh".to_string(),
    ]);
    
    let test_cases = processor.generate_multilingual_tests("Test query")?;
    assert_eq!(test_cases.len(), 2);
    
    Ok(())
}

#[test]
fn test_recurrent_states() -> Result<()> {
    let config = URMConfig::default();
    let decoder = ConvisulGLUDecoder::new(config);
    
    let result = decoder.process("Complex query", "en")?;
    
    // Should have states from both loops
    assert!(result.intermediate_states.len() >= 2);
    
    // Check iteration numbers
    assert_eq!(result.intermediate_states[0].iteration, 1);
    assert_eq!(result.intermediate_states[1].iteration, 2);
    
    Ok(())
}

#[test]
fn test_empty_query() {
    let config = URMConfig::default();
    let decoder = ConvisulGLUDecoder::new(config);
    
    let result = decoder.process("", "en");
    assert!(result.is_ok());
}

#[test]
fn test_long_query() -> Result<()> {
    let config = URMConfig::default();
    let decoder = ConvisulGLUDecoder::new(config);
    
    let long_query = "This is a very long query ".repeat(100);
    let result = decoder.process(&long_query, "en")?;
    
    assert!(result.confidence > 0.0);
    
    Ok(())
}
