use super::*;
use std::any::{Any, TypeId};
use std::time::Duration;

// Extension hatch for downstream widgets that aren't part of the core
// [`Element`] enum. Leaf-only in step 1a.

/// Out-of-tree widget definition managed by the reconciler via
/// [`Element::Custom`].
pub trait CustomElement: 'static {
    /// Standard `Any` accessor; the implementor's body is almost always `self`.
    fn as_any(&self) -> &dyn Any;

    /// Stable type identity used by the reconciler to decide
    /// mount-vs-update; defaults to the underlying `Any::type_id`.
    fn type_id(&self) -> TypeId {
        self.as_any().type_id()
    }

    /// Human-readable name surfaced in diagnostics and `Element::kind_name`.
    fn kind_name(&self) -> &'static str;

    /// Optional key for keyed reconciliation inside multi-child containers.
    fn key(&self) -> Option<&str> {
        None
    }

    /// Structural equality against another element of the same `type_id`;
    /// returning `false` is always safe but skips an `update` short-circuit.
    fn eq_dyn(&self, other: &dyn CustomElement) -> bool;

    /// Boxed clone so [`Element`] stays `Clone`.
    fn clone_dyn(&self) -> Box<dyn CustomElement>;

    /// Create the underlying control via `backend` and return its id.
    fn mount(&self, backend: &mut dyn Backend) -> ControlId;

    /// Apply the diff from `prev` (same `type_id`) to the live control `id`.
    fn update(&self, prev: &dyn CustomElement, id: ControlId, backend: &mut dyn Backend);

    /// Hook fired just before [`Backend::destroy`]; defaults to no-op.
    fn before_destroy(&self, _id: ControlId, _backend: &mut dyn Backend) {}
}

/// Boxed [`CustomElement`] with `Clone` / `Debug` / `PartialEq` so it can
/// live inside the `Element` enum.
pub struct CustomElementHandle(pub Box<dyn CustomElement>);

impl CustomElementHandle {
    pub fn new<C: CustomElement>(c: C) -> Self {
        Self(Box::new(c))
    }
    pub fn get(&self) -> &dyn CustomElement {
        &*self.0
    }
}

impl Clone for CustomElementHandle {
    fn clone(&self) -> Self {
        Self(self.0.clone_dyn())
    }
}

impl std::fmt::Debug for CustomElementHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CustomElement")
            .field("kind", &self.0.kind_name())
            .field("type_id", &CustomElement::type_id(&*self.0))
            .field("key", &self.0.key())
            .finish()
    }
}

impl PartialEq for CustomElementHandle {
    fn eq(&self, other: &Self) -> bool {
        if CustomElement::type_id(&*self.0) != CustomElement::type_id(&*other.0) {
            return false;
        }
        if self.0.key() != other.0.key() {
            return false;
        }
        self.0.eq_dyn(&*other.0)
    }
}

impl_rc_fn_wrapper! {
    /// Renders a fallback subtree given a panic message string.
    pub struct Fallback(dyn Fn(&str) -> Element);
}

impl Fallback {
    pub fn invoke(&self, message: &str) -> Element {
        (self.inner)(message)
    }
}

/// Subtree wrapper that catches panics from descendants and renders the
/// `fallback` element instead. Carried by [`Element::ErrorBoundary`].
#[derive(Clone, Debug, PartialEq)]
pub struct ErrorBoundaryElement {
    pub key: Option<String>,
    pub child: Box<Element>,
    pub fallback: Fallback,
}

/// Wrap `child` in an error-boundary element using `fallback` for recovery.
pub fn error_boundary<F>(child: impl Into<Element>, fallback: F) -> Element
where
    F: Fn(&str) -> Element + 'static,
{
    Element::ErrorBoundary(ErrorBoundaryElement {
        key: None,
        child: Box::new(child.into()),
        fallback: Fallback::new(fallback),
    })
}

pub fn panic_message(payload: Box<dyn Any + Send>) -> String {
    if let Some(s) = payload.downcast_ref::<&'static str>() {
        (*s).to_string()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "<non-string panic>".to_string()
    }
}

// Trait for converting heterogeneous tuples into `Vec<Element>`.
//
// This enables writing `vstack((title, body, footer))` without macros,
// where each tuple element can be a different type that implements
// `Into<Element>`.

/// Converts a collection of items into a `Vec<Element>`.
///
/// Implemented for:
/// - Tuples of up to 12 elements, each `Into<Element>` (heterogeneous lists)
/// - `Vec<Element>` (pre-built dynamic lists)
/// - Fixed-size arrays `[T; N]` where `T: Into<Element>`
pub trait IntoElements {
    fn into_elements(self) -> Vec<Element>;
}

// Vec<Element> passthrough
impl IntoElements for Vec<Element> {
    fn into_elements(self) -> Vec<Element> {
        self
    }
}

// Empty tuple = no children
impl IntoElements for () {
    fn into_elements(self) -> Vec<Element> {
        Vec::new()
    }
}

// Fixed-size arrays of Into<Element>
impl<T: Into<Element>, const N: usize> IntoElements for [T; N] {
    fn into_elements(self) -> Vec<Element> {
        self.into_iter().map(Into::into).collect()
    }
}

// Tuple impls — each element independently implements Into<Element>.
macro_rules! impl_into_elements_for_tuple {
    ($($idx:tt : $T:ident),+) => {
        impl<$($T: Into<Element>),+> IntoElements for ($($T,)+) {
            fn into_elements(self) -> Vec<Element> {
                vec![$(self.$idx.into()),+]
            }
        }
    };
}

impl_into_elements_for_tuple!(0: A);
impl_into_elements_for_tuple!(0: A, 1: B);
impl_into_elements_for_tuple!(0: A, 1: B, 2: C);
impl_into_elements_for_tuple!(0: A, 1: B, 2: C, 3: D);
impl_into_elements_for_tuple!(0: A, 1: B, 2: C, 3: D, 4: E);
impl_into_elements_for_tuple!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F);
impl_into_elements_for_tuple!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G);
impl_into_elements_for_tuple!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H);
impl_into_elements_for_tuple!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I2);
impl_into_elements_for_tuple!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I2, 9: J);
impl_into_elements_for_tuple!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I2, 9: J, 10: K);
impl_into_elements_for_tuple!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I2, 9: J, 10: K, 11: L);

/// Fragment-style element flattened into its parent's child list during
/// reconciliation; only valid inside multi-child containers.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct GroupElement {
    pub key: Option<String>,
    pub children: Vec<Element>,
}

impl GroupElement {
    pub fn new(children: Vec<Element>) -> Self {
        Self {
            key: None,
            children,
        }
    }

    pub fn with_key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }
}

/// Convenience constructor: `group([a, b, c].into())` → `Element::Group(...)`.
pub fn group(children: Vec<Element>) -> Element {
    Element::Group(GroupElement::new(children))
}

/// Single source of truth for every built-in widget variant of [`Element`]:
/// drives the enum arms, `From` impls, and `as_widget` / `modifiers_mut` /
/// `kind_name` dispatch.
///
/// By convention, every variant here is named after its backing
/// `Microsoft.UI.Xaml` class (e.g. `TextBlock`, `StackPanel`, `TextBox`,
/// `ScrollViewer`, `RichTextBlock`).
///
/// To add a new built-in widget element: add a row here, define the widget
/// struct and its `impl Widget`, add the matching `ControlKind` variant in
/// `core/backend.rs`, and add the matching row in
/// `winui/backend.rs::define_handles!`.
macro_rules! define_element {
    ( $( $variant:ident ),* $(,)? ) => {
        /// Sum type of every element kind the reconciler can mount; widget
        /// variants each correspond to a backend control, plus a few
        /// non-widget variants for composition (`Component`, `Group`,
        /// `Custom`, …).
        #[derive(Clone, Debug, PartialEq, Default)]
        pub enum Element {
            $( $variant($variant), )*
            Component(ComponentElement),
            ErrorBoundary(ErrorBoundaryElement),
            Provider(ProviderElement),
            TemplatedList(TemplatedListElement),
            Group(GroupElement),
            Custom(CustomElementHandle),
            #[default]
            Empty,
        }

        $(
            impl From<$variant> for Element {
                fn from(v: $variant) -> Self {
                    Element::$variant(v)
                }
            }
        )*

        impl Element {
            pub fn as_widget(&self) -> Option<&dyn Widget> {
                Some(match self {
                    $( Element::$variant(v) => v, )*
                    Element::Component(_)
                    | Element::ErrorBoundary(_)
                    | Element::Provider(_)
                    | Element::TemplatedList(_)
                    | Element::Group(_)
                    | Element::Custom(_)
                    | Element::Empty => return None,
                })
            }
            fn modifiers_mut(&mut self) -> Option<&mut Modifiers> {
                match self {
                    $( Element::$variant(v) => Some(&mut v.modifiers), )*
                    Element::TemplatedList(tl) => Some(&mut tl.modifiers),
                    Element::Component(_)
                    | Element::ErrorBoundary(_)
                    | Element::Provider(_)
                    | Element::Group(_)
                    | Element::Custom(_)
                    | Element::Empty => None,
                }
            }
            pub fn kind_name(&self) -> &'static str {
                match self {
                    $( Element::$variant(_) => stringify!($variant), )*
                    Element::Component(_) => "Component",
                    Element::ErrorBoundary(_) => "ErrorBoundary",
                    Element::Provider(_) => "Provider",
                    Element::TemplatedList(_) => "TemplatedList",
                    Element::Group(_) => "Group",
                    Element::Custom(c) => c.0.kind_name(),
                    Element::Empty => "Empty",
                }
            }
        }
    };
}

define_element! {
    TextBlock,
    Button,
    StackPanel,
    Border,
    CheckBox,
    TextBox,
    Grid,
    ScrollViewer,
    ToggleSwitch,
    Slider,
    RadioButton,
    NumberBox,
    ProgressBar,
    ProgressRing,
    Expander,
    HyperlinkButton,
    InfoBar,
    InfoBadge,
    PersonPicture,
    Shape,
    Image,
    TabView,
    NavigationView,
    TitleBar,
    Pivot,
    BreadcrumbBar,
    PasswordBox,
    RadioButtons,
    ComboBox,
    Canvas,
    RichTextBlock,
    ContentDialog,
    Viewbox,
    RepeatButton,
    RatingControl,
    ColorPicker,
    DatePicker,
    TimePicker,
    CalendarDatePicker,
    CalendarView,
    ListBox,
    DropDownButton,
    SplitButton,
    AutoSuggestBox,
    SplitView,
    MenuBar,
    ScrollView,
    TreeView,
    CommandBar,
    TeachingTip,
    SelectorBar,
    RichEditBox,
    RelativePanel,
    ToggleButton,
    SwapChainPanel,
    CompositionHost,
    WebView2,
}

macro_rules! non_widget_from_table {
    ( $( $variant:ident : $ty:ty ),* $(,)? ) => {
        $(
            impl From<$ty> for Element {
                fn from(v: $ty) -> Self { Element::$variant(v) }
            }
        )*
    };
}

non_widget_from_table! {
    Component:     ComponentElement,
    ErrorBoundary: ErrorBoundaryElement,
    Provider:      ProviderElement,
    TemplatedList: TemplatedListElement,
    Group:         GroupElement,
    Custom:        CustomElementHandle,
}

impl Element {
    pub fn key(&self) -> Option<&str> {
        if let Some(w) = self.as_widget() {
            return w.key();
        }
        match self {
            Self::Component(c) => c.key.as_deref(),
            Self::ErrorBoundary(eb) => eb.key.as_deref(),
            Self::Provider(p) => p.key.as_deref(),
            Self::TemplatedList(tl) => tl.key.as_deref(),
            Self::Group(g) => g.key.as_deref(),
            Self::Custom(c) => c.0.key(),
            Self::Empty => None,
            _ => unreachable!("covered by as_widget"),
        }
    }
    pub fn modifiers(&self) -> Option<&Modifiers> {
        if let Some(w) = self.as_widget() {
            return Some(w.modifiers());
        }
        match self {
            Self::TemplatedList(tl) => Some(&tl.modifiers),
            Self::Component(_)
            | Self::ErrorBoundary(_)
            | Self::Provider(_)
            | Self::Group(_)
            | Self::Custom(_)
            | Self::Empty => None,
            _ => unreachable!("covered by as_widget"),
        }
    }
    pub fn attached_mut(&mut self) -> Option<&mut AttachedProps> {
        let m = self.modifiers_mut()?;
        Some(m.attached.get_or_insert_with(AttachedProps::default))
    }
    pub fn attached(&self) -> Option<&AttachedProps> {
        self.modifiers().and_then(|m| m.attached.as_ref())
    }
    pub fn grid_row(mut self, row: i32) -> Self {
        if let Some(m) = self.modifiers_mut() {
            let mut p = m.grid.unwrap_or_default();
            p.row = row;
            m.grid = Some(p);
        }
        self
    }
    pub fn grid_column(mut self, column: i32) -> Self {
        if let Some(m) = self.modifiers_mut() {
            let mut p = m.grid.unwrap_or_default();
            p.column = column;
            m.grid = Some(p);
        }
        self
    }
    pub fn grid_row_span(mut self, rs: i32) -> Self {
        if let Some(m) = self.modifiers_mut() {
            let mut p = m.grid.unwrap_or_default();
            p.row_span = rs;
            m.grid = Some(p);
        }
        self
    }
    pub fn grid_column_span(mut self, cs: i32) -> Self {
        if let Some(m) = self.modifiers_mut() {
            let mut p = m.grid.unwrap_or_default();
            p.column_span = cs;
            m.grid = Some(p);
        }
        self
    }
    pub fn canvas_left(mut self, x: f64) -> Self {
        if let Some(bag) = self.attached_mut() {
            let mut p = bag.get::<CanvasPosition>().copied().unwrap_or_default();
            p.left = x;
            bag.set(p);
        }
        self
    }
    pub fn canvas_top(mut self, y: f64) -> Self {
        if let Some(bag) = self.attached_mut() {
            let mut p = bag.get::<CanvasPosition>().copied().unwrap_or_default();
            p.top = y;
            bag.set(p);
        }
        self
    }
    pub fn canvas_z_index(mut self, z: i32) -> Self {
        if let Some(bag) = self.attached_mut() {
            let mut p = bag.get::<CanvasPosition>().copied().unwrap_or_default();
            p.z_index = z;
            bag.set(p);
        }
        self
    }
    pub fn relative_align_left(mut self) -> Self {
        if let Some(bag) = self.attached_mut() {
            let mut p = bag
                .get::<RelativePanelAlignment>()
                .copied()
                .unwrap_or_default();
            p.align_left_with_panel = true;
            bag.set(p);
        }
        self
    }
    pub fn relative_align_right(mut self) -> Self {
        if let Some(bag) = self.attached_mut() {
            let mut p = bag
                .get::<RelativePanelAlignment>()
                .copied()
                .unwrap_or_default();
            p.align_right_with_panel = true;
            bag.set(p);
        }
        self
    }
    pub fn relative_align_top(mut self) -> Self {
        if let Some(bag) = self.attached_mut() {
            let mut p = bag
                .get::<RelativePanelAlignment>()
                .copied()
                .unwrap_or_default();
            p.align_top_with_panel = true;
            bag.set(p);
        }
        self
    }
    pub fn relative_align_bottom(mut self) -> Self {
        if let Some(bag) = self.attached_mut() {
            let mut p = bag
                .get::<RelativePanelAlignment>()
                .copied()
                .unwrap_or_default();
            p.align_bottom_with_panel = true;
            bag.set(p);
        }
        self
    }
    pub fn relative_align_h_center(mut self) -> Self {
        if let Some(bag) = self.attached_mut() {
            let mut p = bag
                .get::<RelativePanelAlignment>()
                .copied()
                .unwrap_or_default();
            p.align_h_center_with_panel = true;
            bag.set(p);
        }
        self
    }
    pub fn relative_align_v_center(mut self) -> Self {
        if let Some(bag) = self.attached_mut() {
            let mut p = bag
                .get::<RelativePanelAlignment>()
                .copied()
                .unwrap_or_default();
            p.align_v_center_with_panel = true;
            bag.set(p);
        }
        self
    }
    fn accessibility_mut(&mut self) -> Option<&mut AccessibilityModifiers> {
        let m = self.modifiers_mut()?;
        Some(
            m.accessibility
                .get_or_insert_with(|| Box::new(AccessibilityModifiers::default())),
        )
    }
    pub fn accessibility(&self) -> Option<&AccessibilityModifiers> {
        self.modifiers().and_then(|m| m.accessibility.as_deref())
    }
    pub fn automation_name(mut self, name: impl Into<String>) -> Self {
        if let Some(a) = self.accessibility_mut() {
            a.automation_name = Some(name.into());
        }
        self
    }
    pub fn automation_id(mut self, id: impl Into<String>) -> Self {
        if let Some(a) = self.accessibility_mut() {
            a.automation_id = Some(id.into());
        }
        self
    }
    pub fn help_text(mut self, text: impl Into<String>) -> Self {
        if let Some(a) = self.accessibility_mut() {
            a.help_text = Some(text.into());
        }
        self
    }
    pub fn accessibility_live_setting(mut self, ls: AutomationLiveSetting) -> Self {
        if let Some(a) = self.accessibility_mut() {
            a.live_setting = Some(ls);
        }
        self
    }
    pub fn heading_level(mut self, level: AutomationHeadingLevel) -> Self {
        if let Some(a) = self.accessibility_mut() {
            a.heading_level = Some(level);
        }
        self
    }
    pub fn keyboard_accelerator(mut self, accel: KeyboardAccelerator) -> Self {
        if let Some(m) = self.modifiers_mut() {
            m.keyboard_accelerators.push(accel);
        }
        self
    }
    /// `true` when both elements are the same variant (and same shape /
    /// custom type id), so an update can be considered.
    pub fn kind_matches(&self, other: &Self) -> bool {
        if std::mem::discriminant(self) != std::mem::discriminant(other) {
            return false;
        }
        if let (Self::Shape(a), Self::Shape(b)) = (self, other) {
            return a.kind == b.kind;
        }
        if let (Self::Custom(a), Self::Custom(b)) = (self, other) {
            return CustomElement::type_id(&*a.0) == CustomElement::type_id(&*b.0);
        }
        true
    }
    /// `true` when the reconciler may diff `self` against `other` in place
    /// rather than unmounting and remounting.
    pub fn can_update(&self, other: &Self) -> bool {
        if !self.kind_matches(other) {
            return false;
        }
        match (self, other) {
            (Self::Component(a), Self::Component(b)) => {
                a.obj.component_type_id() == b.obj.component_type_id()
            }
            (Self::Custom(a), Self::Custom(b)) => {
                CustomElement::type_id(&*a.0) == CustomElement::type_id(&*b.0)
            }
            _ => true,
        }
    }
}

/// `true` when the reconciler can skip diffing entirely — old/new are
/// equal and there are no theme bindings that need re-resolving.
pub fn can_skip_update(old: &Element, new: &Element) -> bool {
    if old != new {
        return false;
    }
    match new.modifiers() {
        Some(m) => m.theme_bindings.as_ref().is_none_or(|map| map.is_empty()),
        None => true,
    }
}

macro_rules! simple_setter {
    ($name:ident, $field:ident, $ty:ty) => {
        fn $name(mut self, v: $ty) -> Self {
            if let Some(m) = self.modifiers_mut() {
                m.$field = Some(v);
            }
            self
        }
    };
    ($name:ident, $field:ident, $ty:ty, into) => {
        fn $name(mut self, v: impl Into<$ty>) -> Self {
            if let Some(m) = self.modifiers_mut() {
                m.$field = Some(v.into());
            }
            self
        }
    };
}

/// Builder-style modifier methods (`.margin(..)`, `.background(..)`,
/// animations, theme bindings, …) implemented for every widget builder
/// and for [`Element`] itself.
pub trait ElementExt: Sized {
    fn modifiers_mut(&mut self) -> Option<&mut Modifiers>;

    simple_setter!(margin, margin, Thickness, into);
    simple_setter!(padding, padding, Thickness, into);
    simple_setter!(width, width, f64);
    simple_setter!(height, height, f64);
    simple_setter!(min_width, min_width, f64);
    simple_setter!(max_width, max_width, f64);
    simple_setter!(min_height, min_height, f64);
    simple_setter!(max_height, max_height, f64);

    fn background(mut self, v: impl Into<BrushBinding>) -> Self {
        if let Some(m) = self.modifiers_mut() {
            apply_brush_binding(m, Prop::Background, v.into(), true);
        }
        self
    }

    fn foreground(mut self, v: impl Into<BrushBinding>) -> Self {
        if let Some(m) = self.modifiers_mut() {
            apply_brush_binding(m, Prop::Foreground, v.into(), false);
        }
        self
    }

    simple_setter!(opacity, opacity, f64);
    simple_setter!(font_family, font_family, String, into);
    simple_setter!(font_size, font_size, f64);
    simple_setter!(
        horizontal_alignment,
        horizontal_alignment,
        HorizontalAlignment
    );
    simple_setter!(vertical_alignment, vertical_alignment, VerticalAlignment);

    fn with_key(self, key: impl Into<String>) -> Self;

    fn with_opacity_transition(mut self, duration: Duration) -> Self {
        if let Some(m) = self.modifiers_mut() {
            with_implicit_transition(m, |it| {
                it.opacity = Some(ScalarTransition::new(duration));
            });
        }
        self
    }

    fn with_scale_transition(mut self, duration: Duration) -> Self {
        if let Some(m) = self.modifiers_mut() {
            with_implicit_transition(m, |it| {
                it.scale = Some(Vector3Transition::new(duration));
            });
        }
        self
    }

    fn with_translation_transition(mut self, duration: Duration) -> Self {
        if let Some(m) = self.modifiers_mut() {
            with_implicit_transition(m, |it| {
                it.translation = Some(Vector3Transition::new(duration));
            });
        }
        self
    }

    fn with_layout_animation(mut self, config: LayoutAnimationConfig) -> Self {
        if let Some(m) = self.modifiers_mut() {
            ensure_animations(m).layout_animation = Some(config);
        }
        self
    }

    fn animate(mut self, config: AnimationConfig) -> Self {
        if let Some(m) = self.modifiers_mut() {
            ensure_animations(m).property_animation = Some(config);
        }
        self
    }

    fn transition(mut self, enter: Option<AnimationConfig>, exit: Option<AnimationConfig>) -> Self {
        if let Some(m) = self.modifiers_mut() {
            let a = ensure_animations(m);
            a.enter_transition = enter;
            a.exit_transition = exit;
        }
        self
    }

    fn grid_row(mut self, row: i32) -> Self {
        if let Some(m) = self.modifiers_mut() {
            let mut p = m.grid.unwrap_or_default();
            p.row = row;
            m.grid = Some(p);
        }
        self
    }

    fn grid_column(mut self, column: i32) -> Self {
        if let Some(m) = self.modifiers_mut() {
            let mut p = m.grid.unwrap_or_default();
            p.column = column;
            m.grid = Some(p);
        }
        self
    }

    fn grid_row_span(mut self, rs: i32) -> Self {
        if let Some(m) = self.modifiers_mut() {
            let mut p = m.grid.unwrap_or_default();
            p.row_span = rs;
            m.grid = Some(p);
        }
        self
    }

    fn grid_column_span(mut self, cs: i32) -> Self {
        if let Some(m) = self.modifiers_mut() {
            let mut p = m.grid.unwrap_or_default();
            p.column_span = cs;
            m.grid = Some(p);
        }
        self
    }

    fn canvas_left(mut self, x: f64) -> Self {
        if let Some(m) = self.modifiers_mut() {
            let bag = m.attached.get_or_insert_with(AttachedProps::default);
            let mut p = bag.get::<CanvasPosition>().copied().unwrap_or_default();
            p.left = x;
            bag.set(p);
        }
        self
    }

    fn canvas_top(mut self, y: f64) -> Self {
        if let Some(m) = self.modifiers_mut() {
            let bag = m.attached.get_or_insert_with(AttachedProps::default);
            let mut p = bag.get::<CanvasPosition>().copied().unwrap_or_default();
            p.top = y;
            bag.set(p);
        }
        self
    }

    fn canvas_z_index(mut self, z: i32) -> Self {
        if let Some(m) = self.modifiers_mut() {
            let bag = m.attached.get_or_insert_with(AttachedProps::default);
            let mut p = bag.get::<CanvasPosition>().copied().unwrap_or_default();
            p.z_index = z;
            bag.set(p);
        }
        self
    }

    fn provide<T>(self, context: &Context<T>, value: T) -> Element
    where
        T: Clone + PartialEq + 'static,
        Self: Into<Element>,
    {
        let mut el: Element = self.into();
        let provision = ContextProvision::new(context.id, value);
        if let Element::Provider(ref mut existing) = el {
            existing.provisions.push(provision);
            el
        } else {
            Element::Provider(ProviderElement {
                key: None,
                provisions: vec![provision],
                child: Box::new(el),
            })
        }
    }

    /// Plain-text tooltip applied via WinUI `ToolTipService`. For rich
    /// content or custom placement, use [`tooltip_with`](Self::tooltip_with).
    fn tooltip(mut self, text: impl Into<String>) -> Self {
        if let Some(m) = self.modifiers_mut() {
            m.tooltip = Some(Box::new(Tooltip::text(text)));
        }
        self
    }

    /// Tooltip setter accepting anything convertible into
    /// [`Tooltip`].
    fn tooltip_with(mut self, t: impl Into<Tooltip>) -> Self {
        if let Some(m) = self.modifiers_mut() {
            m.tooltip = Some(Box::new(t.into()));
        }
        self
    }

    /// Register a `Tapped` (left-tap) handler.
    fn on_tapped(mut self, f: impl IntoUnitCallback) -> Self {
        if let Some(m) = self.modifiers_mut() {
            ensure_pointer_handlers(m).on_tapped = Some(f.into_unit_callback());
        }
        self
    }

    /// Register a `RightTapped` handler (right-click / barrel button /
    /// touch-and-hold).
    fn on_right_tapped(mut self, f: impl IntoUnitCallback) -> Self {
        if let Some(m) = self.modifiers_mut() {
            ensure_pointer_handlers(m).on_right_tapped = Some(f.into_unit_callback());
        }
        self
    }

    /// Register a `PointerPressed` handler; the callback receives the
    /// current button state.
    fn on_pointer_pressed(mut self, f: impl IntoCallback<PointerEventInfo>) -> Self {
        if let Some(m) = self.modifiers_mut() {
            ensure_pointer_handlers(m).on_pointer_pressed = Some(f.into_callback());
        }
        self
    }

    /// Register a `PointerReleased` handler; the callback receives the
    /// button state at release.
    fn on_pointer_released(mut self, f: impl IntoCallback<PointerEventInfo>) -> Self {
        if let Some(m) = self.modifiers_mut() {
            ensure_pointer_handlers(m).on_pointer_released = Some(f.into_callback());
        }
        self
    }

    /// Register a `PointerMoved` handler; the callback receives the current
    /// pointer position and button state. Fires continuously while the pointer
    /// is over the element — use it for drag and hover tracking.
    fn on_pointer_moved(mut self, f: impl IntoCallback<PointerEventInfo>) -> Self {
        if let Some(m) = self.modifiers_mut() {
            ensure_pointer_handlers(m).on_pointer_moved = Some(f.into_callback());
        }
        self
    }

    /// Register a `PointerEntered` handler (pointer enters hit-test bounds);
    /// the callback receives the entry position and button state.
    fn on_pointer_entered(mut self, f: impl IntoCallback<PointerEventInfo>) -> Self {
        if let Some(m) = self.modifiers_mut() {
            ensure_pointer_handlers(m).on_pointer_entered = Some(f.into_callback());
        }
        self
    }

    /// Register a `PointerExited` handler (pointer leaves hit-test bounds).
    fn on_pointer_exited(mut self, f: impl IntoUnitCallback) -> Self {
        if let Some(m) = self.modifiers_mut() {
            ensure_pointer_handlers(m).on_pointer_exited = Some(f.into_unit_callback());
        }
        self
    }

    // ── Accessibility modifiers ──────────────────────────────────────────

    fn automation_name(mut self, name: impl Into<String>) -> Self {
        if let Some(m) = self.modifiers_mut() {
            ensure_accessibility(m).automation_name = Some(name.into());
        }
        self
    }

    fn automation_id(mut self, id: impl Into<String>) -> Self {
        if let Some(m) = self.modifiers_mut() {
            ensure_accessibility(m).automation_id = Some(id.into());
        }
        self
    }

    fn help_text(mut self, text: impl Into<String>) -> Self {
        if let Some(m) = self.modifiers_mut() {
            ensure_accessibility(m).help_text = Some(text.into());
        }
        self
    }

    fn heading_level(mut self, level: AutomationHeadingLevel) -> Self {
        if let Some(m) = self.modifiers_mut() {
            ensure_accessibility(m).heading_level = Some(level);
        }
        self
    }

    fn accessibility_live_setting(mut self, ls: AutomationLiveSetting) -> Self {
        if let Some(m) = self.modifiers_mut() {
            ensure_accessibility(m).live_setting = Some(ls);
        }
        self
    }

    fn keyboard_accelerator(mut self, accel: KeyboardAccelerator) -> Self {
        if let Some(m) = self.modifiers_mut() {
            m.keyboard_accelerators.push(accel);
        }
        self
    }

    // ── RelativePanel attached property helpers ──────────────────────────

    fn relative_align_left(mut self) -> Self {
        if let Some(m) = self.modifiers_mut() {
            let bag = m.attached.get_or_insert_with(AttachedProps::default);
            let mut p = bag
                .get::<RelativePanelAlignment>()
                .copied()
                .unwrap_or_default();
            p.align_left_with_panel = true;
            bag.set(p);
        }
        self
    }

    fn relative_align_right(mut self) -> Self {
        if let Some(m) = self.modifiers_mut() {
            let bag = m.attached.get_or_insert_with(AttachedProps::default);
            let mut p = bag
                .get::<RelativePanelAlignment>()
                .copied()
                .unwrap_or_default();
            p.align_right_with_panel = true;
            bag.set(p);
        }
        self
    }

    fn relative_align_top(mut self) -> Self {
        if let Some(m) = self.modifiers_mut() {
            let bag = m.attached.get_or_insert_with(AttachedProps::default);
            let mut p = bag
                .get::<RelativePanelAlignment>()
                .copied()
                .unwrap_or_default();
            p.align_top_with_panel = true;
            bag.set(p);
        }
        self
    }

    fn relative_align_bottom(mut self) -> Self {
        if let Some(m) = self.modifiers_mut() {
            let bag = m.attached.get_or_insert_with(AttachedProps::default);
            let mut p = bag
                .get::<RelativePanelAlignment>()
                .copied()
                .unwrap_or_default();
            p.align_bottom_with_panel = true;
            bag.set(p);
        }
        self
    }

    fn relative_align_h_center(mut self) -> Self {
        if let Some(m) = self.modifiers_mut() {
            let bag = m.attached.get_or_insert_with(AttachedProps::default);
            let mut p = bag
                .get::<RelativePanelAlignment>()
                .copied()
                .unwrap_or_default();
            p.align_h_center_with_panel = true;
            bag.set(p);
        }
        self
    }

    fn relative_align_v_center(mut self) -> Self {
        if let Some(m) = self.modifiers_mut() {
            let bag = m.attached.get_or_insert_with(AttachedProps::default);
            let mut p = bag
                .get::<RelativePanelAlignment>()
                .copied()
                .unwrap_or_default();
            p.align_v_center_with_panel = true;
            bag.set(p);
        }
        self
    }

    fn resources(
        mut self,
        entries: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
    ) -> Self {
        if let Some(m) = self.modifiers_mut() {
            m.resources = entries
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect();
        }
        self
    }

    fn allow_drop(mut self, v: bool) -> Self {
        if let Some(m) = self.modifiers_mut() {
            m.allow_drop = Some(v);
        }
        self
    }

    fn drag_enter<F: Fn(&mut DragContext) -> DragOperation + Send + Sync + 'static>(
        mut self,
        f: F,
    ) -> Self {
        if let Some(m) = self.modifiers_mut() {
            ensure_drag_handlers(m).on_drag_enter = Some(DragAsyncCallback::new(f));
        }
        self
    }

    fn drag_leave<F: Fn(&DragContext) + 'static>(mut self, f: F) -> Self {
        if let Some(m) = self.modifiers_mut() {
            ensure_drag_handlers(m).on_drag_leave = Some(DragNotifyCallback::new(f));
        }
        self
    }

    fn drag_over<F: Fn(&mut DragContext) -> DragOperation + 'static>(mut self, f: F) -> Self {
        if let Some(m) = self.modifiers_mut() {
            ensure_drag_handlers(m).on_drag_over = Some(DragCallback::new(f));
        }
        self
    }

    fn drag_drop<F: Fn(&mut DragContext) -> DragOperation + Send + Sync + 'static>(
        mut self,
        f: F,
    ) -> Self {
        if let Some(m) = self.modifiers_mut() {
            ensure_drag_handlers(m).on_drag_drop = Some(DragAsyncCallback::new(f));
        }
        self
    }
}

fn ensure_drag_handlers(m: &mut Modifiers) -> &mut DragHandlers {
    if m.allow_drop.is_none() {
        m.allow_drop = Some(true);
    }

    m.drag_handlers
        .get_or_insert_with(|| Box::new(DragHandlers::default()))
}

fn ensure_pointer_handlers(m: &mut Modifiers) -> &mut PointerHandlers {
    m.pointer_handlers
        .get_or_insert_with(|| Box::new(PointerHandlers::default()))
}

fn ensure_accessibility(m: &mut Modifiers) -> &mut AccessibilityModifiers {
    m.accessibility
        .get_or_insert_with(|| Box::new(AccessibilityModifiers::default()))
}

fn apply_brush_binding(m: &mut Modifiers, prop: Prop, binding: BrushBinding, is_background: bool) {
    if let Some(map) = m.theme_bindings.as_deref_mut() {
        map.remove(&prop);
    }
    match binding {
        BrushBinding::Direct(b) => {
            if is_background {
                m.background = Some(b);
            } else {
                m.foreground = Some(b);
            }
        }
        BrushBinding::Theme(t) => {
            if is_background {
                m.background = None;
            } else {
                m.foreground = None;
            }
            m.theme_bindings
                .get_or_insert_with(|| Box::new(rustc_hash::FxHashMap::default()))
                .insert(prop, t);
        }
    }

    if m.theme_bindings.as_deref().is_some_and(|m| m.is_empty()) {
        m.theme_bindings = None;
    }
}

/// Applies a [`BrushBinding`] to a widget-owned brush slot (e.g. a border
/// brush stored on the widget struct rather than in [`Modifiers`]). A direct
/// color is stored in `slot` and any prior theme binding for `prop` is cleared;
/// a theme reference clears `slot` and records the binding. This keeps
/// "last call wins" holding regardless of the direct/theme call order.
pub(crate) fn apply_widget_brush_binding(
    slot: &mut Option<BrushBinding>,
    m: &mut Modifiers,
    prop: Prop,
    binding: BrushBinding,
) {
    if let Some(map) = m.theme_bindings.as_deref_mut() {
        map.remove(&prop);
    }
    match binding {
        BrushBinding::Direct(b) => *slot = Some(BrushBinding::Direct(b)),
        BrushBinding::Theme(t) => {
            *slot = None;
            m.theme_bindings
                .get_or_insert_with(|| Box::new(rustc_hash::FxHashMap::default()))
                .insert(prop, t);
        }
    }

    if m.theme_bindings.as_deref().is_some_and(|m| m.is_empty()) {
        m.theme_bindings = None;
    }
}

fn ensure_animations(m: &mut Modifiers) -> &mut AnimationModifiers {
    m.animations
        .get_or_insert_with(|| Box::new(AnimationModifiers::default()))
}

fn with_implicit_transition(m: &mut Modifiers, f: impl FnOnce(&mut ImplicitTransitions)) {
    let anim = ensure_animations(m);
    let it = anim
        .implicit_transitions
        .get_or_insert_with(Default::default);
    f(it);
}

macro_rules! impl_element_ext {
    ($($ty:ident),* $(,)?) => {
        $(
            impl ElementExt for widgets::$ty {
                fn modifiers_mut(&mut self) -> Option<&mut Modifiers> {
                    Some(&mut self.modifiers)
                }
                fn with_key(mut self, key: impl Into<String>) -> Self {
                    self.key = Some(key.into());
                    self
                }
            }
        )*
    };
}
impl_element_ext!(
    TextBlock,
    Button,
    StackPanel,
    Border,
    CheckBox,
    TextBox,
    Grid,
    ScrollViewer,
    ToggleSwitch,
    Slider,
    RadioButton,
    NumberBox,
    ProgressBar,
    ProgressRing,
    Expander,
    HyperlinkButton,
    InfoBar,
    InfoBadge,
    PersonPicture,
    Shape,
    Image,
    TabView,
    NavigationView,
    TitleBar,
    Pivot,
    BreadcrumbBar,
    PasswordBox,
    RadioButtons,
    ComboBox,
    Canvas,
    ContentDialog,
    Viewbox,
    ScrollView,
    TreeView,
    CommandBar,
    TeachingTip,
    SelectorBar,
    RichEditBox,
    RelativePanel,
    ToggleButton,
    SwapChainPanel,
    CompositionHost,
    WebView2,
);

impl ElementExt for RichTextBlock {
    fn modifiers_mut(&mut self) -> Option<&mut Modifiers> {
        Some(&mut self.modifiers)
    }
    fn with_key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }
}

impl ElementExt for Element {
    fn modifiers_mut(&mut self) -> Option<&mut Modifiers> {
        Self::modifiers_mut(self)
    }

    fn with_key(mut self, key: impl Into<String>) -> Self {
        let key = key.into();
        match &mut self {
            Self::TextBlock(t) => t.key = Some(key),
            Self::Button(b) => b.key = Some(key),
            Self::StackPanel(s) => s.key = Some(key),
            Self::Border(b) => b.key = Some(key),
            Self::CheckBox(c) => c.key = Some(key),
            Self::TextBox(t) => t.key = Some(key),
            Self::Grid(g) => g.key = Some(key),
            Self::ScrollViewer(s) => s.key = Some(key),
            Self::ToggleSwitch(v) => v.key = Some(key),
            Self::Slider(v) => v.key = Some(key),
            Self::RadioButton(v) => v.key = Some(key),
            Self::NumberBox(v) => v.key = Some(key),
            Self::ProgressBar(v) => v.key = Some(key),
            Self::ProgressRing(v) => v.key = Some(key),
            Self::Expander(v) => v.key = Some(key),
            Self::HyperlinkButton(v) => v.key = Some(key),
            Self::InfoBar(v) => v.key = Some(key),
            Self::InfoBadge(v) => v.key = Some(key),
            Self::PersonPicture(v) => v.key = Some(key),
            Self::Shape(v) => v.key = Some(key),
            Self::Image(v) => v.key = Some(key),
            Self::TabView(v) => v.key = Some(key),
            Self::NavigationView(v) => v.key = Some(key),
            Self::TitleBar(v) => v.key = Some(key),
            Self::Pivot(v) => v.key = Some(key),
            Self::BreadcrumbBar(v) => v.key = Some(key),
            Self::PasswordBox(v) => v.key = Some(key),
            Self::RadioButtons(v) => v.key = Some(key),
            Self::ComboBox(v) => v.key = Some(key),
            Self::Canvas(v) => v.key = Some(key),
            Self::ContentDialog(v) => v.key = Some(key),
            Self::RichTextBlock(v) => v.key = Some(key),
            Self::Viewbox(v) => v.key = Some(key),
            Self::RepeatButton(v) => v.key = Some(key),
            Self::RatingControl(v) => v.key = Some(key),
            Self::ColorPicker(v) => v.key = Some(key),
            Self::DatePicker(v) => v.key = Some(key),
            Self::TimePicker(v) => v.key = Some(key),
            Self::CalendarDatePicker(v) => v.key = Some(key),
            Self::CalendarView(v) => v.key = Some(key),
            Self::ListBox(v) => v.key = Some(key),
            Self::DropDownButton(v) => v.key = Some(key),
            Self::SplitButton(v) => v.key = Some(key),
            Self::AutoSuggestBox(v) => v.key = Some(key),
            Self::SplitView(v) => v.key = Some(key),
            Self::MenuBar(v) => v.key = Some(key),
            Self::ScrollView(v) => v.key = Some(key),
            Self::TreeView(v) => v.key = Some(key),
            Self::CommandBar(v) => v.key = Some(key),
            Self::TeachingTip(v) => v.key = Some(key),
            Self::SelectorBar(v) => v.key = Some(key),
            Self::RichEditBox(v) => v.key = Some(key),
            Self::RelativePanel(v) => v.key = Some(key),
            Self::ToggleButton(v) => v.key = Some(key),
            Self::Component(c) => c.key = Some(key),
            Self::ErrorBoundary(eb) => eb.key = Some(key),
            Self::Provider(pe) => pe.key = Some(key),
            Self::TemplatedList(tl) => tl.key = Some(key),
            Self::Group(g) => g.key = Some(key),
            _ => {}
        }
        self
    }
}
