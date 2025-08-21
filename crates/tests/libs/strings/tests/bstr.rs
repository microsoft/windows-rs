use windows::{core::Result, Win32::Foundation::*};
use windows_strings::*;

#[test]
fn test() {
    let b: BSTR = "hello".into();
    assert_eq!(b, "hello");
}

#[test]
fn clone() {
    let a: BSTR = "hello".into();
    assert!(!a.is_empty());
    assert!(a.len() == 5);
    let b = a.clone();
    assert_eq!(a, "hello");
    assert_eq!(b, "hello");
    assert_eq!("hello", a);

    let a = BSTR::default();
    assert!(a.is_empty());
    assert_eq!(a.len(), 0);
    let b = a.clone();
    assert_eq!(a, "");
    assert_eq!(b, "");

    let a = BSTR::new();
    assert!(a.is_empty());
    assert_eq!(a.len(), 0);
    assert_eq!(a.len(), 0);
    assert_eq!(a.len(), 0);

    let wide = &[0x68, 0x65, 0x6c, 0x6c, 0x6f];
    let a = BSTR::from_wide(wide);
    assert!(!a.is_empty());
    assert!(a.len() == 5);
    assert_eq!(a.len(), 5);
    assert_eq!(*a, *wide);
    assert_eq!(a, "hello");

    let a: BSTR = "".into();
    assert!(a.is_empty());
    assert_eq!(a.len(), 0);

    let a: BSTR = unsafe { SysAllocStringLen(None) };
    assert!(a.is_empty());
    assert_eq!(a.len(), 0);

    let a = BSTR::from("a");
    assert_eq!(a, String::from("a"));
    assert_eq!(String::from("a"), a);
}

#[test]
fn interop() -> Result<()> {
    unsafe {
        let b: BSTR = "hello".into();
        SysAddRefString(&b)?;
        SysFreeString(&b);
        Ok(())
    }
}

#[test]
fn deref_as_slice() {
    let deref = BSTR::from("0123456789");
    assert!(!deref.is_empty());
    assert_eq!(deref.len(), 10);
    assert_eq!(BSTR::from_wide(&deref[..=3]), "0123");
    assert!(deref.ends_with(&deref[7..=9]));
    assert_eq!(deref.get(5), Some(b'5' as u16).as_ref());
    let ptr = PCWSTR(deref.as_ptr());
    assert_eq!(deref.cmp(&deref), std::cmp::Ordering::Equal);

    unsafe {
        assert_eq!(*ptr.as_wide(), *deref);
    }

    let empty = BSTR::new();
    assert!(empty.is_empty());
    assert_eq!(empty.len(), 0);
    assert_eq!(*empty, []);

    unsafe {
        assert_eq!(wcslen(empty.as_ptr()), 0);
    }
}

unsafe extern "C" {
    pub fn wcslen(s: *const u16) -> usize;
}
