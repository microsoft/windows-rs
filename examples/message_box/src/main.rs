use bindings::{
    windows::win32::system_services::MB_OK,
    windows::win32::windows_and_messaging::{MessageBoxA, HWND},
};

fn main() {
    unsafe {
        MessageBoxA(HWND(0), "Hello", "World", MB_OK as u32);
    }
}
