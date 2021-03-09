use test_unions::windows::win32::direct3d12::{
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
        r#type: D3D12_INDIRECT_ARGUMENT_TYPE::D3D12_INDIRECT_ARGUMENT_TYPE_VERTEX_BUFFER_VIEW,
        anonymous: D3D12_INDIRECT_ARGUMENT_DESC_0 {
            vertex_buffer: D3D12_INDIRECT_ARGUMENT_DESC_0_4 { slot: 123 },
        },
    };

    assert_eq!(
        desc.r#type,
        D3D12_INDIRECT_ARGUMENT_TYPE::D3D12_INDIRECT_ARGUMENT_TYPE_VERTEX_BUFFER_VIEW
    );

    desc.r#type = D3D12_INDIRECT_ARGUMENT_TYPE::D3D12_INDIRECT_ARGUMENT_TYPE_CONSTANT;

    assert_eq!(
        desc.r#type,
        D3D12_INDIRECT_ARGUMENT_TYPE::D3D12_INDIRECT_ARGUMENT_TYPE_CONSTANT
    );

    unsafe {
        assert_eq!(desc.anonymous.vertex_buffer.slot, 123);
    }
}
