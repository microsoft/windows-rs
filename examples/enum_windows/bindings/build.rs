fn main() {
    windows::build!(
        Windows::Win32::WindowsAndMessaging::{EnumWindows, GetWindowTextW}
    );
}
