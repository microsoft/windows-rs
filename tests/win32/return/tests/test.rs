// TODO: test_win32_return only works on x64 due to a Rust x86 linker bug
#![cfg(target_pointer_width = "64")]

use test_win32_return::*;
use windows::*;
use Component::Win32::Return::*;
use Windows::Win32::Foundation::*;

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

        let mut check = 0;
        ReturnVoid(&mut check);
        assert!(check == 123);

        // Sanity check against well-known values.
        assert!(S_OK.0 == 0);
        assert!(STATUS_SUCCESS.0 == 0);
        assert!(E_APPLICATION_EXITING.0 == 0x8000001A);
        assert!(STATUS_NOT_FOUND.0 == 0xC0000225);

        let result: Result<()> = ReturnHresult(S_OK.0);
        assert!(result.is_ok());

        let result: Result<()> = ReturnNtstatus(STATUS_SUCCESS.0);
        assert!(result.is_ok());

        assert!(ReturnHresult(E_APPLICATION_EXITING.0).unwrap_err().code() == E_APPLICATION_EXITING);
        assert!(ReturnNtstatus(STATUS_NOT_FOUND.0).unwrap_err().code() == STATUS_NOT_FOUND.to_hresult());
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

        let mut check = 0;
        object.ReturnVoid(&mut check);
        assert!(check == 123);

        let result: Result<()> = object.ReturnHresult(S_OK.0);
        assert!(result.is_ok());

        let result: Result<()> = object.ReturnNtstatus(STATUS_SUCCESS.0);
        assert!(result.is_ok());
        
        assert!(object.ReturnHresult(E_APPLICATION_EXITING.0).unwrap_err().code() == E_APPLICATION_EXITING);
        assert!(object.ReturnNtstatus(STATUS_NOT_FOUND.0).unwrap_err().code() == STATUS_NOT_FOUND.to_hresult());

        Ok(())
    }
}
