fn main() {
    windows::build! {
        Windows::Win32::Graphics::Direct3D12::D3D12_INDIRECT_ARGUMENT_DESC,
        Windows::Win32::System::SystemServices::OVERLAPPED,
        Windows::Win32::UI::ColorSystem::WhitePoint,
    };
}
