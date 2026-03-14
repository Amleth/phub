mod db;
mod file;
mod pdf;
mod pdf_slint;

use rusqlite::Connection;
use slint::{ModelRc, VecModel};

use crate::db::{census_file, init_db, list_pdf_files};
use crate::file::{hash_file, move_file};
use crate::pdf::render_pdf_page;
use crate::pdf_slint::image_to_slint;
use std::error::Error;
use std::{fs, io, path::PathBuf};

slint::include_modules!();

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

fn census_files(path: &PathBuf, conn: &Connection) -> io::Result<()> {
    for entry in fs::read_dir(&path)? {
        let entry = entry?;
        let path = entry.path();
        let hash = path.file_stem().unwrap().to_string_lossy().to_string();
        let _ = census_file(&conn, &hash);
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    copy_in_pdf()?;
    let (app_dir, _, pdf_dir) = get_dirs()?;
    let db_path = app_dir.join("phub.sqlite");
    let conn = init_db(&db_path).expect("Failed to open DB");
    let _ = census_files(&pdf_dir, &conn)?;
    let app = AppWindow::new()?;

    let img = render_pdf_page("/Users/amleth/Documents/PHUB/pdf/29a12a034dc5c9acd06dd8491eeb9cb5eba5255da81c9a56ba1dbf90ecee8634.pdf");
    let slint_img = image_to_slint(img);

    let ui_pdfs: Vec<_> = list_pdf_files(&conn)?
        .into_iter()
        .map(|p| PdfFile {
            title: p.title.into(),
            hash: p.hash.into(),
        })
        .collect();

    let model = VecModel::from(ui_pdfs);
    app.set_pdfs(ModelRc::new(model));
    app.set_pdf_page(slint_img);

    app.run()?;
    Ok(())
}
