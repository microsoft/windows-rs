fn main() {
    windows::build!(
        windows::win32::system_services::OVERLAPPED,
        windows::win32::windows_color_system::WhitePoint,
    );
}
