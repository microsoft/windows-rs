fn main() -> std::io::Result<()> {
    for entry in std::fs::read_dir("crates/libs")? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let path = entry.path();
            if path.join("Cargo.toml").exists() {
                println!("{}", path.to_str().expect("path").replace('\\', "/"));
                std::fs::copy("license-mit", path.join("license-mit"))?;
                std::fs::copy("license-apache-2.0", path.join("license-apache-2.0"))?;
            }
        }
    }

    Ok(())
}
