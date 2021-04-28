fn main() {
    windows::build!(
        // Test for https://github.com/microsoft/windows-rs/issues/738 as DebugPropertyInfo has a
        // BSTR field and needs to implement Clone.
        Windows::Win32::Debug::DebugPropertyInfo
    );
}
