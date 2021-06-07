use std::io::prelude::*;

fn update_dir(dir: &std::path::Path) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                update_dir(&path)?;
            } else {
                update_file(&entry)?;
            }
        }
    }

    Ok(())
}

fn update_file(entry: &std::fs::DirEntry) -> std::io::Result<()> {
    let path = entry.path();

    if let Some(extension) = path.extension() {
        if extension == "rs" {
            let mut file = std::fs::File::open(&path)?;
            let mut before = String::new();
            file.read_to_string(&mut before)?;
            drop(file);

            let macro_str = "windows::build! \x7b";
            let use_str = "use \x7b";

            let after = before.replace(macro_str, use_str);

            if before != after {
                println!("{:?}", path);
                let mut file = std::fs::File::create(&path)?;
                file.write_all(after.as_bytes())?;
                drop(file);

                let mut cmd = ::std::process::Command::new("rustfmt");
                cmd.arg(&path);
                cmd.output()?;

                let mut file = std::fs::File::open(&path)?;
                let mut before = String::new();
                file.read_to_string(&mut before)?;
                drop(file);

                let after = before.replace(use_str, macro_str);
                let mut file = std::fs::File::create(&path)?;
                file.write_all(after.as_bytes())?;
            }
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    update_dir(&windows_gen::workspace_dir())
}
