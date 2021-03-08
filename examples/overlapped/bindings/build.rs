fn main() {
    windows::build!(
      windows::win32::file_system::{CreateFileA, ReadFile},
      windows::win32::system_services::{CreateEventA, WaitForSingleObject, GetOverlappedResult, ERROR_IO_PENDING},
      windows::win32::debug::GetLastError,
      windows::win32::windows_programming::CloseHandle,
    );
}
