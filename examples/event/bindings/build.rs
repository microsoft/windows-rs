fn main() {
    windows::build!(
        windows::win32::system_services::{CreateEventW, SetEvent, WaitForSingleObject},
        windows::win32::windows_programming::CloseHandle
    );
}
