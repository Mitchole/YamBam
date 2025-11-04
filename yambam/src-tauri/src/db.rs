use rusqlite::{Connection, Result};
use std::path::PathBuf;

/// Initialize a database connection
/// For Phase 1, we just test connectivity
pub fn init_db(db_path: PathBuf) -> Result<Connection> {
    let conn = Connection::open(db_path)?;

    // Test query
    conn.execute(
        "CREATE TABLE IF NOT EXISTS test (
            id INTEGER PRIMARY KEY,
            message TEXT NOT NULL
        )",
        [],
    )?;

    Ok(conn)
}

/// Test function to verify database works
pub fn test_db_connection() -> Result<String> {
    // Use in-memory database for testing
    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE test (id INTEGER PRIMARY KEY, message TEXT)",
        [],
    )?;

    conn.execute(
        "INSERT INTO test (message) VALUES (?1)",
        ["Hello from SQLite!"],
    )?;

    let message: String = conn.query_row(
        "SELECT message FROM test WHERE id = 1",
        [],
        |row| row.get(0),
    )?;

    Ok(message)
}
