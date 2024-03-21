use rusqlite::{Connection, Result};

pub fn connect() -> Result<Connection> {
    let conn = Connection::open("writing.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS samples (
            id        INTEGER PRIMARY KEY,
            title     TEXT NOT NULL,
            content   TEXT NOT NULL,
            description   TEXT NOT NULL
        )",
        [] // No parameters for this SQL statement
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS contact_requests (
            id        INTEGER PRIMARY KEY,
            name     TEXT NOT NULL,
            email   TEXT NOT NULL,
            message   TEXT NOT NULL
        )",
        [] // No parameters for this SQL statement
    )?;

    Ok(conn)
}