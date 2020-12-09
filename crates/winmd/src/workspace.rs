/// Returns the build's `.windows` directory in the root of the workspace as a `PathBuf`.
pub fn workspace_windows_dir() -> std::path::PathBuf {
    let mut path = workspace_dir();
    path.push(".windows");
    path
}

fn workspace_dir() -> std::path::PathBuf {
    use std::{mem::MaybeUninit, sync::Once};
    static ONCE: Once = Once::new();
    static mut VALUE: MaybeUninit<std::path::PathBuf> = MaybeUninit::uninit();

    ONCE.call_once(|| {
        let output = std::process::Command::new(env!("CARGO"))
            .arg("metadata")
            .arg("--format-version=1")
            .output()
            .expect("Failed to run `cargo metadata`");

        let manifest: Manifest =
            serde_json::from_slice(&output.stdout).expect("Failed to parse `cargo metadata`");

        // This is safe because `Once` provides thread-safe one-time initialization
        unsafe { VALUE = MaybeUninit::new(manifest.workspace_root.into()) }
    });

    // This is safe because `call_once` has already been called.
    unsafe { (*VALUE.as_ptr()).clone() }
}

#[derive(serde::Deserialize)]
struct Manifest {
    workspace_root: String,
}
