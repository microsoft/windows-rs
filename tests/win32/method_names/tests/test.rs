use test_win32_method_names::*;
use windows::runtime::*;
use Component::Win32::MethodNames::*;

#[implement(Component::Win32::MethodNames::IMethodNames)]
struct MethodNames(i32);

#[allow(non_snake_case)]
impl MethodNames {
    fn Method(&self) -> Result<()> {
        Ok(())
    }

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
    unsafe {
        let names: IMethodNames = MethodNames(0).into();
        names.Method()?;

        assert!(names.Property()? == 0);
        names.SetProperty(123)?;
        assert!(names.Property()? == 123);

        Ok(())
    }
}
