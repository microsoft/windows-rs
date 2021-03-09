use test_unions::windows::win32::system_services::{
    HANDLE, OVERLAPPED, OVERLAPPED_0, OVERLAPPED_0_0,
};

#[test]
fn test() {
    let mut o = OVERLAPPED {
        internal: 1,
        internal_high: 2,
        anonymous: OVERLAPPED_0 {
            pointer: std::ptr::null_mut(),
        },
        h_event: HANDLE(0),
    };

    assert_eq!(o.internal, 1);
    o.internal = 10;
    assert_eq!(o.internal, 10);

    assert_eq!(o.internal_high, 2);
    o.internal_high = 20;
    assert_eq!(o.internal_high, 20);

    assert_eq!(o.h_event, HANDLE(0));
    o.h_event = HANDLE(1);
    assert_eq!(o.h_event, HANDLE(1));

    unsafe {
        assert_eq!(o.anonymous.pointer, std::ptr::null_mut());
    }

    unsafe {
        assert_eq!(o.anonymous.anonymous.offset, 0);
        assert_eq!(o.anonymous.anonymous.offset_high, 0);
    }

    o.anonymous.anonymous.offset = 100;
    o.anonymous.anonymous.offset_high = 200;

    unsafe {
        assert_eq!(o.anonymous.anonymous.offset, 100);
        assert_eq!(o.anonymous.anonymous.offset_high, 200);
    }

    o.anonymous.anonymous = OVERLAPPED_0_0 {
        offset: 10,
        offset_high: 20,
    };

    unsafe {
        assert_eq!(o.anonymous.anonymous.offset, 10);
        assert_eq!(o.anonymous.anonymous.offset_high, 20);
    }
}

#[test]
#[cfg(target_arch = "x86_64")]
fn test_arch() {
    assert_eq!(std::mem::size_of::<OVERLAPPED>(), 32);
}

#[test]
#[cfg(target_arch = "x86")]
fn test_arch() {
    assert_eq!(std::mem::size_of::<OVERLAPPED>(), 20);
}
