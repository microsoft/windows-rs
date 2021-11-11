fn main() {
    windows::core::build! {Windows::Win32::Foundation::CO_E_NOTINITIALIZED, Windows::Win32::System::Ole::DoDragDrop};
}
