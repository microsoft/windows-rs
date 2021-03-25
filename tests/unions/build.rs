fn main() {
    windows::build!(
        Windows::Win32::SystemServices::OVERLAPPED,
        Windows::Win32::WindowsColorSystem::WhitePoint,
        Windows::Win32::Direct3D12::D3D12_INDIRECT_ARGUMENT_DESC,
    );
}
