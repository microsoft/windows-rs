fn main() {
    windows::build! {
        Windows::Win32::Foundation::CloseHandle,
        Windows::Win32::Storage::FileSystem::{CreateFileA, ReadFile},
        Windows::Win32::System::Diagnostics::Debug::{GetLastError, WIN32_ERROR},
        Windows::Win32::System::SystemServices::GetOverlappedResult,
        Windows::Win32::System::Threading::{CreateEventA, WaitForSingleObject},
    };
}
