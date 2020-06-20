use std::collections::BTreeSet;
use std::io;
use std::path::{Path, PathBuf};

/// Returns the paths to resolved dependencies
pub fn expand_paths<P: AsRef<Path>>(
    dependency: P,
    result: &mut BTreeSet<PathBuf>,
    recurse: bool,
) -> io::Result<()> {
    let path = dependency.as_ref();

    if path.is_dir() {
        let paths = std::fs::read_dir(path)?;
        for path in paths {
            let path = path.expect("Could not read directory entry");
            let path = path.path();
            if path.is_file() && path.extension() == Some(std::ffi::OsStr::new("winmd")) {
                result.insert(path);
            } else if path.is_dir() && recurse {
                expand_paths(path, result, recurse)?
            }
        }
        Ok(())
    } else if path.is_file() && path.extension() == Some(std::ffi::OsStr::new("winmd")) {
        result.insert(path.to_path_buf());
        Ok(())
    } else if !path.exists() {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("dependency path '{}' does not exist", path.display()),
        ))
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "dependency path '{}' is not a directory or winmd file",
                path.display()
            ),
        ))
    }
}

pub fn nuget_root() -> PathBuf {
    let mut path = workspace_root();
    path.push("target");
    path.push("nuget");
    path
}

pub fn system_metadata_root() -> Option<PathBuf> {
    let wind_dir_env = std::env::var("windir").ok()?;
    let mut path = PathBuf::from(wind_dir_env);
    path.push(SYSTEM32);
    path.push("winmetadata");

    Some(path)
}

fn workspace_root() -> PathBuf {
    let output = std::process::Command::new("cargo")
        .args(&["metadata"])
        .output()
        .expect("Could not run `cargo metadata`")
        .stdout;
    let value: serde_json::Map<String, serde_json::Value> =
        serde_json::from_slice(&output).expect("`cargo metadata` did not return json.");
    let path = match value.get("workspace_root") {
        Some(serde_json::Value::String(s)) => s,
        _ => panic!("`cargo metadata` json was not in expected format"),
    };
    PathBuf::from(path)
}

#[cfg(target_pointer_width = "64")]
const SYSTEM32: &str = "System32";

#[cfg(target_pointer_width = "32")]
const SYSTEM32: &str = "SysNative";
