fn main() {
    // Test for https://github.com/microsoft/win32metadata/issues/449
    windows::build! {
        Windows::Win32::System::ComponentServices::ITransactionImport,
    };
}
