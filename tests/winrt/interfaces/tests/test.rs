use test_winrt_interfaces::*;
use windows::runtime::*;
use Component::Interfaces::*;
use Windows::Win32::Foundation::E_NOINTERFACE;

#[implement(Component::Interfaces::IProperty)]
struct Property(i32);

#[allow(non_snake_case)]
impl Property {
    fn Property(&self) -> Result<i32> {
        Ok(self.0)
    }

    fn SetProperty(&mut self, value: i32) -> Result<()> {
        self.0 = value;
        Ok(())
    }
}

#[test]
fn test() -> Result<()> {
    let p: IProperty = Property(0).into();
    assert!(p.Property()? == 0);
    p.SetProperty(123)?;
    assert!(p.Property()? == 123);

    Ok(())
}

#[test]
fn required() -> Result<()> {
    let good: IRequires = Test::GoodRequires()?;
    good.Requires()?;
    good.Required()?;

    let bad: IRequires = Test::BadRequires()?;
    bad.Requires()?;

    // Even though IRequires requires IRequired, this "bad" implementation doesn't implement the latter.
    // This test ensures that this failure bubbles up properly and can be handled. This also and mainly
    // validates feature detection can be used to make use of versioned APIs gracefully.
    assert!(bad.Required().unwrap_err().code() == E_NOINTERFACE);

    Ok(())
}
