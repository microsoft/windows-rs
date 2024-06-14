use windows_bindgen::*;

fn main() -> Result<()> {
    for path in &bindings() {
        println!("bindings: {path}");
        bindgen(["--etc", path])?;
    }
    Ok(())
}

fn bindings() -> Vec<String> {
    let mut paths = vec![];

    if let Ok(files) = std::fs::read_dir("crates\\tools\\bindings\\src") {
        for file in files.filter_map(|file| file.ok()) {
            let path = file.path();
            if path.to_string_lossy().ends_with(".txt") {
                paths.push(path.to_string_lossy().into_owned());
            }
        }
    }

    paths
}
