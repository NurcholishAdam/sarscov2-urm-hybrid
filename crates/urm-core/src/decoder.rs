//! ConvisulGLU Decoder Implementation
//! 
//! Implements the core decoder with gated linear units for
//! multilingual reasoning with recurrent loops.

use crate::{URMConfig, URMOutput, RecurrentState, TestCase, TokenRepresentation};
use crate::loops::{Loop1, Loop2};
use crate::multilingual::MultilingualProcessor;
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// ConvisulGLU Decoder with dual recurrent loops
#[derive(Debug, Clone)]
pub struct ConvisulGLUDecoder {
    config: URMConfig,
    loop1: Loop1,
    loop2: Loop2,
    multilingual: MultilingualProcessor,
}

impl ConvisulGLUDecoder {
    /// Create a new decoder with the given configuration
    pub fn new(config: URMConfig) -> Self {
        Self {
            loop1: Loop1::new(config.hidden_dim),
            loop2: Loop2::new(config.hidden_dim),
            multilingual: MultilingualProcessor::new(config.languages.clone()),
            config,
        }
    }

    /// Process a query through the recurrent loops
    pub fn process(&self, query: &str, language: &str) -> Result<URMOutput> {
        // Tokenize and embed the query
        let tokens = self.tokenize(query);
        let mut token_representations = Vec::new();
        
        // Initial embeddings
        let embeddings = self.embed_tokens(&tokens, language)?;
        
        // Initialize recurrent state
        let mut state = RecurrentState::new(self.config.hidden_dim);
        let mut intermediate_states = Vec::new();
        
        // Process through Loop1 (token-level)
        for (token, embedding) in tokens.iter().zip(embeddings.iter()) {
            state = self.loop1.forward(&state, embedding)?;
            
            token_representations.push(TokenRepresentation {
                token: token.clone(),
                embedding: embedding.clone(),
                loop1_state: state.hidden.clone(),
                loop2_state: vec![0.0; self.config.hidden_dim],
                language: language.to_string(),
            });
            
            intermediate_states.push(state.clone());
        }
        
        // Process through Loop2 (context-aware refinement)
        for i in 0..token_representations.len() {
            let context = self.build_context(&token_representations, i);
            state = self.loop2.forward(&state, &context)?;
            token_representations[i].loop2_state = state.hidden.clone();
            intermediate_states.push(state.clone());
        }
        
        // Generate test cases
        let test_cases = self.generate_test_cases(query, language, &state)?;
        
        // Calculate confidence
        let confidence = self.calculate_confidence(&state);
        
        Ok(URMOutput {
            final_representation: state.hidden.clone(),
            intermediate_states,
            test_cases,
            confidence,
        })
    }

    /// Tokenize input text
    fn tokenize(&self, text: &str) -> Vec<String> {
        text.split_whitespace()
            .map(|s| s.to_string())
            .collect()
    }

    /// Embed tokens for the given language
    fn embed_tokens(&self, tokens: &[String], language: &str) -> Result<Vec<Vec<f32>>> {
        self.multilingual.embed_tokens(tokens, language)
    }

    /// Build context for Loop2 processing
    fn build_context(&self, representations: &[TokenRepresentation], index: usize) -> Vec<f32> {
        let window_size = 3;
        let start = index.saturating_sub(window_size);
        let end = (index + window_size + 1).min(representations.len());
        
        let mut context = vec![0.0; self.config.hidden_dim];
        let mut count = 0;
        
        for i in start..end {
            for (j, val) in representations[i].loop1_state.iter().enumerate() {
                context[j] += val;
            }
            count += 1;
        }
        
        // Average the context
        if count > 0 {
            for val in context.iter_mut() {
                *val /= count as f32;
            }
        }
        
        context
    }

    /// Generate multilingual test cases
    fn generate_test_cases(&self, query: &str, language: &str, state: &RecurrentState) -> Result<Vec<TestCase>> {
        let mut test_cases = Vec::new();
        
        // Generate test case for the input language
        test_cases.push(TestCase {
            language: language.to_string(),
            query: query.to_string(),
            expected_output: None,
            generated_code: Some(self.generate_code_from_state(state)),
        });
        
        // Generate test cases for other languages
        for lang in &self.config.languages {
            if lang != language {
                let translated = self.multilingual.translate(query, language, lang)?;
                test_cases.push(TestCase {
                    language: lang.clone(),
                    query: translated,
                    expected_output: None,
                    generated_code: Some(self.generate_code_from_state(state)),
                });
            }
        }
        
        Ok(test_cases)
    }

    /// Generate code from recurrent state
    fn generate_code_from_state(&self, state: &RecurrentState) -> String {
        // Simplified code generation based on state
        format!("// Generated from state with confidence\nfn hypothesis() {{\n    // Implementation\n}}")
    }

    /// Calculate confidence score
    fn calculate_confidence(&self, state: &RecurrentState) -> f32 {
        // Calculate confidence based on state magnitude
        let magnitude: f32 = state.hidden.iter().map(|x| x * x).sum::<f32>().sqrt();
        (magnitude / (self.config.hidden_dim as f32).sqrt()).min(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decoder_creation() {
        let config = URMConfig::default();
        let decoder = ConvisulGLUDecoder::new(config);
        assert_eq!(decoder.config.hidden_dim, 768);
    }

    #[test]
    fn test_tokenization() {
        let config = URMConfig::default();
        let decoder = ConvisulGLUDecoder::new(config);
        let tokens = decoder.tokenize("Hello world test");
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], "Hello");
    }

    #[test]
    fn test_process_english() {
        let config = URMConfig::default();
        let decoder = ConvisulGLUDecoder::new(config);
        let result = decoder.process("What are spike protein mutations?", "en");
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.confidence > 0.0);
        assert!(!output.test_cases.is_empty());
    }
}
