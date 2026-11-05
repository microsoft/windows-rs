#![doc = include_str!("../readme.md")]
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

#[allow(clippy::missing_safety_doc, clippy::too_many_arguments)]
mod bindings {
    // The generated `windows_link::link!` declarations resolve through this crate's loader.
    use crate::loader as windows_link;
    include!("bindings.rs");
}
pub use bindings::*;

use std::ffi::{CStr, c_void};
use std::os::windows::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::sync::atomic::{AtomicPtr, Ordering};

/// LLVM version shared by the native bridge, runtime, and resource headers.
pub const LIBCLANG_VERSION: &str = "22.1.8";

// Matches `windows_clang_getABIVersion` in native/extension.cpp.
const ABI_VERSION: u32 = 24;
const BASE: &str = "libclang.dll";
const EXTENSION: &str = "windows_clang.dll";

mod kernel32 {
    use std::ffi::c_void;
    windows_link::link!("kernel32.dll" "system" fn LoadLibraryExW(name: *const u16, file: *mut c_void, flags: u32) -> *mut c_void);
    windows_link::link!("kernel32.dll" "system" fn GetProcAddress(module: *mut c_void, name: *const u8) -> *mut c_void);
    windows_link::link!("kernel32.dll" "system" fn FreeLibrary(module: *mut c_void) -> i32);
    pub const LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR: u32 = 0x100;
    pub const LOAD_LIBRARY_SEARCH_DEFAULT_DIRS: u32 = 0x1000;
}
use kernel32::*;

mod loader {
    // Declares a wrapper that resolves its export on first call and caches the address.
    macro_rules! dynamic_link {
        ($library:literal $abi:literal fn $name:ident($($arg:ident : $ty:ty),*) $(-> $ret:ty)?) => {
            pub type $name = unsafe extern $abi fn($($ty),*) $(-> $ret)?;
            pub unsafe fn $name($($arg: $ty),*) $(-> $ret)? {
                static ADDRESS: $crate::Address = $crate::Address::new(core::ptr::null_mut());
                let address = $crate::resolve(&ADDRESS, $library, concat!(stringify!($name), "\0"));
                unsafe { core::mem::transmute::<*mut core::ffi::c_void, $name>(address)($($arg),*) }
            }
        };
    }
    pub(crate) use dynamic_link as link;
}

// Loaded once per process and never unloaded, so compiler handles cannot outlive the libraries.
static MODULES: OnceLock<Modules> = OnceLock::new();

struct Modules {
    base: Library,
    extension: Library,
}

type Address = AtomicPtr<c_void>;

// Racing threads find the same address, so a relaxed load and store is enough.
#[inline]
fn resolve(cache: &Address, library: &str, name: &str) -> *mut c_void {
    let address = cache.load(Ordering::Relaxed);
    if !address.is_null() {
        return address;
    }
    let address = export(library, name);
    cache.store(address, Ordering::Relaxed);
    address
}

#[cold]
fn export(library: &str, name: &str) -> *mut c_void {
    let modules = MODULES.get().unwrap();
    let module = match library {
        BASE => modules.base.0,
        EXTENSION => modules.extension.0,
        _ => unreachable!(),
    };
    let address = unsafe { GetProcAddress(module, name.as_ptr()) };
    assert!(
        !address.is_null(),
        "{library} does not export {}",
        name.trim_end_matches('\0')
    );
    address
}

/// Loads the version-matched DLL pair for the process. Later calls return immediately.
///
/// `LIBCLANG_PATH` names the directory (or a file within it); otherwise `PATH` is searched.
pub fn load() -> Result<(), String> {
    if MODULES.get().is_none() {
        // A concurrent load opens the same modules, so either result can be kept.
        _ = MODULES.set(load_from(&directory()?)?);
    }
    Ok(())
}

fn directory() -> Result<PathBuf, String> {
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

// Modules kept in `MODULES` are never dropped and stay loaded for the life of the process.
struct Library(*mut c_void);

// Module handles are process-wide.
unsafe impl Send for Library {}
unsafe impl Sync for Library {}

impl Drop for Library {
    fn drop(&mut self) {
        unsafe { FreeLibrary(self.0) };
    }
}

impl Library {
    fn open(directory: &Path, name: &str) -> Result<Self, String> {
        let path = std::path::absolute(directory.join(name)).map_err(|error| error.to_string())?;
        let wide: Vec<u16> = path.as_os_str().encode_wide().chain([0]).collect();
        let module = unsafe {
            LoadLibraryExW(
                wide.as_ptr(),
                std::ptr::null_mut(),
                LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR | LOAD_LIBRARY_SEARCH_DEFAULT_DIRS,
            )
        };
        if module.is_null() {
            Err(format!(
                "{}: {}",
                path.display(),
                std::io::Error::last_os_error()
            ))
        } else {
            Ok(Self(module))
        }
    }

    unsafe fn get<T>(&self, name: &str) -> Result<T, String> {
        let address = unsafe { GetProcAddress(self.0, name.as_ptr()) };
        if address.is_null() {
            Err(format!("missing export {}", name.trim_end_matches('\0')))
        } else {
            Ok(unsafe { std::mem::transmute_copy(&address) })
        }
    }
}

/// Loads both DLLs from one directory and checks the extension ABI and LLVM version.
fn load_from(directory: &Path) -> Result<Modules, String> {
    let base = Library::open(directory, BASE)?;
    let extension = Library::open(directory, EXTENSION)?;
    // Looks up an export by the name of its generated function type.
    macro_rules! symbol {
        ($library:ident, $name:ident) => {
            $library.get::<$name>(concat!(stringify!($name), "\0"))?
        };
    }
    unsafe {
        let abi = symbol!(extension, windows_clang_getABIVersion)();
        if abi != ABI_VERSION {
            return Err(format!(
                "unsupported windows-clang native ABI {abi}; expected {ABI_VERSION}"
            ));
        }
        if symbol!(extension, windows_clang_getRuntimeIdentity)()
            != symbol!(base, clang_getClangVersion) as *const c_void
        {
            return Err("native extension is linked to a different libclang runtime".into());
        }
        let version = symbol!(extension, windows_clang_getLLVMVersion)();
        if version.is_null() || CStr::from_ptr(version).to_bytes() != LIBCLANG_VERSION.as_bytes() {
            return Err(format!("native extension requires LLVM {LIBCLANG_VERSION}"));
        }
        let version = symbol!(base, clang_getClangVersion)();
        let pointer = symbol!(base, clang_getCString)(version);
        let matches = !pointer.is_null()
            && CStr::from_ptr(pointer)
                .to_string_lossy()
                .split_whitespace()
                .any(|word| word == LIBCLANG_VERSION);
        symbol!(base, clang_disposeString)(version);
        if !matches {
            return Err(format!("libclang runtime requires LLVM {LIBCLANG_VERSION}"));
        }
    }
    Ok(Modules { base, extension })
}
#[cfg(test)]
mod tests {
    #[test]
    fn missing_directory() {
        assert!(super::load_from(std::path::Path::new("missing-native-clang-directory")).is_err());
    }
}
