fn main() {
    windows::build!(
        Windows::Win32::UI::WindowsAndMessaging::{MessageBoxA, MB_OK}
    );
}
