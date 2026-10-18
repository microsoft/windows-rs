#![doc = include_str!("../readme.md")]
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

#[allow(clippy::missing_safety_doc, clippy::too_many_arguments)]
mod bindings;
pub use bindings::*;

use std::cell::RefCell;
use std::ffi::CStr;
use std::path::Path;
use std::sync::Arc;

/// LLVM version shared by the native bridge, runtime, and resource headers.
pub const LIBCLANG_VERSION: &str = "22.1.8";

/// Keeps both native libraries alive. Compiler handles must not outlive this owner.
pub struct SharedLibrary {
    functions: Functions,
    _extension: libloading::Library,
    _base: libloading::Library,
}

thread_local! {
    static LIBRARY: RefCell<Option<Arc<SharedLibrary>>> = const { RefCell::new(None) };
}

fn with_library<T>(callback: impl FnOnce(&SharedLibrary) -> T) -> T {
    let library = LIBRARY.with(|slot| slot.borrow().clone()).unwrap();
    callback(&library)
}

/// Replaces this thread's binding, returning its previous owner.
pub fn set_library(library: Option<Arc<SharedLibrary>>) -> Option<Arc<SharedLibrary>> {
    LIBRARY.with(|slot| slot.replace(library))
}

/// Loads the version-matched DLL pair without binding it to the calling thread.
pub fn load_manually() -> Result<SharedLibrary, String> {
    if let Some(path) = std::env::var_os("LIBCLANG_PATH") {
        let path = Path::new(&path);
        return load_from(if path.is_file() {
            path.parent().unwrap()
        } else {
            path
        });
    }
    if let Some(paths) = std::env::var_os("PATH") {
        for path in std::env::split_paths(&paths) {
            if path.join("libclang.dll").is_file() && path.join("libClangSharp.dll").is_file() {
                return load_from(&path);
            }
        }
    }
    Err("set LIBCLANG_PATH to the version-matched libclang.dll/libClangSharp.dll directory".into())
}

/// Loads both DLLs from one directory and checks the extension ABI and LLVM version.
pub fn load_from(directory: &Path) -> Result<SharedLibrary, String> {
    let base = unsafe { libloading::Library::new(directory.join("libclang.dll")) }
        .map_err(|error| format!("{}: {error}", directory.display()))?;
    let extension = unsafe { libloading::Library::new(directory.join("libClangSharp.dll")) }
        .map_err(|error| format!("{}: {error}", directory.display()))?;
    let functions = unsafe { Functions::load(&base, &extension)? };
    let abi = unsafe { (functions.windows_clang_getABIVersion)() };
    if abi != 17 {
        return Err(format!(
            "unsupported windows-clang native ABI {abi}; expected 17"
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
    Ok(SharedLibrary {
        functions,
        _extension: extension,
        _base: base,
    })
}

/// Loads and binds the libraries to this thread.
pub fn load() -> Result<(), String> {
    if LIBRARY.with(|slot| slot.borrow().is_none()) {
        set_library(Some(Arc::new(load_manually()?)));
    }
    Ok(())
}

/// Releases this thread's binding. All of its compiler handles must already be disposed.
pub fn unload() -> Result<(), String> {
    set_library(None)
        .map(|_| ())
        .ok_or_else(|| "libclang is not loaded on this thread".into())
}
