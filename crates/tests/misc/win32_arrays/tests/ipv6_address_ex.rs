#![cfg(windows)]
use windows::Win32::IPV6_ADDRESS_EX;

#[test]
fn test() {
    assert_eq!(size_of::<IPV6_ADDRESS_EX>(), 26);
}
