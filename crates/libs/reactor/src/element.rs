use super::*;
use std::any::Any;
use std::time::Duration;

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

/// One item accepted by a multi-child builder.
#[doc(hidden)]
pub trait IntoChild {
    fn append_to(self, children: &mut Vec<Element>);
}

impl<T: Into<Element>> IntoChild for T {
    fn append_to(self, children: &mut Vec<Element>) {
        let element = self.into();
        if !matches!(element, Element::Empty) {
            children.push(element);
        }
    }
}

/// A child-only fragment flattened by multi-child builders.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Fragment {
    children: Vec<Element>,
}

impl IntoChild for Fragment {
    fn append_to(self, children: &mut Vec<Element>) {
        children.extend(self.children);
    }
}

/// Converts tuples, arrays, vectors, and fragments into a flat child list.
pub trait IntoChildren {
    fn into_children(self) -> Vec<Element>;
}

impl IntoChildren for Vec<Element> {
    fn into_children(mut self) -> Vec<Element> {
        self.retain(|element| !matches!(element, Element::Empty));
        self
    }
}

impl IntoChildren for Fragment {
    fn into_children(self) -> Vec<Element> {
        self.children
    }
}

impl IntoChildren for () {
    fn into_children(self) -> Vec<Element> {
        Vec::new()
    }
}

impl<T: IntoChild, const N: usize> IntoChildren for [T; N] {
    fn into_children(self) -> Vec<Element> {
        let mut children = Vec::with_capacity(N);
        for child in self {
            child.append_to(&mut children);
        }
        children
    }
}

macro_rules! impl_into_children_for_tuple {
    ($($idx:tt : $T:ident),+) => {
        impl<$($T: IntoChild),+> IntoChildren for ($($T,)+) {
            fn into_children(self) -> Vec<Element> {
                let capacity = 0 $(+ {
                    let _ = stringify!($T);
                    1
                })+;
                let mut children = Vec::with_capacity(capacity);
                $(self.$idx.append_to(&mut children);)+
                children
            }
        }
    };
}

impl_into_children_for_tuple!(0: A);
impl_into_children_for_tuple!(0: A, 1: B);
impl_into_children_for_tuple!(0: A, 1: B, 2: C);
impl_into_children_for_tuple!(0: A, 1: B, 2: C, 3: D);
impl_into_children_for_tuple!(0: A, 1: B, 2: C, 3: D, 4: E);
impl_into_children_for_tuple!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F);
impl_into_children_for_tuple!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G);
impl_into_children_for_tuple!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H);
impl_into_children_for_tuple!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I2);
impl_into_children_for_tuple!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I2, 9: J);
impl_into_children_for_tuple!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I2, 9: J, 10: K);
impl_into_children_for_tuple!(
    0: A,
    1: B,
    2: C,
    3: D,
    4: E,
    5: F,
    6: G,
    7: H,
    8: I2,
    9: J,
    10: K,
    11: L
);

/// Creates a child-only fragment for use inside a multi-child builder.
///
/// ```compile_fail
/// use windows_reactor::{Element, fragment, text_block};
///
/// let _: Element = fragment((text_block("a"), text_block("b"))).into();
/// ```
///
/// ```compile_fail
/// use windows_reactor::{border, fragment, text_block};
///
/// let _ = border(fragment((text_block("a"), text_block("b"))));
/// ```
pub fn fragment(children: impl IntoChildren) -> Fragment {
    Fragment {
        children: children.into_children(),
    }
}

/// Built-in widget variants live here so enum arms and dispatch stay aligned.
macro_rules! impl_styling_capabilities {
    ($variant:ident, Control) => {
        impl_styling_capabilities!($variant, Padding);
        impl_styling_capabilities!($variant, Background);
        impl_styling_capabilities!($variant, TextStyle);
    };
    ($variant:ident, PaddedPanel) => {
        impl_styling_capabilities!($variant, Padding);
        impl_styling_capabilities!($variant, Background);
    };
    ($variant:ident, Panel) => {
        impl_styling_capabilities!($variant, Background);
    };
    ($variant:ident, Text) => {
        impl_styling_capabilities!($variant, Padding);
        impl_styling_capabilities!($variant, TextStyle);
    };
    ($variant:ident, Border) => {
        impl_styling_capabilities!($variant, Padding);
        impl_styling_capabilities!($variant, Background);
    };
    ($variant:ident, Visual) => {};
    ($variant:ident, Padding) => {
        impl capability::Padding for $variant {}
    };
    ($variant:ident, Background) => {
        impl capability::Background for $variant {}
    };
    ($variant:ident, TextStyle) => {
        impl capability::TextStyle for $variant {}
    };
}

macro_rules! define_element {
    ( $( $variant:ident : $styling:ident ),* $(,)? ) => {
        /// Element tree node the reconciler can mount.
        #[derive(Clone, Debug, PartialEq, Default)]
        pub enum Element {
            $( $variant($variant), )*
            Component(ComponentElement),
            ErrorBoundary(ErrorBoundaryElement),
            Provider(ProviderElement),
            TemplatedList(TemplatedListElement),
            #[default]
            Empty,
        }

        $(
            impl From<$variant> for Element {
                fn from(v: $variant) -> Self {
                    Element::$variant(v)
                }
            }

            impl KeyExt for $variant {
                fn with_key(mut self, key: impl Into<String>) -> Self {
                    self.key = Some(key.into());
                    self
                }
            }

            impl capability::Native for $variant {
                fn native_modifiers_mut(&mut self) -> &mut Modifiers {
                    &mut self.modifiers
                }
            }

            impl capability::Resources for $variant {}

            impl capability::Layout for $variant {}

            impl capability::Input for $variant {}

            impl capability::Accessibility for $variant {}

            impl capability::Tooltip for $variant {}

            impl capability::GridChild for $variant {}

            impl capability::CanvasChild for $variant {}

            impl capability::RelativePanelChild for $variant {}

            impl capability::Visual for $variant {}

            impl_styling_capabilities!($variant, $styling);
        )*

        impl Element {
            pub fn as_widget(&self) -> Option<&dyn Widget> {
                Some(match self {
                    $( Element::$variant(v) => v, )*
                    Element::Component(_)
                    | Element::ErrorBoundary(_)
                    | Element::Provider(_)
                    | Element::TemplatedList(_)
                    | Element::Empty => return None,
                })
            }
            pub fn kind_name(&self) -> &'static str {
                match self {
                    $( Element::$variant(_) => stringify!($variant), )*
                    Element::Component(_) => "Component",
                    Element::ErrorBoundary(_) => "ErrorBoundary",
                    Element::Provider(_) => "Provider",
                    Element::TemplatedList(_) => "TemplatedList",
                    Element::Empty => "Empty",
                }
            }
        }

        impl KeyExt for Element {
            fn with_key(mut self, key: impl Into<String>) -> Self {
                let key = key.into();
                match &mut self {
                    $( Element::$variant(v) => v.key = Some(key), )*
                    Element::Component(c) => c.key = Some(key),
                    Element::ErrorBoundary(eb) => eb.key = Some(key),
                    Element::Provider(pe) => pe.key = Some(key),
                    Element::TemplatedList(tl) => tl.key = Some(key),
                    Element::Empty => {}
                }
                self
            }
        }
    };
}

define_element! {
    TextBlock: Text,
    Button: Control,
    StackPanel: PaddedPanel,
    Border: Border,
    CheckBox: Control,
    TextBox: Control,
    Grid: PaddedPanel,
    ScrollViewer: Control,
    ToggleSwitch: Control,
    Slider: Control,
    RadioButton: Control,
    NumberBox: Control,
    ProgressBar: Control,
    ProgressRing: Control,
    Expander: Control,
    HyperlinkButton: Control,
    InfoBar: Control,
    InfoBadge: Control,
    PersonPicture: Control,
    Shape: Visual,
    Image: Visual,
    TabView: Control,
    NavigationView: Control,
    TitleBar: Control,
    Pivot: Control,
    BreadcrumbBar: Control,
    PasswordBox: Control,
    RadioButtons: Control,
    ComboBox: Control,
    Canvas: Panel,
    RichTextBlock: Text,
    ContentDialog: Control,
    Viewbox: Visual,
    RepeatButton: Control,
    RatingControl: Control,
    ColorPicker: Control,
    DatePicker: Control,
    TimePicker: Control,
    CalendarDatePicker: Control,
    CalendarView: Control,
    ListBox: Control,
    DropDownButton: Control,
    SplitButton: Control,
    AutoSuggestBox: Control,
    SplitView: Control,
    MenuBar: Control,
    ScrollView: Control,
    TreeView: Control,
    CommandBar: Control,
    TeachingTip: Control,
    SelectorBar: Control,
    RichEditBox: Control,
    RelativePanel: Panel,
    ToggleButton: Control,
    SwapChainPanel: PaddedPanel,
    CompositionHost: Visual,
    WebView2: Visual,
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
            Self::Component(_) | Self::ErrorBoundary(_) | Self::Provider(_) | Self::Empty => None,
            _ => unreachable!("covered by as_widget"),
        }
    }
    pub fn attached(&self) -> Option<&AttachedProps> {
        self.modifiers().and_then(|m| m.attached.as_ref())
    }
    pub fn accessibility(&self) -> Option<&AccessibilityModifiers> {
        self.modifiers().and_then(|m| m.accessibility.as_deref())
    }
    /// `true` when both elements are the same variant and shape, so an update can be considered.
    pub fn kind_matches(&self, other: &Self) -> bool {
        if std::mem::discriminant(self) != std::mem::discriminant(other) {
            return false;
        }
        if let (Self::Shape(a), Self::Shape(b)) = (self, other) {
            return a.kind == b.kind;
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
            _ => true,
        }
    }
}

/// `true` when the reconciler can skip diffing entirely - old/new are
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

/// Stable reconciliation identity for concrete widgets and erased elements.
///
/// Keys should be unique among siblings. Duplicate keys are tolerated, but each old child can be
/// reused only once.
pub trait KeyExt: Sized {
    fn with_key(self, key: impl Into<String>) -> Self;
}

/// Context provision for anything convertible into an [`Element`].
pub trait ProvideExt: Into<Element> + Sized {
    fn provide<T>(self, context: &Context<T>, value: T) -> Element
    where
        T: Clone + PartialEq + 'static,
    {
        let mut el = self.into();
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
}

impl<T: Into<Element>> ProvideExt for T {}

/// Attaches a typed reference to a native widget.
///
/// The associated handle type prevents references from being attached to incompatible widgets.
///
/// ```compile_fail
/// use windows_reactor::*;
///
/// let image = ElementRef::<ImageHandle>::new();
/// let _ = text_box("").element_ref(&image);
/// ```
pub trait ElementRefExt: capability::Native + Sized {
    type Handle: ElementHandle;

    fn element_ref(mut self, reference: &ElementRef<Self::Handle>) -> Self {
        let modifiers = self.native_modifiers_mut();
        modifiers
            .attached
            .get_or_insert_with(AttachedProps::default)
            .set(reference.binding());
        self
    }
}

pub(crate) mod capability {
    use super::Modifiers;

    pub trait Native {
        fn native_modifiers_mut(&mut self) -> &mut Modifiers;
    }

    pub trait Accessibility: Native {
        fn accessibility_modifiers_mut(&mut self) -> &mut Modifiers {
            self.native_modifiers_mut()
        }
    }

    pub trait Background: Native {
        fn background_modifiers_mut(&mut self) -> &mut Modifiers {
            self.native_modifiers_mut()
        }
    }

    pub trait Input: Native {
        fn input_modifiers_mut(&mut self) -> &mut Modifiers {
            self.native_modifiers_mut()
        }
    }

    pub trait GridChild: Native {
        fn grid_child_modifiers_mut(&mut self) -> &mut Modifiers {
            self.native_modifiers_mut()
        }
    }

    pub trait CanvasChild: Native {
        fn canvas_child_modifiers_mut(&mut self) -> &mut Modifiers {
            self.native_modifiers_mut()
        }
    }

    pub trait RelativePanelChild: Native {
        fn relative_panel_child_modifiers_mut(&mut self) -> &mut Modifiers {
            self.native_modifiers_mut()
        }
    }

    pub trait Layout: Native {
        fn layout_modifiers_mut(&mut self) -> &mut Modifiers {
            self.native_modifiers_mut()
        }
    }

    pub trait Padding: Native {
        fn padding_modifiers_mut(&mut self) -> &mut Modifiers {
            self.native_modifiers_mut()
        }
    }

    pub trait Resources: Native {
        fn resource_modifiers_mut(&mut self) -> &mut Modifiers {
            self.native_modifiers_mut()
        }
    }

    pub trait TextStyle: Native {
        fn text_style_modifiers_mut(&mut self) -> &mut Modifiers {
            self.native_modifiers_mut()
        }
    }

    pub trait Tooltip: Native {
        fn tooltip_modifiers_mut(&mut self) -> &mut Modifiers {
            self.native_modifiers_mut()
        }
    }

    pub trait Visual: Native {
        fn visual_modifiers_mut(&mut self) -> &mut Modifiers {
            self.native_modifiers_mut()
        }
    }
}

/// Background styling for controls, panels, and borders.
///
/// ```compile_fail
/// use windows_reactor::{BackgroundExt, text_block};
///
/// let _ = text_block("Label").background("#202020");
/// ```
///
/// ```compile_fail
/// use windows_reactor::{BackgroundExt, Element, button};
///
/// let element: Element = button("Save").into();
/// let _ = element.background("#202020");
/// ```
pub trait BackgroundExt: capability::Background + Sized {
    fn background(mut self, value: impl Into<BrushBinding>) -> Self {
        apply_brush_binding(
            capability::Background::background_modifiers_mut(&mut self),
            Prop::Background,
            value.into(),
            true,
        );
        self
    }
}

impl<T: capability::Background> BackgroundExt for T {}

/// Padding for controls, text blocks, borders, and panels that expose padding.
///
/// ```compile_fail
/// use windows_reactor::{Image, PaddingExt};
///
/// let _ = Image::new("asset.png").padding(8.0);
/// ```
pub trait PaddingExt: capability::Padding + Sized {
    fn padding(mut self, value: impl Into<Thickness>) -> Self {
        capability::Padding::padding_modifiers_mut(&mut self).padding = Some(value.into());
        self
    }
}

impl<T: capability::Padding> PaddingExt for T {}

/// Foreground and font styling for controls and text blocks.
///
/// ```compile_fail
/// use windows_reactor::{TextStyleExt, border, text_block};
///
/// let _ = border(text_block("Panel")).font_size(16.0);
/// ```
///
/// ```compile_fail
/// use windows_reactor::{Shape, TextStyleExt};
///
/// let _ = Shape::rectangle().foreground("#ffffff");
/// ```
pub trait TextStyleExt: capability::TextStyle + Sized {
    fn foreground(mut self, value: impl Into<BrushBinding>) -> Self {
        apply_brush_binding(
            capability::TextStyle::text_style_modifiers_mut(&mut self),
            Prop::Foreground,
            value.into(),
            false,
        );
        self
    }

    fn font_family(mut self, value: impl Into<String>) -> Self {
        capability::TextStyle::text_style_modifiers_mut(&mut self).font_family = Some(value.into());
        self
    }

    fn font_size(mut self, value: f64) -> Self {
        capability::TextStyle::text_style_modifiers_mut(&mut self).font_size = Some(value);
        self
    }
}

impl<T: capability::TextStyle> TextStyleExt for T {}

/// UI Automation modifiers for concrete native widgets.
///
/// ```compile_fail
/// use windows_reactor::{AccessibilityExt, Element, button};
///
/// let element: Element = button("Save").into();
/// let _ = element.automation_name("Save document");
/// ```
pub trait AccessibilityExt: capability::Accessibility + Sized {
    fn automation_name(mut self, name: impl Into<String>) -> Self {
        accessibility_modifiers_mut(&mut self).automation_name = Some(name.into());
        self
    }

    fn automation_id(mut self, id: impl Into<String>) -> Self {
        accessibility_modifiers_mut(&mut self).automation_id = Some(id.into());
        self
    }

    fn help_text(mut self, text: impl Into<String>) -> Self {
        accessibility_modifiers_mut(&mut self).help_text = Some(text.into());
        self
    }

    fn heading_level(mut self, level: AutomationHeadingLevel) -> Self {
        accessibility_modifiers_mut(&mut self).heading_level = Some(level);
        self
    }

    fn accessibility_live_setting(mut self, setting: AutomationLiveSetting) -> Self {
        accessibility_modifiers_mut(&mut self).live_setting = Some(setting);
        self
    }
}

impl<T: capability::Accessibility> AccessibilityExt for T {}

/// Grid row, column, and span placement for concrete native children.
///
/// ```compile_fail
/// use windows_reactor::{Element, GridChildExt, text_block};
///
/// let element: Element = text_block("Cell").into();
/// let _ = element.grid_row(1);
/// ```
pub trait GridChildExt: capability::GridChild + Sized {
    fn grid_row(mut self, row: i32) -> Self {
        let modifiers = capability::GridChild::grid_child_modifiers_mut(&mut self);
        let mut placement = modifiers.grid.unwrap_or_default();
        placement.row = row;
        modifiers.grid = Some(placement);
        self
    }

    fn grid_column(mut self, column: i32) -> Self {
        let modifiers = capability::GridChild::grid_child_modifiers_mut(&mut self);
        let mut placement = modifiers.grid.unwrap_or_default();
        placement.column = column;
        modifiers.grid = Some(placement);
        self
    }

    fn grid_row_span(mut self, row_span: i32) -> Self {
        let modifiers = capability::GridChild::grid_child_modifiers_mut(&mut self);
        let mut placement = modifiers.grid.unwrap_or_default();
        placement.row_span = row_span;
        modifiers.grid = Some(placement);
        self
    }

    fn grid_column_span(mut self, column_span: i32) -> Self {
        let modifiers = capability::GridChild::grid_child_modifiers_mut(&mut self);
        let mut placement = modifiers.grid.unwrap_or_default();
        placement.column_span = column_span;
        modifiers.grid = Some(placement);
        self
    }
}

impl<T: capability::GridChild> GridChildExt for T {}

/// Canvas positioning for concrete native children.
///
/// ```compile_fail
/// use windows_reactor::{CanvasChildExt, Element, text_block};
///
/// let element: Element = text_block("Marker").into();
/// let _ = element.canvas_left(40.0);
/// ```
pub trait CanvasChildExt: capability::CanvasChild + Sized {
    fn canvas_left(mut self, left: f64) -> Self {
        update_canvas_position(&mut self, |position| position.left = left);
        self
    }

    fn canvas_top(mut self, top: f64) -> Self {
        update_canvas_position(&mut self, |position| position.top = top);
        self
    }

    fn canvas_z_index(mut self, z_index: i32) -> Self {
        update_canvas_position(&mut self, |position| position.z_index = z_index);
        self
    }
}

impl<T: capability::CanvasChild> CanvasChildExt for T {}

/// Pointer, keyboard, capture, and drag/drop modifiers for concrete native widgets.
///
/// ```compile_fail
/// use windows_reactor::{Element, InputExt, button};
///
/// let element: Element = button("Open").into();
/// let _ = element.on_tapped(|| {});
/// ```
///
/// ```compile_fail
/// use windows_reactor::{Element, InputExt, KeyboardAccelerator, VirtualKey, VirtualKeyModifiers,
///     button};
///
/// let element: Element = button("Save").into();
/// let accelerator =
///     KeyboardAccelerator::new(VirtualKey::S, VirtualKeyModifiers::Control, || {});
/// let _ = element.keyboard_accelerator(accelerator);
/// ```
pub trait InputExt: capability::Input + Sized {
    fn on_tapped(mut self, f: impl IntoUnitCallback) -> Self {
        ensure_pointer_handlers(capability::Input::input_modifiers_mut(&mut self)).on_tapped =
            Some(f.into_unit_callback());
        self
    }

    fn on_right_tapped(mut self, f: impl IntoUnitCallback) -> Self {
        ensure_pointer_handlers(capability::Input::input_modifiers_mut(&mut self))
            .on_right_tapped = Some(f.into_unit_callback());
        self
    }

    fn on_pointer_pressed(mut self, f: impl IntoCallback<PointerEventInfo>) -> Self {
        ensure_pointer_handlers(capability::Input::input_modifiers_mut(&mut self))
            .on_pointer_pressed = Some(f.into_callback());
        self
    }

    fn on_pointer_released(mut self, f: impl IntoCallback<PointerEventInfo>) -> Self {
        ensure_pointer_handlers(capability::Input::input_modifiers_mut(&mut self))
            .on_pointer_released = Some(f.into_callback());
        self
    }

    fn on_pointer_moved(mut self, f: impl IntoCallback<PointerEventInfo>) -> Self {
        ensure_pointer_handlers(capability::Input::input_modifiers_mut(&mut self))
            .on_pointer_moved = Some(f.into_callback());
        self
    }

    fn on_pointer_entered(mut self, f: impl IntoCallback<PointerEventInfo>) -> Self {
        ensure_pointer_handlers(capability::Input::input_modifiers_mut(&mut self))
            .on_pointer_entered = Some(f.into_callback());
        self
    }

    fn on_pointer_exited(mut self, f: impl IntoUnitCallback) -> Self {
        ensure_pointer_handlers(capability::Input::input_modifiers_mut(&mut self))
            .on_pointer_exited = Some(f.into_unit_callback());
        self
    }

    fn capture_pointer_on_press(mut self) -> Self {
        ensure_pointer_handlers(capability::Input::input_modifiers_mut(&mut self))
            .capture_pointer_on_press = true;
        self
    }

    fn on_pointer_capture_lost(mut self, f: impl IntoUnitCallback) -> Self {
        ensure_pointer_handlers(capability::Input::input_modifiers_mut(&mut self))
            .on_pointer_capture_lost = Some(f.into_unit_callback());
        self
    }

    fn on_pointer_canceled(mut self, f: impl IntoUnitCallback) -> Self {
        ensure_pointer_handlers(capability::Input::input_modifiers_mut(&mut self))
            .on_pointer_canceled = Some(f.into_unit_callback());
        self
    }

    fn keyboard_accelerator(mut self, accel: KeyboardAccelerator) -> Self {
        capability::Input::input_modifiers_mut(&mut self)
            .keyboard_accelerators
            .push(accel);
        self
    }

    fn allow_drop(mut self, value: bool) -> Self {
        capability::Input::input_modifiers_mut(&mut self).allow_drop = Some(value);
        self
    }

    fn drag_enter<F: Fn(&mut DragContext) -> DragOperation + Send + Sync + 'static>(
        mut self,
        f: F,
    ) -> Self {
        ensure_drag_handlers(capability::Input::input_modifiers_mut(&mut self)).on_drag_enter =
            Some(DragAsyncCallback::new(f));
        self
    }

    fn drag_leave<F: Fn(&DragContext) + 'static>(mut self, f: F) -> Self {
        ensure_drag_handlers(capability::Input::input_modifiers_mut(&mut self)).on_drag_leave =
            Some(DragNotifyCallback::new(f));
        self
    }

    fn drag_over<F: Fn(&mut DragContext) -> DragOperation + 'static>(mut self, f: F) -> Self {
        ensure_drag_handlers(capability::Input::input_modifiers_mut(&mut self)).on_drag_over =
            Some(DragCallback::new(f));
        self
    }

    fn drag_drop<F: Fn(&mut DragContext) -> DragOperation + Send + Sync + 'static>(
        mut self,
        f: F,
    ) -> Self {
        ensure_drag_handlers(capability::Input::input_modifiers_mut(&mut self)).on_drag_drop =
            Some(DragAsyncCallback::new(f));
        self
    }
}

impl<T: capability::Input> InputExt for T {}

/// RelativePanel alignment for concrete native children.
///
/// These methods identify valid attached-property targets. The child must still be inserted into a
/// `RelativePanel` for WinUI to use the values.
///
/// ```compile_fail
/// use windows_reactor::{Element, RelativePanelChildExt, text_block};
///
/// let element: Element = text_block("Centered").into();
/// let _ = element.relative_align_h_center();
/// ```
pub trait RelativePanelChildExt: capability::RelativePanelChild + Sized {
    fn relative_align_left(mut self) -> Self {
        update_relative_panel_alignment(&mut self, |alignment| {
            alignment.align_left_with_panel = true;
        });
        self
    }

    fn relative_align_right(mut self) -> Self {
        update_relative_panel_alignment(&mut self, |alignment| {
            alignment.align_right_with_panel = true;
        });
        self
    }

    fn relative_align_top(mut self) -> Self {
        update_relative_panel_alignment(&mut self, |alignment| {
            alignment.align_top_with_panel = true;
        });
        self
    }

    fn relative_align_bottom(mut self) -> Self {
        update_relative_panel_alignment(&mut self, |alignment| {
            alignment.align_bottom_with_panel = true;
        });
        self
    }

    fn relative_align_h_center(mut self) -> Self {
        update_relative_panel_alignment(&mut self, |alignment| {
            alignment.align_h_center_with_panel = true;
        });
        self
    }

    fn relative_align_v_center(mut self) -> Self {
        update_relative_panel_alignment(&mut self, |alignment| {
            alignment.align_v_center_with_panel = true;
        });
        self
    }
}

impl<T: capability::RelativePanelChild> RelativePanelChildExt for T {}

/// Opacity and composition animations for concrete native widgets.
///
/// ```compile_fail
/// use windows_reactor::{Element, VisualExt, text_block};
///
/// let element: Element = text_block("Faded").into();
/// let _ = element.opacity(0.5);
/// ```
///
/// ```compile_fail
/// use std::time::Duration;
/// use windows_reactor::{Element, VisualExt, button};
///
/// let element: Element = button("Open").into();
/// let _ = element.with_opacity_transition(Duration::from_millis(100));
/// ```
pub trait VisualExt: capability::Visual + Sized {
    fn opacity(mut self, opacity: f64) -> Self {
        capability::Visual::visual_modifiers_mut(&mut self).opacity = Some(opacity);
        self
    }

    fn with_opacity_transition(mut self, duration: Duration) -> Self {
        with_implicit_transition(
            capability::Visual::visual_modifiers_mut(&mut self),
            |transitions| {
                transitions.opacity = Some(ScalarTransition::new(duration));
            },
        );
        self
    }

    fn with_scale_transition(mut self, duration: Duration) -> Self {
        with_implicit_transition(
            capability::Visual::visual_modifiers_mut(&mut self),
            |transitions| {
                transitions.scale = Some(Vector3Transition::new(duration));
            },
        );
        self
    }

    fn with_translation_transition(mut self, duration: Duration) -> Self {
        with_implicit_transition(
            capability::Visual::visual_modifiers_mut(&mut self),
            |transitions| {
                transitions.translation = Some(Vector3Transition::new(duration));
            },
        );
        self
    }

    fn with_layout_animation(mut self, config: LayoutAnimationConfig) -> Self {
        ensure_animations(capability::Visual::visual_modifiers_mut(&mut self)).layout_animation =
            Some(config);
        self
    }

    fn animate(mut self, config: AnimationConfig) -> Self {
        ensure_animations(capability::Visual::visual_modifiers_mut(&mut self)).property_animation =
            Some(config);
        self
    }

    fn transition(mut self, enter: Option<AnimationConfig>, exit: Option<AnimationConfig>) -> Self {
        let animations = ensure_animations(capability::Visual::visual_modifiers_mut(&mut self));
        animations.enter_transition = enter;
        animations.exit_transition = exit;
        self
    }
}

impl<T: capability::Visual> VisualExt for T {}

/// Tooltip modifiers for concrete native widgets.
///
/// ```compile_fail
/// use windows_reactor::{Element, TooltipExt, button};
///
/// let element: Element = button("Save").into();
/// let _ = element.tooltip("Save the document");
/// ```
pub trait TooltipExt: capability::Tooltip + Sized {
    /// Plain-text tooltip applied via WinUI `ToolTipService`. For rich content or custom
    /// placement, use [`tooltip_with`](Self::tooltip_with).
    fn tooltip(mut self, text: impl Into<String>) -> Self {
        capability::Tooltip::tooltip_modifiers_mut(&mut self).tooltip =
            Some(Box::new(Tooltip::text(text)));
        self
    }

    /// Tooltip setter accepting anything convertible into [`Tooltip`].
    fn tooltip_with(mut self, tooltip: impl Into<Tooltip>) -> Self {
        capability::Tooltip::tooltip_modifiers_mut(&mut self).tooltip =
            Some(Box::new(tooltip.into()));
        self
    }
}

impl<T: capability::Tooltip> TooltipExt for T {}

/// Framework layout modifiers for concrete native widget builders.
///
/// ```compile_fail
/// use windows_reactor::{Element, LayoutExt, button};
///
/// let element: Element = button("Save").into();
/// let _ = element.width(100.0);
/// ```
pub trait LayoutExt: capability::Layout + Sized {
    fn margin(mut self, value: impl Into<Thickness>) -> Self {
        capability::Layout::layout_modifiers_mut(&mut self).margin = Some(value.into());
        self
    }

    fn width(mut self, value: f64) -> Self {
        capability::Layout::layout_modifiers_mut(&mut self).width = Some(value);
        self
    }

    fn height(mut self, value: f64) -> Self {
        capability::Layout::layout_modifiers_mut(&mut self).height = Some(value);
        self
    }

    fn min_width(mut self, value: f64) -> Self {
        capability::Layout::layout_modifiers_mut(&mut self).min_width = Some(value);
        self
    }

    fn max_width(mut self, value: f64) -> Self {
        capability::Layout::layout_modifiers_mut(&mut self).max_width = Some(value);
        self
    }

    fn min_height(mut self, value: f64) -> Self {
        capability::Layout::layout_modifiers_mut(&mut self).min_height = Some(value);
        self
    }

    fn max_height(mut self, value: f64) -> Self {
        capability::Layout::layout_modifiers_mut(&mut self).max_height = Some(value);
        self
    }

    fn horizontal_alignment(mut self, value: HorizontalAlignment) -> Self {
        capability::Layout::layout_modifiers_mut(&mut self).horizontal_alignment = Some(value);
        self
    }

    fn vertical_alignment(mut self, value: VerticalAlignment) -> Self {
        capability::Layout::layout_modifiers_mut(&mut self).vertical_alignment = Some(value);
        self
    }
}

impl<T: capability::Layout> LayoutExt for T {}

/// Resource-dictionary modifiers for concrete native widget builders.
///
/// Apply resources before converting a widget into [`Element`]. Logical wrappers and erased
/// elements do not implement this capability.
///
/// ```compile_fail
/// use windows_reactor::{Element, ResourceExt, button};
///
/// let element: Element = button("Delete").into();
/// let _ = element.resources([("ButtonBackground", "Red")]);
/// ```
pub trait ResourceExt: capability::Resources + Sized {
    fn resources<K, V>(mut self, entries: impl IntoIterator<Item = (K, V)>) -> Self
    where
        K: Into<String>,
        V: Into<ResourceValue>,
    {
        capability::Resources::resource_modifiers_mut(&mut self).resources = entries
            .into_iter()
            .map(|(key, value)| (key.into(), value.into()))
            .collect();
        self
    }

    fn resource_overrides(
        mut self,
        configure: impl FnOnce(ResourceBuilder) -> ResourceBuilder,
    ) -> Self {
        capability::Resources::resource_modifiers_mut(&mut self).resources =
            configure(ResourceBuilder::default()).entries;
        self
    }
}

impl<T: capability::Resources> ResourceExt for T {}

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

fn accessibility_modifiers_mut<T: capability::Accessibility>(
    value: &mut T,
) -> &mut AccessibilityModifiers {
    ensure_accessibility(capability::Accessibility::accessibility_modifiers_mut(
        value,
    ))
}

fn update_canvas_position<T: capability::CanvasChild>(
    value: &mut T,
    update: impl FnOnce(&mut CanvasPosition),
) {
    let attached = attached_props_mut(capability::CanvasChild::canvas_child_modifiers_mut(value));
    let mut position = attached
        .get::<CanvasPosition>()
        .copied()
        .unwrap_or_default();
    update(&mut position);
    attached.set(position);
}

fn update_relative_panel_alignment<T: capability::RelativePanelChild>(
    value: &mut T,
    update: impl FnOnce(&mut RelativePanelAlignment),
) {
    let attached = attached_props_mut(
        capability::RelativePanelChild::relative_panel_child_modifiers_mut(value),
    );
    let mut alignment = attached
        .get::<RelativePanelAlignment>()
        .copied()
        .unwrap_or_default();
    update(&mut alignment);
    attached.set(alignment);
}

fn attached_props_mut(modifiers: &mut Modifiers) -> &mut AttachedProps {
    modifiers
        .attached
        .get_or_insert_with(AttachedProps::default)
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
