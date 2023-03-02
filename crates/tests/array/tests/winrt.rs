use windows::core::Array;
use windows::Foundation::Uri;

#[test]
fn array() {
    let a = Array::<Uri>::new();
    assert!(a.is_empty());

    let mut a = Array::<Uri>::with_len(2);
    assert!(a[0].is_none());
    assert!(a[1].is_none());

    a[0] = Uri::CreateUri(&windows::core::HSTRING::from("http://kennykerr.ca")).ok();
    a[1] = Uri::CreateUri(&windows::core::HSTRING::from("http://microsoft.com")).ok();

    assert!(a[0].as_ref().unwrap().Domain().unwrap() == "kennykerr.ca");
    assert!(a[1].as_ref().unwrap().Domain().unwrap() == "microsoft.com");
}
