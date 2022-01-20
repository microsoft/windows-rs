use windows::Win32::Foundation::{SysAllocStringLen, BSTR};

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

    let a = BSTR::default();
    assert!(a.is_empty());
    assert!(a.len() == 0);
    let b = a.clone();
    assert_eq!(a, "");
    assert_eq!(b, "");

    let a = BSTR::new();
    assert!(a.is_empty());
    assert!(a.len() == 0);
    assert_eq!(a.len(), 0);
    assert_eq!(a.as_wide().len(), 0);

    let wide = &[0x68, 0x65, 0x6c, 0x6c, 0x6f];
    let a = BSTR::from_wide(wide);
    assert!(!a.is_empty());
    assert!(a.len() == 5);
    assert_eq!(a.as_wide().len(), 5);
    assert_eq!(a.as_wide(), wide);
    assert_eq!(a, "hello");

    let a: BSTR = "".into();
    assert!(a.is_empty());
    assert!(a.len() == 0);

    let a: BSTR = unsafe { SysAllocStringLen("", 0) };
    assert!(a.is_empty());
    assert!(a.len() == 0);
}
