use axum::{
    extract::{Path, State, Extension, Query},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use chrono::Datelike;

use crate::{
    api::AppState,
    auth::middleware::AuthSession,
    db::queries,
    models::{MonthlySpending, CategorySpending},
};

#[derive(Deserialize)]
pub struct DashboardQuery {
    year: Option<i32>,
    month: Option<u32>,
}

pub async fn get_monthly_dashboard(
    State(state): State<AppState>,
    Path(household_id): Path<String>,
    Extension(auth): Extension<AuthSession>,
    Query(q): Query<DashboardQuery>,
) -> Result<Json<MonthlySpending>, (StatusCode, String)> {
    if auth.household_id != household_id {
        return Err((StatusCode::FORBIDDEN, "Access denied".to_string()));
    }

    let now = chrono::Local::now();
    let year = q.year.unwrap_or(now.year());
    let month = q.month.unwrap_or(now.month());

    let db = state.db.lock().await;

    // Get total monthly spending
    let total_spending = queries::get_monthly_spending(&db, &household_id, year, month)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Get category breakdown
    let breakdown = queries::get_category_breakdown(&db, &household_id, year, month)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Convert to percentage and create response
    let category_breakdown: Vec<CategorySpending> = breakdown
        .into_iter()
        .map(|(cat_id, cat_name, amount)| {
            let percentage = if total_spending > 0.0 {
                (amount / total_spending) * 100.0
            } else {
                0.0
            };
            CategorySpending {
                category_id: cat_id,
                category_name: cat_name,
                amount,
                percentage,
            }
        })
        .collect();

    Ok(Json(MonthlySpending {
        month: format!("{:04}-{:02}", year, month),
        total_spending,
        category_breakdown,
    }))
}

pub async fn get_spending_trends(
    State(state): State<AppState>,
    Path(household_id): Path<String>,
    Extension(auth): Extension<AuthSession>,
) -> Result<Json<Vec<(String, f64)>>, (StatusCode, String)> {
    if auth.household_id != household_id {
        return Err((StatusCode::FORBIDDEN, "Access denied".to_string()));
    }

    let db = state.db.lock().await;

    // Get last 12 months of spending
    let mut stmt = db
        .prepare(
            "SELECT strftime('%Y-%m', date) as month, COALESCE(SUM(ABS(amount)), 0) as total
             FROM transactions
             WHERE household_id = ?1 AND amount < 0
             AND date >= date('now', '-12 months')
             GROUP BY month
             ORDER BY month DESC
             LIMIT 12"
        )
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let trends = stmt
        .query_map([&household_id], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, f64>(1)?))
        })
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(trends))
}

pub async fn get_goals_progress(
    State(state): State<AppState>,
    Path(household_id): Path<String>,
    Extension(auth): Extension<AuthSession>,
) -> Result<Json<Vec<(String, f64, f64, f64)>>, (StatusCode, String)> {
    if auth.household_id != household_id {
        return Err((StatusCode::FORBIDDEN, "Access denied".to_string()));
    }

    let db = state.db.lock().await;

    // Get all goals with progress
    let mut stmt = db
        .prepare(
            "SELECT id, name, target_amount, current_amount
             FROM goals
             WHERE household_id = ?1
             ORDER BY deadline"
        )
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let goals = stmt
        .query_map([&household_id], |row| {
            let target: f64 = row.get(2)?;
            let current: f64 = row.get(3)?;
            let progress = if target > 0.0 {
                (current / target) * 100.0
            } else {
                0.0
            };
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?, current, progress))
        })
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Convert to simpler format for response
    let result: Vec<(String, f64, f64, f64)> = goals
        .into_iter()
        .map(|(_id, name, current, progress)| (name, current, progress, 0.0))
        .collect();

    Ok(Json(result))
}
