/// Returns the build's `.windows` directory in the root of the workspace as a `PathBuf`.
pub fn build_windows_dir() -> std::path::PathBuf {
    build_windows_dir_at_workspace(&workspace_dir())
}

/// Returns the build's `.windows` directory in the passed workspace directory as a `PathBuf`.
pub fn build_windows_dir_at_workspace(workspace_dir: &std::path::PathBuf) -> std::path::PathBuf {
    let mut path = workspace_dir.clone();
    path.push(".windows");
    path
}

/// Returns the build's target directory in the passed workspace directory as a `PathBuf`.
pub fn build_target_dir_at_workspace(workspace_dir: &std::path::PathBuf) -> std::path::PathBuf {
    let mut path = workspace_dir.clone();
    path.push("target");
    path
}

/// Returns the build's workspace directory. Executes 'cargo' command to get it because Cargo
/// lacks an environment variable for the workspace dir.
pub fn workspace_dir() -> std::path::PathBuf {
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
