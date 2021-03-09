fn main() {
    windows::build!(
        windows::win32::system_services::OVERLAPPED,
        windows::win32::windows_color_system::WhitePoint,
        windows::win32::direct3d12::D3D12_INDIRECT_ARGUMENT_DESC,
    );
}
