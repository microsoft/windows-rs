fn main() {
    windows::build! {
        Windows::Win32::System::Com::CreateUri, Windows::Win32::System::OleAutomation::BSTR,
    };
}
