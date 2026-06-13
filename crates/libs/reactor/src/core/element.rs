use super::component_element::ComponentElement;
use super::*;

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
            pub fn as_widget(&self) -> Option<&dyn crate::core::widget::Widget> {
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
            pub fn modifiers_mut(&mut self) -> Option<&mut Modifiers> {
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
            Element::Component(c) => c.key.as_deref(),
            Element::ErrorBoundary(eb) => eb.key.as_deref(),
            Element::Provider(p) => p.key.as_deref(),
            Element::TemplatedList(tl) => tl.key.as_deref(),
            Element::Group(g) => g.key.as_deref(),
            Element::Custom(c) => c.0.key(),
            Element::Empty => None,
            _ => unreachable!("covered by as_widget"),
        }
    }
    pub fn modifiers(&self) -> Option<&Modifiers> {
        if let Some(w) = self.as_widget() {
            return Some(w.modifiers());
        }
        match self {
            Element::TemplatedList(tl) => Some(&tl.modifiers),
            Element::Component(_)
            | Element::ErrorBoundary(_)
            | Element::Provider(_)
            | Element::Group(_)
            | Element::Custom(_)
            | Element::Empty => None,
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
    pub fn accessibility_mut(&mut self) -> Option<&mut AccessibilityModifiers> {
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
    pub fn accessibility_live_setting(mut self, ls: LiveSetting) -> Self {
        if let Some(a) = self.accessibility_mut() {
            a.live_setting = Some(ls);
        }
        self
    }
    pub fn heading_level(mut self, level: HeadingLevel) -> Self {
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
    pub fn kind_matches(&self, other: &Element) -> bool {
        if std::mem::discriminant(self) != std::mem::discriminant(other) {
            return false;
        }
        if let (Element::Shape(a), Element::Shape(b)) = (self, other) {
            return a.kind == b.kind;
        }
        if let (Element::Custom(a), Element::Custom(b)) = (self, other) {
            return CustomElement::type_id(&*a.0) == CustomElement::type_id(&*b.0);
        }
        true
    }
    /// `true` when the reconciler may diff `self` against `other` in place
    /// rather than unmounting and remounting.
    pub fn can_update(&self, other: &Element) -> bool {
        if !self.kind_matches(other) {
            return false;
        }
        match (self, other) {
            (Element::Component(a), Element::Component(b)) => {
                a.obj.component_type_id() == b.obj.component_type_id()
            }
            (Element::Custom(a), Element::Custom(b)) => {
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
