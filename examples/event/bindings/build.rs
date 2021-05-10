fn main() {
    windows::build!(
        Windows::Win32::System::Threading::{
            CreateEventW, SetEvent, WaitForSingleObject, WAIT_OBJECT_0,
        },
        Windows::Win32::System::WindowsProgramming::CloseHandle
    );
}
