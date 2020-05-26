use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// Returns the paths to resolved dependencies
pub fn expand_paths<P: AsRef<Path>>(dependency: P, result: &mut BTreeSet<PathBuf>, recurse: bool) {
    let path = dependency.as_ref();

    if path.is_dir() {
        let paths = std::fs::read_dir(path).unwrap_or_else(|e| {
            panic!(
                "Could not read dependecy directory at path {:?}: {}",
                path, e
            )
        });
        for path in paths {
            let path = path.expect("Could not read directory entry");
            let path = path.path();
            if path.is_file() && path.extension() == Some(std::ffi::OsStr::new("winmd")) {
                result.insert(path);
            } else if path.is_dir() && recurse {
                expand_paths(path, result, recurse)
            }
        }
    } else if path.is_file() && path.extension() == Some(std::ffi::OsStr::new("winmd")) {
        result.insert(path.to_path_buf());
    } else {
        panic!("Dependency {} is not a file or directory", path.display());
    }
}

pub fn nuget_root() -> PathBuf {
    let mut path = workspace_root();

    path.push("target");
    path.push("nuget");
    path
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
