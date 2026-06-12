use super::*;

pub(crate) fn is_device_lost(hr: HRESULT) -> bool {
    matches!(
        hr,
        DXGI_ERROR_DEVICE_REMOVED
            | DXGI_ERROR_DEVICE_RESET
            | DXGI_ERROR_DEVICE_HUNG
            | DXGI_ERROR_DRIVER_INTERNAL_ERROR
            | D2DERR_RECREATE_TARGET
    )
}

pub(crate) fn check_device_lost<T>(result: &Result<T>) -> bool {
    match result {
        Ok(_) => false,
        Err(e) => is_device_lost(e.code()),
    }
}
