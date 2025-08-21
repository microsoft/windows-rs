use std::sync::atomic::*;
use windows::{core::*, Foundation::*};

static COUNTER: AtomicIsize = AtomicIsize::new(0);

#[implement(IStringable, IClosable)]
struct Test(String);

impl Test {
    fn new(value: &str) -> Self {
        COUNTER.fetch_add(1, Ordering::Relaxed);
        Self(value.to_string())
    }
}

impl Drop for Test {
    fn drop(&mut self) {
        COUNTER.fetch_sub(1, Ordering::Release);
    }
}

impl IStringable_Impl for Test_Impl {
    fn ToString(&self) -> Result<HSTRING> {
        Ok(self.0.as_str().into())
    }
}

impl IClosable_Impl for Test_Impl {
    fn Close(&self) -> Result<()> {
        Ok(())
    }
}

#[test]
fn identity() -> Result<()> {
    assert_eq!(COUNTER.load(Ordering::Acquire), 0);
    {
        let a: IStringable = Test::new("test").into();
        assert_eq!(COUNTER.load(Ordering::Acquire), 1);
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
        assert_eq!(a.GetRuntimeClassName()?, "Windows.Foundation.IStringable");

        let b: IStringable = a.cast()?;
        let c: &IInspectable = &b.cast()?;
        assert_eq!(c.GetRuntimeClassName()?, "Windows.Foundation.IStringable");

        let d: IClosable = a.cast()?;
        let e: &IInspectable = (&d).into();
        assert_eq!(e.GetRuntimeClassName()?, "Windows.Foundation.IClosable");

        let f: IInspectable = e.cast()?;
        assert_eq!(f.GetRuntimeClassName()?, "Windows.Foundation.IStringable");
    }
    assert_eq!(COUNTER.load(Ordering::Acquire), 0);
    Ok(())
}
