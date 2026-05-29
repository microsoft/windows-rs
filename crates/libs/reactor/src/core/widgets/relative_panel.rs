use super::*;

/// Attached properties for a child of [`RelativePanelWidget`]. Controls
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
pub struct RelativePanelWidget {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub children: Vec<Element>,
}

impl RelativePanelWidget {
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

impl Widget for RelativePanelWidget {
    widget_header!(ControlKind::RelativePanel);
    fn bindings(&self) -> PropBindings {
        Vec::new()
    }
    fn children(&self) -> Children<'_> {
        Children::Keyed(&self.children)
    }
}

pub fn relative_panel<I>(children: I) -> RelativePanelWidget
where
    I: IntoIterator,
    I::Item: Into<Element>,
{
    RelativePanelWidget::new(children)
}
