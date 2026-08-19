use crate::element::Framework;
use crate::element::props::ShapeProps;
use crate::element::tree::ElementKind;
use crate::element::{Brush, Element, IntoBrushOption, ShapeKind};
use crate::framework_properties::FrameworkProps;

pub struct Shape {
    props: ShapeProps,
}

impl Shape {
    pub fn rectangle() -> Framework<Self> {
        Framework::new(Self::new(ShapeKind::Rectangle))
    }

    pub fn ellipse() -> Framework<Self> {
        Framework::new(Self::new(ShapeKind::Ellipse))
    }

    pub fn line(x1: f64, y1: f64, x2: f64, y2: f64) -> Framework<Self> {
        Framework::new({
            for value in [x1, y1, x2, y2] {
                assert!(value.is_finite(), "line coordinates must be finite");
            }
            let mut shape = Self::new(ShapeKind::Line);
            shape.props.line = [x1, y1, x2, y2];
            shape
        })
    }

    fn new(kind: ShapeKind) -> Self {
        Self {
            props: ShapeProps {
                kind,
                fill: None,
                stroke: None,
                stroke_thickness: None,
                corner_radius: None,
                line: [0.0; 4],
                framework: FrameworkProps::default(),
            },
        }
    }

    pub(crate) fn build_with_framework(mut self, framework: FrameworkProps) -> Element {
        self.props.framework = framework;
        Element::new(ElementKind::Shape(Box::new(self.props)))
    }
}

impl Framework<Shape> {
    pub fn fill(mut self, value: impl IntoBrushOption) -> Self {
        self.control.props.fill = value.into_brush_option();
        self
    }

    pub fn fill_rgb(self, r: u8, g: u8, b: u8) -> Self {
        self.fill(Brush::from(crate::element::Color::rgb(r, g, b)))
    }

    pub fn stroke(mut self, value: impl IntoBrushOption) -> Self {
        self.control.props.stroke = value.into_brush_option();
        self
    }

    pub fn stroke_thickness(mut self, value: f64) -> Self {
        assert!(
            value.is_finite() && value >= 0.0,
            "stroke thickness must be finite and nonnegative"
        );
        self.control.props.stroke_thickness = Some(value);
        self
    }

    pub fn corner_radius(mut self, value: f64) -> Self {
        assert!(
            self.control.props.kind == ShapeKind::Rectangle,
            "corner radius is supported only by rectangles"
        );
        assert!(
            value.is_finite() && value >= 0.0,
            "corner radius must be finite and nonnegative"
        );
        self.control.props.corner_radius = Some(value);
        self
    }
}
