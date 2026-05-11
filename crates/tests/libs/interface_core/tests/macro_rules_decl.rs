//! Exercises the `interface_decl!` and `implement_decl!` macro_rules! alternatives to
//! `#[interface]` / `#[implement]`.
//!
//! Both sides — interface declaration and implementer wiring — are done with declarative
//! macros, so this test doubles as proof that the fully proc-macro-free path produces an
//! object that is layout- and ABI-compatible with what the proc-macros would have emitted.

#![expect(non_snake_case)]

use windows_core::*;

const S_OK: HRESULT = HRESULT(0);
const S_FALSE: HRESULT = HRESULT(1);
const E_INVALIDARG: HRESULT = HRESULT(0x80070057_u32 as _);

interface_decl! {
    /// A test COM interface defined via the declarative macro.
    pub unsafe trait ITest(ITest_Vtbl, ITest_Impl) : IUnknown
        = 0x09428a59_5b40_4e4c_9175_e7a78514316d
    {
        /// Void-returning method.
        unsafe fn Void(&self);
        unsafe fn Code(&self, code: HRESULT) -> HRESULT;
        unsafe fn Result(&self, code: HRESULT) -> Result<()>;
    }
}

interface_decl! {
    pub unsafe trait IOther(IOther_Vtbl, IOther_Impl) : IUnknown
        = 0x1a2b3c4d_5e6f_7081_92a3_b4c5d6e7f809
    {
        unsafe fn Plus(&self, a: i32, b: i32) -> i32;
    }
}

pub struct Test;

implement_decl! {
    impl Test as pub Test_Impl: [ITest: ITest_Vtbl, IOther: IOther_Vtbl]
}

impl ITest_Impl for Test_Impl {
    unsafe fn Void(&self) {}
    unsafe fn Code(&self, code: HRESULT) -> HRESULT {
        code
    }
    unsafe fn Result(&self, code: HRESULT) -> Result<()> {
        code.ok()
    }
}

impl IOther_Impl for Test_Impl {
    unsafe fn Plus(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

#[test]
fn test_itest() {
    unsafe {
        let test: ITest = Test.into();

        test.Void();

        assert_eq!(test.Code(S_OK), S_OK);
        assert_eq!(test.Code(S_FALSE), S_FALSE);
        assert_eq!(test.Code(E_INVALIDARG), E_INVALIDARG);

        assert!(test.Result(S_OK).is_ok());
        assert!(test.Result(S_FALSE).is_ok());
        assert_eq!(test.Result(E_INVALIDARG), E_INVALIDARG.ok());
    }
}

#[test]
fn test_iother() {
    unsafe {
        // Construct via the second interface to exercise its vtable slot.
        let other: IOther = Test.into();
        assert_eq!(other.Plus(2, 3), 5);
        assert_eq!(other.Plus(-1, 1), 0);

        // QueryInterface across to ITest and back to IOther.
        let test: ITest = other.cast().unwrap();
        test.Void();
        let other2: IOther = test.cast().unwrap();
        assert_eq!(other2.Plus(7, 8), 15);
    }
}

#[test]
fn iid_matches() {
    assert_eq!(
        ITest::IID,
        GUID::from_u128(0x09428a59_5b40_4e4c_9175_e7a78514316d)
    );
    assert!(ITest_Vtbl::matches(&ITest::IID));
    assert!(!ITest_Vtbl::matches(&IUnknown::IID));
}

#[test]
fn refcount_drops_to_zero() {
    // Construct an object, take an extra reference via cast, then drop both. If the
    // refcount math is wrong the boxed object would leak (caught by miri) or, if doubly
    // freed, cause a crash. This test is mainly here so that the `Release` path runs.
    let test: ITest = Test.into();
    let other: IOther = unsafe { test.cast() }.unwrap();
    drop(test);
    drop(other);
}
