pub fn reader() -> &'static winmd::TypeReader {
    use std::{mem::MaybeUninit, sync::Once};
    static ONCE: Once = Once::new();
    static mut VALUE: MaybeUninit<winmd::TypeReader> = MaybeUninit::uninit();

    ONCE.call_once(|| {
        // This is safe because `Once` provides thread-safe one-time initialization
        unsafe { VALUE = MaybeUninit::new(winmd::TypeReader::from_iter(winmd_paths())) }
    });

    // This is safe because `call_once` has already been called.
    unsafe { &*VALUE.as_ptr() }
}

fn winmd_paths() -> Vec<std::path::PathBuf> {
    let windows_path =
        ::std::env::var("CARGO_MANIFEST_DIR").expect("No `CARGO_MANIFEST_DIR` env variable set");

    let mut windows_path = ::std::path::PathBuf::from(&windows_path);
    windows_path.push(".windows");
    windows_path.push("winmd");

    let mut paths = vec![];
    push_winmd_paths(windows_path, &mut paths);

    // If at this point the paths vector is still empty then go and grab the winmd files from WinMetadata
    // to make it easy for developers to get started without having to figure out where to get metadata.

    if paths.is_empty() {
        if let Ok(dir) = std::env::var("windir") {
            let mut dir = std::path::PathBuf::from(dir);
            dir.push(SYSTEM32);
            dir.push("winmetadata");

            push_winmd_paths(dir, &mut paths);
        }
    }

    paths
}

fn push_winmd_paths(dir: std::path::PathBuf, paths: &mut Vec<std::path::PathBuf>) {
    if let Ok(files) = std::fs::read_dir(dir) {
        for file in files.filter_map(|file| file.ok()) {
            if let Ok(file_type) = file.file_type() {
                if file_type.is_file() {
                    let path = file.path();
                    if let Some("winmd") = path.extension().and_then(|extension| extension.to_str())
                    {
                        paths.push(file.path());
                    }
                }
            }
        }
    }
}

#[cfg(target_pointer_width = "64")]
const SYSTEM32: &str = "System32";

#[cfg(target_pointer_width = "32")]
const SYSTEM32: &str = "SysNative";
