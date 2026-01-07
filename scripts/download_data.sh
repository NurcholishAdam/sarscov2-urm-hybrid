//! Complete URM + SARS-CoV-2 Workflow Example

use urm_core::{ConvisulGLUDecoder, URMConfig};
use sarscov2_graph::CovidKnowledgeGraph;
use confucius_agent::ConfuciusAgent;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🦠 SARS-CoV-2 URM Hybrid Workflow");
    println!("================================\n");

    // Step 1: Initialize URM decoder
    let config = URMConfig::default();
    let decoder = ConvisulGLUDecoder::new(config);
    println!("✓ URM Decoder initialized");

    // Step 2: Load SARS-CoV-2 knowledge graph
    let mut graph = CovidKnowledgeGraph::new();
    println!("✓ Knowledge graph loaded");

    // Step 3: Process multilingual prompt
    let prompt = "What are the key mutations in Omicron variant?";
    let result = decoder.process(prompt, "en")?;
    println!("✓ Processed prompt: {}", prompt);
    println!("  Confidence: {:.2}", result.confidence);

    // Step 4: Generate code with Confucius agent
    let agent = ConfuciusAgent::new("gpt-4".to_string());
    println!("✓ Confucius agent ready");

    println!("\n🎉 Workflow completed successfully!");
    Ok(())
}
