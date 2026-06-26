use super::*;

/// Returns `true` if `hr` is one of the DXGI/Direct2D codes that signal the
/// graphics device was lost and must be recreated (device removed/reset/hung,
/// a driver internal error, or a Direct2D "recreate target" request).
pub fn is_device_lost(hr: HRESULT) -> bool {
    matches!(
        hr,
        DXGI_ERROR_DEVICE_REMOVED
            | DXGI_ERROR_DEVICE_RESET
            | DXGI_ERROR_DEVICE_HUNG
            | DXGI_ERROR_DRIVER_INTERNAL_ERROR
            | D2DERR_RECREATE_TARGET
    )
}

/// Returns `true` if `result` is an error whose code [`is_device_lost`]. An
/// `Ok` result is never device-lost.
pub fn check_device_lost<T>(result: &Result<T>) -> bool {
    match result {
        Ok(_) => false,
        Err(e) => is_device_lost(e.code()),
    }
}
