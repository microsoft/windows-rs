use std::io::prelude::*;

fn update_dir(dir: &std::path::Path, set: bool) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                update_dir(&path, set)?;
            } else {
                update_file(&entry, set)?;
            }
        }
    }

    Ok(())
}

fn update_file(entry: &std::fs::DirEntry, set: bool) -> std::io::Result<()> {
    let path = entry.path();

    if let Some(extension) = path.extension() {
        if extension == "rs" {
            println!("{:?}", path);
            let mut file = std::fs::File::open(&path)?;
            let mut data = String::new();
            file.read_to_string(&mut data)?;
            drop(file);

            let macro_str = "windows::build! \x7b";
            let use_str = "use \x7b // fmt";

            let data = if set { 
                data.replace(macro_str, use_str);
            } else {
                data.replace(use_str, macro_str)
            };

            let mut file = std::fs::File::create(&path)?;
            file.write_all(data.as_bytes())?;
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let workspace = windows_gen::workspace_dir();
   // update_dir(&workspace, true)?;
    update_dir(&workspace, false)?;

    Ok(())
}
