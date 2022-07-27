use windows::{
    core::*,
    Win32::Foundation::{CloseHandle, BOOL, HANDLE, HWND, RECT, WAIT_OBJECT_0},
    Win32::Gaming::HasExpandedResources,
    Win32::Graphics::{Direct2D::CLSID_D2D1Shadow, Direct3D11::D3DDisassemble11Trace, Direct3D12::D3D12_DEFAULT_BLEND_FACTOR_ALPHA, Dxgi::Common::*, Dxgi::*, Hlsl::D3DCOMPILER_DLL},
    Win32::Networking::Ldap::ldapsearch,
    Win32::Security::Authorization::*,
    Win32::System::Com::StructuredStorage::*,
    Win32::System::Com::*,
    Win32::System::{Com::CreateUri, Diagnostics::Debug::*, Threading::*},
    Win32::UI::{
        Accessibility::UIA_ScrollPatternNoScroll,
        Animation::UIAnimationManager,
        Controls::Dialogs::CHOOSECOLORW,
        WindowsAndMessaging::{PROPENUMPROCA, PROPENUMPROCW, WM_KEYUP},
    },
};

use std::convert::TryInto;
use windows::core::GUID;

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
    let rect = RECT { left: 1, top: 2, right: 3, bottom: 4 };

    assert!(rect.left == 1);
    assert!(rect.top == 2);
    assert!(rect.right == 3);
    assert!(rect.bottom == 4);

    let clone = rect.clone();

    assert!(clone == RECT { left: 1, top: 2, right: 3, bottom: 4 });
}

#[test]
fn dxgi_mode_desc() {
    let _ = DXGI_MODE_DESC {
        Width: 1,
        Height: 2,
        RefreshRate: DXGI_RATIONAL { Numerator: 3, Denominator: 5 },
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
    assert!(WM_KEYUP == 257u32);
    assert!(D3D12_DEFAULT_BLEND_FACTOR_ALPHA == 1f32);
    assert!(UIA_ScrollPatternNoScroll == -1f64);
    assert!(D3DCOMPILER_DLL == "d3dcompiler_47.dll");
    assert!(CLSID_D2D1Shadow == GUID::from("C67EA361-1863-4e69-89DB-695D3E9A5B6B"));
}

#[test]
fn function() -> windows::core::Result<()> {
    unsafe {
        let event = CreateEventW(None, true, false, None)?;
        SetEvent(event).ok()?;

        let result = WaitForSingleObject(event, 0);
        assert!(result == WAIT_OBJECT_0.0);

        CloseHandle(event).ok()?;
        Ok(())
    }
}

#[test]
fn bool_as_error() {
    unsafe {
        assert!(helpers::set_thread_ui_language());
        assert!(!SetEvent(HANDLE(0)).as_bool());

        let result: windows::core::Result<()> = SetEvent(HANDLE(0)).ok();
        assert!(result.is_err());

        let error: windows::core::Error = result.unwrap_err();
        assert_eq!(error.code(), windows::core::HRESULT(-2147024890));
        let message: String = error.message().try_into().unwrap();
        assert_eq!(message, "The handle is invalid.");
    }
}

#[test]
fn com() -> windows::core::Result<()> {
    unsafe {
        let stream = CreateStreamOnHGlobal(0, true)?;
        let values = vec![1, 2, 3, 4];

        let mut copied = 0;
        stream.Write(&values, Some(&mut copied)).ok()?;
        assert!(copied == 4);

        let position = stream.Seek(0, STREAM_SEEK_SET)?;
        assert!(position == 0);

        let mut values = vec![0, 0, 0, 0];
        let mut copied = 0;
        stream.Read(&mut values, Some(&mut copied)).ok()?;
        assert!(copied == 4);
        assert!(values == vec![1, 2, 3, 4]);
    }

    Ok(())
}

#[test]
fn com_inheritance() {
    unsafe {
        let factory: IDXGIFactory7 = CreateDXGIFactory1().unwrap();

        // IDXGIFactory
        assert!(factory.MakeWindowAssociation(HWND(0), 0).is_ok());

        // IDXGIFactory1
        assert!(factory.IsCurrent().as_bool());

        // IDXGIFactory2
        factory.IsWindowedStereoEnabled();

        // IDXGIFactory3
        assert!(factory.GetCreationFlags() == 0);

        // IDXGIFactory7 (default)
        assert!(factory.RegisterAdaptersChangedEvent(HANDLE(0)).unwrap_err().code() == DXGI_ERROR_INVALID_CALL);
    }
}

// Tests for https://github.com/microsoft/windows-rs/issues/463
#[test]
fn onecore_imports() -> windows::core::Result<()> {
    unsafe {
        HasExpandedResources()?;

        let uri = CreateUri(w!("http://kennykerr.ca"), URI_CREATE_FLAGS::default(), 0)?;

        let port = uri.GetPort()?;
        assert!(port == 80);

        let result = MiniDumpWriteDump(None, 0, None, MiniDumpNormal, None, None, None);
        assert!(!result.as_bool());

        assert!(D3DDisassemble11Trace(&[], None, 0, 0, 0).is_err());

        Ok(())
    }
}

#[test]
fn interface() -> windows::core::Result<()> {
    unsafe {
        let uri = CreateUri(w!("http://kennykerr.ca"), URI_CREATE_FLAGS::default(), 0)?;

        let domain = uri.GetDomain()?;
        assert!(domain == "kennykerr.ca");
    }
    Ok(())
}

#[test]
fn callback() {
    unsafe {
        let a: PROPENUMPROCA = Some(callback_a);
        assert!(BOOL(789) == a.unwrap()(HWND(123), s!("hello a"), HANDLE(456)));

        let a: PROPENUMPROCW = Some(callback_w);
        assert!(BOOL(789) == a.unwrap()(HWND(123), w!("hello w").into(), HANDLE(456)));
    }
}

extern "system" fn callback_a(param0: HWND, param1: PCSTR, param2: HANDLE) -> BOOL {
    assert!(param0.0 == 123);
    assert!(param2.0 == 456);

    let s = unsafe { param1.to_string().unwrap() };
    assert!(s == "hello a");
    BOOL(789)
}

extern "system" fn callback_w(param0: HWND, param1: PCWSTR, param2: HANDLE) -> BOOL {
    assert!(param0.0 == 123);
    assert!(param2.0 == 456);
    let s = unsafe { param1.to_string().unwrap() };
    assert!(s == "hello w");
    BOOL(789)
}

#[test]
fn empty_struct() {
    let ldap = ldapsearch(123);
    assert!(ldap.0 == 123);
    assert!(core::mem::size_of::<ldapsearch>() == 1);

    assert!(UIAnimationManager == GUID::from("4C1FC63A-695C-47E8-A339-1A194BE3D0B8"));
}
