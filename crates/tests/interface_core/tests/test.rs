#![allow(non_snake_case)]

use windows_core::*;

#[interface("72cd87fa-9c99-42e0-8986-84a76f08fc5a")]
unsafe trait ITest: IUnknown {
    unsafe fn Test(&self) -> HRESULT;
}

#[implement(ITest)]
struct Test;

impl ITest_Impl for Test {
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
