use test_winrt_weak::*;
use windows::runtime::*;
use Windows::Win32::System::WinRT::IWeakReferenceSource;

#[test]
fn test() {
    // Ref count starts at 1
    let count = WeakRefCount::new();
    assert!(count.add_ref() == 2);
    assert!(count.add_ref() == 3);
    assert!(count.release() == 2);
    assert!(count.release() == 1);

    // Query implies add_ref
    unsafe {
        count.query(&IWeakReferenceSource::IID, core::ptr::null_mut());
    }

    // Ref count is now owned by tearoff
    assert!(count.add_ref() == 3);
    assert!(count.release() == 2);
    assert!(count.release() == 1);
    assert!(count.release() == 0);
}
