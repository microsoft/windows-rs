use test_win32_arrays::Windows::Win32::Graphics::Dxgi::DXGI_ADAPTER_DESC1;

#[test]
#[cfg(target_arch = "x86_64")]
fn test_arch() {
    assert_eq!(std::mem::size_of::<DXGI_ADAPTER_DESC1>(), 312);
}

#[test]
#[cfg(target_arch = "x86")]
fn test_arch() {
    assert_eq!(std::mem::size_of::<DXGI_ADAPTER_DESC1>(), 296);
}
