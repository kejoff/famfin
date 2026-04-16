use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use tracing::error;

use crate::auth::{validate_session, verify_signed_session_cookie, is_auth_disabled};
use crate::api::AppState;

/// Authenticated session info passed to handlers
#[derive(Debug, Clone)]
pub struct AuthSession {
    pub household_id: String,
    pub session_id: String,
}

/// Middleware to validate session and inject AuthSession into request extensions
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Response {
    // Check if auth is disabled (opt-out mode for development)
    if is_auth_disabled() {
        return next.run(request).await;
    }

    // Extract session token from cookie header only (not query param to avoid logging)
    let session_token = request
        .headers()
        .get("cookie")
        .and_then(|h| h.to_str().ok())
        .and_then(|cookies| {
            cookies.split(';')
                .find(|c| c.trim().starts_with("session="))
                .and_then(|c| c.trim().strip_prefix("session="))
                .map(str::to_string)
        });

    let Some(token) = session_token else {
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    };

    // All tokens must be HMAC-signed; verify and extract session_id
    let session_id = match verify_signed_session_cookie(&token) {
        Ok(id) => id,
        Err(_) => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    // Validate session in database
    let household_id = {
        let db = state.db.lock().await;
        match validate_session(&db, &session_id) {
            Ok(id) => id,
            Err(e) => {
                let err_msg = e.to_string();
                // Log actual error for debugging
                if err_msg.contains("Session not found") || err_msg.contains("Session expired") {
                    // Expected auth failures - don't log as error
                    return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
                } else {
                    // Unexpected database/system error
                    error!("Session validation failed: {}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, "Internal error").into_response();
                }
            }
        }
    };

    request.extensions_mut().insert(AuthSession {
        household_id,
        session_id,
    });

    next.run(request).await
}
