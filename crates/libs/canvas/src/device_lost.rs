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

/// Returns whether `result` is a device-lost error.
pub fn check_device_lost<T>(result: &Result<T>) -> bool {
    match result {
        Ok(_) => false,
        Err(e) => is_device_lost(e.code()),
    }
}

/// Builds the canonical [`Error`] used when device loss has no surfaced HRESULT.
pub fn device_lost_error() -> Error {
    Error::from_hresult(D2DERR_RECREATE_TARGET)
}

#[cfg(any(feature = "composition", feature = "reactor", test))]
pub(crate) fn classify_draw_results(
    draw_result: Result<()>,
    end_result: Result<()>,
) -> Result<bool> {
    if matches!(&draw_result, Err(error) if is_device_lost(error.code()))
        || matches!(&end_result, Err(error) if is_device_lost(error.code()))
    {
        return Ok(false);
    }

    match end_result {
        Ok(()) => draw_result.map(|()| true),
        Err(error) => Err(error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draw_and_end_results_classify_device_loss_together() {
        let non_device = HRESULT(0x8007_0057_u32 as i32);

        assert!(
            !classify_draw_results(
                Err(device_lost_error()),
                Err(Error::from_hresult(non_device))
            )
            .unwrap()
        );
        assert!(
            !classify_draw_results(
                Err(Error::from_hresult(non_device)),
                Err(device_lost_error())
            )
            .unwrap()
        );
    }

    #[test]
    fn end_error_wins_when_neither_result_is_device_loss() {
        let draw_code = HRESULT(0x8007_0057_u32 as i32);
        let end_code = HRESULT(0x8000_4005_u32 as i32);

        let error = classify_draw_results(
            Err(Error::from_hresult(draw_code)),
            Err(Error::from_hresult(end_code)),
        )
        .unwrap_err();
        assert_eq!(error.code(), end_code);
    }

    #[test]
    fn successful_stages_preserve_the_other_result() {
        let draw_code = HRESULT(0x8007_0057_u32 as i32);
        let end_code = HRESULT(0x8000_4005_u32 as i32);

        assert!(classify_draw_results(Ok(()), Ok(())).unwrap());

        let error = classify_draw_results(Err(Error::from_hresult(draw_code)), Ok(())).unwrap_err();
        assert_eq!(error.code(), draw_code);

        let error = classify_draw_results(Ok(()), Err(Error::from_hresult(end_code))).unwrap_err();
        assert_eq!(error.code(), end_code);
    }
}
