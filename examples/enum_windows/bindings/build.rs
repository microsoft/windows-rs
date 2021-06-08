fn main() {
    windows::build! {
        Windows::Win32::UI::WindowsAndMessaging::{
            EnumWindows, GetWindowTextW,
        },
    };
}
