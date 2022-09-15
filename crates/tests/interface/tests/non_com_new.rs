#![allow(non_snake_case, non_camel_case_types)]

use windows::core::*;

#[interface]
unsafe trait ITest {
    unsafe fn Call(&self) -> i32;
}

struct Test(i32);

impl ITest_Impl for Test {
    unsafe fn Call(&self) -> i32 {
        self.0
    }
}

unsafe fn call(test: &ITest) -> i32 {
    test.Call()
}

#[test]
fn test() {
    unsafe {
        let test = Test(456);
        let interface = ITest::new(&test);
        assert_eq!(call(&interface), 456);
        assert_eq!(interface.Call(), 456);
    }
}
