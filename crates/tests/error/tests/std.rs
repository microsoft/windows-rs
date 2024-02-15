use windows::Win32::Foundation::*;

#[test]
fn conversions() {
    helpers::set_thread_ui_language();

    // Baseline HRESULT
    assert_eq!(E_INVALIDARG.message(), "The parameter is incorrect.");
    assert_eq!(E_INVALIDARG.0, -2147024809);

    // Baseline WIN32_ERROR
    assert_eq!(
        ERROR_INVALID_DATA.to_hresult().message(),
        "The data is invalid."
    );
    assert_eq!(ERROR_INVALID_DATA.0, 13);

    // std::io::Error from HRESULT
    let std_error = std::io::Error::from_raw_os_error(E_INVALIDARG.0);
    assert_eq!(std_error.raw_os_error().unwrap(), E_INVALIDARG.0);
    assert_eq!(
        format!("{std_error}"),
        "The parameter is incorrect. (os error -2147024809)"
    );

    // std::io::Error from WIN32_ERROR
    let std_error = std::io::Error::from_raw_os_error(ERROR_INVALID_DATA.0 as i32);
    assert_eq!(
        std_error.raw_os_error().unwrap(),
        ERROR_INVALID_DATA.0 as i32
    );
    assert_eq!(format!("{std_error}"), "The data is invalid. (os error 13)");

    // Starting with WIN32_ERROR...
    let win_error: windows::core::Error = ERROR_INVALID_DATA.into();
    let std_error: std::io::Error = win_error.into();
    assert_eq!(
        std_error.raw_os_error().unwrap(),
        ERROR_INVALID_DATA.to_hresult().0
    );
    assert_eq!(
        format!("{std_error}"),
        "The data is invalid. (os error -2147024883)"
    );

    // Starting with HRESULT...
    let win_error: windows::core::Error = E_INVALIDARG.into();
    let std_error: std::io::Error = win_error.into();
    assert_eq!(std_error.raw_os_error().unwrap(), E_INVALIDARG.0);
    assert_eq!(
        format!("{std_error}"),
        "The parameter is incorrect. (os error -2147024809)"
    );

    // Starting with std::io::Error...
    let std_error = std::io::Error::from_raw_os_error(ERROR_INVALID_DATA.0 as i32);
    let error: windows::core::Error = std_error.into();
    assert_eq!(error.code(), ERROR_INVALID_DATA.to_hresult());
}
