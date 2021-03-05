use test_unions::windows::win32::system_services::OVERLAPPED;

#[test]
fn test() {}

#[test]
#[cfg(target_arch = "x86_64")]
fn test_arch() {
    assert!(std::mem::size_of::<OVERLAPPED>() == 32);
}

#[test]
#[cfg(target_arch = "x86")]
fn test_arch() {
    assert!(std::mem::size_of::<OVERLAPPED>() == 20);
}
