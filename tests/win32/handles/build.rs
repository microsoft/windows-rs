fn main() {
    windows::build! {
        Windows::Win32::Foundation::{HANDLE, HWND},
        Windows::Win32::System::Diagnostics::Debug::{SetLastError, WIN32_ERROR},
    };
}
