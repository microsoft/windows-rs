use super::*;

pub(super) fn progress_bar(
    runtime: &WinUiRuntime,
    id: NodeId,
) -> WindowsResult<(f64, f64, f64, bool)> {
    let Handle::ProgressBar(value) = &runtime.node(id)?.handle else {
        panic!("native node is not a ProgressBar");
    };
    let range: bindings::IRangeBase = value.cast()?;
    let progress: bindings::IProgressBar = value.cast()?;
    Ok((
        range.Value()?,
        range.Minimum()?,
        range.Maximum()?,
        progress.IsIndeterminate()?,
    ))
}

pub(super) fn progress_ring(
    runtime: &WinUiRuntime,
    id: NodeId,
) -> WindowsResult<(f64, f64, f64, bool, bool)> {
    let Handle::ProgressRing(value) = &runtime.node(id)?.handle else {
        panic!("native node is not a ProgressRing");
    };
    let ring: bindings::IProgressRing = value.cast()?;
    Ok((
        ring.Value()?,
        ring.Minimum()?,
        ring.Maximum()?,
        ring.IsActive()?,
        ring.IsIndeterminate()?,
    ))
}
