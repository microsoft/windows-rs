use serde::Deserialize;
use std::cmp::Ordering;
use std::path::Path;

#[derive(Deserialize)]
pub struct Crate {
    pub package: Package,
    pub lints: Option<Lints>,
    pub path: Option<std::path::PathBuf>,
}

impl PartialEq for Crate {
    fn eq(&self, other: &Self) -> bool {
        self.package.name == other.package.name
    }
}

impl Eq for Crate {}

impl Ord for Crate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.package.name.cmp(&other.package.name)
    }
}

impl PartialOrd for Crate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Deserialize)]
pub struct Lints {
    pub workspace: bool,
}

#[derive(Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub edition: String,
    pub publish: Option<bool>,
    #[serde(rename = "rust-version")]
    pub rust_version: Option<String>,
    pub license: Option<String>,
    pub description: Option<String>,
    pub repository: Option<String>,
    pub readme: Option<String>,
    pub categories: Option<Vec<String>>,
    pub authors: Option<Vec<String>>,
}

pub fn crates<P: AsRef<Path>>(path: P) -> Vec<Crate> {
    let mut crates = find(path);
    crates.sort();
    crates
}

fn find<P: AsRef<Path>>(path: P) -> Vec<Crate> {
    let mut crates = vec![];

    if let Ok(files) = std::fs::read_dir(path) {
        for file in files.filter_map(|file| file.ok()) {
            if let Ok(file_type) = file.file_type() {
                if file_type.is_dir() {
                    crates.append(&mut find(file.path()));
                } else if file.file_name() == "Cargo.toml" {
                    let text = std::fs::read_to_string(file.path()).expect("Cargo.toml");
                    let mut entry: Crate = toml::from_str(&text).expect("toml");
                    entry.path = Some(file.path());
                    crates.push(entry);
                }
            }
        }
    }

    crates
}

/// Reads the string value of a `const NAME: &str = "…";` (or `pub const`) declaration from a
/// Rust source file. Panics loudly if the file cannot be read or the constant is not found.
///
/// This is the single shared mechanism for the *paired* dependency-pin validators: a pin is
/// declared as an ordinary constant in exactly one crate (its owner), and any other tool that
/// must stay in lock-step reads it back from source and asserts agreement — e.g. `tool_wdk`
/// reads `tool_win32`'s `SDK_VERSION`, and `tool_reactor` reads `windows-reactor-setup`'s
/// `RUNTIME_VER` / `WEBVIEW2_VER`. Keeping one reader keeps every such check consistent.
pub fn read_str_const<P: AsRef<Path>>(path: P, name: &str) -> String {
    let path = path.as_ref();
    let text = std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("failed to read `{}`: {e}", path.display()));
    str_const(&text, name).unwrap_or_else(|| {
        panic!(
            "`const {name}` (a `&str`) not found in `{}`",
            path.display()
        )
    })
}

fn str_const(text: &str, name: &str) -> Option<String> {
    let mut offset = 0;
    for line in text.split_inclusive('\n') {
        let head = line.trim_start();
        let head = head.strip_prefix("pub ").unwrap_or(head);
        if let Some(rest) = head.strip_prefix("const ").and_then(|r| r.strip_prefix(name))
            // The declared name must end here (next non-space is the `:` type separator), so
            // `SDK_VERSION` does not spuriously match `SDK_VERSION_EXTRA`.
            && rest.trim_start().starts_with(':')
        {
            // The string literal may sit on this line or a following one (a multi-line
            // `const NAME: &str =\n    "value";`), so scan the file tail from here.
            let tail = &text[offset..];
            let open = tail.find('"')?;
            let close = tail[open + 1..].find('"')? + open + 1;
            return Some(tail[open + 1..close].to_string());
        }
        offset += line.len();
    }
    None
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

#[cfg(test)]
mod tests {
    use super::str_const;

    #[test]
    fn reads_plain_and_pub_and_ignores_prefix_collisions() {
        let src = "\
// a comment mentioning const SDK_VERSION
    const SDK_VERSION_EXTRA: &str = \"nope\";
pub const SDK_VERSION: &str = \"10.0.28000.2270\";
const OTHER: u32 = 7;
const MULTI: &str =
    \"https://example/clang-1.2.3.tar\";
";
        assert_eq!(
            str_const(src, "SDK_VERSION").as_deref(),
            Some("10.0.28000.2270")
        );
        assert_eq!(str_const(src, "SDK_VERSION_EXTRA").as_deref(), Some("nope"));
        assert_eq!(
            str_const(src, "MULTI").as_deref(),
            Some("https://example/clang-1.2.3.tar")
        );
        assert_eq!(str_const(src, "MISSING"), None);
    }
}
