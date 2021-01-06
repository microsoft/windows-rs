use tests::windows::win32::{
    CloseHandle, CreateEventW, SetEvent, UIA_ScrollPatternNoScroll, WaitForSingleObject,
    ACCESS_MODE, ALLJOYN_BIG_ENDIAN, ALLJOYN_CRED_CERT_CHAIN, CHOOSECOLORW,
    D3D12_DEFAULT_BLEND_FACTOR_ALPHA, D3DCOMPILER_DLL, DXGI_ADAPTER_FLAG, DXGI_FORMAT,
    DXGI_MODE_DESC, DXGI_MODE_SCALING, DXGI_MODE_SCANLINE_ORDER, DXGI_RATIONAL, RECT, WM_KEYUP,
};
use winrt::Abi;

#[test]
fn signed_enum32() {
    assert!(ACCESS_MODE::default() == 0.into());
    assert!(ACCESS_MODE::REVOKE_ACCESS.abi() == ACCESS_MODE::REVOKE_ACCESS);
}

#[test]
fn unsigned_enum32() {
    assert!(DXGI_ADAPTER_FLAG::default() == 0.into());
    assert!(
        DXGI_ADAPTER_FLAG::DXGI_ADAPTER_FLAG_SOFTWARE.abi()
            == DXGI_ADAPTER_FLAG::DXGI_ADAPTER_FLAG_SOFTWARE
    );

    let both =
        DXGI_ADAPTER_FLAG::DXGI_ADAPTER_FLAG_SOFTWARE | DXGI_ADAPTER_FLAG::DXGI_ADAPTER_FLAG_REMOTE;
    assert!(both == 3.into());
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

    let clone = rect.clone();

    assert!(
        clone
            == RECT {
                left: 1,
                top: 2,
                right: 3,
                bottom: 4,
            }
    );
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

#[cfg(target_pointer_width = "64")]
#[test]
fn size64() {
    assert!(std::mem::size_of::<ACCESS_MODE>() == 4);
    assert!(std::mem::size_of::<DXGI_ADAPTER_FLAG>() == 4);
    assert!(std::mem::size_of::<RECT>() == 16);
    assert!(std::mem::size_of::<DXGI_MODE_DESC>() == 28);
    assert!(std::mem::size_of::<CHOOSECOLORW>() == 72);
}

#[cfg(target_pointer_width = "32")]
#[test]
fn size32() {
    assert!(std::mem::size_of::<ACCESS_MODE>() == 4);
    assert!(std::mem::size_of::<DXGI_ADAPTER_FLAG>() == 4);
    assert!(std::mem::size_of::<RECT>() == 16);
    assert!(std::mem::size_of::<DXGI_MODE_DESC>() == 28);
    assert!(std::mem::size_of::<CHOOSECOLORW>() == 36);
}

#[test]
fn constant() {
    assert!(ALLJOYN_BIG_ENDIAN == 66u8);
    assert!(ALLJOYN_CRED_CERT_CHAIN == 4u16);
    assert!(WM_KEYUP == 257i32);
    assert!(D3D12_DEFAULT_BLEND_FACTOR_ALPHA == 1f32);
    assert!(UIA_ScrollPatternNoScroll == -1f64);
    assert!(D3DCOMPILER_DLL == "d3dcompiler_47.dll");
}

#[test]
fn function() {
    unsafe {
        let event = CreateEventW(std::ptr::null_mut(), 1, 0, std::ptr::null_mut());
        assert!(event != 1);

        let result = SetEvent(event);
        assert!(result != 0);

        let result = WaitForSingleObject(event, 0);
        assert!(result == 0); // https://github.com/microsoft/win32metadata/issues/77

        let result = CloseHandle(event);
        assert!(result != 0);
    }
}
