//! Typed handler for shape widgets (Rectangle, Ellipse, Line).

use super::EventCtx;
use crate::bindings as Xaml;
use crate::core::widgets::Shape;
use crate::core::widgets::ShapeKind;
use crate::winui::backend::Handle;
use crate::winui::backend::convert::brush_of;

pub fn mount(w: &Shape, handle: &Handle, _ctx: &mut EventCtx) -> windows_core::Result<()> {
    let shape = handle.cast_inner::<Xaml::IShape>()?;
    if let Some(fill) = &w.fill {
        shape.put_Fill(&brush_of(fill)?)?;
    }
    if let Some(stroke) = &w.stroke {
        shape.put_Stroke(&brush_of(stroke)?)?;
    }
    if let Some(st) = w.stroke_thickness {
        shape.put_StrokeThickness(st)?;
    }
    match w.kind {
        ShapeKind::Rectangle => {
            if let Some(cr) = w.corner_radius {
                let r = handle.cast_inner::<Xaml::IRectangle>()?;
                r.put_RadiusX(cr)?;
                r.put_RadiusY(cr)?;
            }
        }
        ShapeKind::Line => {
            let l = handle.cast_inner::<Xaml::ILine>()?;
            l.put_X1(w.line.x1)?;
            l.put_Y1(w.line.y1)?;
            l.put_X2(w.line.x2)?;
            l.put_Y2(w.line.y2)?;
        }
        ShapeKind::Ellipse => {}
    }
    Ok(())
}

pub fn diff(
    old: &Shape,
    new: &Shape,
    handle: &Handle,
    _ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let shape = handle.cast_inner::<Xaml::IShape>()?;
    super::diff_opt!(
        old,
        new,
        fill,
        |b| shape.put_Fill(&brush_of(b)?),
        shape.put_Fill(None)
    );
    super::diff_opt!(
        old,
        new,
        stroke,
        |b| shape.put_Stroke(&brush_of(b)?),
        shape.put_Stroke(None)
    );
    super::diff_opt!(
        old,
        new,
        stroke_thickness,
        |st| shape.put_StrokeThickness(*st),
        shape.put_StrokeThickness(0.0)
    );
    if new.corner_radius != old.corner_radius
        && let ShapeKind::Rectangle = new.kind
    {
        let r = handle.cast_inner::<Xaml::IRectangle>()?;
        let cr = new.corner_radius.unwrap_or(0.0);
        r.put_RadiusX(cr)?;
        r.put_RadiusY(cr)?;
    }
    if new.line != old.line
        && let ShapeKind::Line = new.kind
    {
        let l = handle.cast_inner::<Xaml::ILine>()?;
        l.put_X1(new.line.x1)?;
        l.put_Y1(new.line.y1)?;
        l.put_X2(new.line.x2)?;
        l.put_Y2(new.line.y2)?;
    }
    Ok(())
}
