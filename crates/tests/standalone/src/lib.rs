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
        CoCreateInstance(std::ptr::null(), std::ptr::null_mut(), CLSCTX_ALL, std::ptr::null(), std::ptr::null_mut());
    }
}
