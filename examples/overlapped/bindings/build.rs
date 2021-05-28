fn main() {
    windows::build!(
      Windows::Win32::Foundation::CloseHandle,
      Windows::Win32::Storage::FileSystem::{
        CreateFileA, ReadFile, FILE_GENERIC_READ, FILE_SHARE_READ, OPEN_EXISTING,
        FILE_FLAG_OVERLAPPED,
      },
      Windows::Win32::System::SystemServices::GetOverlappedResult,
      Windows::Win32::System::Threading::{
          CreateEventA, WaitForSingleObject, WAIT_OBJECT_0,
      },
      Windows::Win32::System::Diagnostics::Debug::{GetLastError, ERROR_IO_PENDING},
    );
}
