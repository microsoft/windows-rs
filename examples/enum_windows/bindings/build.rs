fn main() {
    windows::build!(
        windows::win32::windows_and_messaging::{EnumWindows, GetWindowTextW}
    );
}
