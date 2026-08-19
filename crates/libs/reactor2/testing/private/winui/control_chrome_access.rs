use windows_core::Interface;

use super::*;

pub(super) type ControlChrome = (
    Option<(u8, u8, u8, u8)>,
    Option<(u8, u8, u8, u8)>,
    (f64, f64, f64, f64),
);

pub(super) fn control_chrome(runtime: &WinUiRuntime, id: NodeId) -> WindowsResult<ControlChrome> {
    let control = runtime.node(id)?.handle.control()?;
    let thickness = control.BorderThickness()?;
    Ok((
        brush_color(control.Background())?,
        brush_color(control.BorderBrush())?,
        (
            thickness.left,
            thickness.top,
            thickness.right,
            thickness.bottom,
        ),
    ))
}

fn brush_color(brush: WindowsResult<bindings::Brush>) -> WindowsResult<Option<(u8, u8, u8, u8)>> {
    let brush = match brush {
        Ok(brush) => brush,
        Err(error) if error.code().0 == 0 => return Ok(None),
        Err(error) => return Err(error),
    };
    if brush.as_raw().is_null() {
        return Ok(None);
    }
    let color = brush.cast::<bindings::SolidColorBrush>()?.Color()?;
    Ok(Some((color.a, color.r, color.g, color.b)))
}
