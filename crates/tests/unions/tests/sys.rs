use windows_sys::Win32::System::IO::{OVERLAPPED, OVERLAPPED_0_0};

#[test]
fn test() {
    let mut o = OVERLAPPED {
        Internal: 1,
        InternalHigh: 2,
        Anonymous: unsafe { std::mem::zeroed() },
        hEvent: core::ptr::null_mut(),
    };

    assert_eq!(o.Internal, 1);
    o.Internal = 10;
    assert_eq!(o.Internal, 10);

    assert_eq!(o.InternalHigh, 2);
    o.InternalHigh = 20;
    assert_eq!(o.InternalHigh, 20);

    assert_eq!(o.hEvent, core::ptr::null_mut());
    o.hEvent = 1 as _;
    assert_eq!(o.hEvent, 1 as _);

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
