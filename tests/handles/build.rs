fn main() {
    windows::build!(
        Windows::Win32::Foundation::{HANDLE, PSTR, PWSTR},
        Windows::Win32::Graphics::Gdi::HGDIOBJ,
    );
}
