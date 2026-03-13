use rusqlite::{params, Connection};
use std::path::Path;

pub fn init_db(db_path: &Path) -> rusqlite::Result<Connection> {
    let conn = Connection::open(db_path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS pdf_files (
            hash TEXT NOT NULL PRIMARY KEY,
            title TEXT NOT NULL DEFAULT 'Untitled'
        )",
        [],
    )?;
    Ok(conn)
}

pub fn census_file(conn: &Connection, hash: &String) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO pdf_files (hash) VALUES (?1) ON CONFLICT(hash) DO NOTHING",
        params![hash],
    )
    .expect("Failed to census PDF file");
    Ok(())
}

#[derive(Clone)]
pub struct PdfFile {
    pub title: String,
    pub hash: String,
}

pub fn list_pdf_files(conn: &rusqlite::Connection) -> rusqlite::Result<Vec<PdfFile>> {
    let mut stmt = conn.prepare("SELECT title, hash FROM pdf_files")?;

    let rows = stmt.query_map([], |row| {
        Ok(PdfFile {
            title: row.get(0)?,
            hash: row.get(1)?,
        })
    })?;

    let mut pdfs = Vec::new();

    for pdf in rows {
        pdfs.push(pdf?);
    }

    Ok(pdfs)
}
