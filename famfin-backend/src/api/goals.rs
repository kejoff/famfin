use axum::{
    extract::{Path, State, Extension},
    http::StatusCode,
    Json,
};

use crate::{
    api::AppState,
    auth::middleware::AuthSession,
    db::queries,
    models::{CreateGoalRequest, Goal, UpdateGoalRequest},
};

pub async fn create_goal(
    State(state): State<AppState>,
    Path(household_id): Path<String>,
    Extension(auth): Extension<AuthSession>,
    Json(req): Json<CreateGoalRequest>,
) -> Result<(StatusCode, Json<Goal>), (StatusCode, String)> {
    if auth.household_id != household_id {
        return Err((StatusCode::FORBIDDEN, "Access denied".to_string()));
    }

    let db = state.db.lock().await;

    let goal = queries::create_goal(&db, &household_id, &req)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(goal)))
}

pub async fn get_goal(
    State(state): State<AppState>,
    Path((household_id, goal_id)): Path<(String, String)>,
    Extension(auth): Extension<AuthSession>,
) -> Result<Json<Goal>, (StatusCode, String)> {
    if auth.household_id != household_id {
        return Err((StatusCode::FORBIDDEN, "Access denied".to_string()));
    }

    let db = state.db.lock().await;

    let goal = queries::get_goal(&db, &goal_id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Goal not found".to_string()))?;

    if goal.household_id != household_id {
        return Err((StatusCode::FORBIDDEN, "Access denied".to_string()));
    }

    Ok(Json(goal))
}

pub async fn list_goals(
    State(state): State<AppState>,
    Path(household_id): Path<String>,
    Extension(auth): Extension<AuthSession>,
) -> Result<Json<Vec<Goal>>, (StatusCode, String)> {
    if auth.household_id != household_id {
        return Err((StatusCode::FORBIDDEN, "Access denied".to_string()));
    }

    let db = state.db.lock().await;

    let goals = queries::list_goals(&db, &household_id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(goals))
}

pub async fn update_goal(
    State(state): State<AppState>,
    Path((household_id, goal_id)): Path<(String, String)>,
    Extension(auth): Extension<AuthSession>,
    Json(req): Json<UpdateGoalRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    if auth.household_id != household_id {
        return Err((StatusCode::FORBIDDEN, "Access denied".to_string()));
    }

    let db = state.db.lock().await;

    let goal = queries::get_goal(&db, &goal_id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Goal not found".to_string()))?;

    if goal.household_id != household_id {
        return Err((StatusCode::FORBIDDEN, "Access denied".to_string()));
    }

    queries::update_goal(&db, &goal_id, &req)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_goal(
    State(state): State<AppState>,
    Path((household_id, goal_id)): Path<(String, String)>,
    Extension(auth): Extension<AuthSession>,
) -> Result<StatusCode, (StatusCode, String)> {
    if auth.household_id != household_id {
        return Err((StatusCode::FORBIDDEN, "Access denied".to_string()));
    }

    let db = state.db.lock().await;

    let goal = queries::get_goal(&db, &goal_id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Goal not found".to_string()))?;

    if goal.household_id != household_id {
        return Err((StatusCode::FORBIDDEN, "Access denied".to_string()));
    }

    queries::delete_goal(&db, &goal_id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
