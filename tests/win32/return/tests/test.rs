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
fn query_member() -> Result<()> {
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
