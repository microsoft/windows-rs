use test_winrt_interfaces::*;
use windows::*;
use Component::Interfaces::*;

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
