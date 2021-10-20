fn main() {
    windows::build! {
        Windows::Win32::Foundation::{
            SetLastError, BOOLEAN, HANDLE, HWND, PSTR, PWSTR, WIN32_ERROR,
        },
        Windows::Win32::Graphics::Gdi::HBITMAP,
        Windows::Win32::System::Registry::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE},
        Windows::Win32::System::Threading::LPPROC_THREAD_ATTRIBUTE_LIST,
    };
}
