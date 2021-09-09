use test_win32_simple::Component::Win32::*;

#[test]
fn void() {
    unsafe {
        Void();
    }
}

#[test]
fn forty_two_returned() {
    assert_eq!(unsafe { ReturnFortyTwo() }, 42u32);
}
