use tests::{
    windows::win32::com::CreateUri,
    windows::win32::debug::{MiniDumpWriteDump, MINIDUMP_TYPE},
    windows::win32::direct3d11::D3DDisassemble11Trace,
    windows::win32::direct3d12::D3D12_DEFAULT_BLEND_FACTOR_ALPHA,
    windows::win32::direct3d_hlsl::D3DCOMPILER_DLL,
    windows::win32::display_devices::RECT,
    windows::win32::dxgi::{
        CreateDXGIFactory1, IDXGIFactory7, DXGI_ADAPTER_FLAG, DXGI_FORMAT, DXGI_MODE_DESC,
        DXGI_MODE_SCALING, DXGI_MODE_SCANLINE_ORDER, DXGI_RATIONAL,
    },
    windows::win32::game_mode::HasExpandedResources,
    windows::win32::ldap::ldapsearch,
    windows::win32::security::ACCESS_MODE,
    windows::win32::structured_storage::{CreateStreamOnHGlobal, STREAM_SEEK},
    windows::win32::system_services::{
        CreateEventW, SetEvent, WaitForSingleObject, DXGI_ERROR_INVALID_CALL, HANDLE, WM_KEYUP,
    },
    windows::win32::upnp::UIAnimationManager,
    windows::win32::upnp::UIAnimationTransitionLibrary,
    windows::win32::windows_accessibility::UIA_ScrollPatternNoScroll,
    windows::win32::windows_and_messaging::{CHOOSECOLORW, HWND, PROPENUMPROCA, PROPENUMPROCW},
    windows::win32::windows_color_system::WhitePoint,
    windows::win32::windows_programming::CloseHandle,
    windows::{Abi, Guid, Interface, BOOL, FALSE},
};

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
        width: 1,
        height: 2,
        refresh_rate: DXGI_RATIONAL {
            numerator: 3,
            denominator: 5,
        },
        format: DXGI_FORMAT::DXGI_FORMAT_R32_TYPELESS,
        scanline_ordering: DXGI_MODE_SCANLINE_ORDER::DXGI_MODE_SCANLINE_ORDER_PROGRESSIVE,
        scaling: DXGI_MODE_SCALING::DXGI_MODE_SCALING_CENTERED,
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
    assert!(WM_KEYUP == 257i32);
    assert!(D3D12_DEFAULT_BLEND_FACTOR_ALPHA == 1f32);
    assert!(UIA_ScrollPatternNoScroll == -1f64);
    assert!(D3DCOMPILER_DLL == "d3dcompiler_47.dll");
}

#[test]
fn function() -> windows::Result<()> {
    unsafe {
        let event = CreateEventW(
            std::ptr::null_mut(),
            true.into(),
            false.into(),
            std::ptr::null_mut(),
        );
        assert!(event.0 != 0);

        SetEvent(event).ok()?;

        let result = WaitForSingleObject(event, 0);
        assert!(result == 0); // https://github.com/microsoft/win32metadata/issues/77

        CloseHandle(event).ok()?;
        Ok(())
    }
}

#[test]
fn bool_as_error() {
    unsafe {
        assert!(!SetEvent(HANDLE(0)).as_bool());

        let result: windows::Result<()> = SetEvent(HANDLE(0)).ok();
        assert!(result.is_err());

        let error: windows::Error = result.unwrap_err();
        assert_eq!(error.code(), windows::ErrorCode(0x8007_0006));
        assert_eq!(error.message(), "The handle is invalid.");
    }
}

#[test]
fn com() -> windows::Result<()> {
    unsafe {
        let mut stream = None;
        let stream = CreateStreamOnHGlobal(0, true.into(), &mut stream).and_some(stream)?;
        let values = vec![1, 20, 300, 4000];
        let mut copied = 0;

        stream
            .Write(
                values.as_ptr() as _,
                (values.len() * std::mem::size_of::<i32>()) as u32,
                &mut copied,
            )
            .ok()?;

        assert!(copied == (values.len() * std::mem::size_of::<i32>()) as u32);

        stream
            .Write(
                &UIAnimationTransitionLibrary as *const _ as _,
                std::mem::size_of::<windows::Guid>() as u32,
                &mut copied,
            )
            .ok()?;

        assert!(copied == std::mem::size_of::<windows::Guid>() as u32);
        let mut position = 123;

        stream
            .Seek(0, STREAM_SEEK::STREAM_SEEK_SET.0 as u32, &mut position)
            .ok()?;

        assert!(position == 0);
        let mut values = vec![0, 0, 0, 0];
        let mut copied = 0;

        stream
            .Read(
                values.as_mut_ptr() as _,
                (values.len() * std::mem::size_of::<i32>()) as u32,
                &mut copied,
            )
            .ok()?;

        assert!(copied == (values.len() * std::mem::size_of::<i32>()) as u32);
        assert!(values == vec![1, 20, 300, 4000]);
        let mut value: windows::Guid = windows::Guid::default();
        let mut copied = 0;

        stream
            .Read(
                &mut value as *mut _ as _,
                std::mem::size_of::<windows::Guid>() as u32,
                &mut copied,
            )
            .ok()?;

        assert!(copied == std::mem::size_of::<windows::Guid>() as u32);
        assert!(value == UIAnimationTransitionLibrary);
    }

    Ok(())
}

#[test]
fn com_inheritance() {
    unsafe {
        let mut factory: Option<IDXGIFactory7> = None;
        let factory: IDXGIFactory7 = CreateDXGIFactory1(&IDXGIFactory7::IID, factory.set_abi())
            .and_some(factory)
            .unwrap();

        // IDXGIFactory
        assert!(
            factory.GetWindowAssociation(std::ptr::null_mut()).0 == DXGI_ERROR_INVALID_CALL as u32
        );

        // IDXGIFactory1
        assert!(factory.IsCurrent().as_bool());

        // IDXGIFactory2
        factory.IsWindowedStereoEnabled();

        // IDXGIFactory3
        assert!(factory.GetCreationFlags() == 0);

        // IDXGIFactory7 (default)
        assert!(
            factory
                .RegisterAdaptersChangedEvent(HANDLE(0), std::ptr::null_mut())
                .0
                == DXGI_ERROR_INVALID_CALL as u32
        );
    }
}

// Tests for https://github.com/microsoft/windows-rs/issues/463
#[test]
fn onecore_imports() -> windows::Result<()> {
    unsafe {
        let mut has_expanded_resources = FALSE;
        HasExpandedResources(&mut has_expanded_resources).ok()?;

        let mut uri = None;
        let uri = CreateUri(
            windows::HString::from("http://kennykerr.ca")
                .as_wide()
                .as_ptr(),
            0,
            0,
            &mut uri,
        )
        .and_some(uri)?;

        let mut port = 0;
        uri.GetPort(&mut port).ok()?;
        assert!(port == 80);

        let result = MiniDumpWriteDump(
            HANDLE(0),
            0,
            HANDLE(0),
            MINIDUMP_TYPE::MiniDumpNormal,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        assert!(!result.as_bool());

        assert!(D3DDisassemble11Trace(std::ptr::null_mut(), 0, None, 0, 0, 0, &mut None).is_err());

        Ok(())
    }
}

// TODO: light up BSTR as windows::BString

// #[test]
// fn interface() -> windows::Result<()> {
//     unsafe {
//         let s = windows::HString::from("https://kennykerr.ca");
//         let mut uri = None;

//         // TODO: should unwrap with Result<Uri> like WinRT but need https://github.com/microsoft/win32metadata/issues/24
//         let hr = CreateUri(s.as_wide().as_ptr() as *mut u16, 1, 0, &mut uri);
//         windows::ErrorCode(hr as u32).ok()?;

//         assert!(uri.is_some());

//         if let Some(uri) = uri {
//             let mut domain = windows::BString::new();
//             let hr = uri.GetDomain(domain.set_abi() as *mut *mut u16);
//             windows::ErrorCode(hr as u32).ok()?;

//             assert!(domain == "kennykerr.ca");
//         }
//     }
//     Ok(())
// }

#[test]
fn callback() {
    let a: PROPENUMPROCA = callback_a;
    assert!(BOOL(789) == a(HWND(123), "hello a\0".as_ptr() as *const i8, HANDLE(456)));

    let a: PROPENUMPROCW = callback_w;
    assert!(
        BOOL(789)
            == a(
                HWND(123),
                windows::HString::from("hello w\0").as_wide().as_ptr(),
                HANDLE(456)
            )
    );
}

// TODO: second parameter should be *const i8
extern "system" fn callback_a(param0: HWND, param1: *const i8, param2: HANDLE) -> BOOL {
    unsafe {
        assert!(param0.0 == 123);
        assert!(param2.0 == 456);
        let mut len = 0;
        let mut end = param1;

        loop {
            if *end == 0 {
                break;
            }

            len += 1;
            end = end.add(1);
        }

        let s = String::from_utf8_lossy(std::slice::from_raw_parts(param1 as *const u8, len))
            .into_owned();
        assert!(s == "hello a");
        BOOL(789)
    }
}

// TODO: second parameter should be *const u16
extern "system" fn callback_w(param0: HWND, param1: *const u16, param2: HANDLE) -> BOOL {
    unsafe {
        assert!(param0.0 == 123);
        assert!(param2.0 == 456);
        let mut len = 0;
        let mut end = param1;

        loop {
            if *end == 0 {
                break;
            }

            len += 1;
            end = end.add(1);
        }

        let s = String::from_utf16_lossy(std::slice::from_raw_parts(param1, len));
        assert!(s == "hello w");
        BOOL(789)
    }
}

#[test]
fn empty_struct() {
    let ldap = ldapsearch { reserved: 123 };
    assert!(ldap.reserved == 123);
    assert!(std::mem::size_of::<ldapsearch>() == 1);

    assert!(UIAnimationManager == Guid::from("4C1FC63A-695C-47E8-A339-1A194BE3D0B8"));
}

#[test]
fn struct_constants() {
    assert_eq!(WhitePoint::CHROMATICITY, 0);
    assert_eq!(WhitePoint::TEMPERATURE, 1);
    assert_eq!(WhitePoint::D65, 2);
}
