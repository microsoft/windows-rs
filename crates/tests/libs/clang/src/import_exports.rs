use std::collections::BTreeMap;
use std::ffi::{CString, c_void};

#[allow(non_snake_case)]
#[link(name = "kernel32")]
unsafe extern "system" {
    fn LoadLibraryExW(name: *const u16, file: *mut c_void, flags: u32) -> *mut c_void;
    fn GetProcAddress(module: *mut c_void, name: *const u8) -> Option<unsafe extern "system" fn()>;
    fn GetModuleFileNameW(module: *mut c_void, name: *mut u16, size: u32) -> u32;
    fn FreeLibrary(module: *mut c_void) -> i32;
}

struct Module(*mut c_void);

impl Drop for Module {
    fn drop(&mut self) {
        assert_ne!(unsafe { FreeLibrary(self.0) }, 0);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target = std::env::consts::ARCH;
    let bit = match target {
        "x86" => 1,
        "x86_64" => 2,
        "aarch64" => 4,
        _ => return Err(format!("unsupported Windows target: {target}").into()),
    };
    let mut libraries = BTreeMap::<_, Vec<_>>::new();
    for line in [
        include_str!("..\\input\\sdk_import_availability.txt"),
        include_str!("..\\input\\sdk_ndr_import_availability.txt"),
        include_str!("..\\input\\sdk_import_providers.txt"),
    ]
    .into_iter()
    .flat_map(str::lines)
    {
        if line.starts_with('#') {
            continue;
        }
        let columns: Vec<_> = line.split_whitespace().collect();
        let [mask, name, library] = columns.as_slice() else {
            return Err(format!("invalid fixture entry: {line}").into());
        };
        let mask: u8 = mask.parse()?;
        libraries.entry(*library).or_default().push((mask, *name));
    }
    println!("Target,Name,Library,LoadedModule,Observed,SDKImport");
    for (library, names) in libraries {
        let wide: Vec<_> = library.encode_utf16().chain([0]).collect();
        // System-directory lookup also resolves API sets in this process's architecture.
        let module = unsafe { LoadLibraryExW(wide.as_ptr(), std::ptr::null_mut(), 0x800) };
        if module.is_null() {
            return Err(format!("load {library}: {}", std::io::Error::last_os_error()).into());
        }
        let module = Module(module);
        let mut path = [0; 32768];
        let length = unsafe { GetModuleFileNameW(module.0, path.as_mut_ptr(), path.len() as u32) };
        if length == 0 {
            return Err(std::io::Error::last_os_error().into());
        }
        if length as usize >= path.len() {
            return Err(format!("module path too long: {library}").into());
        }
        let path = String::from_utf16(&path[..length as usize])?;
        for (mask, name) in names {
            let symbol = CString::new(name)?;
            let address = unsafe { GetProcAddress(module.0, symbol.as_ptr().cast()) };
            let observed = if address.is_some() {
                "present"
            } else {
                let error = std::io::Error::last_os_error();
                if error.raw_os_error() != Some(127) {
                    return Err(format!("resolve {library}!{name}: {error}").into());
                }
                "missing"
            };
            let expected = if mask & bit != 0 {
                "present"
            } else {
                "missing"
            };
            let row = [target, name, library, &path, observed, expected];
            println!(
                "{}",
                row.map(|value| format!("\"{}\"", value.replace('"', "\"\"")))
                    .join(",")
            );
        }
    }
    Ok(())
}
