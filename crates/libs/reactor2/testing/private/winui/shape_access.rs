use windows_core::Interface;

use super::*;

pub(super) type ShapeProperties = (Option<(u8, u8, u8, u8)>, Option<(u8, u8, u8, u8)>, f64);

pub(super) fn shape_properties(
    runtime: &WinUiRuntime,
    id: NodeId,
) -> WindowsResult<ShapeProperties> {
    let shape: bindings::IShape = match &runtime.node(id)?.handle {
        Handle::Rectangle(value) => value.cast()?,
        Handle::Ellipse(value) => value.cast()?,
        Handle::Line(value) => value.cast()?,
        _ => panic!("native node is not a shape"),
    };
    Ok((
        brush_color(shape.Fill())?,
        brush_color(shape.Stroke())?,
        shape.StrokeThickness()?,
    ))
}

pub(super) fn rectangle_radius(runtime: &WinUiRuntime, id: NodeId) -> WindowsResult<(f64, f64)> {
    let Handle::Rectangle(value) = &runtime.node(id)?.handle else {
        panic!("native node is not a Rectangle");
    };
    Ok((value.RadiusX()?, value.RadiusY()?))
}

pub(super) fn line_points(runtime: &WinUiRuntime, id: NodeId) -> WindowsResult<[f64; 4]> {
    let Handle::Line(value) = &runtime.node(id)?.handle else {
        panic!("native node is not a Line");
    };
    Ok([value.X1()?, value.Y1()?, value.X2()?, value.Y2()?])
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
