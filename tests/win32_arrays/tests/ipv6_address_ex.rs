use test_win32_arrays::windows::win32::ip_helper::IPV6_ADDRESS_EX;

#[test]
fn test() {
    assert_eq!(std::mem::size_of::<IPV6_ADDRESS_EX>(), 26);
}
