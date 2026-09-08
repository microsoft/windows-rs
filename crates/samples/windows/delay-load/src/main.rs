fn main() {
    use windows::{Win32::*, core::*};

    type ShellMessageBoxW = unsafe extern "C" fn(
        happinst: usize,
        hwnd: usize,
        lpctext: PCWSTR,
        lpctitle: PCWSTR,
        fustyle: u32,
        ...
    ) -> i32;

    unsafe {
        let library = LoadLibraryExA(
            s!("shlwapi.dll"),
            None,
            LOAD_LIBRARY_SEARCH_DEFAULT_DIRS as u32,
        );
        if library.0.is_null() {
            println!("Can't load shlwapi.dll");
            return;
        }

        if let Some(address) = GetProcAddress(library, s!("ShellMessageBoxW")) {
            let api: ShellMessageBoxW = std::mem::transmute(address);
            api(0, 0, w!("Message"), w!("Sample"), 1);
        } else {
            println!("Can't find ShellMessageBoxW");
        }

        _ = FreeLibrary(library);
    }
}
