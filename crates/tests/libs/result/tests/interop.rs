// TODO: tests that ensure the ? works across the different Result<T, WIN32_ERROR/NTSTATUS/RPC_STATUS/HRESULT>

use windows::Win32::Foundation::{E_POINTER, NTSTATUS, S_OK, WIN32_ERROR};
use windows::Win32::System::Rpc::RPC_STATUS;
use windows_result::{Error, HRESULT};

#[test]
fn into_hresult() -> Result<(), HRESULT> {
    Result::<(), Error>::Ok(())?;
    Result::<(), HRESULT>::Ok(())?;
    Result::<(), WIN32_ERROR>::Ok(())?;
    Result::<(), NTSTATUS>::Ok(())?;
    Result::<(), RPC_STATUS>::Ok(())?;

    Result::<(), std::io::Error>::Ok(())?;
    Result::<(), std::string::FromUtf16Error>::Ok(())?;
    Result::<(), std::string::FromUtf8Error>::Ok(())?;
    Result::<(), core::num::TryFromIntError>::Ok(())?;

    Ok(())
}

#[test]
fn into_error() -> Result<(), Error> {
    Result::<(), Error>::Ok(())?;
    Result::<(), HRESULT>::Ok(())?;
    Result::<(), WIN32_ERROR>::Ok(())?;
    Result::<(), NTSTATUS>::Ok(())?;
    Result::<(), RPC_STATUS>::Ok(())?;

    Result::<(), std::io::Error>::Ok(())?;
    Result::<(), std::string::FromUtf16Error>::Ok(())?;
    Result::<(), std::string::FromUtf8Error>::Ok(())?;
    Result::<(), core::num::TryFromIntError>::Ok(())?;

    Ok(())
}

#[test]
fn from_result() {
    assert_eq!(S_OK, HRESULT::from(Result::<(), Error>::Ok(())));
    assert_eq!(S_OK, HRESULT::from(Result::<(), HRESULT>::Ok(())));

    assert_eq!(
        E_POINTER,
        HRESULT::from(Result::<(), Error>::Err(E_POINTER.into()))
    );
    assert_eq!(
        E_POINTER,
        HRESULT::from(Result::<(), HRESULT>::Err(E_POINTER))
    );
}
