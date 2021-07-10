fn main() {
    windows::build! {
        // Test for https://github.com/microsoft/windows-rs/issues/738 as DebugPropertyInfo has a
        // BSTR field and needs to implement Clone.
        Windows::Win32::System::Diagnostics::Debug::DebugPropertyInfo,

        // Test to verify format parameters and traits are passed on to the DebugStruct builder
        // in generated struct Debug implementations.
        // https://github.com/microsoft/windows-rs/issues/439
        Windows::Win32::UI::WindowsAndMessaging::CWPSTRUCT,
    };
}
