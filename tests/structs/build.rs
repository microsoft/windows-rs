fn main() {
    // Test for https://github.com/microsoft/windows-rs/issues/738 as DebugPropertyInfo has a
    // BSTR field and needs to implement Clone.
    windows::build! {
        Windows::Win32::System::Diagnostics::Debug::DebugPropertyInfo,
    };
}
