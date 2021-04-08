fn main() {
    windows::build!(
        Windows::Win32::SystemServices::{HANDLE, PSTR, PWSTR},
    );
}
