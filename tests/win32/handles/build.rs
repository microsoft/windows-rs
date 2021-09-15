fn main() {
    windows::build! {
        Windows::Win32::Foundation::HWND,
        Windows::Win32::System::Diagnostics::Debug::{SetLastError, WIN32_ERROR},
    };
}
