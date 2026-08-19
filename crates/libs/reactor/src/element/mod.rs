mod construction;
mod controls;
mod framework;
pub(crate) mod props;
pub(crate) mod tree;
mod values;
mod window;

use std::rc::Rc;
use windows_time::{DateTime, TimeSpan};

use crate::framework_properties::*;
pub use crate::hooks::{
    AsyncSetState, CancellationToken, HookRef, MutationState, MutationTrigger, RenderCx, Resource,
    State,
};
pub use crate::interaction::*;
pub use crate::references::{ElementRef, WindowRef};
pub use crate::resources::{
    ApplicationResource, ApplicationResources, Context, ContextKey, ElementResources,
};
pub use construction::*;
pub use controls::*;
pub use framework::*;
use props::*;
use tree::*;
pub use values::*;
pub use window::*;

pub(crate) type RowFn = Rc<dyn Fn(usize) -> Element>;
pub(crate) type EventFn = Rc<dyn Fn()>;
pub(crate) type WindowSizeEventFn = Rc<dyn Fn(WindowSize)>;
pub(crate) type ColorSchemeEventFn = Rc<dyn Fn(ColorScheme)>;
pub(crate) type TextEventFn = Rc<dyn Fn(String)>;
pub(crate) type BoolEventFn = Rc<dyn Fn(bool)>;
pub(crate) type FloatEventFn = Rc<dyn Fn(f64)>;
pub(crate) type OptionalFloatEventFn = Rc<dyn Fn(Option<f64>)>;
pub(crate) type ColorEventFn = Rc<dyn Fn(Color)>;
pub(crate) type OptionalDateEventFn = Rc<dyn Fn(Option<DateTime>)>;
pub(crate) type DatesEventFn = Rc<dyn Fn(Vec<DateTime>)>;
pub(crate) type OptionalTimeEventFn = Rc<dyn Fn(Option<TimeSpan>)>;
pub(crate) type KeyEventFn = Rc<dyn Fn(u64)>;
pub(crate) type NavigationDisplayModeEventFn = Rc<dyn Fn(NavigationDisplayMode)>;
pub(crate) type OptionalKeyEventFn = Rc<dyn Fn(Option<u64>)>;
pub(crate) type KeysEventFn = Rc<dyn Fn(Vec<u64>)>;
pub(crate) type KeyBoolEventFn = Rc<dyn Fn(u64, bool)>;
pub(crate) type SelectionEventFn = Rc<dyn Fn(CollectionSelection)>;
pub(crate) type OptionalIndexEventFn = Rc<dyn Fn(Option<usize>)>;

pub(crate) fn validate_padding(value: Option<Thickness>) -> Option<Thickness> {
    assert!(
        value.is_none_or(|value| {
            [value.left, value.top, value.right, value.bottom]
                .into_iter()
                .all(|value| value.is_finite() && value >= 0.0)
        }),
        "padding must be finite and nonnegative"
    );
    value
}

pub(crate) fn validate_border_thickness(value: Option<Thickness>) -> Option<Thickness> {
    assert!(
        value.is_none_or(|value| {
            [value.left, value.top, value.right, value.bottom]
                .into_iter()
                .all(|value| value.is_finite() && value >= 0.0)
        }),
        "border thickness must be finite and nonnegative"
    );
    value
}

pub(crate) fn validate_spacing(name: &str, value: f64) -> f64 {
    assert!(
        value.is_finite() && value >= 0.0,
        "{name} must be finite and nonnegative"
    );
    value
}

pub(crate) fn enforce_display_only(framework: &mut FrameworkProps) {
    assert!(
        framework.control().enabled() != Some(true),
        "display-only controls cannot be enabled"
    );
    framework.set_enabled(Some(false));
}

pub struct Element {
    pub(crate) key: Option<u64>,
    pub(crate) kind: ElementKind,
}

pub struct Tooltip {
    content: Element,
    placement: Option<TooltipPlacement>,
}

impl Tooltip {
    pub fn text(text: impl Into<String>) -> Self {
        Self::rich(TextBlock::new(text).build())
    }

    pub fn rich(content: impl Into<Element>) -> Self {
        Self {
            content: content.into(),
            placement: None,
        }
    }

    pub fn placement(mut self, placement: TooltipPlacement) -> Self {
        self.placement = Some(placement);
        self
    }
}

impl From<&str> for Tooltip {
    fn from(value: &str) -> Self {
        Self::text(value)
    }
}

impl From<String> for Tooltip {
    fn from(value: String) -> Self {
        Self::text(value)
    }
}

impl Element {
    pub(crate) fn new(kind: ElementKind) -> Self {
        Self { key: None, kind }
    }

    pub(crate) fn structural_slot(slot: StructuralSlot, child: Self) -> Self {
        Self::new(ElementKind::StructuralSlot {
            slot,
            child: Box::new(child),
        })
    }

    pub fn key(mut self, key: u64) -> Self {
        self.key = Some(key);
        self
    }

    pub fn tooltip(mut self, content: Self) -> Self {
        let key = self.key.take();
        Self {
            key,
            kind: ElementKind::ToolTip(Box::new(ToolTipElement {
                owner: Box::new(self),
                content: Box::new(content),
                placement: None,
            })),
        }
    }

    pub fn tooltip_with(mut self, tooltip: Tooltip) -> Self {
        let key = self.key.take();
        Self {
            key,
            kind: ElementKind::ToolTip(Box::new(ToolTipElement {
                owner: Box::new(self),
                content: Box::new(tooltip.content),
                placement: tooltip.placement,
            })),
        }
    }

    pub fn teaching_tip(mut self, tip: TeachingTip) -> Self {
        let key = self.key.take();
        Self {
            key,
            kind: ElementKind::TeachingTip(Box::new(TeachingTipElement {
                owner: Box::new(self),
                props: TeachingTipProps {
                    title: tip.title,
                    subtitle: tip.subtitle,
                    open: tip.open,
                    light_dismiss: tip.light_dismiss,
                    action_button: tip.action_button,
                    close_button: tip.close_button,
                    on_closed: tip.on_closed,
                    on_action_button_click: tip.on_action_button_click,
                },
            })),
        }
    }
}

#[cfg(test)]
#[path = "../../testing/private/element.rs"]
mod tests;
