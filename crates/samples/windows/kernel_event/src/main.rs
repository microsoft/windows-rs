use windows::{
    Win32::Foundation::CloseHandle,
    Win32::System::Threading::{CreateEventW, SetEvent, WaitForSingleObject},
};

fn main() -> windows::core::Result<()> {
    unsafe {
        let event = CreateEventW(None, true, false, None)?;
        SetEvent(event)?;
        WaitForSingleObject(event, 0);
        CloseHandle(event)
    }
}
