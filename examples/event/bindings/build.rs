fn main() {
    windows::build! {
        Windows::Win32::Foundation::CloseHandle,
        Windows::Win32::System::Threading::{CreateEventW, SetEvent, WaitForSingleObject},
    };
}
