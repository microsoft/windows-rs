fn main() {
    windows::build!(
        windows::win32::system_services::MB_OK,
        windows::win32::windows_and_messaging::MessageBoxA,
    );
}
