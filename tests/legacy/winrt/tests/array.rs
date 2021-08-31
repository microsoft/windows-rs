use test_winrt::Windows::Foundation::Uri;
use windows::Array;

#[test]
fn array() {
    let a = Array::<Uri>::new();
    assert!(a.is_empty());

    let mut a = Array::<Uri>::with_len(2);
    assert!(a[0] == None);
    assert!(a[1] == None);

    a[0] = Uri::CreateUri("http://kennykerr.ca").ok();
    a[1] = Uri::CreateUri("http://microsoft.com").ok();

    // TODO: this seems rather tedious... may warrant a windows::Option<T> that's more convenient
    // that could handle both nullable and IReference<T> behaviors in a single abstraction.
    assert!(a[0].as_ref().unwrap().Domain().unwrap() == "kennykerr.ca");
    assert!(a[1].as_ref().unwrap().Domain().unwrap() == "microsoft.com");
}
