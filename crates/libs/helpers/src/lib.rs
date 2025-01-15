use regex::Regex;
use std::path::Path;

pub fn crates<P: AsRef<Path>>(path: P) -> Vec<(String, String)> {
    let regex = Regex::new(r#"name = "([^"]+)"\sversion = "([^"]+)""#).expect("regex");
    let mut names = find(path, &regex);
    names.sort();
    names
}

fn find<P: AsRef<Path>>(path: P, regex: &Regex) -> Vec<(String, String)> {
    let mut names = vec![];

    if let Ok(files) = std::fs::read_dir(path) {
        for file in files.filter_map(|file| file.ok()) {
            if let Ok(file_type) = file.file_type() {
                if file_type.is_dir() {
                    names.append(&mut find(file.path(), regex));
                } else if file.file_name() == "Cargo.toml" {
                    let text = std::fs::read_to_string(file.path()).expect("Cargo.toml");
                    let captures = regex.captures(&text).expect("captures");
                    let name = captures.get(1).expect("name");
                    let version = captures.get(2).expect("version");
                    names.push((name.as_str().to_string(), version.as_str().to_string()));
                }
            }
        }
    }

    names
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
