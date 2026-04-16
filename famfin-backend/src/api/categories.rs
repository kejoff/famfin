use axum::{
    extract::{Path, State, Extension},
    http::StatusCode,
    Json,
};

use crate::{
    api::AppState,
    auth::middleware::AuthSession,
    db::queries,
    models::{CreateCategoryRequest, TransactionCategory},
};

pub async fn create_category(
    State(state): State<AppState>,
    Path(household_id): Path<String>,
    Extension(auth): Extension<AuthSession>,
    Json(req): Json<CreateCategoryRequest>,
) -> Result<(StatusCode, Json<TransactionCategory>), (StatusCode, String)> {
    if auth.household_id != household_id {
        return Err((StatusCode::FORBIDDEN, "Access denied".to_string()));
    }

    let db = state.db.lock().await;

    let cat = queries::create_category(&db, &household_id, &req)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(cat)))
}

pub async fn list_categories(
    State(state): State<AppState>,
    Path(household_id): Path<String>,
    Extension(auth): Extension<AuthSession>,
) -> Result<Json<Vec<TransactionCategory>>, (StatusCode, String)> {
    if auth.household_id != household_id {
        return Err((StatusCode::FORBIDDEN, "Access denied".to_string()));
    }

    let db = state.db.lock().await;

    let cats = queries::list_categories(&db, &household_id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(cats))
}

pub async fn get_category(
    State(state): State<AppState>,
    Path((household_id, category_id)): Path<(String, String)>,
    Extension(auth): Extension<AuthSession>,
) -> Result<Json<TransactionCategory>, (StatusCode, String)> {
    if auth.household_id != household_id {
        return Err((StatusCode::FORBIDDEN, "Access denied".to_string()));
    }

    let db = state.db.lock().await;

    let cat = queries::get_category(&db, &category_id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Category not found".to_string()))?;

    if cat.household_id != household_id {
        return Err((StatusCode::FORBIDDEN, "Access denied".to_string()));
    }

    Ok(Json(cat))
}
