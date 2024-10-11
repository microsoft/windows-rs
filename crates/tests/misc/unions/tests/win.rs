use windows::Win32::{
    Foundation::HANDLE,
    Graphics::Direct3D12::*,
    System::IO::{OVERLAPPED, OVERLAPPED_0_0},
};

#[test]
fn test() {
    let mut o = OVERLAPPED {
        Internal: 1,
        InternalHigh: 2,
        Anonymous: unsafe { std::mem::zeroed() },
        hEvent: Default::default(),
    };

    assert_eq!(o.Internal, 1);
    o.Internal = 10;
    assert_eq!(o.Internal, 10);

    assert_eq!(o.InternalHigh, 2);
    o.InternalHigh = 20;
    assert_eq!(o.InternalHigh, 20);

    assert_eq!(o.hEvent, HANDLE(0 as _));
    o.hEvent = HANDLE(1 as _);
    assert_eq!(o.hEvent, HANDLE(1 as _));

    unsafe {
        assert_eq!(o.Anonymous.Pointer, core::ptr::null_mut());
    }

    unsafe {
        assert_eq!(o.Anonymous.Anonymous.Offset, 0);
        assert_eq!(o.Anonymous.Anonymous.OffsetHigh, 0);
    }

    o.Anonymous.Anonymous.Offset = 100;
    o.Anonymous.Anonymous.OffsetHigh = 200;

    unsafe {
        assert_eq!(o.Anonymous.Anonymous.Offset, 100);
        assert_eq!(o.Anonymous.Anonymous.OffsetHigh, 200);
    }

    o.Anonymous.Anonymous = OVERLAPPED_0_0 {
        Offset: 10,
        OffsetHigh: 20,
    };

    unsafe {
        assert_eq!(o.Anonymous.Anonymous.Offset, 10);
        assert_eq!(o.Anonymous.Anonymous.OffsetHigh, 20);
    }
}

#[test]
#[cfg(target_arch = "aarch64")]
fn test_arch() {
    assert_eq!(core::mem::size_of::<OVERLAPPED>(), 32);
}

#[test]
#[cfg(target_arch = "x86_64")]
fn test_arch() {
    assert_eq!(core::mem::size_of::<OVERLAPPED>(), 32);
}

#[test]
#[cfg(target_arch = "x86")]
fn test_arch() {
    assert_eq!(core::mem::size_of::<OVERLAPPED>(), 20);
}

#[test]
fn d3d() {
    assert_eq!(core::mem::size_of::<D3D12_INDIRECT_ARGUMENT_DESC>(), 16);

    assert_eq!(D3D12_INDIRECT_ARGUMENT_TYPE_VERTEX_BUFFER_VIEW.0, 3);

    let mut desc = D3D12_INDIRECT_ARGUMENT_DESC {
        Type: D3D12_INDIRECT_ARGUMENT_TYPE_VERTEX_BUFFER_VIEW,
        Anonymous: D3D12_INDIRECT_ARGUMENT_DESC_0 {
            VertexBuffer: D3D12_INDIRECT_ARGUMENT_DESC_0_0 { Slot: 123 },
        },
    };

    assert_eq!(desc.Type, D3D12_INDIRECT_ARGUMENT_TYPE_VERTEX_BUFFER_VIEW);

    desc.Type = D3D12_INDIRECT_ARGUMENT_TYPE_CONSTANT;

    assert_eq!(desc.Type, D3D12_INDIRECT_ARGUMENT_TYPE_CONSTANT);

    unsafe {
        assert_eq!(desc.Anonymous.VertexBuffer.Slot, 123);
    }
}
