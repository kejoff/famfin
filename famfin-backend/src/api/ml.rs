use axum::{
    extract::{Path, Extension, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{
    api::AppState,
    auth::middleware::AuthSession,
};

#[derive(Deserialize)]
pub struct PredictCategoryRequest {
    pub merchant_name: String,
    pub description: Option<String>,
    pub amount: Option<f64>,
}

#[derive(Serialize)]
pub struct PredictCategoryResponse {
    pub predicted_category: Option<String>,
    pub confidence: f64,
    pub model_available: bool,
}

/// Predict transaction category using ML model
pub async fn predict_category(
    State(state): State<AppState>,
    Path(household_id): Path<String>,
    Extension(auth): Extension<AuthSession>,
    Json(req): Json<PredictCategoryRequest>,
) -> Result<Json<PredictCategoryResponse>, (StatusCode, String)> {
    if auth.household_id != household_id {
        return Err((StatusCode::FORBIDDEN, "Access denied".to_string()));
    }

    // Get model from application state (loaded at startup)
    let (category, confidence) = state.ml_model.predict_category(&req.merchant_name);

    Ok(Json(PredictCategoryResponse {
        predicted_category: if category == "unknown" { None } else { Some(category) },
        confidence: confidence as f64,
        model_available: state.ml_model.is_available(),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_predict_response_structure() {
        let resp = PredictCategoryResponse {
            predicted_category: Some("groceries".to_string()),
            confidence: 0.85,
            model_available: true,
        };

        assert_eq!(resp.predicted_category, Some("groceries".to_string()));
        assert!(resp.model_available);
    }
}
