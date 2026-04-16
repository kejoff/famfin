/// LLM-based categorization (future)
///
/// This module will provide transaction categorization using LLM inference
/// via local LLM runtime (e.g., ollama, gpt4all) for privacy-preserving
/// transaction analysis on the device.

pub struct LlmCategorizer {
    available: bool,
}

impl LlmCategorizer {
    /// Initialize LLM categorizer (placeholder)
    pub fn new() -> Self {
        LlmCategorizer {
            available: false,
        }
    }

    /// Check if LLM is available
    pub fn is_available(&self) -> bool {
        self.available
    }
}

impl Default for LlmCategorizer {
    fn default() -> Self {
        Self::new()
    }
}
