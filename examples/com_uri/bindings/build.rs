fn main() {
    windows::build!(
        Windows::Win32::Foundation::BSTR,
        Windows::Win32::System::Com::CreateUri,
    );
}
