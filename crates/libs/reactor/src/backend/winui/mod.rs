use std::cell::RefCell;

use rustc_hash::FxHashMap;

use super::*;

mod convert;
mod diag;
mod generated_attach_event;
mod generated_set_prop;
use convert::*;

/// Single source of truth for the `Handle` enum, its casts, the
/// `ControlKind`→`Handle` constructor, and `describe_kind`. Each variant name
/// matches its backing `bindings::Variant` class and must correspond to a
/// `ControlKind` variant; the class must be activatable.
macro_rules! define_handles {
    ( $( $variant:ident ),* $(,)? ) => {
        enum Handle {
            $( $variant(bindings::$variant), )*
        }

        impl Handle {
            fn cast_inner<T: windows_core::Interface>(&self) -> windows_core::Result<T> {
                match self {
                    $( Handle::$variant(v) => v.cast::<T>(), )*
                }
            }
            fn as_framework_element(&self) -> bindings::FrameworkElement {
                self.cast_inner().unwrap()
            }
            fn as_ui_element(&self) -> bindings::UIElement {
                self.cast_inner().unwrap()
            }
            fn kind_name(&self) -> &'static str {
                match self {
                    $( Handle::$variant(_) => stringify!($variant), )*
                }
            }
        }

        impl WinUIBackend {
            fn make_handle_for_kind(kind: ControlKind) -> Handle {
                match kind {
                    $(
                        ControlKind::$variant => Handle::$variant(
                            <bindings::$variant>::new().unwrap(),
                        ),
                    )*
                }
            }
        }

        fn describe_kind(h: &Handle) -> &'static str {
            match h {
                $( Handle::$variant(_) => stringify!($variant), )*
            }
        }
    };
}

define_handles! {
    AutoSuggestBox,
    Border,
    BreadcrumbBar,
    Button,
    CalendarDatePicker,
    CalendarView,
    Canvas,
    CheckBox,
    ColorPicker,
    ComboBox,
    CommandBar,
    ContentDialog,
    DatePicker,
    DropDownButton,
    Ellipse,
    Expander,
    FlipView,
    Grid,
    GridView,
    HyperlinkButton,
    Image,
    InfoBadge,
    InfoBar,
    Line,
    ListBox,
    ListView,
    MenuBar,
    NavigationView,
    NumberBox,
    PasswordBox,
    PersonPicture,
    Pivot,
    PivotItem,
    ProgressBar,
    ProgressRing,
    RadioButton,
    RadioButtons,
    RatingControl,
    Rectangle,
    RelativePanel,
    RepeatButton,
    RichEditBox,
    RichTextBlock,
    ScrollView,
    ScrollViewer,
    SelectorBar,
    Slider,
    SplitButton,
    SplitView,
    StackPanel,
    SwapChainPanel,
    TabView,
    TabViewItem,
    TeachingTip,
    TextBlock,
    TextBox,
    TimePicker,
    TitleBar,
    ToggleButton,
    ToggleSwitch,
    TreeView,
    Viewbox,
    WebView2,
}

/// [`Backend`] implementation that creates real `Microsoft.UI.Xaml`
/// controls and drives them on the WinUI thread.
pub struct WinUIBackend {
    controls: RefCell<FxHashMap<ControlId, Handle>>,
    event_revokers: RefCell<FxHashMap<(ControlId, Event), Vec<windows_core::EventRevoker>>>,
    templated_selection_revokers: RefCell<FxHashMap<ControlId, windows_core::EventRevoker>>,
    /// Per-list virtualization state for templated ListView/GridView/FlipView.
    templated: RefCell<FxHashMap<ControlId, TemplatedList>>,
    /// Shared `ItemTemplate` for virtualized ListView/GridView: a
    /// `DataTemplate` whose root is a plain `ContentControl`. Parsed once and
    /// reused across every list. WinUI's `ListViewItemPresenter` renders item
    /// content as a *string*, so it cannot host an arbitrary `UIElement`
    /// directly; routing through a real `ContentControl` (the template root)
    /// lets us set `Content` to a reactor-built element and have it render.
    /// The one-time parse is off the per-item hot path.
    content_template: RefCell<Option<bindings::DataTemplate>>,
    /// Pointer-handler revokers (separate from `event_revokers` because
    /// pointer events use the universal `IUIElement` event surface).
    pointer_revokers: RefCell<FxHashMap<ControlId, PointerRevokerSet>>,
    drag_revokers: RefCell<FxHashMap<ControlId, DragRevokerSet>>,
    /// Logical children per parent, mirroring the reconciler. Used to
    /// translate logical indices into visual-tree indices when phantom
    /// children (e.g. `ContentDialog`) are tracked but not attached.
    parent_children: RefCell<FxHashMap<ControlId, Vec<ControlId>>>,
    /// Stored click handlers for MenuBar/MenuFlyout items so they can be
    /// re-wired when the item collection is rebuilt via `set_prop`.
    menu_click_handlers: RefCell<FxHashMap<ControlId, EventHandler>>,
    command_bar_flyout_handlers: RefCell<FxHashMap<ControlId, EventHandler>>,
    /// Registry of theme brush bindings per control, so they can be
    /// re-resolved when the app theme changes (dark ↔ light).
    theme_brush_registry: RefCell<FxHashMap<ControlId, Vec<(Prop, ThemeRef)>>>,
    /// This host's window state, used to apply window-level props (e.g. the
    /// title-bar height) to the correct window when multiple hosts share a
    /// UI thread. `None` for non-window backends (tests use `RecordingBackend`).
    window_state: RefCell<Option<Rc<HostWindowState>>>,
    next_id: RefCell<u32>,
}

#[derive(Default)]
struct PointerRevokerSet {
    tapped: Option<windows_core::EventRevoker>,
    right_tapped: Option<windows_core::EventRevoker>,
    pressed: Option<windows_core::EventRevoker>,
    released: Option<windows_core::EventRevoker>,
    moved: Option<windows_core::EventRevoker>,
    entered: Option<windows_core::EventRevoker>,
    exited: Option<windows_core::EventRevoker>,
}

#[derive(Default)]
struct DragRevokerSet {
    enter: Option<windows_core::EventRevoker>,
    leave: Option<windows_core::EventRevoker>,
    over: Option<windows_core::EventRevoker>,
    drop: Option<windows_core::EventRevoker>,
}

/// Interior-mutable state shared between a templated list's WinUI event
/// handlers (container recycling, drag-reorder) and the backend's
/// `set_templated_*` methods. The handlers fire outside `&mut self`, so the
/// pieces they touch live behind `Rc<RefCell<_>>`.
#[derive(Clone, Default)]
struct TemplatedShared {
    /// The observable `ItemsSource` of boxed `i32` indices driving WinUI
    /// virtualization for ListView/GridView. `None` for FlipView.
    source: Rc<RefCell<Option<windows_collections::IObservableVector<windows_core::IInspectable>>>>,
    /// Logical row index → the content host for that row: the inner
    /// `ContentControl` at the root of the shared item template. Populated
    /// when WinUI prepares a container, cleared when it recycles.
    containers: Rc<RefCell<FxHashMap<usize, bindings::IContentControl>>>,
}

/// Per-list backend bookkeeping for templated (virtualized) lists.
struct TemplatedList {
    shared: TemplatedShared,
    realize_revoker: Option<windows_core::EventRevoker>,
    reorder_revoker: Option<windows_core::EventRevoker>,
}

impl TemplatedList {
    fn new() -> Self {
        Self {
            shared: TemplatedShared::default(),
            realize_revoker: None,
            reorder_revoker: None,
        }
    }
}

impl Default for WinUIBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl WinUIBackend {
    pub fn new() -> Self {
        Self {
            controls: RefCell::new(FxHashMap::default()),
            event_revokers: RefCell::new(FxHashMap::default()),
            templated_selection_revokers: RefCell::new(FxHashMap::default()),
            templated: RefCell::new(FxHashMap::default()),
            content_template: RefCell::new(None),
            pointer_revokers: RefCell::new(FxHashMap::default()),
            drag_revokers: RefCell::new(FxHashMap::default()),
            parent_children: RefCell::new(FxHashMap::default()),
            menu_click_handlers: RefCell::new(FxHashMap::default()),
            command_bar_flyout_handlers: RefCell::new(FxHashMap::default()),
            theme_brush_registry: RefCell::new(FxHashMap::default()),
            window_state: RefCell::new(None),
            next_id: RefCell::new(0),
        }
    }

    /// Attach this backend to its host's window state so window-level props
    /// (e.g. title-bar height) target the correct window.
    pub(crate) fn set_window_state(&self, state: Rc<HostWindowState>) {
        *self.window_state.borrow_mut() = Some(state);
    }
    pub fn get_ui_element(&self, id: ControlId) -> Option<windows_core::IInspectable> {
        self.controls
            .borrow()
            .get(&id)
            .map(|h| h.as_ui_element().cast().unwrap())
    }
    /// Returns the shared virtualization item template, parsing it on first
    /// use. The XAML is a fixed internal invariant, so a parse/cast failure is
    /// a bug (not a recoverable condition) and panics rather than degrading to
    /// a permanently blank list with no diagnostic.
    fn content_template(&self) -> bindings::DataTemplate {
        if let Some(t) = self.content_template.borrow().as_ref() {
            return t.clone();
        }
        let template = bindings::XamlReader::Load(CONTENT_TEMPLATE_XAML)
            .unwrap()
            .cast::<bindings::DataTemplate>()
            .unwrap();
        *self.content_template.borrow_mut() = Some(template.clone());
        template
    }
    pub fn find_titlebar(&self) -> Option<bindings::TitleBar> {
        self.controls.borrow().values().find_map(|h| match h {
            Handle::TitleBar(tb) => Some(tb.clone()),
            _ => None,
        })
    }
    fn alloc_id(&self) -> ControlId {
        let mut counter = self.next_id.borrow_mut();
        *counter += 1;
        ControlId::new(*counter)
    }
    /// Whether this control is a "phantom" child — tracked in the
    /// reconciler's tree but not attached under its parent's visual
    /// `Children`. Only `ContentDialog` qualifies today.
    fn is_phantom_child(&self, id: ControlId) -> bool {
        matches!(
            self.controls.borrow().get(&id),
            Some(Handle::ContentDialog(_))
        )
    }
    /// Logical→visual child index translation that skips phantom children.
    fn visual_index(&self, parent: ControlId, logical: usize) -> usize {
        let kids = self.parent_children.borrow();
        let Some(list) = kids.get(&parent) else {
            return logical;
        };
        let map = self.controls.borrow();
        list.iter()
            .take(logical)
            .filter(|cid| !matches!(map.get(cid), Some(Handle::ContentDialog(_))))
            .count()
    }

    /// Wire Click handlers on all `MenuFlyoutItem`s within a `MenuBar`,
    /// recursing into sub-items.  Returns the collected `EventRevoker`s.
    fn wire_menu_bar_clicks(
        mb: &bindings::MenuBar,
        handler: &EventHandler,
    ) -> Vec<windows_core::EventRevoker> {
        let mut revokers = Vec::new();
        let Ok(bar_items) = mb.Items() else {
            return revokers;
        };
        for mbi in &bar_items {
            if let Ok(flyout_items) = mbi.Items() {
                Self::wire_flyout_items_click(&flyout_items, handler, &mut revokers);
            }
        }
        revokers
    }

    /// Wire Click handlers on all `MenuFlyoutItem`s within a flyout item
    /// collection, recursing into sub-items.
    fn wire_flyout_clicks(
        flyout: &bindings::MenuFlyout,
        handler: &EventHandler,
    ) -> Vec<windows_core::EventRevoker> {
        let mut revokers = Vec::new();
        if let Ok(items) = flyout.Items() {
            Self::wire_flyout_items_click(&items, handler, &mut revokers);
        }
        revokers
    }

    fn wire_flyout_items_click(
        items: &windows_collections::IVector<bindings::MenuFlyoutItemBase>,
        handler: &EventHandler,
        revokers: &mut Vec<windows_core::EventRevoker>,
    ) {
        for base in items {
            if let Ok(item) = base.cast::<bindings::MenuFlyoutItem>() {
                let text = item.Text().unwrap_or_default().clone();
                let handler = handler.clone();
                if let Ok(rev) = item.Click(move |_s, _a| {
                    handler.invoke_string(text.clone());
                }) {
                    revokers.push(rev);
                }
            } else if let Ok(sub) = base.cast::<bindings::MenuFlyoutSubItem>()
                && let Ok(sub_items) = sub.Items()
            {
                Self::wire_flyout_items_click(&sub_items, handler, revokers);
            }
        }
    }

    fn wire_command_bar_clicks(
        commands: &windows_collections::IObservableVector<bindings::ICommandBarElement>,
        handler: &EventHandler,
    ) -> Vec<windows_core::EventRevoker> {
        let mut revokers = Vec::new();
        for el in commands {
            if let Ok(btn) = el.cast::<bindings::AppBarButton>() {
                let label = btn.Label().unwrap_or_default().clone();
                let handler = handler.clone();
                if let Ok(rev) = btn.cast::<bindings::ButtonBase>().and_then(|bb| {
                    bb.Click(move |_s, _a| {
                        handler.invoke_string(label.clone());
                    })
                }) {
                    revokers.push(rev);
                }
            }
        }
        revokers
    }
    /// Insert `child` at visual index `v_index`. Caller must ensure
    /// `child` is non-phantom; the logical mirror is NOT touched.
    fn visual_insert_at(&self, parent: ControlId, v_index: usize, child: ControlId) {
        let map = self.controls.borrow();
        let parent_h = map
            .get(&parent)
            .unwrap_or_else(|| panic!("WinUIBackend::visual_insert_at: unknown parent {parent}"));
        let child_h = map
            .get(&child)
            .unwrap_or_else(|| panic!("WinUIBackend::visual_insert_at: unknown child {child}"));
        let child_ui = child_h.as_ui_element();
        let cc = classify_container(parent_h).unwrap_or_else(|| {
            panic!("WinUIBackend::visual_insert_at: {parent} is not a container")
        });
        container_insert(&cc, v_index, &child_ui);
    }
    /// Remove the child at visual index `v_index`. Logical mirror is NOT touched.
    fn visual_remove_at(&self, parent: ControlId, v_index: usize) {
        let map = self.controls.borrow();
        let parent_h = map
            .get(&parent)
            .unwrap_or_else(|| panic!("WinUIBackend::visual_remove_at: unknown parent {parent}"));
        let cc = classify_container(parent_h).unwrap_or_else(|| {
            panic!("WinUIBackend::visual_remove_at: {parent} is not a container")
        });
        container_remove(&cc, v_index);
    }
    /// In-place replace the child at visual index `v_index`. Caller
    /// must ensure `new` is non-phantom; logical mirror is NOT touched.
    fn visual_set_at(&self, parent: ControlId, v_index: usize, new: ControlId) {
        let map = self.controls.borrow();
        let parent_h = map
            .get(&parent)
            .unwrap_or_else(|| panic!("WinUIBackend::visual_set_at: unknown parent {parent}"));
        let new_h = map
            .get(&new)
            .unwrap_or_else(|| panic!("WinUIBackend::visual_set_at: unknown new {new}"));
        let new_ui = new_h.as_ui_element();
        let cc = classify_container(parent_h)
            .unwrap_or_else(|| panic!("WinUIBackend::visual_set_at: {parent} is not a container"));
        container_set(&cc, v_index, &new_ui);
    }
}

/// Classification of container handles for child-tree operations.
/// Every container variant falls into one of these shapes, which
/// determines how children are appended, removed, moved, etc.
enum ContainerChildren<'a> {
    /// Multi-child panel (StackPanel, Grid, Canvas, RelativePanel) backed
    /// by `IPanel::Children` — a `UIElementCollection` (which derefs to
    /// `IVector<UIElement>`).
    Panel(bindings::UIElementCollection),
    /// Single-child container (Border, Viewbox) that uses `put_Child`.
    SingleChild(&'a Handle),
    /// Single-child `IContentControl` (ScrollViewer, Expander,
    /// TabViewItem, NavigationView, PivotItem).
    ContentControl(bindings::IContentControl),
    /// Single-child container that has a direct `put_Content` method but
    /// does not implement `IContentControl` (ScrollView, SplitView).
    DirectContent(&'a Handle),
    /// `IVector<IInspectable>`-backed multi-child (TabView, Pivot).
    InspectableVector(windows_collections::IVector<windows_core::IInspectable>),
}

/// Classify a `Handle` into its container shape. Returns `None` when the
/// handle does not support children.
fn classify_container(h: &Handle) -> Option<ContainerChildren<'_>> {
    match h {
        Handle::StackPanel(s) => Some(ContainerChildren::Panel(
            s.cast::<bindings::IPanel>().ok()?.Children().ok()?,
        )),
        Handle::Grid(g) => Some(ContainerChildren::Panel(
            g.cast::<bindings::IPanel>().ok()?.Children().ok()?,
        )),
        Handle::Canvas(c) => Some(ContainerChildren::Panel(
            c.cast::<bindings::IPanel>().ok()?.Children().ok()?,
        )),
        Handle::RelativePanel(r) => Some(ContainerChildren::Panel(
            r.cast::<bindings::IPanel>().ok()?.Children().ok()?,
        )),
        Handle::Border(_) | Handle::Viewbox(_) => Some(ContainerChildren::SingleChild(h)),
        Handle::ScrollViewer(s) => Some(ContainerChildren::ContentControl(s.cast().ok()?)),
        Handle::Expander(e) => Some(ContainerChildren::ContentControl(e.cast().ok()?)),
        Handle::TabViewItem(ti) => Some(ContainerChildren::ContentControl(ti.cast().ok()?)),
        Handle::NavigationView(nv) => Some(ContainerChildren::ContentControl(nv.cast().ok()?)),
        Handle::PivotItem(pi) => Some(ContainerChildren::ContentControl(pi.cast().ok()?)),
        Handle::ScrollView(_) | Handle::SplitView(_) => Some(ContainerChildren::DirectContent(h)),
        Handle::TabView(tv) => Some(ContainerChildren::InspectableVector(tv.TabItems().ok()?)),
        Handle::Pivot(p) => Some(ContainerChildren::InspectableVector(
            p.cast::<bindings::IItemsControl>()
                .ok()?
                .Items()
                .ok()?
                .cast()
                .ok()?,
        )),
        _ => None,
    }
}

/// Append a child UIElement to a container.
fn container_append(cc: &ContainerChildren<'_>, child: &bindings::UIElement) {
    match cc {
        ContainerChildren::Panel(vec) => vec.Append(child).unwrap(),
        ContainerChildren::SingleChild(h) => put_single_child(h, Some(child)),
        ContainerChildren::ContentControl(c) => c.SetContent(child).unwrap(),
        ContainerChildren::DirectContent(h) => put_direct_content(h, Some(child)),
        ContainerChildren::InspectableVector(vec) => {
            let insp: windows_core::IInspectable = child.cast().unwrap();
            vec.Append(&insp).unwrap();
        }
    }
}

/// Insert a child UIElement at `index`.
fn container_insert(cc: &ContainerChildren<'_>, index: usize, child: &bindings::UIElement) {
    match cc {
        ContainerChildren::Panel(vec) => vec.InsertAt(index as u32, child).unwrap(),
        ContainerChildren::SingleChild(h) => put_single_child(h, Some(child)),
        ContainerChildren::ContentControl(c) => c.SetContent(child).unwrap(),
        ContainerChildren::DirectContent(h) => put_direct_content(h, Some(child)),
        ContainerChildren::InspectableVector(vec) => {
            let insp: windows_core::IInspectable = child.cast().unwrap();
            vec.InsertAt(index as u32, &insp).unwrap();
        }
    }
}

/// Replace the child at `index`.
fn container_set(cc: &ContainerChildren<'_>, index: usize, child: &bindings::UIElement) {
    match cc {
        ContainerChildren::Panel(vec) => vec.SetAt(index as u32, child).unwrap(),
        ContainerChildren::SingleChild(h) => put_single_child(h, Some(child)),
        ContainerChildren::ContentControl(c) => c.SetContent(child).unwrap(),
        ContainerChildren::DirectContent(h) => put_direct_content(h, Some(child)),
        ContainerChildren::InspectableVector(vec) => {
            let insp: windows_core::IInspectable = child.cast().unwrap();
            vec.SetAt(index as u32, &insp).unwrap();
        }
    }
}

/// Remove the child at `index`.
fn container_remove(cc: &ContainerChildren<'_>, index: usize) {
    match cc {
        ContainerChildren::Panel(vec) => vec.RemoveAt(index as u32).unwrap(),
        ContainerChildren::SingleChild(h) => {
            debug_assert_eq!(index, 0);
            put_single_child(h, None);
        }
        ContainerChildren::ContentControl(c) => {
            debug_assert_eq!(index, 0);
            c.SetContent(None::<&windows_core::IInspectable>).unwrap();
        }
        ContainerChildren::DirectContent(h) => {
            debug_assert_eq!(index, 0);
            put_direct_content(h, None);
        }
        ContainerChildren::InspectableVector(vec) => vec.RemoveAt(index as u32).unwrap(),
    }
}

/// Move a child from `from` to `to` within a container. Single-child
/// containers are no-ops.
fn container_move(cc: &ContainerChildren<'_>, from: usize, to: usize) {
    match cc {
        ContainerChildren::Panel(vec) => {
            let item = vec.GetAt(from as u32).unwrap();
            vec.RemoveAt(from as u32).unwrap();
            vec.InsertAt(to as u32, &item).unwrap();
        }
        ContainerChildren::SingleChild(_)
        | ContainerChildren::ContentControl(_)
        | ContainerChildren::DirectContent(_) => {}
        ContainerChildren::InspectableVector(vec) => {
            let item = vec.GetAt(from as u32).unwrap();
            vec.RemoveAt(from as u32).unwrap();
            vec.InsertAt(to as u32, &item).unwrap();
        }
    }
}

fn put_single_child(h: &Handle, child: Option<&bindings::UIElement>) {
    match h {
        Handle::Border(b) => b.SetChild(child).unwrap(),
        Handle::Viewbox(v) => v.SetChild(child).unwrap(),
        _ => unreachable!(),
    }
}

fn put_direct_content(h: &Handle, child: Option<&bindings::UIElement>) {
    match h {
        Handle::ScrollView(sv) => sv.SetContent(child).unwrap(),
        Handle::SplitView(sv) => sv.SetContent(child).unwrap(),
        _ => unreachable!(),
    }
}

/// Build and apply a XAML Style with {ThemeResource} setters to an element.
/// WinUI handles theme-reactive resolution natively (Light ↔ Dark).
fn apply_theme_resource_style(handle: &Handle, bindings: &[(Prop, ThemeRef)]) {
    let Some((target_type, fe)) = style_target_for_handle(handle) else {
        return;
    };

    let mut setters = String::new();
    for (prop, theme_ref) in bindings {
        let dp_name = match prop {
            Prop::Background => "Background",
            Prop::Foreground => "Foreground",
            Prop::BorderBrush => "BorderBrush",
            _ => continue,
        };
        let resource_key = theme_ref.resource_key();
        setters.push_str(&format!(
            "<Setter Property='{dp_name}' Value='{{ThemeResource {resource_key}}}'/>"
        ));
    }

    if setters.is_empty() {
        diag::dropped(fe.SetStyle(None));
        return;
    }

    let xaml = format!(
        "<Style xmlns='http://schemas.microsoft.com/winfx/2006/xaml/presentation' TargetType='{target_type}'>{setters}</Style>"
    );

    match bindings::XamlReader::Load(&xaml) {
        Ok(obj) => {
            if let Ok(style) = obj.cast::<bindings::Style>() {
                // Clear first to force WinUI to re-resolve {ThemeResource} values
                diag::dropped(fe.SetStyle(None));
                diag::dropped(fe.SetStyle(&style));
            }
        }
        Err(e) => {
            diag::warn(format_args!(
                "ThemeStyle: XamlReader::Load failed: {e:?} xaml={xaml}"
            ));
        }
    }
}

/// Get the XAML TargetType name and IFrameworkElement for a Handle.
fn style_target_for_handle(handle: &Handle) -> Option<(&'static str, bindings::IFrameworkElement)> {
    match handle {
        Handle::Border(b) => b.cast().ok().map(|fe| ("Border", fe)),
        Handle::StackPanel(s) => s.cast().ok().map(|fe| ("StackPanel", fe)),
        Handle::Grid(g) => g.cast().ok().map(|fe| ("Grid", fe)),
        Handle::Button(b) => b.cast().ok().map(|fe| ("Button", fe)),
        Handle::TextBox(t) => t.cast().ok().map(|fe| ("TextBox", fe)),
        Handle::TextBlock(t) => t.cast().ok().map(|fe| ("TextBlock", fe)),
        Handle::Canvas(c) => c.cast().ok().map(|fe| ("Canvas", fe)),
        _ => None,
    }
}

// Composition animation helpers. Both `apply_implicit_transitions` and
// `run_property_animation_inner` reach the element's backing composition
// `Visual` via `ElementCompositionPreview::GetElementVisual` and drive it
// through windows-composition's safe wrappers.

fn easing_for(
    compositor: &windows_composition::Compositor,
    easing: Easing,
) -> windows_composition::CompositionEasingFunction {
    // Control points match the CSS-standard ease-{out,in,in-out} curves.
    let (p1, p2) = match easing {
        Easing::Linear => return compositor.create_linear_easing_function(),
        Easing::EaseOut => (
            windows_numerics::Vector2 { x: 0.0, y: 0.0 },
            windows_numerics::Vector2 { x: 0.58, y: 1.0 },
        ),
        Easing::EaseIn => (
            windows_numerics::Vector2 { x: 0.42, y: 0.0 },
            windows_numerics::Vector2 { x: 1.0, y: 1.0 },
        ),
        Easing::EaseInOut => (
            windows_numerics::Vector2 { x: 0.42, y: 0.0 },
            windows_numerics::Vector2 { x: 0.58, y: 1.0 },
        ),
    };
    compositor.create_cubic_bezier_easing_function(p1, p2)
}

/// Wraps an element's backing composition visual for the animation engine.
fn element_visual(ui: &bindings::UIElement) -> Result<windows_composition::Visual> {
    let raw = bindings::ElementCompositionPreview::GetElementVisual(ui)?;
    windows_composition::Visual::from_host(raw.into())
}

fn apply_implicit_transitions(
    ui: &bindings::UIElement,
    transitions: Option<ImplicitTransitions>,
) -> Result<()> {
    let visual = element_visual(ui)?;
    // No transitions, or every slot empty: clear the implicit-animation collection.
    let Some(t) = transitions.filter(|t| !t.is_empty()) else {
        visual.set_implicit_animations(None);
        return Ok(());
    };
    let compositor = visual.compositor();
    let collection = compositor.create_implicit_animation_collection();

    // Animate from `this.StartingValue` to `this.FinalValue` over `duration`.
    // The DSL transition types only expose duration, so easing is fixed to
    // EaseOut — the standard XAML implicit-transition curve.
    let insert = |target: &str, duration: std::time::Duration, is_scalar: bool| {
        let easing = easing_for(&compositor, Easing::EaseOut);
        if is_scalar {
            let a = compositor.create_scalar_key_frame_animation();
            a.set_duration(duration);
            a.insert_expression_key_frame_with_easing(1.0, "this.FinalValue", &easing);
            a.set_target(target);
            collection.insert(target, &a);
        } else {
            let a = compositor.create_vector3_key_frame_animation();
            a.set_duration(duration);
            a.insert_expression_key_frame_with_easing(1.0, "this.FinalValue", &easing);
            a.set_target(target);
            collection.insert(target, &a);
        }
    };

    if let Some(s) = t.opacity {
        insert("Opacity", s.duration, true);
    }
    if let Some(s) = t.rotation {
        insert("RotationAngleInDegrees", s.duration, true);
    }
    if let Some(v) = t.scale {
        insert("Scale", v.duration, false);
    }
    if let Some(v) = t.translation {
        // KNOWN LIMITATION: `Offset` collides with XAML layout; the
        // correct target is the synthetic `Translation` channel.
        insert("Offset", v.duration, false);
    }
    visual.set_implicit_animations(Some(&collection));
    Ok(())
}

fn run_property_animation_inner(ui: &bindings::UIElement, cfg: AnimationConfig) -> Result<()> {
    let visual = element_visual(ui)?;
    let compositor = visual.compositor();

    if let Some(opacity) = cfg.opacity {
        let a = compositor.create_scalar_key_frame_animation();
        a.set_duration(cfg.duration);
        let easing = easing_for(&compositor, cfg.easing);
        a.insert_key_frame_with_easing(1.0, opacity as f32, &easing);
        visual.start_animation("Opacity", &a);
    }
    if let Some(scale) = cfg.scale {
        // Pivot scale around the element's centre. KNOWN LIMITATION:
        // before the first layout pass ActualWidth/Height are 0 and the
        // previous CenterPoint is reused.
        if let Ok(fe) = ui.cast::<bindings::IFrameworkElement>() {
            let w = fe.ActualWidth().unwrap_or(0.0) as f32;
            let h = fe.ActualHeight().unwrap_or(0.0) as f32;
            if w > 0.0 && h > 0.0 {
                visual.set_center_point(windows_numerics::Vector3 {
                    x: w / 2.0,
                    y: h / 2.0,
                    z: 0.0,
                });
            } else {
                diag::warn(format_args!(
                    "animation: skipping CenterPoint — element not yet laid out"
                ));
            }
        }
        // Preserve current Scale.Z; cfg.scale is a uniform X/Y scalar.
        let current_z = visual.scale().z;
        let a = compositor.create_vector3_key_frame_animation();
        a.set_duration(cfg.duration);
        let easing = easing_for(&compositor, cfg.easing);
        let s = scale as f32;
        a.insert_key_frame_with_easing(
            1.0,
            windows_numerics::Vector3 {
                x: s,
                y: s,
                z: current_z,
            },
            &easing,
        );
        visual.start_animation("Scale", &a);
    }
    Ok(())
}

/// Handle props that apply to any control via base-class interfaces
/// (`IFrameworkElement`, `IUIElement`, `IControl`, `IPanel`, `IShape`, etc.).
/// Each prop is dispatched by trying interface casts in priority order.
/// Returns `Ok(true)` when handled, `Ok(false)` to fall through.
fn try_universal_prop(handle: &Handle, prop: Prop, value: &PropValue) -> Result<bool> {
    match (prop, value) {
        (Prop::FontSize, PropValue::F64(v)) => set_font_f64(handle, *v),
        (Prop::FontSize, PropValue::Unset) => set_font_f64(handle, 14.0),
        (Prop::FontWeight, PropValue::U16(w)) => {
            set_font_weight(handle, bindings::FontWeight { weight: *w })
        }
        (Prop::FontWeight, PropValue::Unset) => {
            set_font_weight(handle, bindings::FontWeight { weight: 400 })
        }
        (Prop::FontFamily, PropValue::Str(s)) => {
            set_font_family(handle, &bindings::FontFamily::CreateInstanceWithName(s)?)
        }
        (Prop::FontFamily, PropValue::Unset) => set_font_family(
            handle,
            &bindings::FontFamily::CreateInstanceWithName("Segoe UI")?,
        ),
        (Prop::Margin, PropValue::Thickness(t)) => {
            handle.as_framework_element().SetMargin(*t)?;
            Ok(true)
        }
        (Prop::Margin, PropValue::Unset) => {
            handle
                .as_framework_element()
                .SetMargin(Thickness::default())?;
            Ok(true)
        }
        (Prop::Width, PropValue::F64(v)) => {
            handle.as_framework_element().SetWidth(*v)?;
            Ok(true)
        }
        (Prop::Width, PropValue::Unset) => {
            handle.as_framework_element().SetWidth(f64::NAN)?;
            Ok(true)
        }
        (Prop::Height, PropValue::F64(v)) => {
            handle.as_framework_element().SetHeight(*v)?;
            Ok(true)
        }
        (Prop::Height, PropValue::Unset) => {
            handle.as_framework_element().SetHeight(f64::NAN)?;
            Ok(true)
        }
        (Prop::MinWidth, PropValue::F64(v)) => {
            handle.as_framework_element().SetMinWidth(*v)?;
            Ok(true)
        }
        (Prop::MinWidth, PropValue::Unset) => {
            handle.as_framework_element().SetMinWidth(0.0)?;
            Ok(true)
        }
        (Prop::MaxWidth, PropValue::F64(v)) => {
            handle.as_framework_element().SetMaxWidth(*v)?;
            Ok(true)
        }
        (Prop::MaxWidth, PropValue::Unset) => {
            handle.as_framework_element().SetMaxWidth(f64::INFINITY)?;
            Ok(true)
        }
        (Prop::MinHeight, PropValue::F64(v)) => {
            handle.as_framework_element().SetMinHeight(*v)?;
            Ok(true)
        }
        (Prop::MinHeight, PropValue::Unset) => {
            handle.as_framework_element().SetMinHeight(0.0)?;
            Ok(true)
        }
        (Prop::MaxHeight, PropValue::F64(v)) => {
            handle.as_framework_element().SetMaxHeight(*v)?;
            Ok(true)
        }
        (Prop::MaxHeight, PropValue::Unset) => {
            handle.as_framework_element().SetMaxHeight(f64::INFINITY)?;
            Ok(true)
        }
        (Prop::HorizontalAlignment, PropValue::I32(v)) => {
            handle
                .as_framework_element()
                .SetHorizontalAlignment(HorizontalAlignment(*v))?;
            Ok(true)
        }
        (Prop::HorizontalAlignment, PropValue::Unset) => {
            handle
                .as_framework_element()
                .SetHorizontalAlignment(HorizontalAlignment::Stretch)?;
            Ok(true)
        }
        (Prop::VerticalAlignment, PropValue::I32(v)) => {
            handle
                .as_framework_element()
                .SetVerticalAlignment(VerticalAlignment(*v))?;
            Ok(true)
        }
        (Prop::VerticalAlignment, PropValue::Unset) => {
            handle
                .as_framework_element()
                .SetVerticalAlignment(VerticalAlignment::Stretch)?;
            Ok(true)
        }
        (Prop::Opacity, PropValue::F64(v)) => {
            handle.as_ui_element().SetOpacity(*v)?;
            Ok(true)
        }
        (Prop::Opacity, PropValue::Unset) => {
            handle.as_ui_element().SetOpacity(1.0)?;
            Ok(true)
        }
        (Prop::AllowDrop, PropValue::Bool(v)) => {
            handle.as_ui_element().SetAllowDrop(*v)?;
            Ok(true)
        }
        (Prop::AllowDrop, PropValue::Unset) => {
            handle.as_ui_element().SetAllowDrop(false)?;
            Ok(true)
        }
        (Prop::IsEnabled, PropValue::Unset) => {
            handle
                .as_ui_element()
                .cast::<bindings::IControl>()?
                .SetIsEnabled(true)?;
            Ok(true)
        }
        (Prop::Resources, PropValue::Resources(map)) => {
            let rd = handle.as_framework_element().Resources()?;
            let imap =
                rd.cast::<windows_collections::IMap<
                    windows_core::IInspectable,
                    windows_core::IInspectable,
                >>()?;
            for (k, v) in map {
                let key = windows_reference::IReference::from(k.as_str());
                let val = windows_reference::IReference::from(v.as_str());
                imap.Insert(&key, &val)?;
            }
            Ok(true)
        }
        (Prop::AttachedGridRow, PropValue::I32(v)) => {
            bindings::Grid::SetRow(&handle.as_framework_element(), *v)?;
            Ok(true)
        }
        (Prop::AttachedGridColumn, PropValue::I32(v)) => {
            bindings::Grid::SetColumn(&handle.as_framework_element(), *v)?;
            Ok(true)
        }
        (Prop::AttachedGridRowSpan, PropValue::I32(v)) => {
            bindings::Grid::SetRowSpan(&handle.as_framework_element(), *v)?;
            Ok(true)
        }
        (Prop::AttachedGridColumnSpan, PropValue::I32(v)) => {
            bindings::Grid::SetColumnSpan(&handle.as_framework_element(), *v)?;
            Ok(true)
        }
        (Prop::AttachedCanvasLeft, PropValue::F64(v)) => {
            bindings::Canvas::SetLeft(&handle.as_ui_element(), *v)?;
            Ok(true)
        }
        (Prop::AttachedCanvasTop, PropValue::F64(v)) => {
            bindings::Canvas::SetTop(&handle.as_ui_element(), *v)?;
            Ok(true)
        }
        (Prop::AttachedCanvasZIndex, PropValue::I32(v)) => {
            bindings::Canvas::SetZIndex(&handle.as_ui_element(), *v)?;
            Ok(true)
        }
        (Prop::AlignLeftWithPanel, PropValue::Bool(v)) => {
            bindings::RelativePanel::SetAlignLeftWithPanel(&handle.as_ui_element(), *v)?;
            Ok(true)
        }
        (Prop::AlignRightWithPanel, PropValue::Bool(v)) => {
            bindings::RelativePanel::SetAlignRightWithPanel(&handle.as_ui_element(), *v)?;
            Ok(true)
        }
        (Prop::AlignTopWithPanel, PropValue::Bool(v)) => {
            bindings::RelativePanel::SetAlignTopWithPanel(&handle.as_ui_element(), *v)?;
            Ok(true)
        }
        (Prop::AlignBottomWithPanel, PropValue::Bool(v)) => {
            bindings::RelativePanel::SetAlignBottomWithPanel(&handle.as_ui_element(), *v)?;
            Ok(true)
        }
        (Prop::AlignHCenterWithPanel, PropValue::Bool(v)) => {
            bindings::RelativePanel::SetAlignHorizontalCenterWithPanel(
                &handle.as_ui_element(),
                *v,
            )?;
            Ok(true)
        }
        (Prop::AlignVCenterWithPanel, PropValue::Bool(v)) => {
            bindings::RelativePanel::SetAlignVerticalCenterWithPanel(&handle.as_ui_element(), *v)?;
            Ok(true)
        }
        (Prop::Padding, PropValue::Thickness(t)) => set_padding(handle, *t),
        (Prop::Padding, PropValue::Unset) => set_padding(handle, Thickness::default()),
        (Prop::Background, PropValue::Color(br)) => set_background(handle, &solid_brush(*br)?),
        (Prop::Background, PropValue::Unset) => set_background(handle, None::<&bindings::Brush>),
        (Prop::Foreground, PropValue::Color(br)) => set_foreground(handle, &solid_brush(*br)?),
        (Prop::Foreground, PropValue::Unset) => set_foreground(handle, None::<&bindings::Brush>),
        (Prop::Fill, PropValue::Color(b)) => {
            handle
                .cast_inner::<bindings::IShape>()?
                .SetFill(&solid_brush(*b)?)?;
            Ok(true)
        }
        (Prop::Fill, PropValue::Unset) => {
            handle.cast_inner::<bindings::IShape>()?.SetFill(None)?;
            Ok(true)
        }
        (Prop::Stroke, PropValue::Color(b)) => {
            handle
                .cast_inner::<bindings::IShape>()?
                .SetStroke(&solid_brush(*b)?)?;
            Ok(true)
        }
        (Prop::Stroke, PropValue::Unset) => {
            handle.cast_inner::<bindings::IShape>()?.SetStroke(None)?;
            Ok(true)
        }
        (Prop::StrokeThickness, PropValue::F64(v)) => {
            handle
                .cast_inner::<bindings::IShape>()?
                .SetStrokeThickness(*v)?;
            Ok(true)
        }
        (Prop::StrokeThickness, PropValue::Unset) => {
            handle
                .cast_inner::<bindings::IShape>()?
                .SetStrokeThickness(0.0)?;
            Ok(true)
        }
        _ => Ok(false),
    }
}

fn set_padding(handle: &Handle, thickness: Thickness) -> Result<bool> {
    match handle {
        Handle::Border(h) => h.SetPadding(thickness)?,
        Handle::StackPanel(h) => h.SetPadding(thickness)?,
        Handle::TextBlock(h) => h.SetPadding(thickness)?,
        Handle::RichTextBlock(h) => h.SetPadding(thickness)?,
        // `Grid` is a `Panel`, not a `Control`, so it has no `IControl::SetPadding`;
        // its padding lives on the `IGrid` interface instead.
        Handle::Grid(h) => h.cast::<bindings::IGrid>()?.SetPadding(thickness)?,
        _ => {
            if let Ok(ctl) = handle.cast_inner::<bindings::IControl>() {
                ctl.SetPadding(thickness)?;
            } else {
                diag::unhandled_modifier("set_prop", Prop::Padding, handle);
            }
        }
    }
    Ok(true)
}

fn set_background(
    handle: &Handle,
    brush: impl windows_core::Param<bindings::Brush>,
) -> Result<bool> {
    match handle {
        Handle::Border(b) => b.SetBackground(brush)?,
        _ => {
            if let Ok(panel) = handle.cast_inner::<bindings::IPanel>() {
                panel.SetBackground(brush)?;
            } else if let Ok(ctl) = handle.cast_inner::<bindings::IControl>() {
                ctl.SetBackground(brush)?;
            } else {
                diag::unhandled_modifier("set_prop", Prop::Background, handle);
            }
        }
    }
    Ok(true)
}

fn set_foreground(
    handle: &Handle,
    brush: impl windows_core::Param<bindings::Brush>,
) -> Result<bool> {
    match handle {
        Handle::TextBlock(h) => h.SetForeground(brush)?,
        Handle::RichTextBlock(h) => h.SetForeground(brush)?,
        _ => {
            if let Ok(ctl) = handle.cast_inner::<bindings::IControl>() {
                ctl.SetForeground(brush)?;
            } else {
                diag::unhandled_modifier("set_prop", Prop::Foreground, handle);
            }
        }
    }
    Ok(true)
}

fn set_border_brush(
    handle: &Handle,
    brush: impl windows_core::Param<bindings::Brush>,
) -> Result<()> {
    match handle {
        Handle::Border(b) => b.SetBorderBrush(brush)?,
        _ => {
            if let Ok(ctl) = handle.cast_inner::<bindings::IControl>() {
                ctl.SetBorderBrush(brush)?;
            } else {
                diag::unhandled_modifier("set_prop", Prop::BorderBrush, handle);
            }
        }
    }
    Ok(())
}

fn set_border_thickness(handle: &Handle, thickness: Thickness) -> Result<()> {
    match handle {
        Handle::Border(b) => b.SetBorderThickness(thickness)?,
        _ => {
            if let Ok(ctl) = handle.cast_inner::<bindings::IControl>() {
                ctl.SetBorderThickness(thickness)?;
            } else {
                diag::unhandled_modifier("set_prop", Prop::BorderThickness, handle);
            }
        }
    }
    Ok(())
}

fn set_font_f64(handle: &Handle, v: f64) -> Result<bool> {
    match handle {
        Handle::TextBlock(h) => h.SetFontSize(v)?,
        Handle::RichTextBlock(h) => h.SetFontSize(v)?,
        _ => {
            if let Ok(ctl) = handle.cast_inner::<bindings::IControl>() {
                ctl.SetFontSize(v)?;
            } else {
                diag::unhandled_modifier("set_prop", Prop::FontSize, handle);
            }
        }
    }
    Ok(true)
}

fn set_font_weight(handle: &Handle, fw: bindings::FontWeight) -> Result<bool> {
    match handle {
        Handle::TextBlock(h) => h.SetFontWeight(fw)?,
        _ => {
            if let Ok(ctl) = handle.cast_inner::<bindings::IControl>() {
                ctl.SetFontWeight(fw)?;
            } else {
                diag::unhandled_modifier("set_prop", Prop::FontWeight, handle);
            }
        }
    }
    Ok(true)
}

fn set_font_family(handle: &Handle, ff: &bindings::FontFamily) -> Result<bool> {
    match handle {
        Handle::TextBlock(h) => h.SetFontFamily(ff)?,
        Handle::RichTextBlock(h) => h.SetFontFamily(ff)?,
        _ => {
            if let Ok(ctl) = handle.cast_inner::<bindings::IControl>() {
                ctl.SetFontFamily(ff)?;
            } else {
                diag::unhandled_modifier("set_prop", Prop::FontFamily, handle);
            }
        }
    }
    Ok(true)
}

/// Populate an existing `IVector<IInspectable>` with string items (clear + append).
fn set_str_items(
    vec: &windows_collections::IVector<windows_core::IInspectable>,
    items: &[String],
) -> Result<()> {
    vec.Clear()?;
    for s in items {
        let insp = windows_reference::IReference::from(s.as_str());
        vec.Append(&insp)?;
    }
    Ok(())
}

/// Build an `IVector<IInspectable>` from a string list (for `put_ItemsSource`).
fn str_list_as_ivector(
    items: &[String],
) -> windows_collections::IVector<windows_core::IInspectable> {
    let vec: Vec<Option<windows_core::IInspectable>> = items
        .iter()
        .map(|s| Some(windows_reference::IReference::from(s.as_str()).into()))
        .collect();
    vec.into()
}

/// Boxes an index as an `IReference<i32>` for a virtualized list's
/// `ItemsSource`. The concrete value is irrelevant to rendering (rows are
/// driven by container position), but distinct boxed indices let a
/// drag-reorder be read back as a permutation.
fn box_index(i: usize) -> windows_core::IInspectable {
    windows_reference::IReference::<i32>::from(i as i32).into()
}

/// Reads an index previously stored by [`box_index`] back out, or `None` if
/// the value isn't the expected boxed `i32`.
fn unbox_index(value: &windows_core::IInspectable) -> Option<usize> {
    let r = value.cast::<windows_reference::IReference<i32>>().ok()?;
    usize::try_from(r.Value().ok()?).ok()
}

/// The XAML for the shared virtualization item template: a bare
/// `ContentControl` stretched to fill its container. We populate its `Content`
/// per row with a reactor-built element.
const CONTENT_TEMPLATE_XAML: &str = "<DataTemplate xmlns='http://schemas.microsoft.com/winfx/2006/xaml/presentation'><ContentControl HorizontalContentAlignment='Stretch' VerticalContentAlignment='Stretch'/></DataTemplate>";

impl Backend for WinUIBackend {
    fn create(&mut self, kind: ControlKind) -> ControlId {
        let id = self.alloc_id();
        let handle = Self::make_handle_for_kind(kind);
        self.controls.borrow_mut().insert(id, handle);
        id
    }
    fn set_prop(&mut self, id: ControlId, prop: Prop, value: &PropValue) {
        let map = self.controls.borrow();
        let handle = map
            .get(&id)
            .unwrap_or_else(|| panic!("WinUIBackend::set_prop: unknown control {id}"));
        let result: Result<()> = (|| -> Result<()> {
            if generated_set_prop::dispatch(handle, prop, value)? {
                return Ok(());
            }
            if try_universal_prop(handle, prop, value)? {
                return Ok(());
            }
            match (prop, value, handle) {
                (Prop::IsTextSelectionEnabled, PropValue::Bool(v), Handle::RichTextBlock(tb)) => {
                    tb.SetIsTextSelectionEnabled(*v)
                }
                (Prop::IsTextSelectionEnabled, PropValue::Unset, Handle::RichTextBlock(tb)) => {
                    tb.SetIsTextSelectionEnabled(false)
                }
                (Prop::TextWrappingWrap, PropValue::I32(v), Handle::RichTextBlock(tb)) => {
                    tb.SetTextWrapping(TextWrapping(*v))
                }
                (Prop::Content, PropValue::Str(s), Handle::Button(b)) => {
                    let cc = b.cast::<bindings::IContentControl>()?;
                    // If the button has an icon+text layout (StackPanel from
                    // Icon), update just the TextBlock child so the icon
                    // is preserved when only the label changes.
                    if let Ok(existing) = cc.Content()
                        && let Ok(panel) = existing.cast::<bindings::IPanel>()
                    {
                        let children = panel.Children()?;
                        if children.Size()? >= 2
                            && let Ok(tb) = children.GetAt(1)?.cast::<bindings::ITextBlock>()
                        {
                            return tb.SetText(s);
                        }
                    }
                    let tb = string_as_textblock(s)?;
                    cc.SetContent(&tb)
                }
                (Prop::Icon, PropValue::Icon(icon), Handle::Button(b)) => {
                    let icon_elem = build_icon_element(icon)?;
                    let cc = b.cast::<bindings::IContentControl>()?;
                    // If the button already has an icon+text StackPanel layout,
                    // replace just the icon child (index 0) to preserve the text.
                    if let Ok(existing) = cc.Content()
                        && let Ok(panel) = existing.cast::<bindings::IPanel>()
                    {
                        let children = panel.Children()?;
                        if children.Size()? >= 2 {
                            children.SetAt(0, &icon_elem.cast::<bindings::UIElement>()?)?;
                            return Ok(());
                        }
                    }
                    let use_icon_only = if let Ok(existing) = cc.Content() {
                        // Already in icon-only mode (existing content is an IconElement).
                        existing.cast::<bindings::IIconElement>().is_ok()
                            || existing
                                .cast::<bindings::ITextBlock>()
                                .ok()
                                .and_then(|tb| tb.Text().ok())
                                .is_some_and(|t| t.is_empty())
                    } else {
                        true
                    };
                    if use_icon_only {
                        cc.SetContent(&icon_elem)
                    } else {
                        let panel = bindings::StackPanel::new()?;
                        panel.SetOrientation(Orientation::Horizontal)?;
                        panel.SetSpacing(8.0)?;
                        let children = panel.cast::<bindings::IPanel>()?.Children()?;
                        children.Append(&icon_elem.cast::<bindings::UIElement>()?)?;
                        if let Ok(existing) = cc.Content()
                            && let Ok(ui) = existing.cast::<bindings::UIElement>()
                        {
                            children.Append(&ui)?;
                        }
                        cc.SetContent(&panel)
                    }
                }
                (Prop::Icon, PropValue::Unset, Handle::Button(b)) => {
                    let cc = b.cast::<bindings::IContentControl>()?;
                    let Ok(existing) = cc.Content() else {
                        return Ok(());
                    };
                    // icon+text StackPanel layout: unwrap back to just the text
                    // child (index 1), discarding the icon.
                    if let Ok(panel) = existing.cast::<bindings::IPanel>() {
                        let children = panel.Children()?;
                        if children.Size()? >= 2 {
                            let text_child = children.GetAt(1)?;
                            children.Clear()?;
                            return cc.SetContent(&text_child);
                        }
                    }
                    // icon-only layout: clear the content entirely.
                    if existing.cast::<bindings::IIconElement>().is_ok() {
                        return cc.SetContent(None::<&windows_core::IInspectable>);
                    }
                    Ok(())
                }
                (Prop::StyleVariant, PropValue::I32(v), Handle::Button(b)) => {
                    let fe = b.cast::<bindings::IFrameworkElement>()?;
                    let style_key = match *v {
                        1 => Some("AccentButtonStyle"),
                        2 => Some("SubtleButtonStyle"),
                        3 => Some("TextBlockButtonStyle"),
                        _ => None, // 0 = Default
                    };
                    if let Some(key_str) = style_key {
                        let resources =
                            bindings::Application::Current().and_then(|app| app.Resources())?;
                        let key = windows_reference::IReference::from(windows_core::HSTRING::from(
                            key_str,
                        ));
                        let map = resources.cast::<windows_collections::IMap<
                            windows_core::IInspectable,
                            windows_core::IInspectable,
                        >>()?;
                        if let Ok(style_obj) = map.Lookup(&key)
                            && let Ok(s) = style_obj.cast::<bindings::Style>()
                        {
                            fe.SetStyle(&s)?;
                        }
                    } else {
                        fe.SetStyle(None)?;
                    }
                    Ok(())
                }
                (Prop::Value, PropValue::Str(s), Handle::TextBox(t)) => {
                    if t.Text().ok().as_deref() == Some(s.as_str()) {
                        return Ok(());
                    }
                    t.SetText(s.as_str())
                }
                (Prop::GridRows, PropValue::GridLengths(rows), Handle::Grid(g)) => {
                    let defs = g.RowDefinitions()?;
                    defs.Clear()?;
                    for r in rows {
                        let rd = bindings::RowDefinition::new()?;
                        rd.SetHeight(to_xaml_gridlength(*r)?)?;
                        defs.Append(&rd)?;
                    }
                    Ok(())
                }
                (Prop::GridColumns, PropValue::GridLengths(cols), Handle::Grid(g)) => {
                    let defs = g.ColumnDefinitions()?;
                    defs.Clear()?;
                    for c in cols {
                        let cd = bindings::ColumnDefinition::new()?;
                        cd.SetWidth(to_xaml_gridlength(*c)?)?;
                        defs.Append(&cd)?;
                    }
                    Ok(())
                }
                (Prop::Step, PropValue::F64(v), Handle::Slider(s)) => {
                    s.SetStepFrequency(*v)?;
                    s.cast::<bindings::IRangeBase>()?.SetSmallChange(*v)
                }
                (Prop::Step, PropValue::Unset, Handle::Slider(s)) => {
                    s.SetStepFrequency(1.0)?;
                    s.cast::<bindings::IRangeBase>()?.SetSmallChange(1.0)
                }
                (Prop::NavigateUri, PropValue::Str(s), Handle::HyperlinkButton(h)) => {
                    let uri = bindings::Uri::CreateUri(s.as_str())?;
                    h.SetNavigateUri(&uri)
                }
                (Prop::NavigateUri, PropValue::Unset, Handle::HyperlinkButton(h)) => {
                    h.SetNavigateUri(None)
                }
                (Prop::IsClosable, PropValue::Bool(v), Handle::TabViewItem(ti)) => {
                    ti.SetIsClosable(*v)
                }
                // ContentDialog — modal popup hosted via ShowAsync.
                (Prop::IsOpen, PropValue::Bool(v), Handle::ContentDialog(d)) => {
                    if *v {
                        // ContentDialog needs a XamlRoot before ShowAsync; reuse
                        // any other live UIElement's (the dialog isn't in the tree).
                        let xroot = self
                            .controls
                            .borrow()
                            .values()
                            .filter_map(|h| match h {
                                Handle::ContentDialog(_) => None,
                                other => other
                                    .as_ui_element()
                                    .cast::<bindings::IUIElement>()
                                    .ok()
                                    .and_then(|u| u.XamlRoot().ok()),
                            })
                            .next();
                        match xroot {
                            Some(root) => {
                                diag::dropped(d.cast::<bindings::IUIElement>()?.SetXamlRoot(&root));
                                diag::dropped(d.ShowAsync());
                            }
                            None => {
                                diag::warn(format_args!(
                                    "ContentDialog.is_open ignored — no XamlRoot available"
                                ));
                            }
                        }
                        Ok(())
                    } else {
                        d.Hide()
                    }
                }
                (Prop::Value, PropValue::I32(v), Handle::InfoBadge(ib)) => {
                    if *v < 0 {
                        ib.SetValue(-1)
                    } else {
                        ib.SetValue(*v)
                    }
                }
                (Prop::DisplayName, PropValue::Unset, Handle::PersonPicture(p)) => {
                    p.SetDisplayName("")
                }
                (Prop::Initials, PropValue::Unset, Handle::PersonPicture(p)) => p.SetInitials(""),
                (Prop::CornerRadius, PropValue::F64(v), Handle::Rectangle(r)) => {
                    r.SetRadiusX(*v).and_then(|_| r.SetRadiusY(*v))
                }
                (Prop::CornerRadius, PropValue::Unset, Handle::Rectangle(r)) => {
                    r.SetRadiusX(0.0).and_then(|_| r.SetRadiusY(0.0))
                }
                (Prop::CornerRadius, PropValue::F64(v), Handle::Border(b)) => {
                    b.SetCornerRadius(bindings::CornerRadius {
                        top_left: *v,
                        top_right: *v,
                        bottom_right: *v,
                        bottom_left: *v,
                    })
                }
                (Prop::CornerRadius, PropValue::Unset, Handle::Border(b)) => {
                    b.SetCornerRadius(bindings::CornerRadius::default())
                }
                (Prop::BorderBrush, PropValue::Color(br), h) => {
                    set_border_brush(h, &solid_brush(*br)?)
                }
                (Prop::BorderBrush, PropValue::Unset, h) => {
                    set_border_brush(h, None::<&bindings::Brush>)
                }
                (Prop::BorderThickness, PropValue::Thickness(t), h) => set_border_thickness(h, *t),
                (Prop::BorderThickness, PropValue::Unset, h) => {
                    set_border_thickness(h, Thickness::default())
                }
                (Prop::LineEndpoints, PropValue::LineEndpoints(p), Handle::Line(l)) => l
                    .SetX1(p.x1)
                    .and_then(|_| l.SetY1(p.y1))
                    .and_then(|_| l.SetX2(p.x2))
                    .and_then(|_| l.SetY2(p.y2)),
                (Prop::ImageSource, PropValue::Str(s), Handle::Image(img)) => {
                    let uri = bindings::Uri::CreateUri(s.as_str())?;
                    let bmp = bindings::BitmapImage::new()?;
                    bmp.SetUriSource(&uri)?;
                    img.SetSource(&bmp.cast::<bindings::ImageSource>()?)
                }
                (Prop::ImageSource, PropValue::SurfaceImageSource(sis), Handle::Image(img)) => {
                    img.SetSource(&sis.image_source()?)
                }
                (Prop::ImageSource, PropValue::Unset, Handle::Image(img)) => img.SetSource(None),
                (Prop::Header, PropValue::Str(s), Handle::TabViewItem(ti)) => {
                    let tb = string_as_textblock(s)?;
                    ti.SetHeader(&tb)
                }
                (Prop::Header, PropValue::Str(s), Handle::Expander(e)) => {
                    let tb = string_as_textblock(s)?;
                    e.SetHeader(&tb)
                }
                (Prop::Header, PropValue::Unset, Handle::Expander(e)) => e.SetHeader(None),
                (Prop::ItemKey, PropValue::Str(s), Handle::TabViewItem(ti)) => {
                    let tag = windows_reference::IReference::from(s.as_str());
                    ti.cast::<bindings::IFrameworkElement>()?.SetTag(&tag)
                }
                (Prop::MenuItems, PropValue::NavMenuItems(items), Handle::NavigationView(nv)) => {
                    let menu = nv.MenuItems()?;
                    menu.Clear()?;
                    for item in items {
                        let nv_item = build_nav_view_item(item)?;
                        menu.Append(&nv_item)?;
                    }
                    Ok(())
                }
                (Prop::SelectedTag, PropValue::Str(tag), Handle::NavigationView(nv)) => {
                    select_nav_item_by_tag(nv, tag)
                }
                (Prop::SelectedTag, PropValue::Unset, Handle::NavigationView(nv)) => {
                    nv.SetSelectedItem(None)
                }
                (Prop::AutoSuggestBox, PropValue::Bool(true), Handle::NavigationView(nv)) => {
                    let asb = bindings::AutoSuggestBox::new()?;
                    nv.SetAutoSuggestBox(&asb)
                }
                (Prop::AutoSuggestBox, PropValue::Bool(false), Handle::NavigationView(nv)) => {
                    nv.SetAutoSuggestBox(None)
                }
                (Prop::AutoSuggestPlaceholder, PropValue::Str(s), Handle::NavigationView(nv)) => {
                    if let Ok(asb) = nv.AutoSuggestBox() {
                        asb.SetPlaceholderText(s.as_str())?;
                    }
                    Ok(())
                }
                (Prop::AutoSuggestItems, PropValue::StrList(items), Handle::NavigationView(nv)) => {
                    if let Ok(asb) = nv.AutoSuggestBox() {
                        asb.cast::<bindings::IItemsControl>()?
                            .SetItemsSource(&str_list_as_ivector(items))?;
                    }
                    Ok(())
                }
                (Prop::Tall, PropValue::Bool(v), Handle::TitleBar(_)) => {
                    if let Some(state) = self.window_state.borrow().as_ref() {
                        state.set_titlebar_height(*v);
                    }
                    Ok(())
                }
                (Prop::IsBackButtonVisible, PropValue::Bool(v), Handle::NavigationView(nv)) => {
                    let val = if *v {
                        bindings::NavigationViewBackButtonVisible::Auto
                    } else {
                        bindings::NavigationViewBackButtonVisible::Collapsed
                    };
                    nv.cast::<bindings::INavigationView2>()?
                        .SetIsBackButtonVisible(val)
                }
                (Prop::ItemHeader, PropValue::Str(s), Handle::PivotItem(pi)) => {
                    let tb = string_as_textblock(s)?;
                    pi.SetHeader(&tb)
                }
                (Prop::Items, PropValue::StrList(items), Handle::BreadcrumbBar(bc)) => {
                    bc.SetItemsSource(&str_list_as_ivector(items))
                }
                (Prop::Value, PropValue::Str(s), Handle::PasswordBox(p)) => {
                    if p.Password().ok().as_deref() == Some(s.as_str()) {
                        return Ok(());
                    }
                    p.SetPassword(s.as_str())
                }
                (Prop::Value, PropValue::Unset, Handle::PasswordBox(p)) => p.SetPassword(""),
                (Prop::Items, PropValue::StrList(items), Handle::RadioButtons(r)) => {
                    set_str_items(&r.Items()?.cast()?, items)
                }
                (Prop::Items, PropValue::StrList(items), Handle::ComboBox(c)) => set_str_items(
                    &c.cast::<bindings::IItemsControl>()?.Items()?.cast()?,
                    items,
                ),
                (Prop::ColorValue, PropValue::Color(c), Handle::ColorPicker(cp)) => cp.SetColor(*c),
                (Prop::Items, PropValue::StrList(items), Handle::ListBox(lb)) => set_str_items(
                    &lb.cast::<bindings::IItemsControl>()?.Items()?.cast()?,
                    items,
                ),
                (Prop::Text, PropValue::Str(s), Handle::AutoSuggestBox(asb)) => {
                    // Skip SetText when the control already has this value —
                    // calling SetText during a user-initiated TextChanged
                    // cycle steals focus from the input field.
                    if asb.Text().ok().as_deref() == Some(s.as_str()) {
                        return Ok(());
                    }
                    asb.SetText(s)
                }
                (Prop::Items, PropValue::StrList(items), Handle::AutoSuggestBox(asb)) => asb
                    .cast::<bindings::IItemsControl>()?
                    .SetItemsSource(&str_list_as_ivector(items)),
                (Prop::DisplayMode, PropValue::I32(m), Handle::SplitView(sv)) => {
                    sv.SetDisplayMode(bindings::SplitViewDisplayMode(*m))
                }
                (Prop::Items, PropValue::MenuBarItems(items), Handle::MenuBar(mb)) => {
                    let winui_items = mb.Items()?;
                    winui_items.Clear()?;
                    for bar_item_def in items {
                        let mbi = bindings::MenuBarItem::new()?;
                        mbi.SetTitle(&bar_item_def.title)?;
                        let flyout_items = mbi.Items()?;
                        for menu_def in &bar_item_def.items {
                            let fi = build_menu_flyout_item_base(menu_def)?;
                            flyout_items.Append(&fi)?;
                        }
                        winui_items.Append(&mbi)?;
                    }
                    let handlers = self.menu_click_handlers.borrow();
                    if let Some(handler) = handlers.get(&id) {
                        let revs = Self::wire_menu_bar_clicks(mb, handler);
                        if !revs.is_empty() {
                            self.event_revokers
                                .borrow_mut()
                                .insert((id, Event::ItemClicked), revs);
                        }
                    }
                    Ok(())
                }
                (
                    Prop::MenuFlyoutItems,
                    PropValue::MenuFlyoutItems(items),
                    Handle::DropDownButton(btn),
                ) => {
                    let flyout = bindings::MenuFlyout::new()?;
                    let flyout_items = flyout.Items()?;
                    for def in items {
                        let fi = build_menu_flyout_item_base(def)?;
                        flyout_items.Append(&fi)?;
                    }
                    btn.cast::<bindings::IButton>()?.SetFlyout(&flyout)?;
                    let handlers = self.menu_click_handlers.borrow();
                    if let Some(handler) = handlers.get(&id) {
                        let revs = Self::wire_flyout_clicks(&flyout, handler);
                        if !revs.is_empty() {
                            self.event_revokers
                                .borrow_mut()
                                .insert((id, Event::ItemClicked), revs);
                        }
                    }
                    Ok(())
                }
                (Prop::MenuFlyoutItems, PropValue::MenuFlyoutItems(items), Handle::Button(btn)) => {
                    let flyout = bindings::MenuFlyout::new()?;
                    let flyout_items = flyout.Items()?;
                    for def in items {
                        let fi = build_menu_flyout_item_base(def)?;
                        flyout_items.Append(&fi)?;
                    }
                    btn.SetFlyout(&flyout)?;
                    let handlers = self.menu_click_handlers.borrow();
                    if let Some(handler) = handlers.get(&id) {
                        let revs = Self::wire_flyout_clicks(&flyout, handler);
                        if !revs.is_empty() {
                            self.event_revokers
                                .borrow_mut()
                                .insert((id, Event::ItemClicked), revs);
                        }
                    }
                    Ok(())
                }
                (
                    Prop::CommandBarFlyoutCommands,
                    PropValue::CommandBarFlyoutDef { primary, secondary },
                    Handle::Button(btn),
                ) => {
                    let flyout = bindings::CommandBarFlyout::new()?;
                    let primary_cmds = flyout.PrimaryCommands()?;
                    let secondary_cmds = flyout.SecondaryCommands()?;
                    for def in primary {
                        let el = build_command_bar_element(def)?;
                        primary_cmds.Append(&el)?;
                    }
                    for def in secondary {
                        let el = build_command_bar_element(def)?;
                        secondary_cmds.Append(&el)?;
                    }
                    btn.SetFlyout(&flyout)?;
                    let handlers = self.command_bar_flyout_handlers.borrow();
                    if let Some(handler) = handlers.get(&id) {
                        let mut revs = Self::wire_command_bar_clicks(&primary_cmds, handler);
                        revs.extend(Self::wire_command_bar_clicks(&secondary_cmds, handler));
                        if !revs.is_empty() {
                            self.event_revokers
                                .borrow_mut()
                                .insert((id, Event::Click), revs);
                        }
                    }
                    Ok(())
                }
                (Prop::Nodes, PropValue::TreeViewNodes(nodes), Handle::TreeView(tv)) => {
                    let root = tv.RootNodes()?;
                    root.Clear()?;
                    for node_def in nodes {
                        let node = build_tree_view_node(node_def)?;
                        root.Append(&node)?;
                    }
                    Ok(())
                }
                (
                    Prop::PrimaryCommands,
                    PropValue::CommandBarCommands(cmds),
                    Handle::CommandBar(cb),
                ) => {
                    let primary = cb.PrimaryCommands()?;
                    primary.Clear()?;
                    for def in cmds {
                        let el = build_command_bar_element(def)?;
                        primary.Append(&el)?;
                    }
                    let handlers = self.menu_click_handlers.borrow();
                    if let Some(handler) = handlers.get(&id) {
                        let revs = Self::wire_command_bar_clicks(&primary, handler);
                        if !revs.is_empty() {
                            self.event_revokers
                                .borrow_mut()
                                .insert((id, Event::Click), revs);
                        }
                    }
                    Ok(())
                }
                (
                    Prop::SecondaryCommands,
                    PropValue::CommandBarCommands(cmds),
                    Handle::CommandBar(cb),
                ) => {
                    let secondary = cb.SecondaryCommands()?;
                    secondary.Clear()?;
                    for def in cmds {
                        let el = build_command_bar_element(def)?;
                        secondary.Append(&el)?;
                    }
                    let handlers = self.menu_click_handlers.borrow();
                    if let Some(handler) = handlers.get(&id) {
                        let revs = Self::wire_command_bar_clicks(&secondary, handler);
                        if !revs.is_empty() {
                            let mut rev_map = self.event_revokers.borrow_mut();
                            rev_map.entry((id, Event::Click)).or_default().extend(revs);
                        }
                    }
                    Ok(())
                }
                (Prop::ActionButton, PropValue::Str(s), Handle::TeachingTip(tt)) => {
                    let boxed: windows_core::IInspectable =
                        windows_reference::IReference::<windows_core::HSTRING>::from(
                            windows_core::HSTRING::from(s.as_str()),
                        )
                        .cast()?;
                    tt.SetActionButtonContent(&boxed)
                }
                (Prop::CloseButton, PropValue::Str(s), Handle::TeachingTip(tt)) => {
                    let boxed: windows_core::IInspectable =
                        windows_reference::IReference::<windows_core::HSTRING>::from(
                            windows_core::HSTRING::from(s.as_str()),
                        )
                        .cast()?;
                    tt.SetCloseButtonContent(&boxed)
                }
                (Prop::Items, PropValue::SelectorBarItems(items), Handle::SelectorBar(sb)) => {
                    let vec = sb.Items()?;
                    vec.Clear()?;
                    for def in items {
                        let item = bindings::SelectorBarItem::new()?;
                        item.SetText(&def.text)?;
                        if let Some(icon) = &def.icon {
                            let icon_elem = build_icon_element(icon)?;
                            item.SetIcon(&icon_elem)?;
                        }
                        vec.Append(&item)?;
                    }
                    Ok(())
                }
                (Prop::Text, PropValue::Str(s), Handle::RichEditBox(reb)) => {
                    let doc = reb.Document()?;
                    let mut current = windows_core::HSTRING::default();
                    doc.GetText(bindings::TextGetOptions::None, &mut current)
                        .ok();
                    if current == s.as_str() {
                        return Ok(());
                    }
                    doc.SetText(bindings::TextSetOptions::None, s.as_str())
                }
                (Prop::Header, PropValue::Str(s), Handle::RichEditBox(reb)) => {
                    let tb = string_as_textblock(s)?;
                    reb.SetHeader(&tb)
                }
                (Prop::Header, PropValue::Unset, Handle::RichEditBox(reb)) => reb.SetHeader(None),
                (Prop::FlyoutContent, PropValue::Str(s), Handle::Button(b)) => {
                    let flyout = bindings::Flyout::new()?;
                    let tb = string_as_textblock(s)?;
                    flyout.SetContent(&tb)?;
                    b.SetFlyout(&flyout)?;
                    Ok(())
                }
                (Prop::FlyoutPlacement, PropValue::I32(v), Handle::Button(b)) => {
                    // The flyout must already exist (FlyoutContent set first).
                    if let Ok(fb) = b.Flyout() {
                        diag::dropped(
                            fb.cast::<bindings::IFlyoutBase>()?
                                .SetPlacement(FlyoutPlacementMode(*v)),
                        );
                    }
                    Ok(())
                }
                (_, PropValue::Unset, _) => Ok(()),
                (p, v, h) => {
                    diag::unhandled_prop(id, p, v, h);
                    Ok(())
                }
            }
        })();
        if let Err(e) = result {
            diag::warn(format_args!("set_prop on {id}: {e:?}"));
        }
    }
    fn append_child(&mut self, parent: ControlId, child: ControlId) {
        self.parent_children
            .borrow_mut()
            .entry(parent)
            .or_default()
            .push(child);
        if self.is_phantom_child(child) {
            return;
        }
        let map = self.controls.borrow();
        let parent_h = map
            .get(&parent)
            .unwrap_or_else(|| panic!("WinUIBackend::append_child: unknown parent {parent}"));
        let child_h = map
            .get(&child)
            .unwrap_or_else(|| panic!("WinUIBackend::append_child: unknown child {child}"));
        let child_ui = child_h.as_ui_element();
        let cc = classify_container(parent_h).unwrap_or_else(|| {
            panic!(
                "WinUIBackend::append_child: {} ({parent}) is not a container",
                parent_h.kind_name()
            )
        });
        container_append(&cc, &child_ui);
    }
    fn remove_child(&mut self, parent: ControlId, index: usize) {
        // Phantom children (e.g. ContentDialog) are never in the parent's
        // visual `Children`; skip the collection mutation for them.
        let phantom = self
            .parent_children
            .borrow()
            .get(&parent)
            .and_then(|v| v.get(index).copied())
            .is_some_and(|cid| self.is_phantom_child(cid));
        let v_index = self.visual_index(parent, index);
        if let Some(list) = self.parent_children.borrow_mut().get_mut(&parent)
            && index < list.len()
        {
            list.remove(index);
        }
        if phantom {
            return;
        }
        let map = self.controls.borrow();
        let parent_h = map
            .get(&parent)
            .unwrap_or_else(|| panic!("WinUIBackend::remove_child: unknown parent {parent}"));
        let cc = classify_container(parent_h)
            .unwrap_or_else(|| panic!("WinUIBackend::remove_child: {parent} is not a container"));
        container_remove(&cc, v_index);
    }
    fn replace_child(&mut self, parent: ControlId, index: usize, new: ControlId) {
        let old = self
            .parent_children
            .borrow()
            .get(&parent)
            .and_then(|v| v.get(index).copied());
        let old_phantom = old.is_some_and(|c| self.is_phantom_child(c));
        let new_phantom = self.is_phantom_child(new);
        let v_index = self.visual_index(parent, index);
        if let Some(list) = self.parent_children.borrow_mut().get_mut(&parent)
            && index < list.len()
        {
            list[index] = new;
        }
        match (old_phantom, new_phantom) {
            // Both invisible — nothing to do on the visual tree.
            (true, true) => {}
            // real → phantom: drop the old visual child, leaving siblings shifted.
            (false, true) => self.visual_remove_at(parent, v_index),
            // phantom → real: insert the new visual child at the slot the
            // phantom would have occupied if it were real.
            (true, false) => self.visual_insert_at(parent, v_index, new),
            // real → real: in-place SetAt.
            (false, false) => self.visual_set_at(parent, v_index, new),
        }
    }
    fn move_child(&mut self, parent: ControlId, from: usize, to: usize) {
        if from == to {
            return;
        }
        // Compute visual indices before mutating the mirror so phantoms
        // are counted against the pre-move state.
        let v_from = self.visual_index(parent, from);
        let v_to = self.visual_index(parent, to);
        let moved_phantom = self
            .parent_children
            .borrow()
            .get(&parent)
            .and_then(|v| v.get(from).copied())
            .is_some_and(|cid| self.is_phantom_child(cid));
        if let Some(list) = self.parent_children.borrow_mut().get_mut(&parent)
            && from < list.len()
            && to < list.len()
        {
            let item = list.remove(from);
            list.insert(to, item);
        }
        if moved_phantom || v_from == v_to {
            return;
        }
        let map = self.controls.borrow();
        let parent_h = map
            .get(&parent)
            .unwrap_or_else(|| panic!("WinUIBackend::move_child: unknown parent {parent}"));
        let cc = classify_container(parent_h)
            .unwrap_or_else(|| panic!("WinUIBackend::move_child: {parent} is not a container"));
        container_move(&cc, v_from, v_to);
    }
    fn insert_child(&mut self, parent: ControlId, index: usize, child: ControlId) {
        let v_index = self.visual_index(parent, index);
        {
            let mut kids = self.parent_children.borrow_mut();
            let list = kids.entry(parent).or_default();
            let clamped = index.min(list.len());
            list.insert(clamped, child);
        }
        if self.is_phantom_child(child) {
            return;
        }
        let map = self.controls.borrow();
        let parent_h = map
            .get(&parent)
            .unwrap_or_else(|| panic!("WinUIBackend::insert_child: unknown parent {parent}"));
        let child_h = map
            .get(&child)
            .unwrap_or_else(|| panic!("WinUIBackend::insert_child: unknown child {child}"));
        let child_ui = child_h.as_ui_element();
        let cc = classify_container(parent_h)
            .unwrap_or_else(|| panic!("WinUIBackend::insert_child: {parent} is not a container"));
        container_insert(&cc, v_index, &child_ui);
    }
    fn set_templated_item_count(&mut self, id: ControlId, count: usize) {
        let map = self.controls.borrow();
        let Some(handle) = map.get(&id) else { return };
        // Only ListView/GridView virtualize through an ItemsSource; FlipView
        // keeps the Items-materialization path in set_templated_row_content.
        let items_control: bindings::IItemsControl = match handle {
            Handle::ListView(lv) => lv.cast().unwrap(),
            Handle::GridView(gv) => gv.cast().unwrap(),
            _ => return,
        };

        let mut lists = self.templated.borrow_mut();
        let entry = lists.entry(id).or_insert_with(TemplatedList::new);
        let source_slot = &entry.shared.source;

        let mut slot = source_slot.borrow_mut();
        match slot.as_ref() {
            None => {
                // First sizing: install the shared item template (so realized
                // containers host a real `ContentControl` we can populate),
                // then build the observable source and hand it to WinUI, which
                // lazily realizes containers as rows scroll into view.
                diag::dropped(items_control.SetItemTemplate(&self.content_template()));
                let values: Vec<Option<windows_core::IInspectable>> =
                    (0..count).map(|i| Some(box_index(i))).collect();
                let source: windows_collections::IObservableVector<windows_core::IInspectable> =
                    values.into();
                diag::dropped(items_control.SetItemsSource(&source));
                *slot = Some(source);
            }
            Some(source) => {
                // Resize in place so existing containers/selection survive and
                // WinUI realizes/recycles only the delta.
                let current = source.Size().unwrap_or(0) as usize;
                if count > current {
                    for i in current..count {
                        diag::dropped(source.Append(&box_index(i)));
                    }
                } else {
                    for _ in count..current {
                        diag::dropped(source.RemoveAtEnd());
                    }
                }
            }
        }
    }
    fn set_templated_row_content(
        &mut self,
        list_id: ControlId,
        row_idx: usize,
        content: Option<ControlId>,
    ) {
        let map = self.controls.borrow();
        let list_h = map
            .get(&list_id)
            .unwrap_or_else(|| panic!("set_templated_row_content: unknown list {list_id}"));
        let content_ui = content.and_then(|c| map.get(&c).map(Handle::as_ui_element));

        // ListView/GridView are virtualized: place the element into the item
        // container WinUI prepared for this row (recorded on the realize event).
        // FlipView is not virtualized and keeps the Items-materialization path.
        match list_h {
            Handle::ListView(_) | Handle::GridView(_) => {
                let container = self
                    .templated
                    .borrow()
                    .get(&list_id)
                    .and_then(|t| t.shared.containers.borrow().get(&row_idx).cloned());
                let Some(container) = container else { return };
                match content_ui {
                    Some(ui) => diag::dropped(container.SetContent(&ui)),
                    None => {
                        diag::dropped(container.SetContent(None::<&windows_core::IInspectable>));
                    }
                }
                return;
            }
            Handle::FlipView(_) => {}
            other => panic!(
                "set_templated_row_content: {} is not a templated list",
                describe_kind(other)
            ),
        }

        let items_control: bindings::IItemsControl = match list_h {
            Handle::FlipView(fv) => fv.cast().unwrap(),
            _ => unreachable!(),
        };
        let items = items_control
            .Items()
            .unwrap()
            .cast::<windows_collections::IVector<windows_core::IInspectable>>()
            .unwrap();
        let current_len = items.Size().unwrap() as usize;
        match content_ui {
            Some(ui) => {
                let insp: windows_core::IInspectable = ui.cast().unwrap();
                if row_idx < current_len {
                    items.SetAt(row_idx as u32, &insp).unwrap();
                } else {
                    while (items.Size().unwrap() as usize) < row_idx {
                        let pad: windows_core::IInspectable =
                            bindings::TextBlock::new().unwrap().cast().unwrap();
                        items.Append(&pad).unwrap();
                    }
                    items.Append(&insp).unwrap();
                }
            }
            None => {
                if row_idx < current_len {
                    items.RemoveAt(row_idx as u32).unwrap();
                }
            }
        }
    }
    fn set_templated_selected_index(&mut self, id: ControlId, index: i32) {
        let map = self.controls.borrow();
        let Some(handle) = map.get(&id) else { return };
        let selector: bindings::ISelector = match handle {
            Handle::ListView(lv) => lv.cast().unwrap(),
            Handle::GridView(gv) => gv.cast().unwrap(),
            Handle::FlipView(fv) => fv.cast().unwrap(),
            _ => return,
        };
        diag::dropped(selector.SetSelectedIndex(index));
    }

    fn set_templated_selection_mode(&mut self, id: ControlId, mode: SelectionMode) {
        let map = self.controls.borrow();
        let Some(handle) = map.get(&id) else { return };
        let lvb: bindings::IListViewBase = match handle {
            Handle::ListView(lv) => lv.cast().unwrap(),
            Handle::GridView(gv) => gv.cast().unwrap(),
            // FlipView doesn't support SelectionMode.
            _ => return,
        };
        use SelectionMode;
        let winui_mode = match mode {
            SelectionMode::None => bindings::ListViewSelectionMode::None,
            SelectionMode::Single => bindings::ListViewSelectionMode::Single,
            SelectionMode::Multiple => bindings::ListViewSelectionMode::Multiple,
            SelectionMode::Extended => bindings::ListViewSelectionMode::Extended,
        };
        diag::dropped(lvb.SetSelectionMode(winui_mode));
    }

    fn set_templated_can_drag_items(&mut self, id: ControlId, value: bool) {
        let map = self.controls.borrow();
        let Some(handle) = map.get(&id) else { return };
        let lvb: bindings::IListViewBase = match handle {
            Handle::ListView(lv) => lv.cast().unwrap(),
            Handle::GridView(gv) => gv.cast().unwrap(),
            _ => return,
        };
        diag::dropped(lvb.SetCanDragItems(value));
    }

    fn set_templated_can_reorder_items(&mut self, id: ControlId, value: bool) {
        let map = self.controls.borrow();
        let Some(handle) = map.get(&id) else { return };
        let lvb: bindings::IListViewBase = match handle {
            Handle::ListView(lv) => lv.cast().unwrap(),
            Handle::GridView(gv) => gv.cast().unwrap(),
            _ => return,
        };
        diag::dropped(lvb.SetCanReorderItems(value));
    }

    fn set_templated_allow_drop(&mut self, id: ControlId, value: bool) {
        let map = self.controls.borrow();
        let Some(handle) = map.get(&id) else { return };
        let ui: bindings::IUIElement = match handle {
            Handle::ListView(lv) => lv.cast().unwrap(),
            Handle::GridView(gv) => gv.cast().unwrap(),
            Handle::FlipView(fv) => fv.cast().unwrap(),
            _ => return,
        };
        diag::dropped(ui.SetAllowDrop(value));
    }

    fn set_header_element(&mut self, id: ControlId, header_id: Option<ControlId>) {
        let map = self.controls.borrow();
        let Some(handle) = map.get(&id) else { return };
        if let Handle::Expander(e) = handle {
            if let Some(hdr_id) = header_id {
                if let Some(hdr_handle) = map.get(&hdr_id) {
                    let ui_elem = hdr_handle.as_ui_element();
                    diag::dropped(e.SetHeader(&ui_elem));
                }
            } else {
                diag::dropped(e.SetHeader(None));
            }
        } else if let Handle::TitleBar(tb) = handle {
            if let Some(hdr_id) = header_id {
                if let Some(hdr_handle) = map.get(&hdr_id) {
                    let ui_elem = hdr_handle.as_ui_element();
                    diag::dropped(tb.SetContent(&ui_elem));
                }
            } else {
                diag::dropped(tb.SetContent(None));
            }
        }
    }

    fn set_pane_element(&mut self, id: ControlId, pane_id: Option<ControlId>) {
        let map = self.controls.borrow();
        let Some(handle) = map.get(&id) else { return };
        if let Handle::SplitView(sv) = handle {
            if let Some(pid) = pane_id {
                if let Some(pane_handle) = map.get(&pid) {
                    let ui_elem = pane_handle.as_ui_element();
                    diag::dropped(sv.SetPane(&ui_elem));
                }
            } else {
                diag::dropped(sv.SetPane(None));
            }
        } else if let Handle::TitleBar(tb) = handle {
            if let Some(pid) = pane_id {
                if let Some(pane_handle) = map.get(&pid) {
                    let ui_elem = pane_handle.as_ui_element();
                    diag::dropped(tb.SetRightHeader(&ui_elem));
                }
            } else {
                diag::dropped(tb.SetRightHeader(None));
            }
        } else if let Handle::NavigationView(nv) = handle {
            if let Some(pid) = pane_id {
                if let Some(pane_handle) = map.get(&pid) {
                    let ui_elem = pane_handle.as_ui_element();
                    diag::dropped(nv.SetPaneFooter(&ui_elem));
                }
            } else {
                diag::dropped(nv.SetPaneFooter(None));
            }
        }
    }

    fn scroll_templated_to_index(&mut self, id: ControlId, index: i32) {
        if index < 0 {
            return;
        }
        let map = self.controls.borrow();
        let Some(handle) = map.get(&id) else { return };
        // FlipView extends Selector (set SelectedIndex to flip);
        // ListView/GridView extend ListViewBase and expose ScrollIntoView.
        let lvb: Option<bindings::IListViewBase> = match handle {
            Handle::ListView(lv) => lv.cast().ok(),
            Handle::GridView(gv) => gv.cast().ok(),
            Handle::FlipView(fv) => {
                diag::dropped(
                    fv.cast::<bindings::ISelector>()
                        .unwrap()
                        .SetSelectedIndex(index),
                );
                None
            }
            _ => return,
        };
        if let Some(lvb) = lvb {
            let items_control: bindings::IItemsControl = match handle {
                Handle::ListView(lv) => lv.cast().unwrap(),
                Handle::GridView(gv) => gv.cast().unwrap(),
                _ => return,
            };
            if let Ok(items) = items_control.Items()
                && let Ok(coll) =
                    items.cast::<windows_collections::IVector<windows_core::IInspectable>>()
            {
                let len = coll.Size().unwrap_or(0);
                if (index as u32) < len
                    && let Ok(item) = coll.GetAt(index as u32)
                {
                    diag::dropped(lvb.ScrollIntoView(&item));
                }
            }
        }
    }
    fn attach_templated_selection_changed(&mut self, id: ControlId, handler: Callback<i32>) {
        let map = self.controls.borrow();
        let Some(handle) = map.get(&id) else { return };

        let selector: bindings::ISelector = match handle {
            Handle::ListView(lv) => lv.cast().unwrap(),
            Handle::GridView(gv) => gv.cast().unwrap(),
            Handle::FlipView(fv) => fv.cast().unwrap(),
            _ => return,
        };
        // Drop any prior registration so repeated attaches don't
        // accumulate handlers.
        self.templated_selection_revokers.borrow_mut().remove(&id);
        let control = selector.clone();
        let revoker = selector
            .SelectionChanged(move |_sender, _args| {
                let idx = control.SelectedIndex().unwrap_or(-1);
                handler.invoke(idx);
            })
            .unwrap_or_else(|e| {
                panic!(
                    "WinUIBackend::attach_templated_selection_changed: \
                 Selector.SelectionChanged registration failed for control {id}: {e}"
                )
            });
        self.templated_selection_revokers
            .borrow_mut()
            .insert(id, revoker);
    }
    fn attach_templated_realization(
        &mut self,
        id: ControlId,
        realize: Rc<dyn Fn(usize)>,
        recycle: Rc<dyn Fn(usize)>,
    ) {
        let map = self.controls.borrow();
        let Some(handle) = map.get(&id) else { return };
        // Only ListView/GridView drive realization through WinUI's container
        // recycling. FlipView isn't a ListViewBase and is realized eagerly by
        // the reconciler.
        let lvb: bindings::IListViewBase = match handle {
            Handle::ListView(lv) => lv.cast().unwrap(),
            Handle::GridView(gv) => gv.cast().unwrap(),
            _ => return,
        };

        let mut lists = self.templated.borrow_mut();
        let entry = lists.entry(id).or_insert_with(TemplatedList::new);
        let containers = Rc::clone(&entry.shared.containers);

        let revoker = lvb
            .ContainerContentChanging(move |_sender, args| {
                let Some(args) = args.as_ref() else { return };
                let Ok(item_container) = args.ItemContainer() else {
                    return;
                };
                // The item container (a `ListViewItem`/`GridViewItem`) hosts
                // our shared template, whose root is a plain `ContentControl`.
                // We populate that inner control — not the item container,
                // whose `ListViewItemPresenter` only renders string content.
                let Ok(root) = item_container.cast::<bindings::IContentControl>() else {
                    return;
                };
                let Some(cc) = root
                    .ContentTemplateRoot()
                    .ok()
                    .and_then(|r| r.cast::<bindings::IContentControl>().ok())
                else {
                    return;
                };
                let recycling = args.InRecycleQueue().unwrap_or(false);
                if recycling {
                    // The container is being detached from its row. Drop our
                    // element synchronously (before the reconciler unmounts it)
                    // and tell the reconciler to recycle that row.
                    diag::dropped(cc.SetContent(None::<&windows_core::IInspectable>));
                    let mut map = containers.borrow_mut();
                    if let Some(row) = map.iter().find(|(_, c)| **c == cc).map(|(row, _)| *row) {
                        map.remove(&row);
                        drop(map);
                        recycle(row);
                    }
                } else {
                    // A container is being prepared for this row. Record its
                    // content host so set_templated_row_content can fill it,
                    // suppress WinUI's default phased rendering, and ask the
                    // reconciler to render the row.
                    let row = args.ItemIndex().unwrap_or(-1);
                    if row < 0 {
                        return;
                    }
                    let row = row as usize;
                    diag::dropped(args.SetHandled(true));
                    containers.borrow_mut().insert(row, cc);
                    realize(row);
                }
            })
            .unwrap_or_else(|e| {
                panic!(
                    "WinUIBackend::attach_templated_realization: \
                     ContainerContentChanging registration failed for control {id}: {e}"
                )
            });
        entry.realize_revoker = Some(revoker);
    }
    fn attach_templated_reorder(&mut self, id: ControlId, handler: Callback<Vec<usize>>) {
        let map = self.controls.borrow();
        let Some(handle) = map.get(&id) else { return };
        let lvb: bindings::IListViewBase = match handle {
            Handle::ListView(lv) => lv.cast().unwrap(),
            Handle::GridView(gv) => gv.cast().unwrap(),
            _ => return,
        };

        let mut lists = self.templated.borrow_mut();
        let entry = lists.entry(id).or_insert_with(TemplatedList::new);
        let source = Rc::clone(&entry.shared.source);

        let revoker = lvb
            .DragItemsCompleted(move |_sender, _args| {
                // WinUI has already reordered the observable source in place.
                // Read the boxed indices back as the new order (a permutation
                // of the pre-drag positions), hand it to the app, then reset
                // the source to identity so the next drag reads cleanly.
                let slot = source.borrow();
                let Some(source) = slot.as_ref() else { return };
                let len = source.Size().unwrap_or(0) as usize;
                let mut order = Vec::with_capacity(len);
                for i in 0..len as u32 {
                    match source.GetAt(i).ok().as_ref().and_then(unbox_index) {
                        Some(idx) => order.push(idx),
                        None => return,
                    }
                }
                let changed = order.iter().enumerate().any(|(i, v)| *v != i);
                if !changed {
                    return;
                }
                for i in 0..len {
                    diag::dropped(source.SetAt(i as u32, &box_index(i)));
                }
                drop(slot);
                handler.invoke(order);
            })
            .unwrap_or_else(|e| {
                panic!(
                    "WinUIBackend::attach_templated_reorder: \
                     DragItemsCompleted registration failed for control {id}: {e}"
                )
            });
        entry.reorder_revoker = Some(revoker);
    }
    fn destroy(&mut self, id: ControlId) {
        // Drop per-control revokers for templated selection and pointer/tap handlers.
        self.templated_selection_revokers.borrow_mut().remove(&id);
        self.templated.borrow_mut().remove(&id);
        self.pointer_revokers.borrow_mut().remove(&id);
        self.drag_revokers.borrow_mut().remove(&id);
        self.controls.borrow_mut().remove(&id);
        self.event_revokers
            .borrow_mut()
            .retain(|(hid, _), _| *hid != id);
        // Drop any tracked child lists rooted at this id.
        let mut kids = self.parent_children.borrow_mut();
        kids.remove(&id);
        for list in kids.values_mut() {
            list.retain(|c| *c != id);
        }
        // Clean up auxiliary per-control maps that were previously missed,
        // preventing stale entries (and their captured closures) from
        // accumulating across mount/unmount cycles.
        self.menu_click_handlers.borrow_mut().remove(&id);
        self.command_bar_flyout_handlers.borrow_mut().remove(&id);
        self.theme_brush_registry.borrow_mut().remove(&id);
    }
    fn attach_event(&mut self, id: ControlId, event: Event, handler: EventHandler) {
        let map = self.controls.borrow();
        let handle = map
            .get(&id)
            .unwrap_or_else(|| panic!("WinUIBackend::attach_event: unknown control {id}"));

        if let Some(revs) = generated_attach_event::dispatch(handle, event, &handler) {
            if !revs.is_empty() {
                self.event_revokers.borrow_mut().insert((id, event), revs);
            }
            return;
        }

        let mut revokers: Vec<windows_core::EventRevoker> = Vec::new();
        match (event, handle) {
            (Event::Closed, Handle::ContentDialog(d)) => {
                revokers.push(
                    d.Closed(move |_sender, args| {
                        let result = args
                            .as_ref()
                            .and_then(|a| a.Result().ok())
                            .unwrap_or(bindings::ContentDialogResult(0));
                        handler.invoke_i32(result.0);
                    })
                    .unwrap(),
                );
            }
            (Event::SelectionChanged, Handle::TabView(tv)) => {
                let control = tv.clone();
                revokers.push(
                    tv.SelectionChanged(move |_sender, _args| {
                        let idx = control.SelectedIndex().unwrap_or(-1);
                        if idx >= 0 {
                            handler.invoke_i32(idx);
                        }
                    })
                    .unwrap(),
                );
            }
            (Event::CloseRequested, Handle::TabView(tv)) => {
                revokers.push(
                    tv.TabCloseRequested(move |_sender, args| {
                        let key = args
                            .as_ref()
                            .and_then(|a| a.Tab().ok())
                            .and_then(|tab| {
                                tab.cast::<bindings::IFrameworkElement>()
                                    .unwrap()
                                    .Tag()
                                    .ok()
                            })
                            .and_then(|tag_obj| {
                                tag_obj
                                    .cast::<windows_reference::IReference<windows_core::HSTRING>>()
                                    .ok()
                                    .and_then(|pv| pv.Value().ok())
                            })
                            .map(|h| h.to_string_lossy())
                            .unwrap_or_default();
                        handler.invoke_string(key);
                    })
                    .unwrap(),
                );
            }
            (Event::SelectionChanged, Handle::NavigationView(nv)) => {
                revokers.push(
                    nv.SelectionChanged(move |_sender, args| {
                        let tag = args
                            .as_ref()
                            .and_then(|a| a.SelectedItem().ok())
                            .and_then(|item| item.cast::<bindings::NavigationViewItem>().ok())
                            .and_then(|nvi| {
                                nvi.cast::<bindings::IFrameworkElement>()
                                    .unwrap()
                                    .Tag()
                                    .ok()
                            })
                            .and_then(|tag_obj| {
                                tag_obj
                                    .cast::<windows_reference::IReference<windows_core::HSTRING>>()
                                    .ok()
                                    .and_then(|pv| pv.Value().ok())
                            })
                            .map(|h| h.to_string_lossy())
                            .unwrap_or_default();
                        handler.invoke_string(tag);
                    })
                    .unwrap(),
                );
            }
            (Event::QuerySubmitted, Handle::NavigationView(nv)) => {
                if let Ok(asb) = nv.AutoSuggestBox() {
                    revokers.push(
                        asb.QuerySubmitted(move |_sender, args| {
                            let query = args
                                .as_ref()
                                .and_then(|a| a.QueryText().ok())
                                .unwrap_or_default();
                            handler.invoke_string(query);
                        })
                        .unwrap(),
                    );
                }
            }
            (Event::TextChanged, Handle::NavigationView(nv)) => {
                if let Ok(asb) = nv.AutoSuggestBox() {
                    revokers.push(
                        asb.TextChanged(move |sender, _args| {
                            let text = sender
                                .as_ref()
                                .and_then(|s| s.Text().ok())
                                .unwrap_or_default();
                            handler.invoke_string(text);
                        })
                        .unwrap(),
                    );
                }
            }
            (Event::SuggestionChosen, Handle::NavigationView(nv)) => {
                if let Ok(asb) = nv.AutoSuggestBox() {
                    revokers.push(
                        asb.SuggestionChosen(move |_sender, args| {
                            let item = args
                                .as_ref()
                                .and_then(|a| a.SelectedItem().ok())
                                .and_then(|insp| {
                                    insp.cast::<windows_reference::IReference<
                                        windows_core::HSTRING,
                                    >>()
                                    .ok()
                                    .and_then(|pv| pv.Value().ok())
                                })
                                .map(|h| h.to_string_lossy())
                                .unwrap_or_default();
                            handler.invoke_string(item);
                        })
                        .unwrap(),
                    );
                }
            }
            (Event::SelectionChanged, Handle::Pivot(p)) => {
                let selector: bindings::ISelector = p.cast().unwrap();
                revokers.push(
                    p.SelectionChanged(move |_sender, _args| {
                        let idx = selector.SelectedIndex().unwrap_or(-1);
                        if idx >= 0 {
                            handler.invoke_i32(idx);
                        }
                    })
                    .unwrap(),
                );
            }
            (Event::SelectionChanged, Handle::ComboBox(c)) => {
                let selector: bindings::ISelector = c.cast().unwrap();
                let control = selector.clone();
                revokers.push(
                    selector
                        .SelectionChanged(move |_sender, _args| {
                            let idx = control.SelectedIndex().unwrap_or(-1);
                            handler.invoke_i32(idx);
                        })
                        .unwrap(),
                );
            }
            (Event::ColorChanged, Handle::ColorPicker(cp)) => {
                revokers.push(
                    cp.ColorChanged(move |_sender, args| {
                        let color =
                            args.as_ref()
                                .and_then(|a| a.NewColor().ok())
                                .unwrap_or(Color {
                                    a: 255,
                                    r: 0,
                                    g: 0,
                                    b: 0,
                                });
                        handler.invoke_color((color.a, color.r, color.g, color.b));
                    })
                    .unwrap(),
                );
            }
            (Event::SelectedDateChanged, Handle::DatePicker(dp)) => {
                revokers.push(
                    dp.SelectedDateChanged(move |_sender, args| {
                        if let Some(a) = args.as_ref()
                            && let Ok(dt) = a.NewDate()
                        {
                            handler.invoke_datetime(dt);
                        }
                    })
                    .unwrap(),
                );
            }
            (Event::SelectedTimeChanged, Handle::TimePicker(tp)) => {
                revokers.push(
                    tp.SelectedTimeChanged(move |_sender, args| {
                        if let Some(a) = args.as_ref()
                            && let Ok(ts) = a.NewTime()
                        {
                            handler.invoke_timespan(TimeSpan::from_ticks(ts.duration));
                        }
                    })
                    .unwrap(),
                );
            }
            (Event::DateChanged, Handle::CalendarDatePicker(cdp)) => {
                revokers.push(
                    cdp.DateChanged(move |_sender, args| {
                        if let Some(a) = args.as_ref()
                            && let Ok(dt) = a.NewDate()
                        {
                            handler.invoke_datetime(dt);
                        }
                    })
                    .unwrap(),
                );
            }
            (Event::SelectionChanged, Handle::ListBox(lb)) => {
                let selector: bindings::ISelector = lb.cast().unwrap();
                let control = selector.clone();
                revokers.push(
                    selector
                        .SelectionChanged(move |_sender, _args| {
                            if let Ok(idx) = control.SelectedIndex() {
                                handler.invoke_i32(idx);
                            }
                        })
                        .unwrap(),
                );
            }
            (Event::TextChanged, Handle::AutoSuggestBox(asb)) => {
                revokers.push(
                    asb.TextChanged(move |sender, args| {
                        // Only fire for user input, not programmatic changes.
                        let is_user_input = args
                            .as_ref()
                            .and_then(|a| a.Reason().ok())
                            .is_some_and(|r| {
                                r == bindings::AutoSuggestionBoxTextChangeReason::UserInput
                            });
                        if is_user_input {
                            let text = sender
                                .as_ref()
                                .and_then(|s| s.Text().ok())
                                .unwrap_or_default();
                            handler.invoke_string(text);
                        }
                    })
                    .unwrap(),
                );
            }
            (Event::QuerySubmitted, Handle::AutoSuggestBox(asb)) => {
                revokers.push(
                    asb.QuerySubmitted(move |_sender, args| {
                        let text = args
                            .as_ref()
                            .and_then(|a| a.QueryText().ok())
                            .unwrap_or_default();
                        handler.invoke_string(text);
                    })
                    .unwrap(),
                );
            }
            (Event::SuggestionChosen, Handle::AutoSuggestBox(asb)) => {
                revokers.push(
                    asb.SuggestionChosen(move |_sender, args| {
                        let item = args
                            .as_ref()
                            .and_then(|a| a.SelectedItem().ok())
                            .and_then(|insp| {
                                insp.cast::<windows_reference::IReference<windows_core::HSTRING>>()
                                    .ok()
                                    .and_then(|pv| pv.Value().ok())
                            })
                            .map(|h| h.to_string_lossy())
                            .unwrap_or_default();
                        handler.invoke_string(item);
                    })
                    .unwrap(),
                );
            }
            (Event::ItemClicked, Handle::MenuBar(mb)) => {
                // Store the handler so set_prop can re-wire on item rebuild.
                self.menu_click_handlers
                    .borrow_mut()
                    .insert(id, handler.clone());
                let revs = Self::wire_menu_bar_clicks(mb, &handler);
                revokers.extend(revs);
            }
            (Event::ItemClicked, Handle::DropDownButton(_) | Handle::Button(_)) => {
                // Store the handler so set_prop can wire when flyout is built.
                self.menu_click_handlers.borrow_mut().insert(id, handler);
            }
            (Event::CommandBarFlyoutClick, Handle::Button(_)) => {
                // Store the handler so set_prop can wire when flyout is built.
                self.command_bar_flyout_handlers
                    .borrow_mut()
                    .insert(id, handler);
            }
            (Event::ItemInvoked, Handle::TreeView(tv)) => {
                revokers.push(
                    tv.ItemInvoked(move |_sender, args| {
                        let text = args
                            .as_ref()
                            .and_then(|a| a.InvokedItem().ok())
                            .and_then(|insp| {
                                insp.cast::<bindings::ITreeViewNode>()
                                    .ok()
                                    .and_then(|node| node.Content().ok())
                            })
                            .and_then(|content| {
                                content
                                    .cast::<windows_reference::IReference<windows_core::HSTRING>>()
                                    .ok()
                                    .and_then(|r| r.Value().ok())
                            })
                            .map(|h| h.to_string_lossy())
                            .unwrap_or_default();
                        handler.invoke_string(text);
                    })
                    .unwrap(),
                );
            }
            (Event::Click, Handle::CommandBar(cb)) => {
                // Store the handler so set_prop can re-wire on rebuild.
                self.menu_click_handlers
                    .borrow_mut()
                    .insert(id, handler.clone());
                if let Ok(primary) = cb.PrimaryCommands() {
                    let revs = Self::wire_command_bar_clicks(&primary, &handler);
                    revokers.extend(revs);
                }
                if let Ok(secondary) = cb.SecondaryCommands() {
                    let revs = Self::wire_command_bar_clicks(&secondary, &handler);
                    revokers.extend(revs);
                }
            }
            (Event::SelectionChanged, Handle::SelectorBar(sb)) => {
                let sb2 = sb.clone();
                revokers.push(
                    sb.SelectionChanged(move |_sender, _args| {
                        if let Ok(selected) = sb2.SelectedItem()
                            && let Ok(text) = selected.Text()
                        {
                            handler.invoke_string(text);
                        }
                    })
                    .unwrap(),
                );
            }
            (Event::TextChanged, Handle::RichEditBox(reb)) => {
                let control = reb.clone();
                revokers.push(
                    reb.TextChanged(move |_sender, _args| {
                        let text = control
                            .Document()
                            .ok()
                            .and_then(|doc| {
                                let mut buf = windows_core::HSTRING::default();
                                doc.GetText(bindings::TextGetOptions::None, &mut buf).ok()?;
                                Some(buf.to_string_lossy())
                            })
                            .unwrap_or_default();
                        handler.invoke_string(text);
                    })
                    .unwrap(),
                );
            }
            (Event::Closed, _) => {
                // Flyout open/close events are not yet wired.
            }
            // Events handled by generated_attach_event::dispatch — if we reach here, the
            // control type was unexpected (generated dispatch returned None).
            (event, _) => {
                panic!("WinUIBackend::attach_event: {event:?} on unexpected control {id}")
            }
        }
        drop(map);
        if !revokers.is_empty() {
            self.event_revokers
                .borrow_mut()
                .insert((id, event), revokers);
        }
    }
    fn detach_event(&mut self, id: ControlId, event: Event) {
        self.event_revokers.borrow_mut().remove(&(id, event));
    }
    fn set_theme_bindings(
        &mut self,
        id: ControlId,
        kind: ControlKind,
        bindings: &[(Prop, ThemeRef)],
    ) {
        let _ = kind;
        // Store bindings for theme-change re-resolution.
        if bindings.is_empty() {
            self.theme_brush_registry.borrow_mut().remove(&id);
            // Clear any applied style
            let map = self.controls.borrow();
            if let Some(handle) = map.get(&id)
                && let Some((_, fe)) = style_target_for_handle(handle)
            {
                diag::dropped(fe.SetStyle(None));
            }
            return;
        }
        self.theme_brush_registry
            .borrow_mut()
            .insert(id, bindings.to_vec());

        let map = self.controls.borrow();
        let Some(handle) = map.get(&id) else {
            return;
        };
        apply_theme_resource_style(handle, bindings);
    }
    fn on_theme_changed(&mut self) {
        // Re-apply all theme styles; WinUI re-resolves {ThemeResource} on style re-application.
        let controls = self.controls.borrow();
        let registry = self.theme_brush_registry.borrow();
        for (id, bindings) in registry.iter() {
            let Some(handle) = controls.get(id) else {
                continue;
            };
            apply_theme_resource_style(handle, bindings);
        }
    }
    fn set_accessibility(&mut self, id: ControlId, accessibility: &AccessibilityModifiers) {
        let map = self.controls.borrow();
        let Some(handle) = map.get(&id) else {
            return;
        };
        let fe = handle.as_framework_element();
        let dep: bindings::DependencyObject = match fe.cast() {
            Ok(d) => d,
            Err(_) => return,
        };
        diag::dropped(bindings::AutomationProperties::SetName(
            &dep,
            accessibility.automation_name.as_deref().unwrap_or(""),
        ));
        diag::dropped(bindings::AutomationProperties::SetAutomationId(
            &dep,
            accessibility.automation_id.as_deref().unwrap_or(""),
        ));
        diag::dropped(bindings::AutomationProperties::SetHelpText(
            &dep,
            accessibility.help_text.as_deref().unwrap_or(""),
        ));
        let live = accessibility
            .live_setting
            .unwrap_or(AutomationLiveSetting::Off);
        diag::dropped(bindings::AutomationProperties::SetLiveSetting(&dep, live));
        let heading = accessibility
            .heading_level
            .unwrap_or(AutomationHeadingLevel::None);
        diag::dropped(bindings::AutomationProperties::SetHeadingLevel(
            &dep, heading,
        ));
    }
    fn set_keyboard_accelerators(&mut self, id: ControlId, accelerators: &[KeyboardAccelerator]) {
        let map = self.controls.borrow();
        let Some(handle) = map.get(&id) else {
            return;
        };
        let fe = handle.as_framework_element();
        let iue: bindings::IUIElement = match fe.cast() {
            Ok(i) => i,
            Err(_) => return,
        };
        let vec: windows_collections::IVector<bindings::KeyboardAccelerator> =
            match iue.KeyboardAccelerators() {
                Ok(v) => v,
                Err(_) => return,
            };
        diag::dropped(vec.Clear());

        // Suppress the default accelerator tooltip that WinUI would otherwise show.
        diag::dropped(iue.SetKeyboardAcceleratorPlacementMode(
            bindings::KeyboardAcceleratorPlacementMode::Hidden,
        ));

        for accel in accelerators {
            let Ok(ka) = bindings::KeyboardAccelerator::new() else {
                continue;
            };
            let Ok(ika) = ka.cast::<bindings::IKeyboardAccelerator>() else {
                continue;
            };
            diag::dropped(ika.SetKey(accel.key));
            diag::dropped(ika.SetModifiers(accel.modifiers));
            let cb = accel.on_invoked.clone();
            let _ = ika
                .Invoked(move |_sender, args| {
                    if let Some(a) = args.as_ref() {
                        diag::dropped(a.SetHandled(true));
                    }
                    cb.invoke(());
                })
                .ok()
                .map(|r| r.into_token());
            diag::dropped(vec.Append(&ka));
        }
    }
    fn set_implicit_transitions(
        &mut self,
        id: ControlId,
        transitions: Option<ImplicitTransitions>,
    ) {
        let map = self.controls.borrow();
        let Some(handle) = map.get(&id) else {
            return;
        };
        let ui: bindings::UIElement = handle.as_ui_element();
        if let Err(e) = apply_implicit_transitions(&ui, transitions) {
            diag::warn(format_args!("set_implicit_transitions failed: {e:?}"));
        }
    }
    fn set_layout_animation(&mut self, _id: ControlId, _config: Option<LayoutAnimationConfig>) {
        // Layout-driven size/offset animations are not yet wired up
        // (requires SetIsTranslationEnabled + Translation channel).
    }
    fn run_property_animation(&mut self, id: ControlId, config: Option<AnimationConfig>) {
        let Some(cfg) = config else {
            return;
        };
        let map = self.controls.borrow();
        let Some(handle) = map.get(&id) else {
            return;
        };
        let ui: bindings::UIElement = handle.as_ui_element();
        if let Err(e) = run_property_animation_inner(&ui, cfg) {
            diag::warn(format_args!("run_property_animation failed: {e:?}"));
        }
    }
    fn set_rich_text_paragraphs(&mut self, id: ControlId, paragraphs: &[RichTextParagraph]) {
        let map = self.controls.borrow();
        let Some(handle) = map.get(&id) else {
            return;
        };
        let Handle::RichTextBlock(rtb) = handle else {
            return;
        };
        let Ok(blocks) = rtb.Blocks() else { return };
        diag::dropped(blocks.Clear());
        for para_def in paragraphs {
            let Ok(para) = bindings::Paragraph::new() else {
                continue;
            };
            let Ok(inlines) = para.Inlines() else {
                continue;
            };
            for inline in &para_def.inlines {
                match inline {
                    RichTextInline::Run(r) => {
                        let Ok(run) = bindings::Run::new() else {
                            continue;
                        };
                        diag::dropped(run.SetText(&r.text));
                        if r.is_bold {
                            diag::dropped(run.cast::<bindings::ITextElement>().and_then(|te| {
                                te.SetFontWeight(bindings::FontWeight { weight: 700 })
                            }));
                        }
                        diag::dropped(
                            run.cast::<bindings::Inline>()
                                .and_then(|i| inlines.Append(&i)),
                        );
                    }
                    RichTextInline::LineBreak => {
                        // LineBreak inline — use a Run with newline.
                        let Ok(run) = bindings::Run::new() else {
                            continue;
                        };
                        diag::dropped(run.SetText("\n"));
                        diag::dropped(
                            run.cast::<bindings::Inline>()
                                .and_then(|i| inlines.Append(&i)),
                        );
                    }
                    RichTextInline::Hyperlink(h) => {
                        // Hyperlink rendered as plain text (no navigation support yet).
                        let Ok(run) = bindings::Run::new() else {
                            continue;
                        };
                        diag::dropped(run.SetText(&h.text));
                        diag::dropped(
                            run.cast::<bindings::Inline>()
                                .and_then(|i| inlines.Append(&i)),
                        );
                    }
                }
            }
            diag::dropped(
                para.cast::<bindings::Block>()
                    .and_then(|b| blocks.Append(&b)),
            );
        }
    }

    fn set_tooltip(&mut self, id: ControlId, tooltip: Option<&Tooltip>) {
        let map = self.controls.borrow();
        let Some(handle) = map.get(&id) else {
            return;
        };
        let fe = handle.as_framework_element();
        let dep: bindings::DependencyObject = match fe.cast() {
            Ok(d) => d,
            Err(_) => return,
        };

        // Apply (or clear) the tooltip value. ToolTipService::SetToolTip
        // accepts any IInspectable; plain strings are auto-wrapped.
        let inspectable: Option<windows_core::IInspectable> = match tooltip {
            None => None,
            Some(t) => match &t.content {
                TooltipContent::Text(s) => {
                    let reference = windows_reference::IReference::from(s.as_str());
                    Some(reference.into())
                }
                TooltipContent::Rich(elem) => {
                    let tt = match bindings::ToolTip::new() {
                        Ok(t) => t,
                        Err(e) => {
                            diag::warn(format_args!("ToolTip::new failed: {e:?}"));
                            return;
                        }
                    };
                    if let Some(ui) = mount_static_tooltip_element(elem)
                        && let Ok(cc) = tt.cast::<bindings::IContentControl>()
                    {
                        diag::dropped(cc.SetContent(&ui));
                    }
                    Some(tt.into())
                }
            },
        };
        diag::dropped(bindings::ToolTipService::SetToolTip(
            &dep,
            inspectable.as_ref(),
        ));

        // Fall back to Top so cleared placements actually reset the slot.
        let placement = tooltip
            .and_then(|t| t.placement)
            .map_or(bindings::PlacementMode::Top, map_placement);
        diag::dropped(bindings::ToolTipService::SetPlacement(&dep, placement));
    }

    fn set_pointer_handlers(&mut self, id: ControlId, handlers: Option<&PointerHandlers>) {
        // Tear down any previous handlers before reattaching, so a
        // per-render conditional doesn't leak event tokens.
        let prev = self.pointer_revokers.borrow_mut().remove(&id);
        let map = self.controls.borrow();
        let Some(handle) = map.get(&id) else {
            return;
        };
        let ui = handle.as_ui_element();
        drop(prev);

        let Some(handlers) = handlers else {
            return;
        };
        let mut tokens = PointerRevokerSet::default();

        if let Some(cb) = handlers.on_tapped.clone() {
            tokens.tapped = ui
                .Tapped(move |_sender, _args| {
                    cb.invoke(());
                })
                .ok();
        }

        if let Some(cb) = handlers.on_right_tapped.clone() {
            tokens.right_tapped = ui
                .RightTapped(move |_sender, _args| {
                    cb.invoke(());
                })
                .ok();
        }

        if let Some(cb) = handlers.on_pointer_pressed.clone() {
            let element = ui.clone();
            tokens.pressed = ui
                .PointerPressed(move |_sender, args| {
                    let info = pointer_event_info(&element, args);
                    cb.invoke(info);
                })
                .ok();
        }

        if let Some(cb) = handlers.on_pointer_released.clone() {
            let element = ui.clone();
            tokens.released = ui
                .PointerReleased(move |_sender, args| {
                    let info = pointer_event_info(&element, args);
                    cb.invoke(info);
                })
                .ok();
        }

        if let Some(cb) = handlers.on_pointer_moved.clone() {
            let element = ui.clone();
            tokens.moved = ui
                .PointerMoved(move |_sender, args| {
                    let info = pointer_event_info(&element, args);
                    cb.invoke(info);
                })
                .ok();
        }

        if let Some(cb) = handlers.on_pointer_entered.clone() {
            let element = ui.clone();
            tokens.entered = ui
                .PointerEntered(move |_sender, args| {
                    let info = pointer_event_info(&element, args);
                    cb.invoke(info);
                })
                .ok();
        }

        if let Some(cb) = handlers.on_pointer_exited.clone() {
            tokens.exited = ui
                .PointerExited(move |_sender, _args| {
                    cb.invoke(());
                })
                .ok();
        }

        self.pointer_revokers.borrow_mut().insert(id, tokens);
    }

    fn set_drag_handlers(&mut self, id: ControlId, handlers: Option<&DragHandlers>) {
        let prev = self.drag_revokers.borrow_mut().remove(&id);
        let map = self.controls.borrow();
        let Some(handle) = map.get(&id) else {
            return;
        };
        let ui = handle.as_ui_element();
        drop(prev);

        let Some(handlers) = handlers else {
            return;
        };
        let mut tokens = DragRevokerSet::default();

        if let Some(callback) = handlers.on_drag_enter.clone() {
            let marshaller = WinUIDispatcher::for_current_thread()
                .map(|dispatcher| dispatcher.marshaller())
                .ok();

            tokens.enter = ui
                .DragEnter(move |_sender, args| {
                    let Some(drag_event_args) = args.as_ref() else {
                        return;
                    };

                    let formats = drag_event_args
                        .DataView()
                        .ok()
                        .map(|data_package_view| read_available_formats(&data_package_view))
                        .unwrap_or_default();

                    let agile_deferral = drag_event_args
                        .GetDeferral()
                        .ok()
                        .and_then(|deferral| windows_core::AgileReference::new(&deferral).ok());

                    let agile_args = windows_core::AgileReference::new(drag_event_args).ok();

                    let callback = callback.clone();
                    let marshaller = marshaller.clone();
                    windows_threading::submit(move || {
                        let Some(marshaller) = marshaller else {
                            if let Some(deferral) =
                                agile_deferral.and_then(|agile_ref| agile_ref.resolve().ok())
                            {
                                diag::dropped(deferral.Complete());
                            }
                            return;
                        };
                        dispatch_accept(
                            marshaller,
                            callback,
                            formats,
                            agile_args,
                            agile_deferral,
                            vec![],
                            None,
                        );
                    });
                })
                .ok();
        }

        if let Some(cb) = handlers.on_drag_leave.clone() {
            tokens.leave = ui
                .DragLeave(move |_sender, args| {
                    let ctx = build_drag_context(args.as_ref());
                    cb.call(&ctx);
                })
                .ok();
        }

        if let Some(cb) = handlers.on_drag_over.clone() {
            tokens.over = ui
                .DragOver(move |_sender, args| {
                    accept_or_reject(&cb, args.as_ref());
                })
                .ok();
        }

        if let Some(callback) = handlers.on_drag_drop.clone() {
            let marshaller = WinUIDispatcher::for_current_thread()
                .map(|dispatcher| dispatcher.marshaller())
                .ok();

            tokens.drop = ui
                .Drop(move |_sender, args| {
                    let Some(drag_event_args) = args.as_ref() else {
                        return;
                    };

                    let data_view = drag_event_args.DataView().ok();

                    let formats = data_view
                        .as_ref()
                        .map(read_available_formats)
                        .unwrap_or_default();

                    let agile_deferral = drag_event_args
                        .GetDeferral()
                        .ok()
                        .and_then(|deferral| windows_core::AgileReference::new(&deferral).ok());

                    let agile_data_view = data_view.and_then(|data_package_view| {
                        windows_core::AgileReference::new(&data_package_view).ok()
                    });

                    let agile_args = windows_core::AgileReference::new(drag_event_args).ok();

                    let callback = callback.clone();
                    let marshaller = marshaller.clone();

                    windows_threading::submit(move || {
                        use crate::drag::DroppedItem;

                        let resolved_data_view = agile_data_view
                            .and_then(|agile_reference| agile_reference.resolve().ok());

                        let items: Vec<DroppedItem> = if formats.storage_items {
                            resolved_data_view
                                .as_ref()
                                .and_then(|data_package_view| {
                                    data_package_view.GetStorageItemsAsync().ok()
                                })
                                .and_then(|async_operation| async_operation.join().ok())
                                .map(|v| {
                                    let size = v.Size().unwrap_or(0);
                                    (0..size)
                                        .filter_map(|i| v.GetAt(i).ok())
                                        .map(|item| DroppedItem {
                                            path: item.Path().unwrap_or_default(),
                                            name: item.Name().unwrap_or_default(),
                                            is_folder: item.Attributes().is_ok_and(|attrs| {
                                                attrs.contains(bindings::FileAttributes::Directory)
                                            }),
                                        })
                                        .collect()
                                })
                                .unwrap_or_default()
                        } else {
                            vec![]
                        };

                        let text: Option<String> = if formats.text {
                            resolved_data_view
                                .as_ref()
                                .and_then(|data_package_view| data_package_view.GetTextAsync().ok())
                                .and_then(|async_operation| async_operation.join().ok())
                                .and_then(|h| String::try_from(&h).ok())
                        } else {
                            None
                        };

                        let Some(marshaller) = marshaller else {
                            if let Some(deferral) =
                                agile_deferral.and_then(|agile_ref| agile_ref.resolve().ok())
                            {
                                diag::dropped(deferral.Complete());
                            }
                            return;
                        };
                        dispatch_accept(
                            marshaller,
                            callback,
                            formats,
                            agile_args,
                            agile_deferral,
                            items,
                            text,
                        );
                    });
                })
                .ok();
        }

        self.drag_revokers.borrow_mut().insert(id, tokens);
    }

    fn get_native_element(&self, id: ControlId) -> Option<windows_core::IInspectable> {
        self.get_ui_element(id)
    }
}

const FORMAT_TEXT: &str = "Text";
const FORMAT_HTML: &str = "HTML Format";
const FORMAT_RTF: &str = "Rich Text Format";
const FORMAT_BITMAP: &str = "Bitmap";
const FORMAT_STORAGE_ITEMS: &str = "Shell IDList Array";
const FORMAT_URI_AND_WEB_LINK: &str = "UniformResourceLocatorW";
const FORMAT_APPLICATION_LINK: &str = "ApplicationLink";

#[derive(Copy, Clone, Default)]
struct AvailableFormats {
    text: bool,
    html: bool,
    rtf: bool,
    bitmap: bool,
    storage_items: bool,
    uri: bool,
    web_link: bool,
    application_link: bool,
}

fn read_available_formats(data_package_view: &bindings::DataPackageView) -> AvailableFormats {
    let mut available_formats = AvailableFormats::default();
    let Ok(formats) = data_package_view.AvailableFormats() else {
        return available_formats;
    };

    for s in &formats {
        match s.to_string_lossy().as_str() {
            FORMAT_TEXT => available_formats.text = true,
            FORMAT_HTML => available_formats.html = true,
            FORMAT_RTF => available_formats.rtf = true,
            FORMAT_BITMAP => available_formats.bitmap = true,
            FORMAT_STORAGE_ITEMS => available_formats.storage_items = true,
            FORMAT_URI_AND_WEB_LINK => {
                available_formats.uri = true;
                available_formats.web_link = true;
            }
            FORMAT_APPLICATION_LINK => available_formats.application_link = true,
            _ => {}
        }
    }
    available_formats
}

fn build_drag_context(args: Option<&bindings::DragEventArgs>) -> DragContext {
    use crate::drag::{DragContext, DroppedItem};
    let mut ctx = DragContext {
        has_text: false,
        has_html: false,
        has_rtf: false,
        has_bitmap: false,
        has_storage_items: false,
        has_uri: false,
        has_web_link: false,
        has_application_link: false,
        caption: None,
        glyph_visible: None,
        content_visible: None,
        get_text_fn: None,
        get_storage_items_fn: None,
    };
    let Some(a) = args else { return ctx };
    let Ok(dv) = a.DataView() else {
        return ctx;
    };

    let formats = read_available_formats(&dv);
    ctx.has_text = formats.text;
    ctx.has_html = formats.html;
    ctx.has_rtf = formats.rtf;
    ctx.has_bitmap = formats.bitmap;
    ctx.has_storage_items = formats.storage_items;
    ctx.has_uri = formats.uri;
    ctx.has_web_link = formats.web_link;
    ctx.has_application_link = formats.application_link;

    let dv_text = dv.clone();
    ctx.get_text_fn = Some(Box::new(move || {
        let h = dv_text.GetTextAsync().ok()?.join().ok()?;
        String::try_from(&h).ok()
    }));

    ctx.get_storage_items_fn = Some(Box::new(move || {
        let items = dv.GetStorageItemsAsync().ok().and_then(|op| op.join().ok());
        let Some(items) = items else {
            return Vec::new();
        };
        items
            .into_iter()
            .map(|item| DroppedItem {
                path: item.Path().unwrap_or_default(),
                name: item.Name().unwrap_or_default(),
                is_folder: item
                    .Attributes()
                    .is_ok_and(|a| a.contains(bindings::FileAttributes::Directory)),
            })
            .collect()
    }));

    ctx
}

fn dispatch_accept(
    m: UiMarshaller,
    cb: DragAsyncCallback,
    formats: AvailableFormats,
    iargs_agile: Option<windows_core::AgileReference<bindings::DragEventArgs>>,
    deferral_agile: Option<windows_core::AgileReference<bindings::DragOperationDeferral>>,
    items: Vec<DroppedItem>,
    text: Option<String>,
) {
    use crate::drag::{DragContext, DragOperation};
    m.dispatch(move || {
        let get_storage_items_fn = if items.is_empty() {
            None
        } else {
            let v = items.clone();
            Some(Box::new(move || v.clone()) as Box<dyn Fn() -> Vec<DroppedItem>>)
        };
        let get_text_fn =
            text.map(|t| Box::new(move || Some(t.clone())) as Box<dyn Fn() -> Option<String>>);
        let mut ctx = DragContext {
            has_text: formats.text,
            has_html: formats.html,
            has_rtf: formats.rtf,
            has_bitmap: formats.bitmap,
            has_storage_items: formats.storage_items,
            has_uri: formats.uri,
            has_web_link: formats.web_link,
            has_application_link: formats.application_link,
            caption: None,
            glyph_visible: None,
            content_visible: None,
            get_text_fn,
            get_storage_items_fn,
        };
        let op = cb.call(&mut ctx);
        if let Some(iargs) = iargs_agile.and_then(|a| a.resolve().ok()) {
            let accepted = match op {
                DragOperation::None => bindings::DataPackageOperation::None,
                DragOperation::Copy => bindings::DataPackageOperation::Copy,
                DragOperation::Move => bindings::DataPackageOperation::Move,
                DragOperation::Link => bindings::DataPackageOperation::Link,
            };
            diag::dropped(iargs.SetAcceptedOperation(accepted));
            if (ctx.caption.is_some()
                || ctx.glyph_visible.is_some()
                || ctx.content_visible.is_some())
                && let Ok(ui) = iargs.DragUIOverride()
            {
                if let Some(v) = ctx.caption {
                    diag::dropped(ui.SetCaption(&v));
                }
                if let Some(v) = ctx.glyph_visible {
                    diag::dropped(ui.SetIsGlyphVisible(v));
                }
                if let Some(v) = ctx.content_visible {
                    diag::dropped(ui.SetIsContentVisible(v));
                }
            }
        }
        if let Some(d) = deferral_agile.and_then(|a| a.resolve().ok()) {
            diag::dropped(d.Complete());
        }
    });
}

trait CallAccept {
    fn call(&self, ctx: &mut DragContext) -> DragOperation;
}
impl CallAccept for DragCallback {
    fn call(&self, ctx: &mut DragContext) -> DragOperation {
        self.call(ctx)
    }
}
impl CallAccept for DragAsyncCallback {
    fn call(&self, ctx: &mut DragContext) -> DragOperation {
        self.call(ctx)
    }
}

fn accept_or_reject<C: CallAccept>(cb: &C, args: Option<&bindings::DragEventArgs>) {
    use crate::drag::DragOperation;
    let Some(a) = args else { return };

    let mut ctx = build_drag_context(Some(a));

    let result = cb.call(&mut ctx);

    let accepted = match result {
        DragOperation::None => bindings::DataPackageOperation::None,
        DragOperation::Copy => bindings::DataPackageOperation::Copy,
        DragOperation::Move => bindings::DataPackageOperation::Move,
        DragOperation::Link => bindings::DataPackageOperation::Link,
    };
    diag::dropped(a.SetAcceptedOperation(accepted));

    if (ctx.caption.is_some() || ctx.glyph_visible.is_some() || ctx.content_visible.is_some())
        && let Ok(ui) = a.DragUIOverride()
    {
        if let Some(v) = ctx.caption {
            diag::dropped(ui.SetCaption(&v));
        }
        if let Some(v) = ctx.glyph_visible {
            diag::dropped(ui.SetIsGlyphVisible(v));
        }
        if let Some(v) = ctx.content_visible {
            diag::dropped(ui.SetIsContentVisible(v));
        }
    }
}

/// Extract the pointer position and button state for a `PointerPressed` /
/// `PointerReleased` callback; falls back to defaults on any null hop.
///
/// `element` is captured once at attach time (the handler's own element), so
/// there is no per-event `QueryInterface`: the arg/point/properties classes
/// each `Deref` to their default interface, and the coordinates are read
/// relative to `element` directly.
fn pointer_event_info(
    element: &bindings::UIElement,
    args: windows_core::InRef<'_, bindings::PointerRoutedEventArgs>,
) -> PointerEventInfo {
    let mut info = PointerEventInfo::default();
    let Some(args) = args.as_ref() else {
        return info;
    };
    let Ok(point) = args.GetCurrentPoint(element) else {
        return info;
    };
    if let Ok(pos) = point.Position() {
        info.x = pos.x as f64;
        info.y = pos.y as f64;
    }
    let Ok(props) = point.Properties() else {
        return info;
    };
    info.is_left_button_pressed = props.IsLeftButtonPressed().unwrap_or(false);
    info.is_right_button_pressed = props.IsRightButtonPressed().unwrap_or(false);
    info.is_middle_button_pressed = props.IsMiddleButtonPressed().unwrap_or(false);
    info
}

fn map_placement(p: TooltipPlacement) -> bindings::PlacementMode {
    use TooltipPlacement;
    match p {
        TooltipPlacement::Top => bindings::PlacementMode::Top,
        TooltipPlacement::Bottom => bindings::PlacementMode::Bottom,
        TooltipPlacement::Left => bindings::PlacementMode::Left,
        TooltipPlacement::Right => bindings::PlacementMode::Right,
        TooltipPlacement::Mouse => bindings::PlacementMode::Mouse,
    }
}

/// Best-effort static mount for tooltip content. Supports `TextBlock`,
/// linear `StackPanel`, and `Image`; unsupported kinds fall back to a
/// `TextBlock` showing `kind_name()`.
fn mount_static_tooltip_element(el: &Element) -> Option<bindings::UIElement> {
    match el {
        Element::TextBlock(t) => {
            let tb = bindings::TextBlock::new().ok()?;
            tb.SetText(t.text.as_str()).ok()?;
            tb.cast::<bindings::UIElement>().ok()
        }
        Element::StackPanel(s) => {
            let sp = bindings::StackPanel::new().ok()?;
            sp.SetOrientation(s.orientation).ok()?;
            sp.SetSpacing(s.spacing).ok()?;
            let children = sp.cast::<bindings::IPanel>().ok()?.Children().ok()?;
            for child in &s.children {
                if let Some(cui) = mount_static_tooltip_element(child) {
                    diag::dropped(children.Append(&cui));
                }
            }
            sp.cast::<bindings::UIElement>().ok()
        }
        Element::Image(img) => {
            let i = bindings::Image::new().ok()?;
            match &img.source {
                ImageSource::Uri(uri_str) => {
                    if let Ok(uri) = bindings::Uri::CreateUri(uri_str.as_str())
                        && let Ok(bmp) = bindings::BitmapImage::new()
                    {
                        diag::dropped(bmp.SetUriSource(&uri));
                        if let Ok(src) = bmp.cast::<bindings::ImageSource>() {
                            diag::dropped(i.SetSource(&src));
                        }
                    }
                }
                ImageSource::Surface(sis) => {
                    if let Ok(src) = sis.image_source() {
                        diag::dropped(i.SetSource(&src));
                    }
                }
                ImageSource::None => {}
            }
            i.cast::<bindings::UIElement>().ok()
        }
        _ => {
            // Fallback: surface the kind_name so the developer sees a
            // hint rather than an empty popup.
            let tb = bindings::TextBlock::new().ok()?;
            tb.SetText(el.kind_name()).ok()?;
            tb.cast::<bindings::UIElement>().ok()
        }
    }
}
