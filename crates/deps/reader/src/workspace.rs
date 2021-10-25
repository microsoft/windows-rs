use super::*;

pub fn workspace_winmds() -> &'static [File] {
    use std::{mem::MaybeUninit, sync::Once};
    static ONCE: Once = Once::new();
    static mut VALUE: MaybeUninit<Vec<File>> = MaybeUninit::uninit();

    ONCE.call_once(|| unsafe { VALUE = MaybeUninit::new(get_workspace_winmds()) });

    unsafe { &*VALUE.as_ptr() }
}

fn json_value(key: &str) -> String {
    let json = cargo_metadata();
    let json_key = format!(r#""{}":""#, key);

    let beginning_index = json
        .rfind(&json_key)
        .unwrap_or_else(|| panic!("Cargo metadata did not contain `{}` key.", key))
        + json_key.len();

    let ending_index = json[beginning_index..].find('"').unwrap_or_else(|| {
        panic!(
            "Cargo metadata ended before closing `\"` in `{}` value",
            key
        )
    });

    json[beginning_index..beginning_index + ending_index].replace("\\\\", "\\")
}

#[doc(hidden)]
pub fn workspace_dir() -> String {
    json_value("workspace_root")
}

pub fn target_dir() -> String {
    json_value("target_directory")
}

fn cargo_metadata() -> &'static str {
    use std::{mem::MaybeUninit, sync::Once};
    static ONCE: Once = Once::new();
    static mut VALUE: MaybeUninit<String> = MaybeUninit::uninit();

    ONCE.call_once(|| {
        let output = std::process::Command::new(env!("CARGO"))
            .arg("metadata")
            .arg("--format-version=1")
            .arg("--no-deps")
            .arg("--offline")
            .output()
            .expect("Failed to run `cargo metadata`");

        unsafe {
            VALUE = MaybeUninit::new(
                String::from_utf8(output.stdout).expect("Cargo metadata is not utf-8"),
            )
        }
    });

    // This is safe because `call_once` has already been called.
    unsafe { &*VALUE.as_ptr() }
}

fn get_workspace_winmds() -> Vec<File> {
    fn push_dir(result: &mut Vec<File>, dir: &std::path::Path) {
        if let Ok(files) = std::fs::read_dir(&dir) {
            for file in files.filter_map(|file| file.ok()) {
                if let Ok(file_type) = file.file_type() {
                    if file_type.is_file() {
                        let path = file.path();
                        if path.extension().and_then(|extension| extension.to_str())
                            == Some("winmd")
                        {
                            result.push(File::new(path));
                        }
                    }
                }
            }
        }
    }

    let mut result = vec![];

    let mut dir: std::path::PathBuf = workspace_dir().into();
    dir.push(".windows");
    dir.push("winmd");
    push_dir(&mut result, &dir);

    if !result.iter().any(|file| file.name.starts_with("Windows.")) {
        result.push(File::from_bytes(
            "Windows.winmd".to_string(),
            include_bytes!("../default/Windows.winmd").to_vec(),
        ));
        result.push(File::from_bytes(
            "Windows.Win32.winmd".to_string(),
            include_bytes!("../default/Windows.Win32.winmd").to_vec(),
        ));
        result.push(File::from_bytes(
            "Windows.Win32.Interop.winmd".to_string(),
            include_bytes!("../default/Windows.Win32.Interop.winmd").to_vec(),
        ));
    }

    result
}
