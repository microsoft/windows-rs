fn main() {
    windows::build!(
        Windows::Win32::System::SystemServices::{BOOL, DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, DLL_THREAD_ATTACH, DLL_THREAD_DETACH, HINSTANCE},
    );
}
