#![doc = include_str!("../readme.md")]
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

#[allow(clippy::missing_safety_doc, clippy::too_many_arguments)]
mod bindings;
pub use bindings::*;

use std::ffi::CStr;
use std::path::Path;
use std::sync::{Mutex, OnceLock};

/// LLVM version shared by the native bridge, runtime, and resource headers.
pub const LIBCLANG_VERSION: &str = "22.1.8";

const BASE: &str = "libclang.dll";
const EXTENSION: &str = "windows_clang.dll";

// Loaded once per process and never unloaded, so compiler handles cannot outlive the libraries.
static FUNCTIONS: OnceLock<Functions> = OnceLock::new();

fn functions() -> &'static Functions {
    FUNCTIONS.get().unwrap()
}

/// Loads the version-matched DLL pair for the process. Later calls return immediately.
///
/// `LIBCLANG_PATH` names the directory (or a file within it); otherwise `PATH` is searched.
pub fn load() -> Result<(), String> {
    static LOADING: Mutex<()> = Mutex::new(());
    let _loading = LOADING.lock().unwrap();
    if FUNCTIONS.get().is_none() {
        _ = FUNCTIONS.set(load_from(&directory()?)?);
    }
    Ok(())
}

fn directory() -> Result<std::path::PathBuf, String> {
    if let Some(path) = std::env::var_os("LIBCLANG_PATH") {
        let path = Path::new(&path);
        return Ok(if path.is_file() {
            path.parent().unwrap()
        } else {
            path
        }
        .to_path_buf());
    }
    if let Some(paths) = std::env::var_os("PATH") {
        for path in std::env::split_paths(&paths) {
            if path.join(BASE).is_file() && path.join(EXTENSION).is_file() {
                return Ok(path);
            }
        }
    }
    Err(format!(
        "set LIBCLANG_PATH to the version-matched {BASE}/{EXTENSION} directory"
    ))
}

/// Loads both DLLs from one directory and checks the extension ABI and LLVM version.
fn load_from(directory: &Path) -> Result<Functions, String> {
    let open = |name| {
        unsafe { libloading::Library::new(directory.join(name)) }
            .map_err(|error| format!("{}: {error}", directory.display()))
    };
    let base = open(BASE)?;
    let extension = open(EXTENSION)?;
    let functions = unsafe { Functions::load(&base, &extension)? };
    let abi = unsafe { (functions.windows_clang_getABIVersion)() };
    if abi != 22 {
        return Err(format!(
            "unsupported windows-clang native ABI {abi}; expected 22"
        ));
    }
    if unsafe { (functions.windows_clang_getRuntimeIdentity)() }
        != functions.clang_getClangVersion as *const () as *const std::ffi::c_void
    {
        return Err("native extension is linked to a different libclang runtime".into());
    }
    let version = unsafe { (functions.windows_clang_getLLVMVersion)() };
    if version.is_null()
        || unsafe { CStr::from_ptr(version) }.to_bytes() != LIBCLANG_VERSION.as_bytes()
    {
        return Err(format!("native extension requires LLVM {LIBCLANG_VERSION}"));
    }
    let version = unsafe { (functions.clang_getClangVersion)() };
    let pointer = unsafe { (functions.clang_getCString)(version) };
    let matches = !pointer.is_null()
        && unsafe { CStr::from_ptr(pointer) }
            .to_string_lossy()
            .split_whitespace()
            .any(|word| word == LIBCLANG_VERSION);
    unsafe { (functions.clang_disposeString)(version) };
    if !matches {
        return Err(format!("libclang runtime requires LLVM {LIBCLANG_VERSION}"));
    }
    // The function pointers remain valid for the life of the process.
    std::mem::forget(extension);
    std::mem::forget(base);
    Ok(functions)
}

#[cfg(test)]
mod tests {
    #[test]
    fn missing_directory() {
        assert!(super::load_from(std::path::Path::new("missing-native-clang-directory")).is_err());
    }
}
