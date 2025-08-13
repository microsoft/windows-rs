#![expect(non_snake_case)]

use windows::{core::*, Win32::Foundation::*};

// The interface macro detects `-> Result<()>` and implements the interface as if it is returning an HRESULT.
// This would be simpler if `HRESULT` could just implement `Try` but that is not yet stable.
#[interface("d83c9307-b31c-4cbc-8ab7-be9b24abba33")]
unsafe trait IResult: IUnknown {
    unsafe fn GetOk(&self) -> Result<()>;
    unsafe fn GetPointer(&self) -> Result<()>;
    unsafe fn GetInvalid(&self) -> Result<()>;
}

// Here is the same interface but declared as returning `HRESULT` explicitly.
#[interface("d83c9307-b31c-4cbc-8ab7-be9b24abba33")]
unsafe trait IHResult: IUnknown {
    unsafe fn GetOk(&self) -> HRESULT;
    unsafe fn GetPointer(&self) -> HRESULT;
    unsafe fn GetInvalid(&self) -> HRESULT;
}

// Here is an implementation of the former.
#[implement(IResult)]
struct HasResult;

impl IResult_Impl for HasResult_Impl {
    unsafe fn GetOk(&self) -> Result<()> {
        Ok(())
    }

    unsafe fn GetPointer(&self) -> Result<()> {
        Err(Error::from_hresult(E_POINTER))
    }

    unsafe fn GetInvalid(&self) -> Result<()> {
        Err(Error::from_hresult(E_INVALIDARG))
    }
}

// And an implementation of the latter.
#[implement(IHResult)]
struct HasHResult;

impl IHResult_Impl for HasHResult_Impl {
    unsafe fn GetOk(&self) -> HRESULT {
        S_OK
    }

    unsafe fn GetPointer(&self) -> HRESULT {
        E_POINTER
    }

    unsafe fn GetInvalid(&self) -> HRESULT {
        E_INVALIDARG
    }
}

// And here we test that they are equivalent.
#[test]
fn test() {
    let result: IResult = HasResult.into();
    test_impl(&result);

    let result: IHResult = HasHResult.into();
    test_impl(&result);
}

fn test_impl(object: &IUnknown) {
    let result = object.cast::<IResult>().unwrap();

    unsafe {
        assert_eq!(result.GetOk(), Ok(()));
        assert_eq!(result.GetPointer(), Err(Error::from_hresult(E_POINTER)));
        assert_eq!(result.GetInvalid(), Err(Error::from_hresult(E_INVALIDARG)));
    }

    let hresult = result.cast::<IHResult>().unwrap();

    unsafe {
        assert_eq!(hresult.GetOk(), S_OK);
        assert_eq!(hresult.GetPointer(), E_POINTER);
        assert_eq!(hresult.GetInvalid(), E_INVALIDARG);
    }
}
