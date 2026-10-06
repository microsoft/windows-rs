use super::*;

// Shared by the Win32, WDK, and WebView2 generators.
const SDK_VERSION: &str = "10.0.28000.2270";

pub fn sdk_include_root() -> PathBuf {
    nuget_package("microsoft.windows.sdk.cpp", SDK_VERSION)
        .join("c")
        .join("Include")
        .join(marketing_dir(SDK_VERSION))
}

pub fn sdk_lib_root() -> PathBuf {
    nuget_package("microsoft.windows.sdk.cpp.x64", SDK_VERSION).join("c")
}

pub fn sdk_include_dirs() -> Vec<String> {
    let base = sdk_include_root();
    checked_dirs(["ucrt", "um", "shared", "winrt", "cppwinrt"].map(|dir| base.join(dir)))
}

pub fn sdk_lib_dirs() -> Vec<String> {
    // Function-to-DLL mappings are architecture-independent, so all scrapes use the x64 libs.
    let base = sdk_lib_root();
    checked_dirs(["um", "ucrt"].map(|dir| base.join(dir).join("x64")))
}

pub fn checked_dirs(dirs: impl IntoIterator<Item = PathBuf>) -> Vec<String> {
    dirs.into_iter()
        .map(|dir| {
            assert!(
                dir.is_dir(),
                "pinned SDK/WDK package directory is missing: `{}`",
                dir.display()
            );
            dir.to_str()
                .unwrap_or_else(|| {
                    panic!("package directory is not a UTF-8 path: `{}`", dir.display())
                })
                .replace('\\', "/")
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn package_directories_preserve_order_and_reject_missing_paths() {
        let root = TempDir::new(&std::env::temp_dir());
        let dirs = [root.0.join("second"), root.0.join("first")];
        for dir in &dirs {
            std::fs::create_dir(dir).unwrap();
        }
        assert_eq!(
            checked_dirs(dirs.clone()),
            dirs.map(|dir| dir.to_str().unwrap().replace('\\', "/"))
        );
        assert!(std::panic::catch_unwind(|| checked_dirs([root.0.join("missing")])).is_err());
        let file = root.0.join("file");
        std::fs::write(&file, "").unwrap();
        assert!(std::panic::catch_unwind(|| checked_dirs([file])).is_err());
    }
}
