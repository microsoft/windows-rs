fn main() {
    windows::build!(
        // Test for https://github.com/microsoft/win32metadata/issues/449
        Windows::Win32::ComponentServices::ITransactionImport
    );
}
