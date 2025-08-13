use windows::{
    core::*,
    Win32::Foundation::{CloseHandle, HANDLE, HWND, RECT},
    Win32::Gaming::HasExpandedResources,
    Win32::Graphics::Direct3D::Fxc::*,
    Win32::Graphics::{
        Direct2D::CLSID_D2D1Shadow, Direct3D11::D3DDisassemble11Trace,
        Direct3D12::D3D12_DEFAULT_BLEND_FACTOR_ALPHA, Dxgi::Common::*, Dxgi::*,
    },
    Win32::Networking::Ldap::PLDAPSearch,
    Win32::Security::Authorization::*,
    Win32::System::Com::StructuredStorage::*,
    Win32::System::Com::*,
    Win32::System::{Diagnostics::Debug::*, Threading::*},
    Win32::UI::{
        Accessibility::UIA_ScrollPatternNoScroll,
        Animation::UIAnimationManager,
        Controls::Dialogs::CHOOSECOLORW,
        WindowsAndMessaging::{PROPENUMPROCA, PROPENUMPROCW, WM_KEYUP},
    },
};

#[test]
fn signed_enum32() {
    assert!(ACCESS_MODE::default().0 == 0);
    let e: ACCESS_MODE = REVOKE_ACCESS;
    assert!(e == REVOKE_ACCESS);
}

#[test]
fn unsigned_enum32() {
    assert!(DXGI_ADAPTER_FLAG::default().0 == 0);

    let both = DXGI_ADAPTER_FLAG_SOFTWARE | DXGI_ADAPTER_FLAG_REMOTE;
    assert!(both.0 == 3);
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

    let clone = rect;

    assert!(
        clone
            == RECT {
                left: 1,
                top: 2,
                right: 3,
                bottom: 4
            }
    );
}

#[test]
fn dxgi_mode_desc() {
    _ = DXGI_MODE_DESC {
        Width: 1,
        Height: 2,
        RefreshRate: DXGI_RATIONAL {
            Numerator: 3,
            Denominator: 5,
        },
        Format: DXGI_FORMAT_R32_TYPELESS,
        ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER_PROGRESSIVE,
        Scaling: DXGI_MODE_SCALING_CENTERED,
    };
}

#[cfg(target_pointer_width = "64")]
#[test]
fn size64() {
    assert!(core::mem::size_of::<ACCESS_MODE>() == 4);
    assert!(core::mem::size_of::<DXGI_ADAPTER_FLAG>() == 4);
    assert!(core::mem::size_of::<RECT>() == 16);
    assert!(core::mem::size_of::<DXGI_MODE_DESC>() == 28);
    assert_eq!(core::mem::size_of::<CHOOSECOLORW>(), 72);
}

#[cfg(target_pointer_width = "32")]
#[test]
fn size32() {
    assert!(core::mem::size_of::<ACCESS_MODE>() == 4);
    assert!(core::mem::size_of::<DXGI_ADAPTER_FLAG>() == 4);
    assert!(core::mem::size_of::<RECT>() == 16);
    assert!(core::mem::size_of::<DXGI_MODE_DESC>() == 28);
    assert!(core::mem::size_of::<CHOOSECOLORW>() == 36);
}

#[test]
fn constant() {
    assert_eq!(WM_KEYUP, 257u32);
    assert_eq!(D3D12_DEFAULT_BLEND_FACTOR_ALPHA, 1f32);
    assert_eq!(UIA_ScrollPatternNoScroll, -1f64);
    assert_eq!(
        CLSID_D2D1Shadow,
        GUID::try_from("C67EA361-1863-4e69-89DB-695D3E9A5B6B").unwrap()
    );

    let b: PCSTR = D3DCOMPILER_DLL_A;
    let c: PCWSTR = D3DCOMPILER_DLL_W;

    unsafe {
        assert_eq!(b.to_string().unwrap(), "d3dcompiler_47.dll");
        assert_eq!(c.to_string().unwrap(), "d3dcompiler_47.dll");
    }
}

#[test]
fn function() -> windows::core::Result<()> {
    unsafe {
        let event = CreateEventW(None, true, false, None)?;
        SetEvent(event)?;

        WaitForSingleObject(event, 0);

        CloseHandle(event)?;
        Ok(())
    }
}

#[test]
fn bool_as_error() {
    unsafe {
        helpers::set_thread_ui_language();
        let error = SetEvent(HANDLE(0 as _)).unwrap_err();

        assert_eq!(error.code(), windows::core::HRESULT(-2147024890));
        let message: String = error.message();
        assert_eq!(message, "The handle is invalid.");
    }
}

#[test]
fn com() -> windows::core::Result<()> {
    unsafe {
        let stream = CreateStreamOnHGlobal(Default::default(), true)?;
        let values = [1u8, 2u8, 3u8, 4u8];

        let mut copied = 0;
        stream
            .Write(values.as_ptr() as _, values.len() as u32, Some(&mut copied))
            .ok()?;
        assert!(copied == 4);

        let mut position = 0;
        stream.Seek(0, STREAM_SEEK_SET, Some(&mut position))?;
        assert!(position == 0);

        let mut values = vec![0, 0, 0, 0];
        let mut copied = 0;
        stream
            .Read(
                values.as_mut_ptr() as _,
                values.len() as u32,
                Some(&mut copied),
            )
            .ok()?;
        assert!(copied == 4);
        assert_eq!(values, vec![1u8, 2u8, 3u8, 4u8]);
    }

    Ok(())
}

#[test]
fn com_inheritance() {
    unsafe {
        let factory: IDXGIFactory7 = CreateDXGIFactory1().unwrap();

        // IDXGIFactory
        assert!(factory
            .MakeWindowAssociation(HWND::default(), DXGI_MWA_FLAGS::default())
            .is_ok());

        // IDXGIFactory1
        assert!(factory.IsCurrent().as_bool());

        // IDXGIFactory2
        _ = factory.IsWindowedStereoEnabled();

        // IDXGIFactory3
        assert!(factory.GetCreationFlags() == DXGI_CREATE_FACTORY_FLAGS(0));

        // IDXGIFactory7 (default)
        assert!(
            factory
                .RegisterAdaptersChangedEvent(HANDLE::default())
                .unwrap_err()
                .code()
                == DXGI_ERROR_INVALID_CALL
        );
    }
}

// Tests for https://github.com/microsoft/windows-rs/issues/463
#[test]
fn onecore_imports() -> windows::core::Result<()> {
    unsafe {
        _ = HasExpandedResources()?;

        let uri = CreateUri(w!("http://kennykerr.ca"), URI_CREATE_FLAGS::default(), None)?;

        let port = uri.GetPort()?;
        assert!(port == 80);

        MiniDumpWriteDump(
            Default::default(),
            0,
            Default::default(),
            MiniDumpNormal,
            None,
            None,
            None,
        )
        .unwrap_err();

        assert!(D3DDisassemble11Trace(std::ptr::null(), 0, None, 0, 0, 0).is_err());

        Ok(())
    }
}

#[test]
fn interface() -> windows::core::Result<()> {
    unsafe {
        let uri = CreateUri(w!("http://kennykerr.ca"), URI_CREATE_FLAGS::default(), None)?;

        let domain = uri.GetDomain()?;
        assert!(domain == "kennykerr.ca");
    }
    Ok(())
}

#[test]
#[expect(clippy::unnecessary_literal_unwrap)] // callback type is intentionally being tested
fn callback() {
    unsafe {
        let a: PROPENUMPROCA = Some(callback_a);
        assert!(BOOL(789) == a.unwrap()(HWND(123 as _), s!("hello a"), HANDLE(456 as _)));

        let a: PROPENUMPROCW = Some(callback_w);
        assert!(BOOL(789) == a.unwrap()(HWND(123 as _), w!("hello w"), HANDLE(456 as _)));
    }
}

extern "system" fn callback_a(param0: HWND, param1: PCSTR, param2: HANDLE) -> BOOL {
    assert!(param0.0 == 123 as _);
    assert!(param2.0 == 456 as _);

    let s = unsafe { param1.to_string().unwrap() };
    assert!(s == "hello a");
    BOOL(789)
}

extern "system" fn callback_w(param0: HWND, param1: PCWSTR, param2: HANDLE) -> BOOL {
    assert!(param0.0 == 123 as _);
    assert!(param2.0 == 456 as _);
    let s = unsafe { param1.to_string().unwrap() };
    assert!(s == "hello w");
    BOOL(789)
}

#[test]
fn empty_struct() {
    let ldap = PLDAPSearch(123);
    assert!(ldap.0 == 123);
    assert!(core::mem::size_of::<PLDAPSearch>() == core::mem::size_of::<usize>());

    assert!(UIAnimationManager == GUID::try_from("4C1FC63A-695C-47E8-A339-1A194BE3D0B8").unwrap());
}
