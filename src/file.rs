use sha2::{Digest, Sha256};
use std::fs;
use std::io::BufReader;
use std::io::{self};
use std::path::Path;

pub fn hash_file(path: &Path) -> io::Result<String> {
    let file = fs::File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();

    io::copy(&mut reader, &mut hasher)?;

    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

pub fn move_file(src: &Path, dst: &Path) -> io::Result<()> {
    match fs::rename(src, dst) {
        Ok(_) => Ok(()),
        Err(_) => {
            // fallback for cross-drive
            fs::copy(src, dst)?;
            fs::remove_file(src)?;
            Ok(())
        }
    }
}
