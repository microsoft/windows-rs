#![cfg(windows)]
use windows::{Win32::*, core::WIN32_ERROR, core::*};

#[test]
fn conversions() {
    helpers::set_thread_ui_language();

    // Baseline HRESULT
    assert_eq!(E_INVALIDARG.message(), "The parameter is incorrect.");
    assert_eq!(E_INVALIDARG.0, -2147024809);

    // Baseline WIN32_ERROR
    assert_eq!(
        WIN32_ERROR(ERROR_INVALID_DATA).to_hresult().message(),
        "The data is invalid."
    );
    assert_eq!(ERROR_INVALID_DATA, 13);

    // std::io::Error from HRESULT
    let std_error = std::io::Error::from_raw_os_error(E_INVALIDARG.0);
    assert_eq!(std_error.raw_os_error().unwrap(), E_INVALIDARG.0);
    assert_eq!(
        format!("{std_error}"),
        "The parameter is incorrect. (os error -2147024809)"
    );

    // std::io::Error from WIN32_ERROR
    let std_error = std::io::Error::from_raw_os_error(ERROR_INVALID_DATA as i32);
    assert_eq!(std_error.raw_os_error().unwrap(), ERROR_INVALID_DATA as i32);
    assert_eq!(format!("{std_error}"), "The data is invalid. (os error 13)");

    // Starting with WIN32_ERROR (FACILITY_WIN32 HRESULT)... the conversion
    // should unwrap to the underlying Win32 code rather than using the full
    // HRESULT, so that std::io::Error::raw_os_error() carries a real Win32
    // error code (matching what .NET's Marshal.GetExceptionForHR and Rust's
    // own Win32 error decoding produce).
    let win_error: Error = WIN32_ERROR(ERROR_INVALID_DATA).into();
    let std_error: std::io::Error = win_error.into();
    assert_eq!(std_error.raw_os_error().unwrap(), ERROR_INVALID_DATA as i32);
    assert_eq!(format!("{std_error}"), "The data is invalid. (os error 13)");

    // ERROR_FILE_NOT_FOUND is decoded by std::io::Error::kind() as NotFound,
    // which only works because we now unwrap to the Win32 code.
    let win_error: Error = WIN32_ERROR(ERROR_FILE_NOT_FOUND).into();
    let std_error: std::io::Error = win_error.into();
    assert_eq!(
        std_error.raw_os_error().unwrap(),
        ERROR_FILE_NOT_FOUND as i32
    );
    assert_eq!(std_error.kind(), std::io::ErrorKind::NotFound);

    // E_INVALIDARG is HRESULT_FROM_WIN32(ERROR_INVALID_PARAMETER), so its
    // facility is FACILITY_WIN32 and it unwraps to the underlying Win32 code
    // (87). This matches what .NET's Marshal.GetExceptionForHR produces and
    // lets std::io::Error::kind decode it as InvalidInput.
    let win_error: Error = E_INVALIDARG.into();
    let std_error: std::io::Error = win_error.into();
    assert_eq!(
        std_error.raw_os_error().unwrap(),
        ERROR_INVALID_PARAMETER as i32
    );
    assert_eq!(
        format!("{std_error}"),
        "The parameter is incorrect. (os error 87)"
    );
    assert_eq!(std_error.kind(), std::io::ErrorKind::InvalidInput);

    // Starting with a non-FACILITY_WIN32 HRESULT (E_NOTIMPL, facility 0)...
    // the full HRESULT is preserved because the underlying value is not a
    // Win32 error code and cannot be losslessly represented as one.
    let win_error: Error = E_NOTIMPL.into();
    let std_error: std::io::Error = win_error.into();
    assert_eq!(std_error.raw_os_error().unwrap(), E_NOTIMPL.0);

    // E_FAIL is also non-FACILITY_WIN32 and is preserved unchanged.
    let win_error: Error = E_FAIL.into();
    let std_error: std::io::Error = win_error.into();
    assert_eq!(std_error.raw_os_error().unwrap(), E_FAIL.0);

    // Starting with std::io::Error...
    let std_error = std::io::Error::from_raw_os_error(ERROR_INVALID_DATA as i32);
    let error: Error = std_error.into();
    assert_eq!(error.code(), WIN32_ERROR(ERROR_INVALID_DATA).to_hresult());

    // Round-trip a FACILITY_WIN32 HRESULT through std::io::Error and back.
    let win_error: Error = WIN32_ERROR(ERROR_INVALID_DATA).into();
    let std_error: std::io::Error = win_error.into();
    let round_tripped: Error = std_error.into();
    assert_eq!(
        round_tripped.code(),
        WIN32_ERROR(ERROR_INVALID_DATA).to_hresult()
    );
}
