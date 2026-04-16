use axum::{
    extract::{Path, Query, State, Extension},
    http::StatusCode,
    Json,
};
use serde::Deserialize;

use crate::{
    api::AppState,
    auth::middleware::AuthSession,
    db::queries,
    models::{CreateTransactionRequest, Transaction, UpdateTransactionRequest},
};

#[derive(Deserialize)]
pub struct ListQuery {
    limit: Option<i32>,
    offset: Option<i32>,
}

pub async fn create_transaction(
    State(state): State<AppState>,
    Path(household_id): Path<String>,
    Extension(auth): Extension<AuthSession>,
    Json(req): Json<CreateTransactionRequest>,
) -> Result<(StatusCode, Json<Transaction>), (StatusCode, String)> {
    // Verify household ownership
    if auth.household_id != household_id {
        return Err((StatusCode::FORBIDDEN, "Access denied".to_string()));
    }

    let db = state.db.lock().await;

    let txn = queries::create_transaction(&db, &household_id, &req)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(txn)))
}

pub async fn get_transaction(
    State(state): State<AppState>,
    Path((household_id, transaction_id)): Path<(String, String)>,
    Extension(auth): Extension<AuthSession>,
) -> Result<Json<Transaction>, (StatusCode, String)> {
    if auth.household_id != household_id {
        return Err((StatusCode::FORBIDDEN, "Access denied".to_string()));
    }

    let db = state.db.lock().await;

    let txn = queries::get_transaction(&db, &transaction_id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Transaction not found".to_string()))?;

    if txn.household_id != household_id {
        return Err((StatusCode::FORBIDDEN, "Access denied".to_string()));
    }

    Ok(Json(txn))
}

pub async fn list_transactions(
    State(state): State<AppState>,
    Path(household_id): Path<String>,
    Extension(auth): Extension<AuthSession>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Vec<Transaction>>, (StatusCode, String)> {
    if auth.household_id != household_id {
        return Err((StatusCode::FORBIDDEN, "Access denied".to_string()));
    }

    let db = state.db.lock().await;

    let limit = query.limit.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);

    let txns = queries::list_transactions(&db, &household_id, limit, offset)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(txns))
}

pub async fn update_transaction(
    State(state): State<AppState>,
    Path((household_id, transaction_id)): Path<(String, String)>,
    Extension(auth): Extension<AuthSession>,
    Json(req): Json<UpdateTransactionRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    if auth.household_id != household_id {
        return Err((StatusCode::FORBIDDEN, "Access denied".to_string()));
    }

    let db = state.db.lock().await;

    // Verify ownership
    let txn = queries::get_transaction(&db, &transaction_id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Transaction not found".to_string()))?;

    if txn.household_id != household_id {
        return Err((StatusCode::FORBIDDEN, "Access denied".to_string()));
    }

    queries::update_transaction(&db, &transaction_id, &req)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
