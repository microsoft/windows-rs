use super::*;

/// Attached properties for a child of [`RelativePanel`]. Controls
/// alignment relative to the panel edges/center.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct RelativePanelAlignment {
    pub align_left_with_panel: bool,
    pub align_right_with_panel: bool,
    pub align_top_with_panel: bool,
    pub align_bottom_with_panel: bool,
    pub align_h_center_with_panel: bool,
    pub align_v_center_with_panel: bool,
}

/// `Microsoft.UI.Xaml.Controls.RelativePanel`. A constraint-based layout
/// where children are positioned relative to the panel edges or center.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct RelativePanel {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub children: Vec<Element>,
}

impl RelativePanel {
    pub fn new<I>(children: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Element>,
    {
        Self {
            children: children.into_iter().map(Into::into).collect(),
            ..Default::default()
        }
    }
}

impl Widget for RelativePanel {
    widget_header!(ControlKind::RelativePanel);
    fn bindings(&self) -> PropBindings {
        generated::relative_panel_bindings(self)
    }
    fn children(&self) -> Children<'_> {
        Children::Keyed(&self.children)
    }
}

pub fn relative_panel<I>(children: I) -> RelativePanel
where
    I: IntoIterator,
    I::Item: Into<Element>,
{
    RelativePanel::new(children)
}
