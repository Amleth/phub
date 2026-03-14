xattr -d com.apple.quarantine libpdfium.dylib
cargo clean
rm Cargo.lock
cargo build
