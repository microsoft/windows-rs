#![cfg(test)]
#![cfg_attr(windows_raw_dylib, feature(raw_dylib))]
#![allow(clashing_extern_declarations)]
#![feature(strict_provenance)]

mod b_arch;
mod b_bstr;
mod b_depends;
mod b_enumeration;
mod b_enumerator;
mod b_guid;
mod b_hresult;
mod b_hstring;
mod b_inspectable;
mod b_none;
mod b_pcstr;
mod b_pcwstr;
mod b_pstr;
mod b_pwstr;
mod b_std;
mod b_test;
mod b_unknown;

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
        assert_eq!(b_std::INVALID_HANDLE_VALUE, ::core::ptr::invalid_mut(!0));
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
