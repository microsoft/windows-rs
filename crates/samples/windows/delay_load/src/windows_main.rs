use windows::{core::*, Win32::Foundation::*, Win32::System::LibraryLoader::*};

/// # Safety
///
/// The `PCSTR` parameters need to be valid for reads up until and including the next `\0`.
pub unsafe fn delay_load<T>(library: PCSTR, function: PCSTR) -> Option<T> {
    unsafe {
        let library = LoadLibraryExA(library, None, LOAD_LIBRARY_SEARCH_DEFAULT_DIRS);

        let Ok(library) = library else {
            return None;
        };

        let address = GetProcAddress(library, function);

        if address.is_some() {
            return Some(std::mem::transmute_copy(&address));
        }

        _ = FreeLibrary(library);
        None
    }
}

fn main() {
    unsafe {
        if let Some(api) = delay_load::<ShellMessageBoxW>(s!("shlwapi.dll"), s!("ShellMessageBoxW"))
        {
            api(0, 0, w!("Message"), w!("Sample"), 1);
        } else {
            println!("Can't find API");
        }
    }
}

type ShellMessageBoxW = unsafe extern "C" fn(
    happinst: usize,
    hwnd: usize,
    lpctext: PCWSTR,
    lpctitle: PCWSTR,
    fustyle: u32,
    ...
) -> i32;
