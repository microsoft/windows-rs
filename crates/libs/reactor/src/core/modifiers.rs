use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use rustc_hash::FxHashMap;

use super::*;

/// Visual modifiers shared by every widget; carried on each element struct
/// and applied uniformly via `FrameworkElement`-level setters at the
/// backend.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Modifiers {
    pub margin: Option<Thickness>,
    pub padding: Option<Thickness>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub min_width: Option<f64>,
    pub max_width: Option<f64>,
    pub min_height: Option<f64>,
    pub max_height: Option<f64>,
    pub horizontal_alignment: Option<HorizontalAlignment>,
    pub vertical_alignment: Option<VerticalAlignment>,
    pub opacity: Option<f64>,
    pub background: Option<Brush>,
    pub foreground: Option<Brush>,
    pub font_family: Option<String>,
    pub font_size: Option<f64>,
    pub theme_bindings: Option<Box<FxHashMap<Prop, ThemeRef>>>,
    pub animations: Option<Box<AnimationModifiers>>,
    pub attached: Option<AttachedProps>,
    pub accessibility: Option<Box<AccessibilityModifiers>>,
    pub keyboard_accelerators: Vec<KeyboardAccelerator>,
    pub tooltip: Option<Box<Tooltip>>,
    pub pointer_handlers: Option<Box<PointerHandlers>>,
    /// Fast-path for grid row/column placement — avoids the `AttachedProps`
    /// HashMap + Box + thread_local overhead for the most common attached prop.
    pub grid: Option<GridPlacement>,
    pub resources: HashMap<String, String>,
}

impl Modifiers {
    pub fn is_empty(&self) -> bool {
        self.margin.is_none()
            && self.padding.is_none()
            && self.width.is_none()
            && self.height.is_none()
            && self.min_width.is_none()
            && self.max_width.is_none()
            && self.min_height.is_none()
            && self.max_height.is_none()
            && self.horizontal_alignment.is_none()
            && self.vertical_alignment.is_none()
            && self.opacity.is_none()
            && self.background.is_none()
            && self.foreground.is_none()
            && self.font_family.is_none()
            && self.font_size.is_none()
            && self.theme_bindings.as_ref().is_none_or(|m| m.is_empty())
            && self.animations.as_ref().is_none_or(|a| a.is_empty())
            && self.attached.as_ref().is_none_or(|a| a.is_empty())
            && self.accessibility.as_deref().is_none_or(|a| a.is_empty())
            && self.keyboard_accelerators.is_empty()
            && self.tooltip.is_none()
            && self
                .pointer_handlers
                .as_deref()
                .is_none_or(|p| p.is_empty())
            && self.grid.is_none()
            && self.resources.is_empty()
    }
}

/// Type-erased bag of attached properties (e.g. [`GridPlacement`]) keyed
/// by [`TypeId`]; values must be inserted via [`AttachedProps::set`].
#[derive(Default, Debug)]
pub struct AttachedProps(FxHashMap<TypeId, Box<dyn AttachedValue>>);

impl Clone for AttachedProps {
    fn clone(&self) -> Self {
        let mut copy = FxHashMap::default();
        for (k, v) in &self.0 {
            copy.insert(*k, v.clone_box());
        }
        Self(copy)
    }
}

impl PartialEq for AttachedProps {
    fn eq(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }
        for (k, v) in &self.0 {
            let Some(ov) = other.0.get(k) else {
                return false;
            };
            if !v.eq_box(ov.as_any()) {
                return false;
            }
        }
        true
    }
}

impl AttachedProps {
    pub fn set<T: Clone + PartialEq + 'static>(&mut self, v: T) {
        self.0.insert(TypeId::of::<T>(), Box::new(v));
    }
    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.0
            .get(&TypeId::of::<T>())
            .and_then(|b| b.as_any().downcast_ref::<T>())
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GridPlacement {
    pub row: i32,
    pub column: i32,
    pub row_span: i32,
    pub column_span: i32,
}

impl Default for GridPlacement {
    fn default() -> Self {
        Self {
            row: 0,
            column: 0,
            row_span: 1,
            column_span: 1,
        }
    }
}

/// Trait object carrying clone/eq in its vtable so `AttachedProps` doesn't
/// need a separate type-registry thread-local.
trait AttachedValue: Any {
    fn clone_box(&self) -> Box<dyn AttachedValue>;
    fn eq_box(&self, other: &dyn Any) -> bool;
    fn as_any(&self) -> &dyn Any;
}

impl<T: Clone + PartialEq + 'static> AttachedValue for T {
    fn clone_box(&self) -> Box<dyn AttachedValue> {
        Box::new(self.clone())
    }
    fn eq_box(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<T>().is_some_and(|o| self == o)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl std::fmt::Debug for dyn AttachedValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("AttachedValue")
    }
}

// --- Pointer event handlers ---

/// Bundle of per-element pointer / tap callbacks; each slot is
/// individually optional.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct PointerHandlers {
    pub on_tapped: Option<Callback<()>>,
    pub on_right_tapped: Option<Callback<()>>,
    pub on_pointer_pressed: Option<Callback<PointerEventInfo>>,
    pub on_pointer_released: Option<Callback<PointerEventInfo>>,
    pub on_pointer_exited: Option<Callback<()>>,
}

impl PointerHandlers {
    pub fn is_empty(&self) -> bool {
        self.on_tapped.is_none()
            && self.on_right_tapped.is_none()
            && self.on_pointer_pressed.is_none()
            && self.on_pointer_released.is_none()
            && self.on_pointer_exited.is_none()
    }
}

/// Button state captured at a `PointerPressed` / `PointerReleased`
/// callback. Non-mouse pointer kinds report all three as `false`.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct PointerEventInfo {
    pub is_left_button_pressed: bool,
    pub is_right_button_pressed: bool,
    pub is_middle_button_pressed: bool,
}

// --- Accessibility ---

/// UI Automation properties applied to every widget kind via
/// [`Modifiers::accessibility`].
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct AccessibilityModifiers {
    pub automation_name: Option<String>,
    pub automation_id: Option<String>,
    pub help_text: Option<String>,
    pub live_setting: Option<LiveSetting>,
    pub heading_level: Option<HeadingLevel>,
}

impl AccessibilityModifiers {
    pub fn is_empty(&self) -> bool {
        self.automation_name.is_none()
            && self.automation_id.is_none()
            && self.help_text.is_none()
            && self.live_setting.is_none()
            && self.heading_level.is_none()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LiveSetting {
    Off,
    Polite,
    Assertive,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HeadingLevel {
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
    Level6,
    Level7,
    Level8,
    Level9,
}

// --- Tooltip ---

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
