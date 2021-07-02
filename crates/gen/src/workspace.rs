use super::*;

pub fn crate_winmds() -> &'static [File] {
    use std::{mem::MaybeUninit, sync::Once};
    static ONCE: Once = Once::new();
    static mut VALUE: MaybeUninit<Vec<File>> = MaybeUninit::uninit();

    ONCE.call_once(|| {
        // This is safe because `Once` provides thread-safe one-time initialization
        unsafe { VALUE = MaybeUninit::new(get_crate_winmds()) }
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

fn get_crate_winmds() -> Vec<File> {
    fn push_dir(result: &mut Vec<File>, dir: &std::path::PathBuf) {
        if let Ok(files) = std::fs::read_dir(&dir) {
            for file in files.filter_map(|file| file.ok()) {
                if let Ok(file_type) = file.file_type() {
                    if file_type.is_file() {
                        let path = file.path();
                        if let Some("winmd") =
                            path.extension().and_then(|extension| extension.to_str())
                        {
                            result.push(File::new(path));
                        }
                    }
                }
            }
        }
    }

    let mut result = vec![];

    let mut dir: std::path::PathBuf = std::env::var("CARGO_MANIFEST_DIR")
        .expect("No `CARGO_MANIFEST_DIR` env variable set")
        .into();

    dir.push(".windows");
    dir.push("winmd");
    push_dir(&mut result, &dir);

    let dir = std::env::var("PATH").expect("No `PATH` env variable set");
    let end = dir.find(';').expect("Path not ending in `;`");
    let mut dir: std::path::PathBuf = dir[..end].into();
    dir.pop();
    dir.pop();
    dir.push(".windows");
    dir.push("winmd");
    push_dir(&mut result, &dir);

    result
}
