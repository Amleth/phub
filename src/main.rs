mod db;
mod file;

use crate::db::init_db;
use crate::file::{hash_file, move_file};
use std::{fs, io, path::PathBuf};

fn get_dirs() -> io::Result<(PathBuf, PathBuf, PathBuf)> {
    let documents = dirs::document_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Documents folder not found"))?;

    let app_dir = documents.join("PHUB");
    let in_dir = app_dir.join("in");
    let pdf_dir = app_dir.join("pdf");

    fs::create_dir_all(&in_dir)?;
    fs::create_dir_all(&pdf_dir)?;

    Ok((app_dir, in_dir, pdf_dir))
}

fn copy_in_pdf() -> io::Result<()> {
    let (_, in_dir, pdf_dir) = get_dirs()?;

    for entry in fs::read_dir(&in_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file()
            && path
                .extension()
                .map_or(false, |ext| ext.eq_ignore_ascii_case("pdf"))
        {
            let hash = hash_file(&path)?; // compute SHA-256
            println!("{} -> {}", path.display(), hash);

            let target_path = pdf_dir.join(format!("{}.pdf", hash));
            move_file(&path, &target_path)?; // move safely
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    copy_in_pdf()?;
    let (app_dir, _, _) = get_dirs()?;
    let db_path = app_dir.join("phub.sqlite");
    let _conn = init_db(&db_path).expect("Failed to open DB");
    Ok(())
}
