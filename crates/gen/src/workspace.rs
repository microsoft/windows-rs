use super::*;

/// Returns the build's `.windows` directory in the root of the workspace as a `PathBuf`.
pub fn workspace_windows_dir() -> std::path::PathBuf {
    let mut path = workspace_dir();
    path.push(".windows");
    path
}

pub fn workspace_winmds() -> &'static [File] {
    use std::{mem::MaybeUninit, sync::Once};
    static ONCE: Once = Once::new();
    static mut VALUE: MaybeUninit<Vec<File>> = MaybeUninit::uninit();

    ONCE.call_once(|| {
        // This is safe because `Once` provides thread-safe one-time initialization
        unsafe { VALUE = MaybeUninit::new(get_workspace_winmds()) }
    });

    // This is safe because `call_once` has already been called.
    unsafe { &*VALUE.as_ptr() }
}

pub fn workspace_dir() -> std::path::PathBuf {
    use std::{mem::MaybeUninit, sync::Once};
    static ONCE: Once = Once::new();
    static mut VALUE: MaybeUninit<std::path::PathBuf> = MaybeUninit::uninit();

    // TODO: calling `cargo metadata` just to get the workspace dir takes about 40ms.

    ONCE.call_once(|| {
        let output = std::process::Command::new(env!("CARGO"))
            .arg("metadata")
            .arg("--format-version=1")
            .arg("--no-deps")
            .arg("--offline")
            .output()
            .expect("Failed to run `cargo metadata`");

        const JSON_KEY: &str = r#""workspace_root":"#;
        let json = String::from_utf8(output.stdout).expect("Cargo metadata is not utf-8");
        let beginning_index = json
            .rfind(JSON_KEY)
            .expect("Cargo metadata did not contain `workspace_root` key.")
            + JSON_KEY.len()
            + 1;

        let ending_index = json[beginning_index..]
            .find('"')
            .expect("Cargo metadata ended before closing '\"' in `workspace_root` value");

        let workspace_root =
            json[beginning_index..beginning_index + ending_index].replace("\\\\", "\\");

        // This is safe because `Once` provides thread-safe one-time initialization
        unsafe { VALUE = MaybeUninit::new(workspace_root.into()) }
    });

    // This is safe because `call_once` has already been called.
    unsafe { (*VALUE.as_ptr()).clone() }
}

fn get_workspace_winmds() -> Vec<File> {
    let mut windows_path = workspace_windows_dir();
    windows_path.push("winmd");

    let mut result = vec![];

    if let Ok(files) = std::fs::read_dir(windows_path) {
        for file in files.filter_map(|file| file.ok()) {
            if let Ok(file_type) = file.file_type() {
                if file_type.is_file() {
                    let path = file.path();
                    if let Some("winmd") = path.extension().and_then(|extension| extension.to_str())
                    {
                        result.push(File::new(path));
                    }
                }
            }
        }
    }

    // TODO: include_bytes is very slow - it takes an extra 60ms compared with memory mapped files.
    // https://github.com/rust-lang/rust/issues/65818

    if !result.iter().any(|file| file.name.starts_with("Windows.")) {
        result.push(File::from_bytes(
            "Windows.Win32.winmd".to_string(),
            include_bytes!("../default/Windows.Win32.winmd").to_vec(),
        ));
        result.push(File::from_bytes(
            "Windows.WinRT.winmd".to_string(),
            include_bytes!("../default/Windows.WinRT.winmd").to_vec(),
        ));
    }

    result
}
