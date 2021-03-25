fn main() {
    windows::build!(
        Windows::Win32::Automation::BSTR,
        Windows::Win32::Com::CreateUri,
    );
}
