use windows::Win32::NetworkManagement::IpHelper::IPV6_ADDRESS_EX;

#[test]
fn test() {
    assert_eq!(core::mem::size_of::<IPV6_ADDRESS_EX>(), 26);
}
