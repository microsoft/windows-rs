//! Exercises the `interface_decl!` macro_rules! alternative to `#[interface]`.
//!
//! The interface side is declared with the declarative macro, while the implementation
//! side reuses the existing `#[implement]` proc-macro. This proves that
//! `interface_decl!`-generated items (`ITest`, `ITest_Vtbl`, `ITest_Impl`) are layout- and
//! API-compatible with what `#[interface]` would have emitted.

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

#[implement(ITest)]
struct Test;

impl ITest_Impl for Test_Impl {
    unsafe fn Void(&self) {}
    unsafe fn Code(&self, code: HRESULT) -> HRESULT {
        code
    }
    unsafe fn Result(&self, code: HRESULT) -> Result<()> {
        code.ok()
    }
}

#[test]
fn test() {
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
fn iid_matches() {
    assert_eq!(
        ITest::IID,
        GUID::from_u128(0x09428a59_5b40_4e4c_9175_e7a78514316d)
    );
    assert!(ITest_Vtbl::matches(&ITest::IID));
    assert!(!ITest_Vtbl::matches(&IUnknown::IID));
}
