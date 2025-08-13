#![expect(non_snake_case)]

use windows_core::*;

pub const S_OK: HRESULT = HRESULT(0);
pub const S_FALSE: HRESULT = HRESULT(1);
pub const E_INVALIDARG: HRESULT = HRESULT(0x80070057_u32 as _);

#[interface("09428a59-5b40-4e4c-9175-e7a78514316d")]
unsafe trait ITest: IUnknown {
    unsafe fn Void(&self);
    unsafe fn Code(&self, code: HRESULT) -> HRESULT;
    unsafe fn Result(&self, code: HRESULT) -> Result<()>;
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
