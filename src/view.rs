#[derive(Clone)]
struct PdfFile {
    title: String,
    hash: String,
}

fn list_pdfs(conn: &rusqlite::Connection) -> rusqlite::Result<Vec<PdfFile>> {
    let mut stmt = conn.prepare("SELECT hash, title FROM pdf_files")?;

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
