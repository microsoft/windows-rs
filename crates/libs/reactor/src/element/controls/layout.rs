use crate::element::Framework;
use crate::element::props::*;
use crate::element::tree::*;
use crate::element::values::*;
use crate::element::{Element, validate_border_thickness, validate_padding, validate_spacing};
use crate::framework_properties::FrameworkProps;
pub struct StackPanel {
    children: Vec<Element>,
    pub(crate) orientation: Orientation,
    pub(crate) spacing: f64,
    pub(crate) padding: Option<Thickness>,
}

pub struct Border {
    child: Box<Element>,
    background: Option<Brush>,
    border_brush: Option<Brush>,
    border_thickness: Option<Thickness>,
    corner_radius: Option<CornerRadius>,
    padding: Option<Thickness>,
}

pub struct Grid {
    children: Vec<Element>,
    pub(crate) columns: Vec<GridLength>,
    pub(crate) rows: Vec<GridLength>,
    pub(crate) column_spacing: f64,
    pub(crate) row_spacing: f64,
}

pub struct Canvas {
    children: Vec<Element>,
}

pub struct RelativePanel {
    children: Vec<Element>,
}

impl RelativePanel {
    pub fn new<T>(children: impl IntoIterator<Item = T>) -> Framework<Self>
    where
        T: Into<RelativePanelChild>,
    {
        Framework::new({
            Self {
                children: children
                    .into_iter()
                    .map(Into::into)
                    .map(RelativePanelChild::build_edge)
                    .collect(),
            }
        })
    }

    pub(crate) fn build_with_framework(self, framework: FrameworkProps) -> Element {
        Element::new(ElementKind::RelativePanel(PanelProps {
            children: self.children,
            framework,
        }))
    }
}

impl Canvas {
    pub fn new<T>(children: impl IntoIterator<Item = T>) -> Framework<Self>
    where
        T: Into<CanvasChild>,
    {
        Framework::new({
            Self {
                children: children
                    .into_iter()
                    .map(Into::into)
                    .map(CanvasChild::build_edge)
                    .collect(),
            }
        })
    }

    pub(crate) fn build_with_framework(self, framework: FrameworkProps) -> Element {
        Element::new(ElementKind::Canvas(PanelProps {
            children: self.children,
            framework,
        }))
    }
}

impl Grid {
    pub fn new<T>(children: impl IntoIterator<Item = T>) -> Framework<Self>
    where
        T: Into<GridChild>,
    {
        Framework::new({
            Self {
                children: children
                    .into_iter()
                    .map(Into::into)
                    .map(GridChild::build_edge)
                    .collect(),
                columns: Vec::new(),
                rows: Vec::new(),
                column_spacing: 0.0,
                row_spacing: 0.0,
            }
        })
    }

    pub(crate) fn build_with_framework(self, framework: FrameworkProps) -> Element {
        Element::new(ElementKind::Grid(GridProps {
            children: self.children,
            columns: self.columns,
            rows: self.rows,
            column_spacing: self.column_spacing,
            row_spacing: self.row_spacing,
            framework,
        }))
    }
}

impl Border {
    pub fn new(child: Element) -> Framework<Self> {
        Framework::new({
            Self {
                child: Box::new(child),
                background: None,
                border_brush: None,
                border_thickness: None,
                corner_radius: None,
                padding: None,
            }
        })
    }

    pub(crate) fn build_with_framework(self, framework: FrameworkProps) -> Element {
        Element::new(ElementKind::Border(Box::new(BorderElement {
            child: self.child,
            props: BorderProps {
                background: self.background,
                border_brush: self.border_brush,
                border_thickness: self.border_thickness,
                corner_radius: self.corner_radius,
                padding: self.padding,
                framework,
            },
        })))
    }
}

pub struct GridChild {
    child: Element,
    pub(crate) placement: GridPlacement,
}

impl GridChild {
    pub fn new(child: Element) -> Self {
        Self {
            child,
            placement: GridPlacement::default(),
        }
    }

    pub fn row(mut self, value: impl Into<Option<i32>>) -> Self {
        self.placement.row = validate_grid_index("row", value.into()).unwrap_or(-1);
        self
    }

    pub fn column(mut self, value: impl Into<Option<i32>>) -> Self {
        self.placement.column = validate_grid_index("column", value.into()).unwrap_or(-1);
        self
    }

    pub fn row_span(mut self, value: impl Into<Option<i32>>) -> Self {
        self.placement.row_span = validate_grid_span("row span", value.into()).unwrap_or(0);
        self
    }

    pub fn column_span(mut self, value: impl Into<Option<i32>>) -> Self {
        self.placement.column_span = validate_grid_span("column span", value.into()).unwrap_or(0);
        self
    }

    fn build_edge(mut self) -> Element {
        let key = self.child.key.take();
        Element {
            key,
            kind: ElementKind::AttachedChild {
                placement: AttachedPlacement::Grid(self.placement),
                child: Box::new(self.child),
            },
        }
    }
}

pub struct CanvasChild {
    child: Element,
    pub(crate) placement: CanvasPlacement,
}

impl CanvasChild {
    pub fn new(child: Element) -> Self {
        Self {
            child,
            placement: CanvasPlacement::default(),
        }
    }

    pub fn left(mut self, value: impl Into<Option<f64>>) -> Self {
        set_canvas_value(
            &mut self.placement.left,
            &mut self.placement.flags,
            CanvasPlacement::LEFT,
            value.into(),
        );
        self
    }

    pub fn top(mut self, value: impl Into<Option<f64>>) -> Self {
        set_canvas_value(
            &mut self.placement.top,
            &mut self.placement.flags,
            CanvasPlacement::TOP,
            value.into(),
        );
        self
    }

    pub fn z_index(mut self, value: impl Into<Option<i32>>) -> Self {
        let value = value.into();
        assert!(
            value.is_none_or(|value| value <= 1_000_000),
            "Canvas z-index must not exceed 1,000,000"
        );
        set_canvas_value(
            &mut self.placement.z_index,
            &mut self.placement.flags,
            CanvasPlacement::Z_INDEX,
            value,
        );
        self
    }

    fn build_edge(mut self) -> Element {
        let key = self.child.key.take();
        Element {
            key,
            kind: ElementKind::AttachedChild {
                placement: AttachedPlacement::Canvas(self.placement),
                child: Box::new(self.child),
            },
        }
    }
}

impl From<Element> for CanvasChild {
    fn from(value: Element) -> Self {
        Self::new(value)
    }
}

pub struct RelativePanelChild {
    child: Element,
    pub(crate) placement: RelativePanelPlacement,
}

impl RelativePanelChild {
    pub fn new(child: Element) -> Self {
        Self {
            child,
            placement: RelativePanelPlacement::default(),
        }
    }

    pub fn align_left(mut self, value: impl Into<Option<bool>>) -> Self {
        self.placement
            .set(RelativePanelPlacement::LEFT, value.into());
        self
    }

    pub fn align_right(mut self, value: impl Into<Option<bool>>) -> Self {
        self.placement
            .set(RelativePanelPlacement::RIGHT, value.into());
        self
    }

    pub fn align_top(mut self, value: impl Into<Option<bool>>) -> Self {
        self.placement
            .set(RelativePanelPlacement::TOP, value.into());
        self
    }

    pub fn align_bottom(mut self, value: impl Into<Option<bool>>) -> Self {
        self.placement
            .set(RelativePanelPlacement::BOTTOM, value.into());
        self
    }

    pub fn align_horizontal_center(mut self, value: impl Into<Option<bool>>) -> Self {
        self.placement
            .set(RelativePanelPlacement::HORIZONTAL_CENTER, value.into());
        self
    }

    pub fn align_vertical_center(mut self, value: impl Into<Option<bool>>) -> Self {
        self.placement
            .set(RelativePanelPlacement::VERTICAL_CENTER, value.into());
        self
    }

    fn build_edge(mut self) -> Element {
        let key = self.child.key.take();
        Element {
            key,
            kind: ElementKind::AttachedChild {
                placement: AttachedPlacement::RelativePanel(self.placement),
                child: Box::new(self.child),
            },
        }
    }
}

impl From<Element> for RelativePanelChild {
    fn from(value: Element) -> Self {
        Self::new(value)
    }
}

fn set_canvas_value<T: Default>(field: &mut T, flags: &mut u32, bit: u32, value: Option<T>) {
    if let Some(value) = value {
        *field = value;
        *flags |= bit;
    } else {
        *field = T::default();
        *flags &= !bit;
    }
}

impl From<Element> for GridChild {
    fn from(value: Element) -> Self {
        Self::new(value)
    }
}

fn validate_grid_index(name: &str, value: Option<i32>) -> Option<i32> {
    assert!(
        value.is_none_or(|value| value >= 0),
        "Grid {name} must be nonnegative"
    );
    value
}

fn validate_grid_length(value: GridLength) -> GridLength {
    match value {
        GridLength::Auto => {}
        GridLength::Pixel(value) | GridLength::Star(value) => {
            assert!(
                value.is_finite() && value >= 0.0,
                "Grid length must be finite and nonnegative"
            );
        }
    }
    value
}

fn validate_grid_span(name: &str, value: Option<i32>) -> Option<i32> {
    assert!(
        value.is_none_or(|value| value >= 1),
        "Grid {name} must be positive"
    );
    value
}

impl StackPanel {
    pub fn new(children: impl IntoIterator<Item = Element>) -> Framework<Self> {
        Framework::new({
            Self {
                children: children.into_iter().collect(),
                orientation: Orientation::Vertical,
                spacing: 0.0,
                padding: None,
            }
        })
    }

    pub(crate) fn build_with_framework(self, framework: FrameworkProps) -> Element {
        Element::new(ElementKind::StackPanel(StackPanelProps {
            children: self.children,
            orientation: self.orientation,
            spacing: self.spacing,
            padding: self.padding,
            framework,
        }))
    }
}

impl Framework<Grid> {
    pub fn columns(mut self, values: impl IntoIterator<Item = GridLength>) -> Self {
        self.control.columns = values.into_iter().map(validate_grid_length).collect();
        self
    }

    pub fn rows(mut self, values: impl IntoIterator<Item = GridLength>) -> Self {
        self.control.rows = values.into_iter().map(validate_grid_length).collect();
        self
    }

    pub fn column_spacing(mut self, value: f64) -> Self {
        self.control.column_spacing = validate_spacing("Grid column spacing", value);
        self
    }

    pub fn row_spacing(mut self, value: f64) -> Self {
        self.control.row_spacing = validate_spacing("Grid row spacing", value);
        self
    }
}

impl Framework<Border> {
    pub fn background(mut self, value: impl IntoBrushOption) -> Self {
        self.control.background = value.into_brush_option();
        self
    }

    pub fn border_brush(mut self, value: impl IntoBrushOption) -> Self {
        self.control.border_brush = value.into_brush_option();
        self
    }

    pub fn border_thickness(mut self, value: impl Into<Option<Thickness>>) -> Self {
        self.control.border_thickness = validate_border_thickness(value.into());
        self
    }

    pub fn corner_radius(mut self, value: impl Into<Option<CornerRadius>>) -> Self {
        let value = value.into();
        assert!(
            value.is_none_or(|value| {
                [
                    value.top_left,
                    value.top_right,
                    value.bottom_right,
                    value.bottom_left,
                ]
                .into_iter()
                .all(|value| value.is_finite() && value >= 0.0)
            }),
            "corner radius must be finite and nonnegative"
        );
        self.control.corner_radius = value;
        self
    }

    pub fn padding(mut self, value: impl Into<Option<Thickness>>) -> Self {
        self.control.padding = validate_padding(value.into());
        self
    }
}

impl Framework<StackPanel> {
    pub fn orientation(mut self, value: Orientation) -> Self {
        self.control.orientation = value;
        self
    }

    pub fn spacing(mut self, value: f64) -> Self {
        self.control.spacing = validate_spacing("StackPanel spacing", value);
        self
    }

    pub fn padding(mut self, value: impl Into<Option<Thickness>>) -> Self {
        self.control.padding = validate_padding(value.into());
        self
    }
}
