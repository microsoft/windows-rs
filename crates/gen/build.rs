fn main() {
    let mut source : ::std::path::PathBuf = ::std::env::var("CARGO_MANIFEST_DIR").expect("No `CARGO_MANIFEST_DIR` env var").into();
    source.push(".windows");
    source.push("winmd");

    let mut destination : ::std::path::PathBuf = ::std::env::var("OUT_DIR").expect("No `OUT_DIR` env var").into();

    loop {
        destination.pop();
        destination.push("Cargo.toml");

        if destination.exists() {
            destination.pop();
            destination.push("target");
            destination.push(".windows");
            destination.push("winmd");
            break;
        }

        destination.pop();
    }

    if let ::std::result::Result::Ok(files) = ::std::fs::read_dir(source) {
        for file in files.filter_map(|file| file.ok())  {
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
