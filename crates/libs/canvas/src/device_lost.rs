use windows_core::HRESULT;

// Well-known HRESULT values that indicate a GPU device has been lost.
const DXGI_ERROR_DEVICE_REMOVED: HRESULT = HRESULT(0x887A0005_u32 as i32);
const DXGI_ERROR_DEVICE_RESET: HRESULT = HRESULT(0x887A0007_u32 as i32);
const DXGI_ERROR_DEVICE_HUNG: HRESULT = HRESULT(0x887A0006_u32 as i32);
const DXGI_ERROR_DRIVER_INTERNAL_ERROR: HRESULT = HRESULT(0x887A0020_u32 as i32);
const D2DERR_RECREATE_TARGET: HRESULT = HRESULT(0x8899000C_u32 as i32);

/// Returns `true` if the given HRESULT indicates the GPU device was lost.
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

/// Check a `windows_core::Result` and return `true` if the error is device-lost.
pub(crate) fn check_device_lost<T>(result: &windows_core::Result<T>) -> bool {
    match result {
        Ok(_) => false,
        Err(e) => is_device_lost(e.code()),
    }
}
