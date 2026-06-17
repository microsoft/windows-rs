use super::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ShapeKind {
    Rectangle,
    Ellipse,
    Line,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Shape {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub kind: ShapeKind,
    pub fill: Option<Color>,
    pub stroke: Option<Color>,
    pub stroke_thickness: Option<f64>,
    pub corner_radius: Option<f64>,
    pub line: LineEndpoints,
}
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct LineEndpoints {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}
impl Default for Shape {
    fn default() -> Self {
        Self {
            key: None,
            modifiers: Modifiers::default(),
            kind: ShapeKind::Rectangle,
            fill: None,
            stroke: None,
            stroke_thickness: None,
            corner_radius: None,
            line: LineEndpoints::default(),
        }
    }
}
impl Shape {
    pub fn rectangle() -> Self {
        Self {
            kind: ShapeKind::Rectangle,
            ..Default::default()
        }
    }
    pub fn ellipse() -> Self {
        Self {
            kind: ShapeKind::Ellipse,
            ..Default::default()
        }
    }
    pub fn line(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        Self {
            kind: ShapeKind::Line,
            line: LineEndpoints { x1, y1, x2, y2 },
            ..Default::default()
        }
    }
    pub fn fill(mut self, v: Color) -> Self {
        self.fill = Some(v);
        self
    }
    pub fn fill_rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        self.fill = Some(Color::rgb(r, g, b));
        self
    }
    pub fn stroke(mut self, v: Color) -> Self {
        self.stroke = Some(v);
        self
    }
    pub fn stroke_thickness(mut self, v: f64) -> Self {
        self.stroke_thickness = Some(v);
        self
    }
    pub fn corner_radius(mut self, v: f64) -> Self {
        self.corner_radius = Some(v);
        self
    }
}

impl Widget for Shape {
    fn kind(&self) -> ControlKind {
        match self.kind {
            ShapeKind::Rectangle => ControlKind::Rectangle,
            ShapeKind::Ellipse => ControlKind::Ellipse,
            ShapeKind::Line => ControlKind::Line,
        }
    }
    fn key(&self) -> Option<&str> {
        self.key.as_deref()
    }
    fn modifiers(&self) -> &Modifiers {
        &self.modifiers
    }
    fn bindings(&self) -> PropBindings {
        let mut out = Vec::with_capacity(5);
        if let Some(fill) = &self.fill {
            out.push(Binding::Prop(Prop::Fill, PropValue::Color(*fill)));
        }
        if let Some(stroke) = &self.stroke {
            out.push(Binding::Prop(
                Prop::Stroke,
                PropValue::Color(*stroke),
            ));
        }
        if let Some(th) = self.stroke_thickness {
            out.push(Binding::Prop(Prop::StrokeThickness, PropValue::F64(th)));
        }
        if let Some(cr) = self.corner_radius
            && matches!(self.kind, ShapeKind::Rectangle) {
                out.push(Binding::Prop(Prop::CornerRadius, PropValue::F64(cr)));
            }
        if matches!(self.kind, ShapeKind::Line) {
            out.push(Binding::Prop(
                Prop::LineEndpoints,
                PropValue::LineEndpoints(self.line),
            ));
        }
        out
    }
}
