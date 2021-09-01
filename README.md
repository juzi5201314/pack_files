# pack_files
A simple proc macro. The folder is packaged during compilation and then released at runtime.

## example
```rust
// Relative to the path of `CARGO_MANIFEST_DIR`
pack_file!("./assets");
```
This will expand to:
```
{
    if !std::path::PathBuf::from("./assets/a.png").exists() {
        use std::io::Write;
        std::fs::create_dir_all("./assets").unwrap();
        let mut file = std::fs::OpenOptions::new().create(true).write(true).truncate(true).open("./assets/a.png").unwrap();
        file.write_all(&[..u8]).unwrap();
        file.sync_data().unwrap();
    }
}
{
    if !std::path::PathBuf::from("./assets/html/index.html").exists() {
        use std::io::Write;
        std::fs::create_dir_all("./assets/html").unwrap();
        let mut file = std::fs::OpenOptions::new().create(true).write(true).truncate(true).open("./assets/html/index.html").unwrap();
        file.write_all(&[..u8]).unwrap();
        file.sync_data().unwrap();
    }
}
```

## Known issues
* When the files are very large and large, packaging takes longer than expected. Maybe it can be solved in parallel or asynchronously?
