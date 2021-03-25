fn main() {
    windows::build!(
      Windows::Win32::FileSystem::{CreateFileA, ReadFile},
      Windows::Win32::SystemServices::{
          CreateEventA, WaitForSingleObject, GetOverlappedResult, ERROR_IO_PENDING, INVALID_HANDLE_VALUE,
      },
      Windows::Win32::Debug::GetLastError,
      Windows::Win32::WindowsProgramming::CloseHandle,
    );
}
