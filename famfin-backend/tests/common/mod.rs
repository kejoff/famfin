use rusqlite::Connection;
use anyhow::Result;

refinery::embed_migrations!("migrations");

/// Initialize test database with full schema via migrations
pub fn init_test_db() -> Result<Connection> {
    let mut conn = Connection::open_in_memory()?;

    // Enable foreign keys for test database
    conn.execute("PRAGMA foreign_keys = ON", [])?;

    // Run migrations
    migrations::runner().run(&mut conn)?;

    Ok(conn)
}
