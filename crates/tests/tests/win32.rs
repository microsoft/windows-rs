use tests::windows::win32::{
    ACCESS_MODE, DXGI_ADAPTER_FLAG, DXGI_FORMAT, DXGI_MODE_DESC, DXGI_MODE_SCALING,
    DXGI_MODE_SCANLINE_ORDER, DXGI_RATIONAL, RECT,
};
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
    let rect = RECT {
        left: 1,
        top: 2,
        right: 3,
        bottom: 4,
    };

    assert!(rect.left == 1);
    assert!(rect.top == 2);
    assert!(rect.right == 3);
    assert!(rect.bottom == 4);
}

#[test]
fn dxgi_mode_desc() {
    let _ = DXGI_MODE_DESC {
        Width: 1,
        Height: 2,
        RefreshRate: DXGI_RATIONAL {
            Numerator: 3,
            Denominator: 5,
        },
        Format: DXGI_FORMAT::DXGI_FORMAT_R32_TYPELESS,
        ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER::DXGI_MODE_SCANLINE_ORDER_PROGRESSIVE,
        Scaling: DXGI_MODE_SCALING::DXGI_MODE_SCALING_CENTERED,
    };
}
