fn main() {
    windows::build!(
        Windows::Win32::System::OleAutomation::BSTR,
        Windows::Win32::System::Com::CreateUri,
    );
}
