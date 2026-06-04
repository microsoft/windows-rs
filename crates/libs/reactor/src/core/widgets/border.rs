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
    /// the stroke to actually render. Accepts a direct [`Brush`] or a
    /// [`ThemeRef`] for theme-aware strokes.
    pub fn border_brush(mut self, v: impl Into<BrushBinding>) -> Self {
        match v.into() {
            BrushBinding::Direct(b) => {
                self.border_brush = Some(BrushBinding::Direct(b));
            }
            BrushBinding::Theme(t) => {
                self.border_brush = None;
                self.modifiers
                    .theme_bindings
                    .get_or_insert_with(|| {
                        Box::new(rustc_hash::FxHashMap::default())
                    })
                    .insert(Prop::BorderBrush, t);
            }
        }
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
        Vec::new()
    }
    fn children(&self) -> Children<'_> {
        Children::PositionalSingle(&self.child)
    }
}

pub fn border(child: impl Into<Element>) -> Border {
    Border::new(child)
}
