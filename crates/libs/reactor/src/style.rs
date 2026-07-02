use std::{
    any::{Any, TypeId},
    borrow::Cow,
    cell::Cell,
    collections::HashMap,
    time::Duration,
};

use rustc_hash::FxHashMap;

use super::*;

impl Thickness {
    pub const fn uniform(v: f64) -> Self {
        Self {
            left: v,
            top: v,
            right: v,
            bottom: v,
        }
    }
    pub const fn xy(x: f64, y: f64) -> Self {
        Self {
            left: x,
            top: y,
            right: x,
            bottom: y,
        }
    }
}

impl From<f64> for Thickness {
    fn from(v: f64) -> Self {
        Self::uniform(v)
    }
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { a: 255, r, g, b }
    }
    pub const fn transparent() -> Self {
        Self {
            a: 0,
            r: 0,
            g: 0,
            b: 0,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GridLength {
    Auto,
    Pixel(f64),
    Star(f64),
}

impl GridLength {
    pub const STAR: Self = Self::Star(1.0);
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct WindowSize {
    pub width: f64,
    pub height: f64,
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct InnerConstraints {
    pub min_width: Option<f64>,
    pub min_height: Option<f64>,
    pub max_width: Option<f64>,
    pub max_height: Option<f64>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ScalarTransition {
    pub duration: Duration,
}

impl ScalarTransition {
    pub const fn new(duration: Duration) -> Self {
        Self { duration }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Vector3Transition {
    pub duration: Duration,
    pub components: Vector3Axes,
}

impl Vector3Transition {
    pub const fn new(duration: Duration) -> Self {
        Self {
            duration,
            components: Vector3Axes::All,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Vector3Axes {
    X,
    Y,
    Z,
    Xy,
    All,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct ImplicitTransitions {
    pub opacity: Option<ScalarTransition>,
    pub rotation: Option<ScalarTransition>,
    pub scale: Option<Vector3Transition>,
    pub translation: Option<Vector3Transition>,
}

impl ImplicitTransitions {
    pub fn is_empty(&self) -> bool {
        self.opacity.is_none()
            && self.rotation.is_none()
            && self.scale.is_none()
            && self.translation.is_none()
    }
}

/// Implicit transitions applied to layout-managed properties of an
/// element (offset/size). Fed to the backend via `set_layout_animation`.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LayoutAnimationConfig {
    pub duration: Duration,
    pub use_spring: bool,
    pub damping_ratio: f32,
    pub period: f32,
    pub animate_offset: bool,
    pub animate_size: bool,
}

impl Default for LayoutAnimationConfig {
    fn default() -> Self {
        Self {
            duration: Duration::from_millis(300),
            use_spring: false,
            damping_ratio: 0.6,
            period: 0.08,
            animate_offset: true,
            animate_size: false,
        }
    }
}

impl LayoutAnimationConfig {
    pub fn linear(duration: Duration) -> Self {
        Self {
            duration,
            ..Self::default()
        }
    }

    pub fn spring() -> Self {
        Self {
            use_spring: true,
            ..Self::default()
        }
    }

    pub fn animate_size(mut self, v: bool) -> Self {
        self.animate_size = v;
        self
    }

    pub fn animate_offset(mut self, v: bool) -> Self {
        self.animate_offset = v;
        self
    }
}

/// One-shot property animation (opacity / scale / …) driven by
/// `Backend::run_property_animation`.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AnimationConfig {
    pub opacity: Option<f64>,
    pub scale: Option<f64>,
    pub duration: Duration,
    pub easing: Easing,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            opacity: None,
            scale: None,
            duration: Duration::from_millis(300),
            easing: Easing::EaseOut,
        }
    }
}

impl AnimationConfig {
    pub fn fade_in(duration: Duration) -> Self {
        Self {
            opacity: Some(1.0),
            duration,
            easing: Easing::EaseOut,
            ..Self::default()
        }
    }

    pub fn fade_out(duration: Duration) -> Self {
        Self {
            opacity: Some(0.0),
            duration,
            easing: Easing::EaseIn,
            ..Self::default()
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Easing {
    Linear,
    EaseOut,
    EaseIn,
    EaseInOut,
}

/// Combined animation block stored on [`Modifiers`]`.animations`. All
/// fields are optional and applied independently by the backend.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct AnimationModifiers {
    pub implicit_transitions: Option<ImplicitTransitions>,
    pub layout_animation: Option<LayoutAnimationConfig>,
    pub property_animation: Option<AnimationConfig>,
    pub enter_transition: Option<AnimationConfig>,
    pub exit_transition: Option<AnimationConfig>,
}

impl AnimationModifiers {
    pub fn is_empty(&self) -> bool {
        self.implicit_transitions
            .as_ref()
            .is_none_or(|t| t.is_empty())
            && self.layout_animation.is_none()
            && self.property_animation.is_none()
            && self.enter_transition.is_none()
            && self.exit_transition.is_none()
    }
}

/// Effective color scheme reported by
/// [`RenderCx::use_color_scheme`].
#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Eq)]
pub enum ColorScheme {
    #[default]
    Light,
    Dark,
}

thread_local! {
    static CURRENT_COLOR_SCHEME: Cell<ColorScheme> = const { Cell::new(ColorScheme::Light) };
}

/// Read the host's last-known [`ColorScheme`] for the current UI thread.
pub fn current_color_scheme() -> ColorScheme {
    CURRENT_COLOR_SCHEME.with(|c| c.get())
}

/// Update the per-thread [`ColorScheme`]; called by the host on
/// `ActualThemeChanged` (and once during initial attach).
pub fn set_current_color_scheme(scheme: ColorScheme) {
    CURRENT_COLOR_SCHEME.with(|c| c.set(scheme));
}

/// Symbolic reference to a WinUI XAML theme resource (resolved at apply
/// time so the binding tracks light/dark switches).
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum ThemeRef {
    Accent,
    AccentSecondary,
    AccentTertiary,
    AccentDisabled,
    PrimaryText,
    SecondaryText,
    TertiaryText,
    DisabledText,
    AccentText,
    SolidBackground,
    CardBackground,
    SmokeFill,
    SubtleFill,
    LayerFill,
    ControlFill,
    ControlFillSecondary,
    ControlFillTertiary,
    ControlFillDisabled,
    ControlFillInputActive,
    CardStroke,
    SurfaceStroke,
    DividerStroke,
    ControlStroke,
    ControlStrokeSecondary,
    SystemAttention,
    SystemSuccess,
    SystemCaution,
    SystemCritical,
    SystemNeutral,
    SystemSolidNeutral,
    SystemAttentionBackground,
    SystemSuccessBackground,
    SystemCautionBackground,
    SystemCriticalBackground,
    SystemNeutralBackground,
    SystemSolidAttention,
    Custom(Cow<'static, str>),
}

impl ThemeRef {
    pub fn custom(key: impl Into<Cow<'static, str>>) -> Self {
        Self::Custom(key.into())
    }

    pub fn resource_key(&self) -> &str {
        match self {
            Self::Accent => "AccentFillColorDefaultBrush",
            Self::AccentSecondary => "AccentFillColorSecondaryBrush",
            Self::AccentTertiary => "AccentFillColorTertiaryBrush",
            Self::AccentDisabled => "AccentFillColorDisabledBrush",

            Self::PrimaryText => "TextFillColorPrimaryBrush",
            Self::SecondaryText => "TextFillColorSecondaryBrush",
            Self::TertiaryText => "TextFillColorTertiaryBrush",
            Self::DisabledText => "TextFillColorDisabledBrush",
            Self::AccentText => "AccentTextFillColorPrimaryBrush",

            Self::SolidBackground => "SolidBackgroundFillColorBaseBrush",
            Self::CardBackground => "CardBackgroundFillColorDefaultBrush",
            Self::SmokeFill => "SmokeFillColorDefaultBrush",
            Self::SubtleFill => "SubtleFillColorSecondaryBrush",
            Self::LayerFill => "LayerFillColorDefaultBrush",

            Self::ControlFill => "ControlFillColorDefaultBrush",
            Self::ControlFillSecondary => "ControlFillColorSecondaryBrush",
            Self::ControlFillTertiary => "ControlFillColorTertiaryBrush",
            Self::ControlFillDisabled => "ControlFillColorDisabledBrush",
            Self::ControlFillInputActive => "ControlFillColorInputActiveBrush",

            Self::CardStroke => "CardStrokeColorDefaultBrush",
            Self::SurfaceStroke => "SurfaceStrokeColorDefaultBrush",
            Self::DividerStroke => "DividerStrokeColorDefaultBrush",
            Self::ControlStroke => "ControlStrokeColorDefaultBrush",
            Self::ControlStrokeSecondary => "ControlStrokeColorSecondaryBrush",

            Self::SystemAttention => "SystemFillColorAttentionBrush",
            Self::SystemSuccess => "SystemFillColorSuccessBrush",
            Self::SystemCaution => "SystemFillColorCautionBrush",
            Self::SystemCritical => "SystemFillColorCriticalBrush",
            Self::SystemNeutral => "SystemFillColorNeutralBrush",
            Self::SystemSolidNeutral => "SystemFillColorSolidNeutralBrush",
            Self::SystemAttentionBackground => "SystemFillColorAttentionBackgroundBrush",
            Self::SystemSuccessBackground => "SystemFillColorSuccessBackgroundBrush",
            Self::SystemCautionBackground => "SystemFillColorCautionBackgroundBrush",
            Self::SystemCriticalBackground => "SystemFillColorCriticalBackgroundBrush",
            Self::SystemNeutralBackground => "SystemFillColorNeutralBackgroundBrush",
            Self::SystemSolidAttention => "SystemFillColorSolidAttentionBackgroundBrush",

            Self::Custom(s) => s.as_ref(),
        }
    }
}

/// Brush slot that can be either a literal [`Color`]
/// or a [`ThemeRef`]; used for `background` / `foreground` modifiers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BrushBinding {
    Direct(Color),
    Theme(ThemeRef),
}

impl From<Color> for BrushBinding {
    fn from(c: Color) -> Self {
        Self::Direct(c)
    }
}

impl From<ThemeRef> for BrushBinding {
    fn from(v: ThemeRef) -> Self {
        Self::Theme(v)
    }
}

#[expect(non_upper_case_globals)]
pub mod tokens {
    use super::ThemeRef;

    pub const Accent: ThemeRef = ThemeRef::Accent;

    pub const AccentSecondary: ThemeRef = ThemeRef::AccentSecondary;

    pub const AccentTertiary: ThemeRef = ThemeRef::AccentTertiary;

    pub const AccentDisabled: ThemeRef = ThemeRef::AccentDisabled;

    pub const PrimaryText: ThemeRef = ThemeRef::PrimaryText;

    pub const SecondaryText: ThemeRef = ThemeRef::SecondaryText;

    pub const TertiaryText: ThemeRef = ThemeRef::TertiaryText;

    pub const DisabledText: ThemeRef = ThemeRef::DisabledText;

    pub const AccentText: ThemeRef = ThemeRef::AccentText;

    pub const SolidBackground: ThemeRef = ThemeRef::SolidBackground;

    pub const CardBackground: ThemeRef = ThemeRef::CardBackground;

    pub const SmokeFill: ThemeRef = ThemeRef::SmokeFill;

    pub const SubtleFill: ThemeRef = ThemeRef::SubtleFill;

    pub const LayerFill: ThemeRef = ThemeRef::LayerFill;

    pub const ControlFill: ThemeRef = ThemeRef::ControlFill;

    pub const ControlFillSecondary: ThemeRef = ThemeRef::ControlFillSecondary;

    pub const ControlFillTertiary: ThemeRef = ThemeRef::ControlFillTertiary;

    pub const ControlFillDisabled: ThemeRef = ThemeRef::ControlFillDisabled;

    pub const ControlFillInputActive: ThemeRef = ThemeRef::ControlFillInputActive;

    pub const CardStroke: ThemeRef = ThemeRef::CardStroke;

    pub const SurfaceStroke: ThemeRef = ThemeRef::SurfaceStroke;

    pub const DividerStroke: ThemeRef = ThemeRef::DividerStroke;

    pub const ControlStroke: ThemeRef = ThemeRef::ControlStroke;

    pub const ControlStrokeSecondary: ThemeRef = ThemeRef::ControlStrokeSecondary;

    pub const SystemAttention: ThemeRef = ThemeRef::SystemAttention;

    pub const SystemSuccess: ThemeRef = ThemeRef::SystemSuccess;

    pub const SystemCaution: ThemeRef = ThemeRef::SystemCaution;

    pub const SystemCritical: ThemeRef = ThemeRef::SystemCritical;

    pub const SystemNeutral: ThemeRef = ThemeRef::SystemNeutral;

    pub const SystemSolidNeutral: ThemeRef = ThemeRef::SystemSolidNeutral;
}

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
    pub background: Option<Color>,
    pub foreground: Option<Color>,
    pub font_family: Option<String>,
    pub font_size: Option<f64>,
    pub theme_bindings: Option<Box<FxHashMap<Prop, ThemeRef>>>,
    pub animations: Option<Box<AnimationModifiers>>,
    pub attached: Option<AttachedProps>,
    pub accessibility: Option<Box<AccessibilityModifiers>>,
    pub keyboard_accelerators: Vec<KeyboardAccelerator>,
    pub tooltip: Option<Box<Tooltip>>,
    pub pointer_handlers: Option<Box<PointerHandlers>>,
    pub allow_drop: Option<bool>,
    pub drag_handlers: Option<Box<DragHandlers>>,
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
            && self.allow_drop.is_none()
            && self.drag_handlers.as_deref().is_none_or(|d| d.is_empty())
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
    pub on_pointer_moved: Option<Callback<PointerEventInfo>>,
    pub on_pointer_entered: Option<Callback<PointerEventInfo>>,
    pub on_pointer_exited: Option<Callback<()>>,
}

impl PointerHandlers {
    pub fn is_empty(&self) -> bool {
        self.on_tapped.is_none()
            && self.on_right_tapped.is_none()
            && self.on_pointer_pressed.is_none()
            && self.on_pointer_released.is_none()
            && self.on_pointer_moved.is_none()
            && self.on_pointer_entered.is_none()
            && self.on_pointer_exited.is_none()
    }
}

/// Pointer state captured at a pointer callback (`PointerPressed`,
/// `PointerReleased`, `PointerMoved`, or `PointerEntered`). `x`/`y` are the
/// pointer position in DIPs, relative to the top-left of the element the
/// handler is attached to. Non-mouse pointer kinds report all three button
/// flags as `false`.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct PointerEventInfo {
    pub x: f64,
    pub y: f64,
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
    pub live_setting: Option<AutomationLiveSetting>,
    pub heading_level: Option<AutomationHeadingLevel>,
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
        Self::text(s)
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
