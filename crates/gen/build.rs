#[cfg(windows)]
fn main() {
    let mut source: ::std::path::PathBuf = ::std::env::var("CARGO_MANIFEST_DIR")
        .expect("No `CARGO_MANIFEST_DIR` env var")
        .into();

    source.push(".windows");
    source.push("winmd");

    let destination = std::env::var("PATH").expect("No `PATH` env variable set");
    let end = destination.find(';').expect("Path not ending in `;`");
    let mut destination: std::path::PathBuf = destination[..end].into();
    destination.pop();
    destination.pop();
    destination.push(".windows");
    destination.push("winmd");

    if let ::std::result::Result::Ok(files) = ::std::fs::read_dir(source) {
        for file in files.filter_map(|file| file.ok()) {
            if let ::std::result::Result::Ok(file_type) = file.file_type() {
                if file_type.is_file() {
                    let path = file.path();
                    if let ::std::option::Option::Some(filename) = path.file_name() {
                        let _ = std::fs::create_dir_all(&destination);
                        destination.push(filename);
                        let _ = ::std::fs::copy(path, &destination);
                        destination.pop();
                    }
                }
            }
        }
    }
}

#[cfg(not(windows))]
fn main() {}
