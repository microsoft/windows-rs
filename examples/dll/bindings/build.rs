fn main() {
    windows::build!(
        Windows::Win32::System::SystemServices::{HINSTANCE, DLL_PROCESS_ATTACH, DLL_THREAD_ATTACH, DLL_THREAD_DETACH, DLL_PROCESS_DETACH},
        Windows::Win32::UI::WindowsAndMessaging::{MessageBoxA, MB_OK},
    );
}
