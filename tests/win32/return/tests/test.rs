// TODO: test_win32_return only works on x64 due to a Rust x86 linker bug
#![cfg(target_pointer_width = "64")]

use test_win32_return::*;
use windows::*;
use Component::Win32::Return::*;

#[test]
fn functions() {
    unsafe {
        assert!(ReturnValue() == 123);

        assert!(
            ReturnStruct()
                == ReturnType {
                    a: 123,
                    b: 456,
                    c: 789
                }
        );
    }
}

#[test]
fn members() -> Result<()> {
    unsafe {
        let object: IReturn = CreateReturn()?;

        assert!(object.ReturnValue() == 123);

        assert!(
            object.ReturnStruct()
                == ReturnType {
                    a: 123,
                    b: 456,
                    c: 789
                }
        );

        Ok(())
    }
}
