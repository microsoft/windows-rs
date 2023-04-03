#![cfg(test)]

mod bindings;
use bindings::*;

#[test]
fn test() {
    unsafe {
        let event = CreateEventW(std::ptr::null(), 1, 0, std::ptr::null());
        SetEvent(event);
        WaitForSingleObject(event, 0);
        CloseHandle(event);
        CoCreateInstance(
            std::ptr::null(),
            std::ptr::null_mut(),
            CLSCTX_ALL,
            std::ptr::null(),
            std::ptr::null_mut(),
        );
        assert_eq!(STGTY_REPEAT, 256);

        let expected = GUID::from_u128(0x4c1fc63a_695c_47e8_a339_1a194be3d0b8);
        assert!(UIAnimationManager.data1 == expected.data1);
        assert!(UIAnimationManager.data2 == expected.data2);
        assert!(UIAnimationManager.data3 == expected.data3);
        assert!(UIAnimationManager.data4 == expected.data4);
    }
}
