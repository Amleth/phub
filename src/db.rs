use std::path::Path;

use rusqlite::{params, Connection};

pub fn init_db(db_path: &Path) -> rusqlite::Result<Connection> {
    let conn = Connection::open(db_path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS pdf_files (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            hash TEXT NOT NULL,
            title TEXT NOT NULL
        )",
        [],
    )?;
    Ok(conn)
}
