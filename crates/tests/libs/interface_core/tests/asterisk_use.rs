#![expect(non_snake_case)]

// This tests uses `windows_core` via an asterisk to test that the interface/implement macros support this mode
// as opposed to no_use.rs which tests the opposite.

use windows_core::*;

#[interface("72cd87fa-9c99-42e0-8986-84a76f08fc5a")]
unsafe trait ITest: IUnknown {
    unsafe fn Test(&self) -> HRESULT;
}

#[implement(ITest)]
struct Test;

impl ITest_Impl for Test_Impl {
    unsafe fn Test(&self) -> HRESULT {
        HRESULT(123)
    }
}

#[test]
fn test() {
    unsafe {
        let test: ITest = Test.into();
        assert_eq!(test.Test(), HRESULT(123));
    }
}
