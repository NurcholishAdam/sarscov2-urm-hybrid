//! Recurrent Loop Implementations
//! 
//! Loop1: Token-level processing with gated linear units
//! Loop2: Context-aware refinement with attention

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Recurrent state maintained across loop iterations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurrentState {
    pub hidden: Vec<f32>,
    pub cell: Vec<f32>,
    pub iteration: usize,
}

impl RecurrentState {
    pub fn new(hidden_dim: usize) -> Self {
        Self {
            hidden: vec![0.0; hidden_dim],
            cell: vec![0.0; hidden_dim],
            iteration: 0,
        }
    }
}

/// Loop1: Initial token-level processing
#[derive(Debug, Clone)]
pub struct Loop1 {
    hidden_dim: usize,
    gate_weights: Vec<Vec<f32>>,
    transform_weights: Vec<Vec<f32>>,
}

impl Loop1 {
    pub fn new(hidden_dim: usize) -> Self {
        Self {
            hidden_dim,
            gate_weights: Self::init_weights(hidden_dim, hidden_dim),
            transform_weights: Self::init_weights(hidden_dim, hidden_dim),
        }
    }

    /// Forward pass through Loop1
    pub fn forward(&self, state: &RecurrentState, input: &[f32]) -> Result<RecurrentState> {
        let mut new_state = state.clone();
        new_state.iteration += 1;

        // Gated linear unit computation
        let gate = self.compute_gate(&state.hidden, input);
        let transform = self.compute_transform(&state.hidden, input);

        // Update hidden state with gating
        for i in 0..self.hidden_dim {
            new_state.hidden[i] = gate[i] * transform[i] + (1.0 - gate[i]) * state.hidden[i];
        }

        // Update cell state
        for i in 0..self.hidden_dim {
            new_state.cell[i] = 0.9 * state.cell[i] + 0.1 * new_state.hidden[i];
        }

        Ok(new_state)
    }

    /// Compute gate values
    fn compute_gate(&self, hidden: &[f32], input: &[f32]) -> Vec<f32> {
        let mut gate = vec![0.0; self.hidden_dim];
        
        for i in 0..self.hidden_dim {
            let mut sum = 0.0;
            for j in 0..self.hidden_dim.min(hidden.len()) {
                sum += self.gate_weights[i][j] * hidden[j];
            }
            for j in 0..input.len().min(self.hidden_dim) {
                sum += input[j] * 0.1;
            }
            gate[i] = sigmoid(sum);
        }
        
        gate
    }

    /// Compute transformation
    fn compute_transform(&self, hidden: &[f32], input: &[f32]) -> Vec<f32> {
        let mut transform = vec![0.0; self.hidden_dim];
        
        for i in 0..self.hidden_dim {
            let mut sum = 0.0;
            for j in 0..self.hidden_dim.min(hidden.len()) {
                sum += self.transform_weights[i][j] * hidden[j];
            }
            for j in 0..input.len().min(self.hidden_dim) {
                sum += input[j] * 0.2;
            }
            transform[i] = tanh(sum);
        }
        
        transform
    }

    /// Initialize weight matrices
    fn init_weights(rows: usize, cols: usize) -> Vec<Vec<f32>> {
        let scale = (2.0 / (rows + cols) as f32).sqrt();
        (0..rows)
            .map(|i| {
                (0..cols)
                    .map(|j| {
                        // Simple deterministic initialization
                        let val = ((i * 7 + j * 13) % 100) as f32 / 100.0 - 0.5;
                        val * scale
                    })
                    .collect()
            })
            .collect()
    }
}

/// Loop2: Context-aware refinement
#[derive(Debug, Clone)]
pub struct Loop2 {
    hidden_dim: usize,
    attention_weights: Vec<Vec<f32>>,
    refinement_weights: Vec<Vec<f32>>,
}

impl Loop2 {
    pub fn new(hidden_dim: usize) -> Self {
        Self {
            hidden_dim,
            attention_weights: Self::init_weights(hidden_dim, hidden_dim),
            refinement_weights: Self::init_weights(hidden_dim, hidden_dim),
        }
    }

    /// Forward pass through Loop2
    pub fn forward(&self, state: &RecurrentState, context: &[f32]) -> Result<RecurrentState> {
        let mut new_state = state.clone();
        new_state.iteration += 1;

        // Compute attention over context
        let attention = self.compute_attention(&state.hidden, context);
        
        // Apply attention to refine hidden state
        let refinement = self.compute_refinement(&state.hidden, &attention);

        // Update hidden state with refinement
        for i in 0..self.hidden_dim {
            new_state.hidden[i] = 0.7 * state.hidden[i] + 0.3 * refinement[i];
        }

        // Update cell state with context
        for i in 0..self.hidden_dim.min(context.len()) {
            new_state.cell[i] = 0.8 * state.cell[i] + 0.2 * context[i];
        }

        Ok(new_state)
    }

    /// Compute attention weights
    fn compute_attention(&self, hidden: &[f32], context: &[f32]) -> Vec<f32> {
        let mut attention = vec![0.0; self.hidden_dim];
        
        for i in 0..self.hidden_dim {
            let mut sum = 0.0;
            for j in 0..self.hidden_dim.min(hidden.len()) {
                sum += self.attention_weights[i][j] * hidden[j];
            }
            for j in 0..context.len().min(self.hidden_dim) {
                sum += context[j] * 0.15;
            }
            attention[i] = sigmoid(sum);
        }
        
        // Normalize attention
        let total: f32 = attention.iter().sum();
        if total > 0.0 {
            for val in attention.iter_mut() {
                *val /= total;
            }
        }
        
        attention
    }

    /// Compute refinement based on attention
    fn compute_refinement(&self, hidden: &[f32], attention: &[f32]) -> Vec<f32> {
        let mut refinement = vec![0.0; self.hidden_dim];
        
        for i in 0..self.hidden_dim {
            let mut sum = 0.0;
            for j in 0..self.hidden_dim.min(hidden.len()) {
                sum += self.refinement_weights[i][j] * hidden[j] * attention[j];
            }
            refinement[i] = tanh(sum);
        }
        
        refinement
    }

    /// Initialize weight matrices
    fn init_weights(rows: usize, cols: usize) -> Vec<Vec<f32>> {
        let scale = (2.0 / (rows + cols) as f32).sqrt();
        (0..rows)
            .map(|i| {
                (0..cols)
                    .map(|j| {
                        let val = ((i * 11 + j * 17) % 100) as f32 / 100.0 - 0.5;
                        val * scale
                    })
                    .collect()
            })
            .collect()
    }
}

/// Sigmoid activation function
fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}

/// Hyperbolic tangent activation function
fn tanh(x: f32) -> f32 {
    x.tanh()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recurrent_state_creation() {
        let state = RecurrentState::new(768);
        assert_eq!(state.hidden.len(), 768);
        assert_eq!(state.cell.len(), 768);
        assert_eq!(state.iteration, 0);
    }

    #[test]
    fn test_loop1_forward() {
        let loop1 = Loop1::new(768);
        let state = RecurrentState::new(768);
        let input = vec![0.5; 768];
        
        let result = loop1.forward(&state, &input);
        assert!(result.is_ok());
        
        let new_state = result.unwrap();
        assert_eq!(new_state.iteration, 1);
    }

    #[test]
    fn test_loop2_forward() {
        let loop2 = Loop2::new(768);
        let state = RecurrentState::new(768);
        let context = vec![0.3; 768];
        
        let result = loop2.forward(&state, &context);
        assert!(result.is_ok());
        
        let new_state = result.unwrap();
        assert_eq!(new_state.iteration, 1);
    }

    #[test]
    fn test_sigmoid() {
        assert!((sigmoid(0.0) - 0.5).abs() < 0.01);
        assert!(sigmoid(10.0) > 0.99);
        assert!(sigmoid(-10.0) < 0.01);
    }

    #[test]
    fn test_tanh() {
        assert!(tanh(0.0).abs() < 0.01);
        assert!(tanh(10.0) > 0.99);
        assert!(tanh(-10.0) < -0.99);
    }
}
