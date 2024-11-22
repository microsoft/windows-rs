use test_bindgen2::deps2::Windows::Win32::{Foundation::*, System::LibraryLoader::*};

#[test]
fn test() {
    unsafe {
        let library = LoadLibraryExA(
            "kernel32.dll\0".as_ptr(),
            core::ptr::null_mut(),
            LOAD_LIBRARY_SEARCH_DEFAULT_DIRS,
        );

        assert!(!library.is_null());
        let address = GetProcAddress(library, "GetTickCount\0".as_ptr()).unwrap();
        let fp = core::mem::transmute::<_, extern "system" fn() -> u32>(address);
        assert_ne!(fp(), 0);
        FreeLibrary(library);
    }
}
