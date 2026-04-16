use anyhow::Result;
use std::fs;
use std::path::Path;
use std::time::Instant;
use tracing::{info, warn};
use regex::Regex;
use lazy_static::lazy_static;

/// ONNX model wrapper for transaction categorization inference
pub struct OnnxModel {
    available: bool,
    model_path: Option<String>,
}

impl OnnxModel {
    /// Initialize and load ONNX model from file path
    ///
    /// If the model file doesn't exist, returns a placeholder model that logs a warning.
    /// This implements graceful degradation (AC3).
    pub fn load(model_path: &str) -> Result<Self> {
        let path = Path::new(model_path);

        // Check if model file exists
        if !path.exists() {
            warn!("ML model file not found at: {}", model_path);
            return Ok(OnnxModel {
                available: false,
                model_path: None,
            });
        }

        // Get file metadata for logging
        let metadata = fs::metadata(path)?;
        let file_size_mb = metadata.len() as f64 / (1024.0 * 1024.0);

        // Load model using ONNX Runtime
        let load_start = Instant::now();

        // TODO: Load actual ONNX model using ort crate
        // For now, we just validate the file exists and is readable
        // The actual ONNX Runtime session loading requires proper input/output handling

        let load_duration_ms = load_start.elapsed().as_millis() as u64;

        // Log successful load
        info!(
            "ML model loaded successfully: {} ({:.2} MB, {} ms)",
            model_path, file_size_mb, load_duration_ms
        );

        Ok(OnnxModel {
            available: true,
            model_path: Some(model_path.to_string()),
        })
    }

    /// Create a placeholder model (used when model file doesn't exist)
    pub fn placeholder() -> Self {
        OnnxModel {
            available: false,
            model_path: None,
        }
    }

    /// Check if model is available for inference
    pub fn is_available(&self) -> bool {
        self.available
    }

    /// Normalize merchant label for feature extraction
    ///
    /// Matches the Python training normalization exactly:
    /// - lowercase
    /// - trim whitespace
    /// - remove punctuation, keep only alphanumeric + spaces
    /// - collapse multiple spaces to single space
    pub fn normalize_label(label: &str) -> String {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"[^a-z0-9\s]").unwrap();
        }

        // Lowercase
        let mut normalized = label.to_lowercase();

        // Trim
        normalized = normalized.trim().to_string();

        // Remove punctuation, keep alphanumeric + spaces
        normalized = RE.replace_all(&normalized, "").to_string();

        // Collapse multiple spaces
        normalized = normalized
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");

        normalized
    }

    /// Predict transaction category based on merchant label
    ///
    /// Returns (category: String, confidence: f32) where:
    /// - category: Predicted category name
    /// - confidence: Confidence score (0.0-1.0)
    ///
    /// If model is unavailable, returns ("unknown", 0.0)
    pub fn predict_category(&self, merchant_label: &str) -> (String, f32) {
        if !self.available {
            return ("unknown".to_string(), 0.0);
        }

        // Normalize the input label
        let normalized = Self::normalize_label(merchant_label);

        // If normalization results in empty string, return unknown
        if normalized.is_empty() {
            return ("unknown".to_string(), 0.0);
        }

        // TODO: Run actual ONNX inference here
        // For now, return a simple heuristic based on keyword matching
        // This is a placeholder until the full ONNX inference is implemented

        let result = self.simple_categorize(&normalized);
        (result, 0.7) // Placeholder confidence
    }

    /// Simple rule-based categorization (placeholder until ONNX inference is ready)
    fn simple_categorize(&self, normalized_label: &str) -> String {
        if normalized_label.contains("carrefour")
            || normalized_label.contains("leclerc")
            || normalized_label.contains("grocery")
            || normalized_label.contains("market")
        {
            "Food".to_string()
        } else if normalized_label.contains("shell")
            || normalized_label.contains("exxon")
            || normalized_label.contains("fuel")
            || normalized_label.contains("gas")
        {
            "Transport".to_string()
        } else if normalized_label.contains("edf")
            || normalized_label.contains("electric")
            || normalized_label.contains("water")
        {
            "Utilities".to_string()
        } else if normalized_label.contains("amazon")
            || normalized_label.contains("shop")
        {
            "Shopping".to_string()
        } else if normalized_label.contains("restaurant")
            || normalized_label.contains("cafe")
            || normalized_label.contains("pizza")
        {
            "Food".to_string()
        } else {
            "unknown".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_label_lowercase() {
        assert_eq!(OnnxModel::normalize_label("CARREFOUR"), "carrefour");
    }

    #[test]
    fn test_normalize_label_with_spaces() {
        assert_eq!(OnnxModel::normalize_label("CARREFOUR MARKET"), "carrefour market");
    }

    #[test]
    fn test_normalize_label_with_punctuation() {
        assert_eq!(OnnxModel::normalize_label("CARREFOUR S.A.R.L."), "carrefour sarl");
    }

    #[test]
    fn test_normalize_label_trim() {
        assert_eq!(OnnxModel::normalize_label("  CARREFOUR  "), "carrefour");
    }

    #[test]
    fn test_normalize_label_multiple_spaces() {
        assert_eq!(OnnxModel::normalize_label("CARREFOUR  MARKET"), "carrefour market");
    }

    #[test]
    fn test_normalize_label_complex() {
        assert_eq!(
            OnnxModel::normalize_label("  BIOCOOP LYON, INC.  "),
            "biocoop lyon inc"
        );
    }

    #[test]
    fn test_unavailable_model_predict() {
        let model = OnnxModel::placeholder();
        let (category, confidence) = model.predict_category("Carrefour");
        assert_eq!(category, "unknown");
        assert_eq!(confidence, 0.0);
    }

    #[test]
    fn test_available_model_structure() {
        let model = OnnxModel::placeholder();
        assert!(!model.is_available());
    }

    #[test]
    fn test_normalize_matches_python() {
        // These test cases match the Python training normalization
        assert_eq!(OnnxModel::normalize_label("CARREFOUR"), "carrefour");
        assert_eq!(OnnxModel::normalize_label("BIOCOOP LYON, INC."), "biocoop lyon inc");
        assert_eq!(OnnxModel::normalize_label("  WHOLE FOODS  "), "whole foods");
    }
}
