fn main() {
    windows::build! {
        Windows::Win32::Foundation::{BOOLEAN, HANDLE, HWND, PSTR, PWSTR},
        Windows::Win32::Graphics::Gdi::HBITMAP,
        Windows::Win32::System::Diagnostics::Debug::{SetLastError, WIN32_ERROR},
        Windows::Win32::System::Threading::LPPROC_THREAD_ATTRIBUTE_LIST,
    };
}
