use test_winrt_method_names::*;
use windows::core::*;
use Component::MethodNames::*;
use Windows::Foundation::*;

#[implement(Component::MethodNames::IMethodNames)]
struct MethodNames(i64);

#[allow(non_snake_case)]
impl MethodNames {
    fn Method(&self) -> Result<()> {
        Ok(())
    }

    fn Property(&self) -> Result<i64> {
        Ok(self.0)
    }

    fn SetProperty(&mut self, value: i64) -> Result<()> {
        self.0 = value;
        Ok(())
    }

    fn Event(&self, _handler: &Option<EventHandler<i64>>) -> Result<EventRegistrationToken> {
        Ok(EventRegistrationToken { Value: self.0 })
    }

    fn RemoveEvent(&mut self, token: &EventRegistrationToken) -> Result<()> {
        self.0 = token.Value;
        Ok(())
    }
}

#[test]
fn test() -> Result<()> {
    let names: IMethodNames = MethodNames(0).into();
    names.Method()?;

    assert!(names.Property()? == 0);
    names.SetProperty(123)?;
    assert!(names.Property()? == 123);

    assert!(names.Event(None)? == EventRegistrationToken { Value: 123 });
    names.RemoveEvent(EventRegistrationToken { Value: 456 })?;
    assert!(names.Property()? == 456);

    Ok(())
}
