fn main() {
    if !cfg!(target_env = "msvc") {
        return;
    }

    if let Ok(files) = std::fs::read_dir("tests") {
        for file in files.filter_map(|file| file.ok()) {
            if let Ok(file_type) = file.file_type() {
                if file_type.is_file() {
                    let path = file.path();
                    if path.extension().is_some_and(|extension| extension == "cpp") {
                        compile(&path.to_string_lossy());
                    }
                }
            }
        }
    }
}

fn compile(path: &str) {
    cc::Build::new()
        .cpp(true)
        .std("c++20")
        .flag("/EHsc")
        .file(path)
        .compile("test");
}
