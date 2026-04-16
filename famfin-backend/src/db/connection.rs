use rusqlite::Connection;
use anyhow::Result;
use tracing::info;

refinery::embed_migrations!("migrations");

/// Initialize database connection with SQLCipher encryption and migrations
///
/// CRITICAL: PRAGMA key must be set BEFORE any SQL execution, including FOREIGN_KEYS.
/// This function:
/// 1. Opens database connection
/// 2. Sets SQLCipher encryption key (PRAGMA key)
/// 3. Enables foreign keys
/// 4. Runs migrations with timeout enforcement
///
/// Migrations must complete within 30 seconds (systemd StartupTimeoutSec=30).
pub fn init_db(db_path: &str) -> Result<Connection> {
    let mut conn = Connection::open(db_path)?;

    // Get CIPHER_KEY from environment (required, no fallback)
    let cipher_key = std::env::var("CIPHER_KEY")
        .unwrap_or_else(|_| {
            panic!("FATAL: CIPHER_KEY environment variable not set. Required for SQLCipher.");
        });

    // Set SQLCipher encryption key using pragma_update (prevents SQL injection)
    // PRAGMA key = ... must be set BEFORE any other SQL
    conn.pragma_update(None, "key", &cipher_key)?;

    // Enable foreign keys (after encryption key is set)
    conn.execute("PRAGMA foreign_keys = ON", [])?;

    // Run migrations with timeout enforcement
    run_migrations_with_timeout(&mut conn)?;

    Ok(conn)
}

/// Run migrations with startup timeout enforcement
///
/// Migrations must complete within ~30 seconds (systemd StartupTimeoutSec enforces this).
/// If a migration hangs for > 30s, the process will be killed by systemd (non-zero exit).
/// This prevents infinite restart loops when migrations are broken.
fn run_migrations_with_timeout(conn: &mut Connection) -> Result<()> {
    let start = std::time::Instant::now();
    migrations::runner().run(conn)?;
    let elapsed = start.elapsed().as_secs();

    info!("Migrations completed in {}s", elapsed);

    // Warn if approaching the 30s systemd timeout
    if elapsed > 20 {
        eprintln!(
            "⚠️  WARNING: Migrations took {}s — approaching 30s systemd timeout",
            elapsed
        );
    }

    Ok(())
}

/// Initialize in-memory test database (no encryption)
///
/// Test database uses in-memory SQLite which:
/// - Does not require encryption key
/// - Is automatically cleaned up after each test
/// - Provides isolation between tests
///
/// This is correct behavior: tests should be fast and not require key management.
pub fn init_test_db() -> Result<Connection> {
    let mut conn = Connection::open_in_memory()?;

    // Enable foreign keys for test database
    conn.execute("PRAGMA foreign_keys = ON", [])?;

    // Run migrations (no encryption key needed for in-memory DB)
    migrations::runner().run(&mut conn)?;

    Ok(conn)
}
