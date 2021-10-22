use test_win32::Windows::Win32::{
    Foundation::{CloseHandle, BOOL, HANDLE, HWND, PSTR, PWSTR, RECT},
    Gaming::HasExpandedResources,
    Graphics::{
        Direct2D::CLSID_D2D1Shadow, Direct3D11::D3DDisassemble11Trace,
        Direct3D12::D3D12_DEFAULT_BLEND_FACTOR_ALPHA, Dxgi::*, Hlsl::D3DCOMPILER_DLL,
    },
    Networking::Ldap::ldapsearch,
    Security::Authorization::*,
    Storage::StructuredStorage::*,
    System::{Com::CreateUri, Diagnostics::Debug::*, Threading::*},
    UI::{
        Accessibility::UIA_ScrollPatternNoScroll,
        Animation::{UIAnimationManager, UIAnimationTransitionLibrary},
        WindowsAndMessaging::{CHOOSECOLORW, PROPENUMPROCA, PROPENUMPROCW, WM_KEYUP},
    },
};

use windows::runtime::Guid;

#[test]
fn signed_enum32() {
    assert!(ACCESS_MODE::default() == 0.into());
    let e: ACCESS_MODE = REVOKE_ACCESS;
    assert!(e == REVOKE_ACCESS);
}

#[test]
fn unsigned_enum32() {
    assert!(DXGI_ADAPTER_FLAG::default() == 0.into());

    let both = DXGI_ADAPTER_FLAG_SOFTWARE | DXGI_ADAPTER_FLAG_REMOTE;
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
        Format: DXGI_FORMAT_R32_TYPELESS,
        ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER_PROGRESSIVE,
        Scaling: DXGI_MODE_SCALING_CENTERED,
    };
}

#[cfg(target_pointer_width = "64")]
#[test]
fn size64() {
    assert!(std::mem::size_of::<ACCESS_MODE>() == 4);
    assert!(std::mem::size_of::<DXGI_ADAPTER_FLAG>() == 4);
    assert!(std::mem::size_of::<RECT>() == 16);
    assert!(std::mem::size_of::<DXGI_MODE_DESC>() == 28);
    assert_eq!(std::mem::size_of::<CHOOSECOLORW>(), 72);
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
    assert!(WM_KEYUP == 257u32);
    assert!(D3D12_DEFAULT_BLEND_FACTOR_ALPHA == 1f32);
    assert!(UIA_ScrollPatternNoScroll == -1f64);
    assert!(D3DCOMPILER_DLL == "d3dcompiler_47.dll");
    assert!(CLSID_D2D1Shadow == Guid::from("C67EA361-1863-4e69-89DB-695D3E9A5B6B"));
}

#[test]
fn function() -> windows::runtime::Result<()> {
    unsafe {
        let event = CreateEventW(
            std::ptr::null_mut(),
            true,
            false,
            PWSTR(std::ptr::null_mut()),
        );
        assert!(event.0 != 0);

        SetEvent(event).ok()?;

        let result = WaitForSingleObject(event, 0);
        assert!(result == WAIT_OBJECT_0);

        CloseHandle(event).ok()?;
        Ok(())
    }
}

#[test]
fn bool_as_error() {
    unsafe {
        assert!(helpers::set_thread_ui_language("en-US"));
        assert!(!SetEvent(HANDLE(0)).as_bool());

        let result: windows::runtime::Result<()> = SetEvent(HANDLE(0)).ok();
        assert!(result.is_err());

        let error: windows::runtime::Error = result.unwrap_err();
        assert_eq!(error.code(), windows::runtime::HRESULT(0x8007_0006));
        assert_eq!(error.message(), "The handle is invalid.");
    }
}

#[test]
fn com() -> windows::runtime::Result<()> {
    unsafe {
        let stream = CreateStreamOnHGlobal(0, true)?;
        let values = vec![1, 20, 300, 4000];
        let copied = stream.Write(
            values.as_ptr() as _,
            (values.len() * std::mem::size_of::<i32>()) as u32,
        )?;

        assert!(copied == (values.len() * std::mem::size_of::<i32>()) as u32);

        let copied = stream.Write(
            &UIAnimationTransitionLibrary as *const _ as _,
            std::mem::size_of::<windows::runtime::Guid>() as u32,
        )?;

        assert!(copied == std::mem::size_of::<windows::runtime::Guid>() as u32);
        let position = stream.Seek(0, STREAM_SEEK_SET)?;

        assert!(position == 0);
        let mut values = vec![0, 0, 0, 0];
        let mut copied = 0;

        stream.Read(
            values.as_mut_ptr() as _,
            (values.len() * std::mem::size_of::<i32>()) as u32,
            &mut copied,
        )?;

        assert!(copied == (values.len() * std::mem::size_of::<i32>()) as u32);
        assert!(values == vec![1, 20, 300, 4000]);
        let mut value: windows::runtime::Guid = windows::runtime::Guid::default();
        let mut copied = 0;

        stream.Read(
            &mut value as *mut _ as _,
            std::mem::size_of::<windows::runtime::Guid>() as u32,
            &mut copied,
        )?;

        assert!(copied == std::mem::size_of::<windows::runtime::Guid>() as u32);
        assert!(value == UIAnimationTransitionLibrary);
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
        assert!(
            factory
                .RegisterAdaptersChangedEvent(HANDLE(0))
                .unwrap_err()
                .code()
                == DXGI_ERROR_INVALID_CALL
        );
    }
}

// Tests for https://github.com/microsoft/windows-rs/issues/463
#[test]
fn onecore_imports() -> windows::runtime::Result<()> {
    unsafe {
        HasExpandedResources()?;

        let uri = CreateUri(
            PWSTR(
                windows::runtime::HSTRING::from("http://kennykerr.ca")
                    .as_wide()
                    .as_ptr() as _,
            ),
            Default::default(),
            0,
        )?;

        let port = uri.GetPort()?;
        assert!(port == 80);

        let result = MiniDumpWriteDump(
            None,
            0,
            None,
            MiniDumpNormal,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        assert!(!result.as_bool());

        assert!(D3DDisassemble11Trace(std::ptr::null_mut(), 0, None, 0, 0, 0).is_err());

        Ok(())
    }
}

#[test]
fn interface() -> windows::runtime::Result<()> {
    unsafe {
        let uri = CreateUri("http://kennykerr.ca", Default::default(), 0)?;

        let domain = uri.GetDomain()?;
        assert!(domain == "kennykerr.ca");
    }
    Ok(())
}

#[test]
fn callback() {
    unsafe {
        let a: PROPENUMPROCA = callback_a;
        assert!(BOOL(789) == a(HWND(123), PSTR("hello a\0".as_ptr() as _), HANDLE(456)));

        let a: PROPENUMPROCW = callback_w;
        assert!(
            BOOL(789)
                == a(
                    HWND(123),
                    PWSTR(
                        windows::runtime::HSTRING::from("hello w\0")
                            .as_wide()
                            .as_ptr() as _
                    ),
                    HANDLE(456)
                )
        );
    }
}

// TODO: second parameter should be *const i8
extern "system" fn callback_a(param0: HWND, param1: PSTR, param2: HANDLE) -> BOOL {
    unsafe {
        assert!(param0.0 == 123);
        assert!(param2.0 == 456);
        let mut len = 0;
        let mut end = param1.0;

        loop {
            if *end == 0 {
                break;
            }

            len += 1;
            end = end.add(1);
        }

        let s = String::from_utf8_lossy(std::slice::from_raw_parts(param1.0 as *const u8, len))
            .into_owned();
        assert!(s == "hello a");
        BOOL(789)
    }
}

// TODO: second parameter should be *const u16
extern "system" fn callback_w(param0: HWND, param1: PWSTR, param2: HANDLE) -> BOOL {
    unsafe {
        assert!(param0.0 == 123);
        assert!(param2.0 == 456);
        let mut len = 0;
        let mut end = param1.0;

        loop {
            if *end == 0 {
                break;
            }

            len += 1;
            end = end.add(1);
        }

        let s = String::from_utf16_lossy(std::slice::from_raw_parts(param1.0, len));
        assert!(s == "hello w");
        BOOL(789)
    }
}

#[test]
fn empty_struct() {
    let ldap = ldapsearch(123);
    assert!(ldap.0 == 123);
    assert!(std::mem::size_of::<ldapsearch>() == 1);

    assert!(UIAnimationManager == Guid::from("4C1FC63A-695C-47E8-A339-1A194BE3D0B8"));
}
