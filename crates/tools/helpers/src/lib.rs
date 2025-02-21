use std::path::Path;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Crate {
    pub package: Package,
}

#[derive(Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct Package {
    pub name: String,
    pub version: String,
    #[serde(rename = "rust-version")]
    pub rust_version: Option<String>,
}

pub fn crates<P: AsRef<Path>>(path: P) -> Vec<Package> {
    let mut crates = find(path);
    crates.sort();
    crates
}

fn find<P: AsRef<Path>>(path: P) -> Vec<Package> {
    let mut crates = vec![];

    if let Ok(files) = std::fs::read_dir(path) {
        for file in files.filter_map(|file| file.ok()) {
            if let Ok(file_type) = file.file_type() {
                if file_type.is_dir() {
                    crates.append(&mut find(file.path()));
                } else if file.file_name() == "Cargo.toml" {
                    let text = std::fs::read_to_string(file.path()).expect("Cargo.toml");
                    let toml : Crate = toml::from_str(&text).expect("toml");
                    crates.push(toml.package);
                }
            }
        }
    }

    crates
}

pub fn set_thread_ui_language() {
    // Enables testing without pulling in a dependency on the `windows` crate.
    windows_link::link!("kernel32.dll" "system" fn SetThreadPreferredUILanguages(flags : u32, language : *const u16, _ : *mut u32) -> i32);
    pub const MUI_LANGUAGE_NAME: u32 = 8u32;

    let language: Vec<_> = "en-US".encode_utf16().chain(std::iter::once(0)).collect();
    unsafe {
        assert_eq!(
            1,
            SetThreadPreferredUILanguages(
                MUI_LANGUAGE_NAME,
                language.as_ptr(),
                std::ptr::null_mut()
            )
        );
    }
}
