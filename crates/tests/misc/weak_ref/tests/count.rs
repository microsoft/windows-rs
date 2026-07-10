#![cfg(windows)]
use windows::Win32::weakreference::IWeakReferenceSource;
use windows::core::Interface;
use windows::core::imp::WeakRefCount;

#[test]
fn test() {
    // Ref count starts at 1
    let count = WeakRefCount::new();
    assert_eq!(count.add_ref(), 2);
    assert_eq!(count.add_ref(), 3);
    assert_eq!(count.release(), 2);
    assert_eq!(count.release(), 1);

    // Query implies add_ref
    unsafe {
        count.query(&IWeakReferenceSource::IID, core::ptr::null_mut());
    }

    // Ref count is now owned by tearoff
    assert_eq!(count.add_ref(), 3);
    assert_eq!(count.release(), 2);
    assert_eq!(count.release(), 1);
    assert_eq!(count.release(), 0);
}
