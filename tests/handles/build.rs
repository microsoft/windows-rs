fn main() {
    windows::build! {
        Windows::Win32::Graphics::Gdi::HGDIOBJ,
        Windows::Win32::System::SystemServices::{HANDLE, PSTR, PWSTR},
    };
}
