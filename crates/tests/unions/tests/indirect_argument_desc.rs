use windows::Win32::Graphics::Direct3D12::*;

#[test]
fn test() {
    assert_eq!(core::mem::size_of::<D3D12_INDIRECT_ARGUMENT_DESC>(), 16);

    assert_eq!(D3D12_INDIRECT_ARGUMENT_TYPE_VERTEX_BUFFER_VIEW.0, 3);

    let mut desc = D3D12_INDIRECT_ARGUMENT_DESC {
        Type: D3D12_INDIRECT_ARGUMENT_TYPE_VERTEX_BUFFER_VIEW,
        Anonymous: D3D12_INDIRECT_ARGUMENT_DESC_0 { VertexBuffer: D3D12_INDIRECT_ARGUMENT_DESC_0_4 { Slot: 123 } },
    };

    assert_eq!(desc.Type, D3D12_INDIRECT_ARGUMENT_TYPE_VERTEX_BUFFER_VIEW);

    desc.Type = D3D12_INDIRECT_ARGUMENT_TYPE_CONSTANT;

    assert_eq!(desc.Type, D3D12_INDIRECT_ARGUMENT_TYPE_CONSTANT);

    unsafe {
        assert_eq!(desc.Anonymous.VertexBuffer.Slot, 123);
    }
}
