#![allow(non_snake_case, non_camel_case_types)]

use windows::core::*;

// TODO: define a new pseudo interface and then implement and call it in various ways

#[interface]
unsafe trait ITest { // needs to be a struct since the macro turns it into a struct anyway
    unsafe fn Call(&self) -> i32;
}

struct Test(i32);

impl ITest_Impl for Test {
    unsafe fn Call(&self) -> i32 {
        self.0
    }
}

fn call(test: &ITest) {
   unsafe { println!("call {}", test.Call()); }
}

fn main() {
    unsafe {
        let test = Test(456);
        let interface = ITest::new(&test);
        call(&interface);
        assert_eq!(interface.Call(), 456);
        println!("Call {}", interface.Call());
    }
}
