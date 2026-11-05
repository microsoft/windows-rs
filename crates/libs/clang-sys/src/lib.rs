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
                use core::sync::atomic::{AtomicPtr, Ordering};
                static ADDRESS: AtomicPtr<core::ffi::c_void> = AtomicPtr::new(core::ptr::null_mut());
                let mut address = ADDRESS.load(Ordering::Relaxed);
                if address.is_null() {
                    address = $crate::export($library, concat!(stringify!($name), "\0"));
                    ADDRESS.store(address, Ordering::Relaxed);
                }
                unsafe { core::mem::transmute::<*mut core::ffi::c_void, $name>(address)($($arg),*) }
            }
        };
    }
    pub(crate) use dynamic_link as link;
}

// Loaded once per process and never unloaded, so compiler handles cannot outlive the libraries.
static MODULES: OnceLock<Modules> = OnceLock::new();

struct Modules {
    base: *mut c_void,
    extension: *mut c_void,
}

// Module handles are process-wide and remain loaded for the life of the process.
unsafe impl Send for Modules {}
unsafe impl Sync for Modules {}

fn export(library: &str, name: &str) -> *mut c_void {
    let modules = MODULES.get().unwrap();
    let module = match library {
        BASE => modules.base,
        EXTENSION => modules.extension,
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

// Frees the module unless loading succeeds.
struct Library(*mut c_void);

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

    unsafe fn get<T>(&self, name: &CStr) -> Result<T, String> {
        let address = unsafe { GetProcAddress(self.0, name.as_ptr().cast()) };
        if address.is_null() {
            Err(format!("missing export {}", name.to_string_lossy()))
        } else {
            Ok(unsafe { std::mem::transmute_copy(&address) })
        }
    }
}

/// Loads both DLLs from one directory and checks the extension ABI and LLVM version.
fn load_from(directory: &Path) -> Result<Modules, String> {
    let base = Library::open(directory, BASE)?;
    let extension = Library::open(directory, EXTENSION)?;
    unsafe {
        let abi = extension.get::<windows_clang_getABIVersion>(c"windows_clang_getABIVersion")?();
        if abi != ABI_VERSION {
            return Err(format!(
                "unsupported windows-clang native ABI {abi}; expected {ABI_VERSION}"
            ));
        }
        let identity = extension
            .get::<windows_clang_getRuntimeIdentity>(c"windows_clang_getRuntimeIdentity")?(
        );
        if identity != base.get::<*const c_void>(c"clang_getClangVersion")? {
            return Err("native extension is linked to a different libclang runtime".into());
        }
        let version =
            extension.get::<windows_clang_getLLVMVersion>(c"windows_clang_getLLVMVersion")?();
        if version.is_null() || CStr::from_ptr(version).to_bytes() != LIBCLANG_VERSION.as_bytes() {
            return Err(format!("native extension requires LLVM {LIBCLANG_VERSION}"));
        }
        let version = base.get::<clang_getClangVersion>(c"clang_getClangVersion")?();
        let pointer = base.get::<clang_getCString>(c"clang_getCString")?(version);
        let matches = !pointer.is_null()
            && CStr::from_ptr(pointer)
                .to_string_lossy()
                .split_whitespace()
                .any(|word| word == LIBCLANG_VERSION);
        base.get::<clang_disposeString>(c"clang_disposeString")?(version);
        if !matches {
            return Err(format!("libclang runtime requires LLVM {LIBCLANG_VERSION}"));
        }
    }
    let modules = Modules {
        base: base.0,
        extension: extension.0,
    };
    std::mem::forget((base, extension));
    Ok(modules)
}

#[cfg(test)]
mod tests {
    #[test]
    fn missing_directory() {
        assert!(super::load_from(std::path::Path::new("missing-native-clang-directory")).is_err());
    }
}
