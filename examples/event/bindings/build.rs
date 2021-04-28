fn main() {
    windows::build!(
        Windows::Win32::System::Threading::{CreateEventW, SetEvent, WaitForSingleObject},
        Windows::Win32::System::WindowsProgramming::CloseHandle
    );
}
