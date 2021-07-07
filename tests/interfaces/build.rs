fn main() {
    windows::build! {
        // Test for https://github.com/microsoft/win32metadata/issues/449
        Windows::Win32::System::ComponentServices::ITransactionImport,

        // Test for https://github.com/microsoft/windows-rs/issues/924
        Windows::Win32::System::WinRT::{ICompositorInterop, ISystemMediaTransportControlsInterop},
    };
}
