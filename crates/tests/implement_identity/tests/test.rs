#![allow(non_snake_case)]

use windows::{core::*, Foundation::*};

static mut COUNTER: isize = 0;

#[implement(IStringable, IClosable)]
struct Test(String, i128);

impl Test {
    fn new(value: &str) -> Self {
        unsafe {
            COUNTER += 1;
        }
        Self(value.to_string(), 0)
    }
}

impl Drop for Test {
    fn drop(&mut self) {
        unsafe {
            COUNTER -= 1;
        }
    }
}

impl IStringable_Impl for Test {
    fn ToString(&self) -> Result<HSTRING> {
        Ok(self.0.as_str().into())
    }
}

impl IClosable_Impl for Test {
    fn Close(&self) -> Result<()> {
        Ok(())
    }
}

#[test]
fn identity() -> Result<()> {
    unsafe {
        assert_eq!(COUNTER, 0);
        {
            let a: IStringable = Test::new("test").into();
            assert!(a.ToString()? == "test");

            let b: IClosable = a.cast()?;
            b.Close()?;

            let c: IUnknown = b.cast()?;

            let d: IInspectable = c.cast()?;

            assert!(a == d.cast()?);
        }
        {
            let a: IUnknown = Test::new("test").into();
            let b: IClosable = a.cast()?;
            let c: IStringable = b.cast()?;
            assert!(c.ToString()? == "test");
        }
        {
            let a: IInspectable = Test::new("test").into();
            let b: IStringable = a.cast()?;
            assert!(b.ToString()? == "test");
        }
        {
            let a: IInspectable = Test::new("test").into();
            assert_eq!(a.GetRuntimeClassName()?, "");

            let b: IStringable = a.cast()?;
            let c: IInspectable = b.into();
            assert_eq!(c.GetRuntimeClassName()?, "Windows.Foundation.IStringable");

            let d: IClosable = a.cast()?;
            let e: IInspectable = d.into();
            assert_eq!(e.GetRuntimeClassName()?, "Windows.Foundation.IClosable");

            let f: IInspectable = e.cast()?;
            assert_eq!(f.GetRuntimeClassName()?, "");
        }
        assert_eq!(COUNTER, 0);
        Ok(())
    }
}
