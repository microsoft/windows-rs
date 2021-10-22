fn main() {
    windows::runtime::build! {
        // Test for https://github.com/microsoft/windows-rs/issues/738 as DebugPropertyInfo has a
        // BSTR field and needs to implement Clone.
        Windows::Win32::System::Diagnostics::Debug::DebugPropertyInfo,

        // Test to verify property key constants are generated
        // https://github.com/microsoft/win32metadata/issues/339
        Windows::Win32::System::SystemServices::DEVPKEY_Device_BiosDeviceName,

        // Test to verify format parameters and traits are passed on to the DebugStruct builder
        // in generated struct Debug implementations.
        // https://github.com/microsoft/windows-rs/issues/439
        Windows::Win32::UI::WindowsAndMessaging::CWPSTRUCT,
    };
}
