fn main() {
    windows::build!(
        Windows::Win32::System::SystemServices::{HANDLE, PSTR, PWSTR},
        Windows::Win32::Graphics::Gdi::HGDIOBJ,
    );
}
