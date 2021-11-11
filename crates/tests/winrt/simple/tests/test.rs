use test_winrt_simple::*;
use windows::core::*;
use Component::*;

#[implement(Component::Simple::IInterface)]
struct Interface();

#[allow(non_snake_case)]
impl Interface {
    fn Method(&self) -> Result<()> {
        Ok(())
    }
}

#[test]
fn test() -> Result<()> {
    let class = Simple::Class::new()?;
    class.Method()?;

    let interface: Simple::IInterface = Interface().into();
    interface.Method()?;

    let delegate = Simple::Delegate::new(|| Ok(()));
    delegate.Invoke()?;

    let _struct = Simple::Struct { First: 1, Second: 2 };

    let _enum = Simple::Enum::First;

    Ok(())
}
