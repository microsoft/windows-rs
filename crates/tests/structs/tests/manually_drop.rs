#![allow(non_camel_case_types)]

use windows::core::*;

static mut COUNTER: isize = 0;

#[interface("40d7baf7-b4cf-40e0-9ed3-d124b0ee3fda")]
unsafe trait ITest: IUnknown {
    unsafe fn counter(&self) -> isize;
}

#[implement(ITest)]
struct Test();

impl Test {
    fn new() -> Self {
        unsafe {
            COUNTER += 1;
        }
        Self()
    }
}

impl Drop for Test {
    fn drop(&mut self) {
        unsafe {
            COUNTER -= 1;
        }
    }
}

impl ITest_Impl for Test {
    unsafe fn counter(&self) -> isize {
        COUNTER
    }
}

#[test]
fn test() -> Result<()> {
    unsafe {
        assert_eq!(COUNTER, 0);
        {
            let a: ITest = Test::new().into();
            assert_eq!(a.counter(), 1);
            let b: ITest = Test::new().into();
            assert_eq!(b.counter(), 2);
            assert_eq!(COUNTER, 2);
        }
        assert_eq!(COUNTER, 0);
        {
            let a: ITest = Test::new().into();
            let m = ManuallyDrop::<ITest>::new(&a);
            assert_eq!(a.counter(), 1);
            assert_eq!(a.abi(), m.abi());
            let r: Option<&ITest> = m.as_ref();
            assert_eq!(r, Some(&a));
            assert_eq!(m.unwrap(), &a);

            let none = ManuallyDrop::<ITest>::none();
            assert_eq!(none.as_ref(), None);
        }
        assert_eq!(COUNTER, 0);
        Ok(())
    }
}
