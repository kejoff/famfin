use axum::{
    extract::{Path, State, Extension},
    http::{StatusCode, HeaderMap},
    Json,
};
use tracing::{debug, error};

use crate::{
    api::AppState,
    auth,
    auth::middleware::AuthSession,
    db::queries,
    models::{CreateHouseholdResponse, LoginRequest, LoginResponse, NewHouseholdRequest},
};

/// Log error and return generic response to prevent info leakage
fn log_and_hide_error(msg: &str, err: impl std::fmt::Display) -> (StatusCode, String) {
    error!("{}: {}", msg, err);
    (StatusCode::INTERNAL_SERVER_ERROR, "Internal error".to_string())
}

pub async fn login(
    State(state): State<AppState>,
    Path(household_id): Path<String>,
    Json(req): Json<LoginRequest>,
) -> Result<(StatusCode, HeaderMap, Json<LoginResponse>), (StatusCode, String)> {
    debug!("login attempt: id='{}' len={} pw_len={}", household_id, household_id.len(), req.password.len());
    let db = state.db.lock().await;

    // Get household (generic error - don't reveal if household exists)
    let household = queries::get_household(&db, &household_id)
        .map_err(|e| {
            error!("get_household error: {}", e);
            (StatusCode::UNAUTHORIZED, "Invalid credentials".to_string())
        })?
        .ok_or_else(|| {
            debug!("login: household '{}' not found", household_id);
            (StatusCode::UNAUTHORIZED, "Invalid credentials".to_string())
        })?;

    // Verify password (constant-time)
    let password_valid = auth::verify_password(&req.password, &household.password_hash)
        .map_err(|e| log_and_hide_error("Password verification", e))?;

    if !password_valid {
        debug!("login: wrong password for '{}'", household_id);
        return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()));
    }

    let expires_at = auth::get_session_expiration();

    let session = queries::create_session(&db, &household_id, &expires_at)
        .map_err(|e| log_and_hide_error("create_session", e))?;

    // Generate HMAC-signed cookie value and set Set-Cookie header
    let signed_cookie_value = auth::generate_signed_session_cookie(&session.id);
    let cookie = auth::build_session_cookie(&signed_cookie_value);

    let mut headers = HeaderMap::new();
    headers.insert("Set-Cookie", cookie.to_string().parse().unwrap());

    Ok((
        StatusCode::OK,
        headers,
        Json(LoginResponse {
            session_id: signed_cookie_value.clone(),  // Return signed token, not session_id
            expires_at: session.expires_at,
            household_id: household_id.clone(),
        }),
    ))
}

pub async fn logout(
    State(state): State<AppState>,
    Path(household_id): Path<String>,
    Extension(auth): Extension<AuthSession>,
) -> Result<(StatusCode, HeaderMap), (StatusCode, String)> {
    if auth.household_id != household_id {
        return Err((StatusCode::FORBIDDEN, "Access denied".to_string()));
    }

    let db = state.db.lock().await;

    // Delete session from database
    queries::delete_session(&db, &auth.session_id)
        .map_err(|e| log_and_hide_error("delete_session", e))?;

    // Remove session cookie by setting Max-Age=0
    let secure = std::env::var("COOKIE_SECURE")
        .map(|v| v.to_lowercase() != "false")
        .unwrap_or(true);
    let secure_flag = if secure { "Secure; " } else { "" };
    let cookie_str = format!(
        "session=; Max-Age=0; HttpOnly; {}SameSite=Lax; Path=/",
        secure_flag
    );
    let mut headers = HeaderMap::new();
    headers.insert("Set-Cookie", cookie_str.parse().unwrap());

    Ok((StatusCode::NO_CONTENT, headers))
}

/// Get current session info (for frontend to check if logged in)
pub async fn get_me(
    Extension(auth): Extension<AuthSession>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "household_id": auth.household_id,
    }))
}

pub async fn create_household(
    State(state): State<AppState>,
    Json(req): Json<NewHouseholdRequest>,
) -> Result<(StatusCode, HeaderMap, Json<CreateHouseholdResponse>), (StatusCode, String)> {
    let db = state.db.lock().await;

    // Validate household id: alphanumeric, dash, underscore, 3-64 chars
    let household_id = req.id.trim().to_string();
    if household_id.len() < 3 || household_id.len() > 64
        || !household_id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        return Err((
            StatusCode::BAD_REQUEST,
            "Identifiant invalide (3-64 caractères, alphanumériques, - _)".to_string(),
        ));
    }

    // Hash password (validation errors are user-facing, not hidden)
    let password_hash = auth::hash_password(&req.password)
        .map_err(|e| {
            let msg = e.to_string();
            if msg.contains("Password must") {
                // Validation errors shown to user
                (StatusCode::BAD_REQUEST, msg)
            } else {
                // System errors hidden
                log_and_hide_error("Password hashing", e)
            }
        })?;

    let now = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);

    db.execute(
        "INSERT INTO households (id, name, password_hash, created_at) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![&household_id, &req.name, &password_hash, &now],
    )
    .map_err(|e| {
        if e.to_string().to_lowercase().contains("unique") {
            (StatusCode::CONFLICT, "Identifiant déjà utilisé".to_string())
        } else {
            log_and_hide_error("INSERT household", e)
        }
    })?;

    let expires_at = auth::get_session_expiration();

    let session = queries::create_session(&db, &household_id, &expires_at)
        .map_err(|e| log_and_hide_error("create_session", e))?;

    // Generate HMAC-signed cookie value and set Set-Cookie header
    let signed_cookie_value = auth::generate_signed_session_cookie(&session.id);
    let cookie = auth::build_session_cookie(&signed_cookie_value);

    let mut headers = HeaderMap::new();
    headers.insert("Set-Cookie", cookie.to_string().parse().unwrap());

    Ok((
        StatusCode::CREATED,
        headers,
        Json(CreateHouseholdResponse {
            id: household_id,
            name: req.name,
            session_id: signed_cookie_value.clone(),  // Return signed token, not session_id
            expires_at: session.expires_at,
        }),
    ))
}
