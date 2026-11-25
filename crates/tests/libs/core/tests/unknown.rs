#![cfg(windows)]
#![expect(non_snake_case)]

use std::sync::{
    Arc,
    atomic::{AtomicU32, Ordering},
};
use windows_core::*;

#[interface("f7ea748b-8121-41c1-aaee-406ba6f148a9")]
unsafe trait ITest: IUnknown {
    unsafe fn Test(&self) -> u32;
}

#[implement(ITest)]
struct Test {
    drop: Arc<AtomicU32>,
}

impl ITest_Impl for Test_Impl {
    unsafe fn Test(&self) -> u32 {
        self.drop.load(Ordering::Relaxed)
    }
}

impl Drop for Test {
    fn drop(&mut self) {
        self.drop.fetch_add(1, Ordering::Relaxed);
    }
}

#[test]
fn test_unknown() {
    unsafe {
        let dropped = Arc::new(AtomicU32::new(0));
        let test: ITest = Test {
            drop: dropped.clone(),
        }
        .into();

        {
            let raw_borrowed: *mut std::ffi::c_void = test.as_raw();
            let unknown_borrowed: &IUnknown = IUnknown::from_raw_borrowed(&raw_borrowed).unwrap();
            assert_eq!(unknown_borrowed.as_raw(), test.as_raw());
            let test_query: ITest = unknown_borrowed.cast().unwrap();
            assert_eq!(test_query.Test(), 0);
        }
        {
            let raw_owned: *mut std::ffi::c_void = test.clone().into_raw();
            let unknown_owned: IUnknown = IUnknown::from_raw(raw_owned);
            assert_eq!(unknown_owned.as_raw(), test.as_raw());
            let test_query: ITest = unknown_owned.cast().unwrap();
            assert_eq!(test_query.Test(), 0);
        }

        assert!(IUnknown::from_raw_borrowed(&std::ptr::null_mut()).is_none());

        assert_eq!(test.Test(), 0);
        drop(test);
        assert_eq!(dropped.load(Ordering::Relaxed), 1);
    }
}

#[test]
fn test_pointer_conversion_functions() {
    unsafe {
        let dropped = Arc::new(AtomicU32::new(0));
        let test: ITest = Test {
            drop: dropped.clone(),
        }
        .into();

        {
            let raw_borrowed: *mut std::ffi::c_void = test.as_raw();
            let test_borrowed: &ITest = ITest::from_raw_borrowed(&raw_borrowed).unwrap();
            assert_eq!(test_borrowed.as_raw(), test.as_raw());
            assert_eq!(test_borrowed.Test(), 0);
        }
        {
            let raw_owned: *mut std::ffi::c_void = test.clone().into_raw();
            let unknown_owned: ITest = ITest::from_raw(raw_owned);
            assert_eq!(unknown_owned.as_raw(), test.as_raw());
            assert_eq!(unknown_owned.Test(), 0);
        }

        assert_eq!(test.Test(), 0);
        drop(test);
        assert_eq!(dropped.load(Ordering::Relaxed), 1);
    }
}

#[test]
fn interface_adoption_transfers_ownership_and_rejects_null() {
    let dropped = Arc::new(AtomicU32::new(0));
    let test: ITest = Test {
        drop: dropped.clone(),
    }
    .into();
    let raw = test.into_raw();
    let adopted: ITest = unsafe { imp::from_abi(raw) }.unwrap();
    assert_eq!(adopted.as_raw(), raw);
    assert_eq!(unsafe { adopted.Test() }, 0);
    drop(adopted);
    assert_eq!(dropped.load(Ordering::Relaxed), 1);
    assert_eq!(
        unsafe { imp::from_abi::<ITest>(core::ptr::null_mut()) }
            .unwrap_err()
            .code(),
        HRESULT(0)
    );
}
