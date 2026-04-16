pub mod middleware;

use anyhow::Result;
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, SaltString},
    Argon2, PasswordVerifier,
};
use base64::Engine;
use hmac::{Hmac, Mac};
use rand::rngs::OsRng;
use rusqlite::Connection;
use sha2::Sha256;
use cookie::Cookie;

/// Validate password strength (min 12 chars, must have upper/lower/digit)
pub fn validate_password_strength(password: &str) -> Result<()> {
    if password.len() < 12 {
        return Err(anyhow::anyhow!("Password must be at least 12 characters"));
    }

    let has_upper = password.chars().any(|c| c.is_uppercase());
    let has_lower = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_numeric());

    if !has_upper || !has_lower || !has_digit {
        return Err(anyhow::anyhow!(
            "Password must contain uppercase, lowercase, and digits"
        ));
    }

    Ok(())
}

/// Hash a plaintext password using Argon2
pub fn hash_password(password: &str) -> Result<String> {
    validate_password_strength(password)?;

    let salt = SaltString::generate(OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!("Failed to hash password: {}", e))?
        .to_string();
    Ok(password_hash)
}

/// Verify a plaintext password against a hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| anyhow::anyhow!("Invalid password hash: {}", e))?;

    let argon2 = Argon2::default();
    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(()) => Ok(true),
        Err(argon2::password_hash::Error::Password) => Ok(false),
        Err(e) => Err(anyhow::anyhow!("Password verification error: {}", e)),
    }
}

/// Generate a session cookie value (simple approach: session ID from UUID)
pub fn generate_session_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// Session expiration: 8 hours from now
pub fn get_session_expiration() -> String {
    let expires_at = chrono::Utc::now() + chrono::Duration::hours(8);
    expires_at.to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
}

/// Validate a session ID and return the household_id if valid
/// Returns specific error messages: "Session not found", "Session expired", or other errors
pub fn validate_session(conn: &Connection, session_id: &str) -> Result<String> {
    use crate::db::queries;

    let session = queries::get_session(conn, session_id)?
        .ok_or_else(|| anyhow::anyhow!("Session not found"))?;

    // Check if session has expired
    let now = chrono::Utc::now();
    let expires_at = chrono::DateTime::parse_from_rfc3339(&session.expires_at)
        .map_err(|e| anyhow::anyhow!("Invalid session expiration time: {}", e))?;

    if now > expires_at {
        return Err(anyhow::anyhow!("Session expired"));
    }

    Ok(session.household_id)
}

/// Helper to validate session from request and verify household ownership
pub fn verify_session_ownership(
    conn: &Connection,
    session_token: &str,
    household_id: &str,
) -> Result<()> {
    let authenticated_household = validate_session(conn, session_token)?;

    if authenticated_household != household_id {
        return Err(anyhow::anyhow!("Unauthorized: household mismatch"));
    }

    Ok(())
}

/// Get session HMAC key from environment (required)
/// Panics if not set (no development fallback)
fn get_session_hmac_key() -> Vec<u8> {
    std::env::var("SESSION_HMAC_KEY")
        .unwrap_or_else(|_| {
            panic!("FATAL: SESSION_HMAC_KEY environment variable not set. Set to a 32+ byte random key.");
        })
        .as_bytes()
        .to_vec()
}

/// Generate HMAC-signed session cookie value
/// Format: {session_id}.{base64_hmac}
pub fn generate_signed_session_cookie(session_id: &str) -> String {
    type HmacSha256 = Hmac<Sha256>;

    let key = get_session_hmac_key();
    let mut mac = HmacSha256::new_from_slice(&key)
        .expect("HMAC key size valid");
    mac.update(session_id.as_bytes());
    let signature = base64::engine::general_purpose::STANDARD
        .encode(mac.finalize().into_bytes());
    format!("{}.{}", session_id, signature)
}

/// Verify HMAC-signed session ID
/// Returns Ok(session_id) if valid, Err if invalid/tampered
pub fn verify_signed_session_cookie(cookie_value: &str) -> Result<String> {
    type HmacSha256 = Hmac<Sha256>;

    let parts: Vec<&str> = cookie_value.split('.').collect();
    if parts.len() != 2 {
        return Err(anyhow::anyhow!("Invalid session cookie format"));
    }

    let session_id = parts[0];
    let provided_signature = parts[1];

    // Validate session_id is a valid UUID
    if !is_valid_uuid(session_id) {
        return Err(anyhow::anyhow!("Invalid session ID format"));
    }

    // Decode and verify HMAC
    let provided_bytes = base64::engine::general_purpose::STANDARD
        .decode(provided_signature)
        .map_err(|_| anyhow::anyhow!("Invalid signature encoding"))?;

    let key = get_session_hmac_key();
    let mut mac = HmacSha256::new_from_slice(&key)
        .expect("HMAC key size valid");
    mac.update(session_id.as_bytes());

    // Constant-time comparison
    if constant_time_compare(&provided_bytes, &mac.finalize().into_bytes()) {
        Ok(session_id.to_string())
    } else {
        Err(anyhow::anyhow!("Invalid session signature"))
    }
}

/// Validate UUID format (8-4-4-4-12 hex digits separated by dashes)
fn is_valid_uuid(s: &str) -> bool {
    if s.len() != 36 {
        return false;
    }
    let parts: Vec<&str> = s.split('-').collect();
    parts.len() == 5
        && parts[0].len() == 8 && parts[0].chars().all(|c| c.is_ascii_hexdigit())
        && parts[1].len() == 4 && parts[1].chars().all(|c| c.is_ascii_hexdigit())
        && parts[2].len() == 4 && parts[2].chars().all(|c| c.is_ascii_hexdigit())
        && parts[3].len() == 4 && parts[3].chars().all(|c| c.is_ascii_hexdigit())
        && parts[4].len() == 12 && parts[4].chars().all(|c| c.is_ascii_hexdigit())
}

/// Constant-time byte comparison (prevents timing attacks)
fn constant_time_compare(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut result = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    result == 0
}

/// Build httpOnly session cookie with security flags
/// Flags:
/// - HttpOnly: prevents JavaScript access (XSS protection)
/// - Secure: only on HTTPS (prod); disabled on HTTP dev via COOKIE_SECURE=false
/// - SameSite=Lax: CSRF protection while allowing proxied dev server
/// - Max-Age=28800: 8 hour session timeout
pub fn build_session_cookie(signed_value: &str) -> Cookie<'static> {
    let secure = std::env::var("COOKIE_SECURE")
        .map(|v| v.to_lowercase() != "false")
        .unwrap_or(true);

    let mut cookie = Cookie::new("session", signed_value.to_string());
    cookie.set_http_only(true);
    cookie.set_secure(secure);
    cookie.set_same_site(cookie::SameSite::Lax);
    cookie.set_max_age(cookie::time::Duration::hours(8));
    cookie.set_path("/");
    cookie
}

/// Check if session authentication is disabled (debug mode only)
#[cfg(debug_assertions)]
pub fn is_auth_disabled() -> bool {
    std::env::var("AUTH_DISABLED")
        .map(|v| v.to_lowercase() == "true")
        .unwrap_or(false)
}

#[cfg(not(debug_assertions))]
pub fn is_auth_disabled() -> bool {
    false
}
