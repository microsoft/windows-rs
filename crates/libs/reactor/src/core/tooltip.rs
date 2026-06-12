//! Tooltip data model carried on
//! [`Modifiers::tooltip`](super::Modifiers::tooltip) and applied via
//! WinUI `ToolTipService` (an attached property on `FrameworkElement`).

use super::*;

/// Tooltip configuration applied via WinUI `ToolTipService`. Build from
/// a plain string or `Tooltip::rich(element)` for templated content.
#[derive(Clone, Debug, PartialEq)]
pub struct Tooltip {
    pub content: TooltipContent,
    pub placement: Option<TooltipPlacement>,
}

impl Tooltip {
    /// Plain-text tooltip; WinUI wraps the string in a default
    /// `ToolTip` `TextBlock`.
    pub fn text(s: impl Into<String>) -> Self {
        Self {
            content: TooltipContent::Text(s.into()),
            placement: None,
        }
    }

    /// Rich tooltip; `element` is mounted as the `Content` of a
    /// `ToolTip` instance at apply time.
    pub fn rich(element: impl Into<Element>) -> Self {
        Self {
            content: TooltipContent::Rich(Box::new(element.into())),
            placement: None,
        }
    }

    pub fn placement(mut self, p: TooltipPlacement) -> Self {
        self.placement = Some(p);
        self
    }
}

impl<S: Into<String>> From<S> for Tooltip {
    fn from(s: S) -> Self {
        Tooltip::text(s)
    }
}

/// Tooltip payload: a plain string or a templated child element.
#[derive(Clone, Debug, PartialEq)]
pub enum TooltipContent {
    Text(String),
    Rich(Box<Element>),
}

/// Rust mirror of `Microsoft.UI.Xaml.Controls.Primitives.PlacementMode`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TooltipPlacement {
    Top,
    Bottom,
    Left,
    Right,
    Mouse,
}
