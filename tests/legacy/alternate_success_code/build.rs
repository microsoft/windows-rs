fn main() {
    windows::build! {
        Windows::Win32::Foundation::CO_E_NOTINITIALIZED, Windows::Win32::System::Com::DoDragDrop,
    };
}
