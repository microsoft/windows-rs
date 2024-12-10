#![cfg(test)]
#![allow(clashing_extern_declarations, unused_imports)]

mod b_arch;
mod b_arch_dependencies;
mod b_bstr;
mod b_calendar;
mod b_constant_types;
mod b_depends;
mod b_enumeration;
mod b_enumerator;
mod b_guid;
mod b_hresult;
mod b_hstring;
mod b_inspectable;
mod b_nested;
mod b_none;
mod b_overloads;
mod b_pcstr;
mod b_pcwstr;
mod b_prepend;
mod b_pstr;
mod b_pwstr;
mod b_std;
mod b_stringable;
mod b_test;
mod b_unknown;
mod b_uri;
mod b_variant;
mod b_vtbl_0;
mod b_vtbl_1;
mod b_vtbl_2;
mod b_vtbl_3;
mod b_vtbl_4;
mod b_win_enumerator;

#[allow(non_snake_case)]
mod included {
    include!("b_include_me.rs");
}

#[test]
fn bstr() {
    unsafe {
        b_bstr::SysAllocString(std::ptr::null());
    }
}

#[test]
fn guid() {
    unsafe {
        b_guid::CoCreateGuid(std::ptr::null_mut());
    }
}

#[test]
fn hresult() {
    unsafe {
        b_hresult::CoInitialize(std::ptr::null());
    }
}

#[test]
fn hstring() {
    unsafe {
        b_hstring::WindowsGetStringLen(std::ptr::null_mut());
    }
}

#[test]
fn inspectable() {
    unsafe {
        b_inspectable::RoActivateInstance(std::ptr::null_mut(), &mut std::ptr::null_mut());
    }
}

#[test]
fn none() {
    unsafe {
        b_none::GetTickCount();
    }
}

#[test]
fn pcstr() {
    unsafe {
        b_pcstr::lstrlenA(std::ptr::null());
    }
}

#[test]
fn pcwstr() {
    unsafe {
        b_pcwstr::lstrlenW(std::ptr::null());
    }
}

#[test]
fn pstr() {
    unsafe {
        let mut buffer = [0; 10];
        b_pstr::VarI1FromDate(0.0, buffer.as_mut_ptr());
    }
}

#[test]
fn pwstr() {
    let _ = b_pwstr::CALPOLESTR {
        cElems: 0,
        pElems: std::ptr::null_mut(),
    };
}

#[test]
fn unknown() {
    unsafe {
        b_unknown::CoIsHandlerConnected(std::ptr::null_mut());
    }
}

#[test]
fn enumerator() {
    assert_eq!(b_enumerator::WAIT_IO_COMPLETION, 192);
    assert_eq!(b_enumerator::WAIT_TIMEOUT, 258);
}

#[test]
fn std() {
    unsafe {
        assert_eq!(b_std::CloseHandle(std::ptr::null_mut()), 0);
        let mut buffer = [0u8; 8];
        assert!(b_std::RtlGenRandom(
            buffer.as_mut_ptr() as _,
            buffer.len() as _
        ));
        assert_ne!(&buffer, &[0u8; 8]);
    }
}

#[test]
fn test() {
    use b_test::*;
    unsafe {
        let event = CreateEventW(std::ptr::null(), 1, 0, std::ptr::null());
        SetEvent(event);
        WaitForSingleObject(event, 0);
        CloseHandle(event);
        CoCreateInstance(
            std::ptr::null(),
            std::ptr::null_mut(),
            CLSCTX_ALL,
            std::ptr::null(),
            std::ptr::null_mut(),
        );
        assert_eq!(STGTY_REPEAT, 256);

        let expected = GUID::from_u128(0x4c1fc63a_695c_47e8_a339_1a194be3d0b8);
        assert!(UIAnimationManager.data1 == expected.data1);
        assert!(UIAnimationManager.data2 == expected.data2);
        assert!(UIAnimationManager.data3 == expected.data3);
        assert!(UIAnimationManager.data4 == expected.data4);
    }
}

#[test]
fn uri() -> windows_core::Result<()> {
    use b_uri::*;
    let uri = Uri::CreateUri(windows_core::h!("https://kennykerr.ca/"))?;
    assert_eq!(uri.Domain()?, "kennykerr.ca");
    Ok(())
}

#[test]
fn calendar() -> windows_core::Result<()> {
    use b_calendar::*;
    let calendar = Calendar::new()?;
    let year = calendar.Year()?;
    calendar.SetYear(year)?;
    Ok(())
}

#[test]
fn from_included() {
    unsafe {
        included::GetVersion();
    }
}

#[test]
fn prepend() {
    use b_prepend::*;
    let mut dates = [
        DateTime { UniversalTime: 123 },
        DateTime { UniversalTime: 42 },
    ];
    dates.sort();
    assert_eq!(
        &dates,
        &[
            DateTime { UniversalTime: 42 },
            DateTime { UniversalTime: 123 }
        ]
    );
}
