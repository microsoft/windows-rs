// Remove when upstream metadata generator supports other targets
#![cfg(all(windows, target_pointer_width = "64"))]

use test_win32_simple::*;
use windows::*;
use Component::Win32::*;

#[implement(Component::Win32::ISimpleInterface)]
struct Interface();

#[allow(non_snake_case)]
impl Interface {
    fn Method(&self) -> Result<()> {
        Ok(())
    }
}

extern "system" fn callback() {}

#[test]
fn test() -> Result<()> {
    unsafe {
        let constant: Guid = SimpleConstant;
        assert!(constant == "7FE2C3B5-4EF1-4B35-889D-03BA46CDD1EF".into());

        SimpleFunction();

        let interface: ISimpleInterface = Interface().into();
        interface.Method()?;

        let _struct = SimpleStruct {
            First: 1,
            Second: 2,
        };

        let callback: SimpleCallback = callback;
        callback();

        let _enum: SimpleEnum = First;

        Ok(())
    }
}
