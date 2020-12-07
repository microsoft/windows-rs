use lazy_static::lazy_static;

/// Returns the build's `.windows` directory in the root of the workspace as a `PathBuf`.
pub fn build_windows_dir() -> std::path::PathBuf {
    let mut path = WORKSPACE_DIR.clone();
    path.push(".windows");
    path
}

/// Returns the build's target directory in the passed workspace directory as a `PathBuf`.
pub fn workspace_target_dir() -> std::path::PathBuf {
    let mut path = WORKSPACE_DIR.clone();
    path.push("target");
    path
}

lazy_static! {
    static ref WORKSPACE_DIR: std::path::PathBuf = workspace_dir();
}

fn workspace_dir() -> std::path::PathBuf {
    // This is all rather complicated and expensive because Cargo lacks an environment variable for the workspace dir.

    let output = std::process::Command::new(env!("CARGO"))
        .arg("metadata")
        .arg("--format-version=1")
        .output()
        .expect("Failed to run `cargo metadata`");

    let manifest: Manifest =
        serde_json::from_slice(&output.stdout).expect("Failed to parse `cargo metadata`");

    manifest.workspace_root.into()
}

#[derive(serde::Deserialize)]
struct Manifest {
    workspace_root: String,
}
