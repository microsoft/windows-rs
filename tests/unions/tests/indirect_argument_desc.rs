use test_unions::Windows::Win32::Direct3D12::{
    D3D12_INDIRECT_ARGUMENT_DESC, D3D12_INDIRECT_ARGUMENT_DESC_0, D3D12_INDIRECT_ARGUMENT_DESC_0_4,
    D3D12_INDIRECT_ARGUMENT_TYPE,
};

#[test]
fn test() {
    assert_eq!(std::mem::size_of::<D3D12_INDIRECT_ARGUMENT_DESC>(), 16);

    assert_eq!(
        D3D12_INDIRECT_ARGUMENT_TYPE::D3D12_INDIRECT_ARGUMENT_TYPE_VERTEX_BUFFER_VIEW.0,
        3
    );

    let mut desc = D3D12_INDIRECT_ARGUMENT_DESC {
        Type: D3D12_INDIRECT_ARGUMENT_TYPE::D3D12_INDIRECT_ARGUMENT_TYPE_VERTEX_BUFFER_VIEW,
        Anonymous: D3D12_INDIRECT_ARGUMENT_DESC_0 {
            VertexBuffer: D3D12_INDIRECT_ARGUMENT_DESC_0_4 { Slot: 123 },
        },
    };

    assert_eq!(
        desc.Type,
        D3D12_INDIRECT_ARGUMENT_TYPE::D3D12_INDIRECT_ARGUMENT_TYPE_VERTEX_BUFFER_VIEW
    );

    desc.Type = D3D12_INDIRECT_ARGUMENT_TYPE::D3D12_INDIRECT_ARGUMENT_TYPE_CONSTANT;

    assert_eq!(
        desc.Type,
        D3D12_INDIRECT_ARGUMENT_TYPE::D3D12_INDIRECT_ARGUMENT_TYPE_CONSTANT
    );

    unsafe {
        assert_eq!(desc.Anonymous.VertexBuffer.Slot, 123);
    }
}
