use super::*;

impl WinUiRuntime {
    pub(super) fn apply_shape_update(&self, id: NodeId, update: &ShapeUpdate) -> WindowsResult<()> {
        let shape: bindings::IShape = match &self.node(id)?.handle {
            Handle::Rectangle(value) => value.cast()?,
            Handle::Ellipse(value) => value.cast()?,
            Handle::Line(value) => value.cast()?,
            _ => panic!("shape update target is not a shape"),
        };
        let fill = update.fill.as_ref().map(native_brush).transpose()?;
        let stroke = update.stroke.as_ref().map(native_brush).transpose()?;
        shape.SetFill(fill.as_ref())?;
        shape.SetStroke(stroke.as_ref())?;
        shape.SetStrokeThickness(update.stroke_thickness.unwrap_or(1.0))?;

        match (&self.node(id)?.handle, update.kind) {
            (Handle::Rectangle(value), ShapeKind::Rectangle) => {
                let radius = update.corner_radius.unwrap_or(0.0);
                value.SetRadiusX(radius)?;
                value.SetRadiusY(radius)
            }
            (Handle::Ellipse(_), ShapeKind::Ellipse) => Ok(()),
            (Handle::Line(value), ShapeKind::Line) => {
                value.SetX1(update.line[0])?;
                value.SetY1(update.line[1])?;
                value.SetX2(update.line[2])?;
                value.SetY2(update.line[3])
            }
            _ => panic!("shape update target does not match the shape kind"),
        }
    }
}
