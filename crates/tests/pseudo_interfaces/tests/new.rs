#![allow(non_snake_case)]

use windows::core::*;

// TODO: define a new pseudo interface and then implement and call it in various ways

#[interface]
unsafe trait ITest {
    unsafe fn Call(&self) -> i32;
}

#[implement(ITest)]
struct Test;

impl ITest_Impl for Test {
    unsafe fn Call(&self) -> i32 {
        123
    }
}

#[test]
fn test_new()  {
    unsafe {
        // TODO: the `into` will create a heap object but ITest
        // doesn't have a Drop impl so there's no way to free it nor should there be.
        let test: ITest = Test.into();
        assert_eq!(test.Call(), 123);
    }
}
