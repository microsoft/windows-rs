use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Border {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub corner_radius: Option<f64>,
    pub border_brush: Option<BrushBinding>,
    pub border_thickness: Option<Thickness>,
    pub child: Box<Element>,
}
impl Border {
    pub fn new(child: impl Into<Element>) -> Self {
        Self {
            key: None,
            modifiers: Modifiers::default(),
            corner_radius: None,
            border_brush: None,
            border_thickness: None,
            child: Box::new(child.into()),
        }
    }

    /// Uniform corner radius in DIPs. Maps to WinUI `Border.CornerRadius`
    /// (all four corners). Useful for card / pill / chip layouts.
    pub fn corner_radius(mut self, v: f64) -> Self {
        self.corner_radius = Some(v);
        self
    }

    /// Brush used to paint the border stroke. Maps to WinUI
    /// `Border.BorderBrush`. Pair with [`Border::border_thickness`] for
    /// the stroke to actually render. Accepts a direct [`Color`] or a
    /// [`ThemeRef`] for theme-aware strokes.
    pub fn border_brush(mut self, v: impl Into<BrushBinding>) -> Self {
        apply_widget_brush_binding(
            &mut self.border_brush,
            &mut self.modifiers,
            Prop::BorderBrush,
            v.into(),
        );
        self
    }

    /// Stroke thickness, in DIPs, on each side. Maps to WinUI
    /// `Border.BorderThickness`.
    pub fn border_thickness(mut self, v: Thickness) -> Self {
        self.border_thickness = Some(v);
        self
    }
}
impl Default for Border {
    fn default() -> Self {
        Self {
            key: None,
            modifiers: Modifiers::default(),
            corner_radius: None,
            border_brush: None,
            border_thickness: None,
            child: Box::new(Element::Empty),
        }
    }
}

impl Widget for Border {
    widget_header!(ControlKind::Border);
    fn bindings(&self) -> PropBindings {
        let mut out = generated::border_bindings(self);
        if let Some(BrushBinding::Direct(br)) = &self.border_brush {
            out.push(Binding::Prop(
                Prop::BorderBrush,
                PropValue::Color(*br),
            ));
        }
        if let Some(v) = self.border_thickness {
            out.push(Binding::Prop(
                Prop::BorderThickness,
                PropValue::Thickness(v),
            ));
        }
        if let Some(v) = self.corner_radius {
            out.push(Binding::Prop(Prop::CornerRadius, PropValue::F64(v)));
        }
        out
    }
    fn children(&self) -> Children<'_> {
        Children::PositionalSingle(&self.child)
    }
}

pub fn border(child: impl Into<Element>) -> Border {
    Border::new(child)
}
