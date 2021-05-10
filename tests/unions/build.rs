fn main() {
    windows::build!(
        Windows::Win32::System::SystemServices::OVERLAPPED,
        Windows::Win32::UI::ColorSystem::WhitePoint,
        Windows::Win32::Graphics::Direct3D12::{
            D3D12_INDIRECT_ARGUMENT_DESC, D3D12_INDIRECT_ARGUMENT_TYPE_VERTEX_BUFFER_VIEW,
            D3D12_INDIRECT_ARGUMENT_TYPE_CONSTANT,
        },
    );
}
