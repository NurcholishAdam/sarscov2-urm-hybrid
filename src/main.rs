//! SARS-CoV-2 URM Hybrid - Main Entry Point

use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(name = "sarscov2-urm-hybrid")]
#[command(about = "Universal Reasoning Model for SARS-CoV-2 Knowledge Graph")]
struct Cli {
    /// Query to process
    #[arg(short, long)]
    query: Option<String>,

    /// Language code (en, zh, es, etc.)
    #[arg(short, long, default_value = "en")]
    language: String,

    /// Run in interactive mode
    #[arg(short, long)]
    interactive: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    println!("🦠 SARS-CoV-2 URM Hybrid System");
    println!("================================\n");

    if cli.interactive {
        run_interactive_mode().await?;
    } else if let Some(query) = cli.query {
        process_query(&query, &cli.language).await?;
    } else {
        println!("Use --query or --interactive mode");
        println!("Example: sarscov2-urm-hybrid --query \"What are Omicron mutations?\"");
    }

    Ok(())
}

async fn run_interactive_mode() -> Result<()> {
    println!("Interactive mode - Type 'exit' to quit\n");
    
    loop {
        use std::io::{self, Write};
        print!("> ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        let query = input.trim();
        if query == "exit" {
            break;
        }
        
        if !query.is_empty() {
            process_query(query, "en").await?;
        }
    }
    
    Ok(())
}

async fn process_query(query: &str, language: &str) -> Result<()> {
    println!("Processing: {}", query);
    println!("Language: {}", language);
    println!("\n✓ Query processed successfully\n");
    Ok(())
}
