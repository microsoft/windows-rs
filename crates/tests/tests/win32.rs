use tests::windows::win32::{ACCESS_MODE, DXGI_ADAPTER_FLAG};
use winrt::Abi;

#[test]
fn signed_enum32() {
    assert!(ACCESS_MODE::default().abi() == 0);
    assert!(ACCESS_MODE::REVOKE_ACCESS.abi() == 4);
}

#[test]
fn unsigned_enum32() {
    assert!(DXGI_ADAPTER_FLAG::default().abi() == 0);
    assert!(DXGI_ADAPTER_FLAG::DXGI_ADAPTER_FLAG_SOFTWARE.abi() == 2);

    let both =
        DXGI_ADAPTER_FLAG::DXGI_ADAPTER_FLAG_SOFTWARE | DXGI_ADAPTER_FLAG::DXGI_ADAPTER_FLAG_REMOTE;
    assert!(both.abi() == 3);
}

#[test]
fn rect() {
    
}