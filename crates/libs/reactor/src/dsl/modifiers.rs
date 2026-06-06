use std::time::Duration;

use super::*;

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
            m.tooltip = Some(Box::new(crate::core::tooltip::Tooltip::text(text)));
        }
        self
    }

    /// Tooltip setter accepting anything convertible into
    /// [`crate::core::tooltip::Tooltip`].
    fn tooltip_with(mut self, t: impl Into<crate::core::tooltip::Tooltip>) -> Self {
        if let Some(m) = self.modifiers_mut() {
            m.tooltip = Some(Box::new(t.into()));
        }
        self
    }

    /// Register a `Tapped` (left-tap) handler.
    fn on_tapped<F: Fn() + 'static>(mut self, f: F) -> Self {
        if let Some(m) = self.modifiers_mut() {
            ensure_pointer_handlers(m).on_tapped =
                Some(crate::core::callback::Callback::new(move |()| f()));
        }
        self
    }

    /// Register a `RightTapped` handler (right-click / barrel button /
    /// touch-and-hold).
    fn on_right_tapped<F: Fn() + 'static>(mut self, f: F) -> Self {
        if let Some(m) = self.modifiers_mut() {
            ensure_pointer_handlers(m).on_right_tapped =
                Some(crate::core::callback::Callback::new(move |()| f()));
        }
        self
    }

    /// Register a `PointerPressed` handler; the callback receives the
    /// current button state.
    fn on_pointer_pressed<F>(mut self, f: F) -> Self
    where
        F: Fn(crate::core::pointer::PointerEventInfo) + 'static,
    {
        if let Some(m) = self.modifiers_mut() {
            ensure_pointer_handlers(m).on_pointer_pressed =
                Some(crate::core::callback::Callback::new(f));
        }
        self
    }

    /// Register a `PointerReleased` handler; the callback receives the
    /// button state at release.
    fn on_pointer_released<F>(mut self, f: F) -> Self
    where
        F: Fn(crate::core::pointer::PointerEventInfo) + 'static,
    {
        if let Some(m) = self.modifiers_mut() {
            ensure_pointer_handlers(m).on_pointer_released =
                Some(crate::core::callback::Callback::new(f));
        }
        self
    }

    /// Register a `PointerExited` handler (pointer leaves hit-test bounds).
    fn on_pointer_exited<F: Fn() + 'static>(mut self, f: F) -> Self {
        if let Some(m) = self.modifiers_mut() {
            ensure_pointer_handlers(m).on_pointer_exited =
                Some(crate::core::callback::Callback::new(move |()| f()));
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

    fn heading_level(mut self, level: HeadingLevel) -> Self {
        if let Some(m) = self.modifiers_mut() {
            ensure_accessibility(m).heading_level = Some(level);
        }
        self
    }

    fn accessibility_live_setting(mut self, ls: LiveSetting) -> Self {
        if let Some(m) = self.modifiers_mut() {
            ensure_accessibility(m).live_setting = Some(ls);
        }
        self
    }

    fn keyboard_accelerator(mut self, accel: KeyboardAccelerator) -> Self {
        if let Some(m) = self.modifiers_mut() {
            m.keyboard_accelerators
                .get_or_insert_with(|| Box::new(Vec::new()))
                .push(accel);
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
            let map = entries
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect();
            m.resources = Some(Box::new(map));
        }
        self
    }
}

fn ensure_pointer_handlers(m: &mut Modifiers) -> &mut crate::core::pointer::PointerHandlers {
    m.pointer_handlers
        .get_or_insert_with(|| Box::new(crate::core::pointer::PointerHandlers::default()))
}

fn ensure_accessibility(
    m: &mut Modifiers,
) -> &mut crate::core::accessibility::AccessibilityModifiers {
    m.accessibility.get_or_insert_with(|| {
        Box::new(crate::core::accessibility::AccessibilityModifiers::default())
    })
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
            impl ElementExt for crate::core::widgets::$ty {
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
);

impl ElementExt for crate::core::rich_text::RichTextBlock {
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
        Element::modifiers_mut(self)
    }

    fn with_key(mut self, key: impl Into<String>) -> Self {
        let key = key.into();
        match &mut self {
            Element::TextBlock(t) => t.key = Some(key),
            Element::Button(b) => b.key = Some(key),
            Element::StackPanel(s) => s.key = Some(key),
            Element::Border(b) => b.key = Some(key),
            Element::CheckBox(c) => c.key = Some(key),
            Element::TextBox(t) => t.key = Some(key),
            Element::Grid(g) => g.key = Some(key),
            Element::ScrollViewer(s) => s.key = Some(key),
            Element::ToggleSwitch(v) => v.key = Some(key),
            Element::Slider(v) => v.key = Some(key),
            Element::RadioButton(v) => v.key = Some(key),
            Element::NumberBox(v) => v.key = Some(key),
            Element::ProgressBar(v) => v.key = Some(key),
            Element::ProgressRing(v) => v.key = Some(key),
            Element::Expander(v) => v.key = Some(key),
            Element::HyperlinkButton(v) => v.key = Some(key),
            Element::InfoBar(v) => v.key = Some(key),
            Element::InfoBadge(v) => v.key = Some(key),
            Element::PersonPicture(v) => v.key = Some(key),
            Element::Shape(v) => v.key = Some(key),
            Element::Image(v) => v.key = Some(key),
            Element::TabView(v) => v.key = Some(key),
            Element::NavigationView(v) => v.key = Some(key),
            Element::TitleBar(v) => v.key = Some(key),
            Element::Pivot(v) => v.key = Some(key),
            Element::BreadcrumbBar(v) => v.key = Some(key),
            Element::PasswordBox(v) => v.key = Some(key),
            Element::RadioButtons(v) => v.key = Some(key),
            Element::ComboBox(v) => v.key = Some(key),
            Element::Canvas(v) => v.key = Some(key),
            Element::ContentDialog(v) => v.key = Some(key),
            Element::RichTextBlock(v) => v.key = Some(key),
            Element::Viewbox(v) => v.key = Some(key),
            Element::RepeatButton(v) => v.key = Some(key),
            Element::RatingControl(v) => v.key = Some(key),
            Element::ColorPicker(v) => v.key = Some(key),
            Element::DatePicker(v) => v.key = Some(key),
            Element::TimePicker(v) => v.key = Some(key),
            Element::CalendarDatePicker(v) => v.key = Some(key),
            Element::CalendarView(v) => v.key = Some(key),
            Element::ListBox(v) => v.key = Some(key),
            Element::DropDownButton(v) => v.key = Some(key),
            Element::SplitButton(v) => v.key = Some(key),
            Element::AutoSuggestBox(v) => v.key = Some(key),
            Element::SplitView(v) => v.key = Some(key),
            Element::MenuBar(v) => v.key = Some(key),
            Element::ScrollView(v) => v.key = Some(key),
            Element::TreeView(v) => v.key = Some(key),
            Element::CommandBar(v) => v.key = Some(key),
            Element::TeachingTip(v) => v.key = Some(key),
            Element::SelectorBar(v) => v.key = Some(key),
            Element::RichEditBox(v) => v.key = Some(key),
            Element::RelativePanel(v) => v.key = Some(key),
            Element::ToggleButton(v) => v.key = Some(key),
            Element::Component(c) => c.key = Some(key),
            Element::ErrorBoundary(eb) => eb.key = Some(key),
            Element::Provider(pe) => pe.key = Some(key),
            Element::TemplatedList(tl) => tl.key = Some(key),
            Element::Group(g) => g.key = Some(key),
            _ => {}
        }
        self
    }
}
