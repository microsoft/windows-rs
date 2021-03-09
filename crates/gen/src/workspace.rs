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
            .arg("--no-deps")
            .output()
            .expect("Failed to run `cargo metadata`");

        const JSON_KEY: &str = r#""workspace_root":"#;
        let json = String::from_utf8(output.stdout).expect("Cargo metadata is not utf-8");
        let beginning_index = json
            .find(JSON_KEY)
            .expect("Cargo metadata did not contain `workspace_root` key.")
            + JSON_KEY.len()
            + 1;

        let ending_index = json[beginning_index..]
            .find("\"")
            .expect("Cargo metadata ended before closing '\"' in `workspace_root` value");

        let workspace_root = &json[beginning_index..beginning_index + ending_index];

        // This is safe because `Once` provides thread-safe one-time initialization
        unsafe { VALUE = MaybeUninit::new(workspace_root.into()) }
    });

    // This is safe because `call_once` has already been called.
    unsafe { (*VALUE.as_ptr()).clone() }
}
