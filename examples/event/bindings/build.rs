fn main() {
    windows::build!(
        Windows::Win32::SystemServices::{CreateEventW, SetEvent, WaitForSingleObject},
        Windows::Win32::WindowsProgramming::CloseHandle
    );
}
