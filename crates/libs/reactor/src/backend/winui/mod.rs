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
}

/// [`Backend`] implementation that creates real `Microsoft.UI.Xaml`
/// controls and drives them on the WinUI thread.
pub struct WinUIBackend {
    controls: RefCell<FxHashMap<ControlId, Handle>>,
    event_revokers: RefCell<FxHashMap<(ControlId, Event), Vec<windows_core::EventRevoker>>>,
    templated_selection_revokers: RefCell<FxHashMap<ControlId, windows_core::EventRevoker>>,
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
    next_id: RefCell<u32>,
}

#[derive(Default)]
struct PointerRevokerSet {
    tapped: Option<windows_core::EventRevoker>,
    right_tapped: Option<windows_core::EventRevoker>,
    pressed: Option<windows_core::EventRevoker>,
    released: Option<windows_core::EventRevoker>,
    exited: Option<windows_core::EventRevoker>,
}

#[derive(Default)]
struct DragRevokerSet {
    enter: Option<windows_core::EventRevoker>,
    leave: Option<windows_core::EventRevoker>,
    over: Option<windows_core::EventRevoker>,
    drop: Option<windows_core::EventRevoker>,
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
            pointer_revokers: RefCell::new(FxHashMap::default()),
            drag_revokers: RefCell::new(FxHashMap::default()),
            parent_children: RefCell::new(FxHashMap::default()),
            menu_click_handlers: RefCell::new(FxHashMap::default()),
            command_bar_flyout_handlers: RefCell::new(FxHashMap::default()),
            theme_brush_registry: RefCell::new(FxHashMap::default()),
            next_id: RefCell::new(0),
        }
    }
    pub fn get_ui_element(&self, id: ControlId) -> Option<windows_core::IInspectable> {
        self.controls
            .borrow()
            .get(&id)
            .map(|h| h.as_ui_element().cast().unwrap())
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
        let Ok(bar_items) = mb.get_Items() else {
            return revokers;
        };
        for i in 0..bar_items.Size().unwrap_or(0) {
            if let Ok(mbi) = bar_items.GetAt(i)
                && let Ok(flyout_items) = mbi.get_Items()
            {
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
        if let Ok(items) = flyout.get_Items() {
            Self::wire_flyout_items_click(&items, handler, &mut revokers);
        }
        revokers
    }

    fn wire_flyout_items_click(
        items: &windows_collections::IVector<bindings::MenuFlyoutItemBase>,
        handler: &EventHandler,
        revokers: &mut Vec<windows_core::EventRevoker>,
    ) {
        for i in 0..items.Size().unwrap_or(0) {
            let Ok(base) = items.GetAt(i) else { continue };
            if let Ok(item) = base.cast::<bindings::MenuFlyoutItem>() {
                let text = item.get_Text().unwrap_or_default().clone();
                let handler = handler.clone();
                if let Ok(rev) = item.Click(move |_s, _a| {
                    handler.invoke_string(text.clone());
                }) {
                    revokers.push(rev);
                }
            } else if let Ok(sub) = base.cast::<bindings::MenuFlyoutSubItem>()
                && let Ok(sub_items) = sub.get_Items()
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
        for i in 0..commands.Size().unwrap_or(0) {
            let Ok(el) = commands.GetAt(i) else { continue };
            if let Ok(btn) = el.cast::<bindings::AppBarButton>() {
                let label = btn.get_Label().unwrap_or_default().clone();
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
    /// by `IPanel::Children` — a `UIElementCollection` (`IVector<UIElement>`).
    Panel(windows_collections::IVector<bindings::UIElement>),
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
            s.cast::<bindings::IPanel>()
                .ok()?
                .get_Children()
                .ok()?
                .cast()
                .ok()?,
        )),
        Handle::Grid(g) => Some(ContainerChildren::Panel(
            g.cast::<bindings::IPanel>()
                .ok()?
                .get_Children()
                .ok()?
                .cast()
                .ok()?,
        )),
        Handle::Canvas(c) => Some(ContainerChildren::Panel(
            c.cast::<bindings::IPanel>()
                .ok()?
                .get_Children()
                .ok()?
                .cast()
                .ok()?,
        )),
        Handle::RelativePanel(r) => Some(ContainerChildren::Panel(
            r.cast::<bindings::IPanel>()
                .ok()?
                .get_Children()
                .ok()?
                .cast()
                .ok()?,
        )),
        Handle::Border(_) | Handle::Viewbox(_) => Some(ContainerChildren::SingleChild(h)),
        Handle::ScrollViewer(s) => Some(ContainerChildren::ContentControl(s.cast().ok()?)),
        Handle::Expander(e) => Some(ContainerChildren::ContentControl(e.cast().ok()?)),
        Handle::TabViewItem(ti) => Some(ContainerChildren::ContentControl(ti.cast().ok()?)),
        Handle::NavigationView(nv) => Some(ContainerChildren::ContentControl(nv.cast().ok()?)),
        Handle::PivotItem(pi) => Some(ContainerChildren::ContentControl(pi.cast().ok()?)),
        Handle::ScrollView(_) | Handle::SplitView(_) => Some(ContainerChildren::DirectContent(h)),
        Handle::TabView(tv) => Some(ContainerChildren::InspectableVector(
            tv.get_TabItems().ok()?,
        )),
        Handle::Pivot(p) => Some(ContainerChildren::InspectableVector(
            p.cast::<bindings::IItemsControl>()
                .ok()?
                .get_Items()
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
        ContainerChildren::ContentControl(c) => c.put_Content(child).unwrap(),
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
        ContainerChildren::ContentControl(c) => c.put_Content(child).unwrap(),
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
        ContainerChildren::ContentControl(c) => c.put_Content(child).unwrap(),
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
            c.put_Content(None::<&windows_core::IInspectable>).unwrap();
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
        Handle::Border(b) => b.put_Child(child).unwrap(),
        Handle::Viewbox(v) => v.put_Child(child).unwrap(),
        _ => unreachable!(),
    }
}

fn put_direct_content(h: &Handle, child: Option<&bindings::UIElement>) {
    match h {
        Handle::ScrollView(sv) => sv.put_Content(child).unwrap(),
        Handle::SplitView(sv) => sv.put_Content(child).unwrap(),
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
        let _ = fe.put_Style(None);
        return;
    }

    let xaml = format!(
        "<Style xmlns='http://schemas.microsoft.com/winfx/2006/xaml/presentation' TargetType='{target_type}'>{setters}</Style>"
    );

    match bindings::XamlReader::Load(&xaml) {
        Ok(obj) => {
            if let Ok(style) = obj.cast::<bindings::Style>() {
                // Clear first to force WinUI to re-resolve {ThemeResource} values
                let _ = fe.put_Style(None);
                let _ = fe.put_Style(&style);
            }
        }
        Err(e) => {
            eprintln!("[ThemeStyle] XamlReader::Load failed: {e:?} xaml={xaml}");
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
        Handle::TextBlock(t) => t.cast().ok().map(|fe| ("TextBlock", fe)),
        Handle::Canvas(c) => c.cast().ok().map(|fe| ("Canvas", fe)),
        _ => None,
    }
}

// Composition animation helpers. Both `set_implicit_transitions` and
// `run_property_animation` reach the element's Visual via
// `ElementCompositionPreview::GetElementVisual`.

fn anim_duration_to_timespan(d: std::time::Duration) -> TimeSpan {
    TimeSpan::try_from(d).unwrap_or(TimeSpan::MAX)
}

fn easing_for(
    compositor: &bindings::ICompositor,
    easing: Easing,
) -> Result<bindings::CompositionEasingFunction> {
    use Easing;
    // Control points match the CSS-standard ease-{out,in,in-out} curves.
    let (p1, p2) = match easing {
        Easing::Linear => {
            let lin = compositor.CreateLinearEasingFunction()?;
            return lin.cast::<bindings::CompositionEasingFunction>();
        }
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
    let cubic = compositor.CreateCubicBezierEasingFunction(p1, p2)?;
    cubic.cast::<bindings::CompositionEasingFunction>()
}

fn apply_implicit_transitions(
    ui: &bindings::UIElement,
    transitions: Option<ImplicitTransitions>,
) -> Result<()> {
    use Easing;
    let visual = bindings::ElementCompositionPreview::GetElementVisual(ui)?;
    let obj2 = visual.cast::<bindings::ICompositionObject2>()?;
    // No transitions, or every slot empty: clear the implicit-animation collection.
    let Some(t) = transitions.filter(|t| !t.is_empty()) else {
        obj2.put_ImplicitAnimations(None)?;
        return Ok(());
    };
    let compositor = visual
        .cast::<bindings::ICompositionObject>()?
        .get_Compositor()?;
    let icomp = compositor.cast::<bindings::ICompositor>()?;
    let icomp2 = compositor.cast::<bindings::ICompositor2>()?;
    let collection = icomp2.CreateImplicitAnimationCollection()?;
    let map = collection
        .cast::<windows_collections::IMap<windows_core::HSTRING, bindings::ICompositionAnimationBase>>(
        )?;

    // Animate from `this.StartingValue` to `this.FinalValue` over `duration`.
    // The DSL transition types only expose duration, so easing is fixed to
    // EaseOut — the standard XAML implicit-transition curve.
    let insert = |target: &str, duration: std::time::Duration, is_scalar: bool| -> Result<()> {
        let easing = easing_for(&icomp, Easing::EaseOut)?;
        let target_hs = windows_core::HSTRING::from(target);
        let final_expr = "this.FinalValue";
        let anim_base: bindings::ICompositionAnimationBase = if is_scalar {
            let a = icomp.CreateScalarKeyFrameAnimation()?;
            let kfa = a.cast::<bindings::IKeyFrameAnimation>()?;
            kfa.put_Duration(anim_duration_to_timespan(duration))?;
            kfa.InsertExpressionKeyFrameWithEasingFunction(1.0, final_expr, &easing)?;
            let anim2 = a.cast::<bindings::ICompositionAnimation2>()?;
            anim2.put_Target(target)?;
            a.cast::<bindings::ICompositionAnimationBase>()?
        } else {
            let a = icomp.CreateVector3KeyFrameAnimation()?;
            let kfa = a.cast::<bindings::IKeyFrameAnimation>()?;
            kfa.put_Duration(anim_duration_to_timespan(duration))?;
            kfa.InsertExpressionKeyFrameWithEasingFunction(1.0, final_expr, &easing)?;
            let anim2 = a.cast::<bindings::ICompositionAnimation2>()?;
            anim2.put_Target(target)?;
            a.cast::<bindings::ICompositionAnimationBase>()?
        };
        map.Insert(&target_hs, &anim_base)?;
        Ok(())
    };

    if let Some(s) = t.opacity {
        insert("Opacity", s.duration, true)?;
    }
    if let Some(s) = t.rotation {
        insert("RotationAngleInDegrees", s.duration, true)?;
    }
    if let Some(v) = t.scale {
        insert("Scale", v.duration, false)?;
    }
    if let Some(v) = t.translation {
        // KNOWN LIMITATION: `Offset` collides with XAML layout; the
        // correct target is the synthetic `Translation` channel.
        insert("Offset", v.duration, false)?;
    }
    obj2.put_ImplicitAnimations(&collection)?;
    Ok(())
}

fn run_property_animation_inner(ui: &bindings::UIElement, cfg: AnimationConfig) -> Result<()> {
    let visual = bindings::ElementCompositionPreview::GetElementVisual(ui)?;
    let visual_obj = visual.cast::<bindings::ICompositionObject>()?;
    let compositor = visual_obj.get_Compositor()?;
    let icomp = compositor.cast::<bindings::ICompositor>()?;

    if let Some(opacity) = cfg.opacity {
        let a = icomp.CreateScalarKeyFrameAnimation()?;
        let ia = a.cast::<bindings::IScalarKeyFrameAnimation>()?;
        a.cast::<bindings::IKeyFrameAnimation>()?
            .put_Duration(anim_duration_to_timespan(cfg.duration))?;
        let easing = easing_for(&icomp, cfg.easing)?;
        ia.InsertKeyFrameWithEasingFunction(1.0, opacity as f32, &easing)?;
        let anim = a.cast::<bindings::CompositionAnimation>()?;
        visual_obj.StartAnimation("Opacity", &anim)?;
    }
    if let Some(scale) = cfg.scale {
        // Pivot scale around the element's centre. KNOWN LIMITATION:
        // before the first layout pass ActualWidth/Height are 0 and the
        // previous CenterPoint is reused.
        let iv = visual.cast::<bindings::IVisual>()?;
        if let Ok(fe) = ui.cast::<bindings::IFrameworkElement>() {
            let w = fe.get_ActualWidth().unwrap_or(0.0) as f32;
            let h = fe.get_ActualHeight().unwrap_or(0.0) as f32;
            if w > 0.0 && h > 0.0 {
                let _ = iv.put_CenterPoint(windows_numerics::Vector3 {
                    x: w / 2.0,
                    y: h / 2.0,
                    z: 0.0,
                });
            } else if cfg!(debug_assertions) {
                eprintln!(
                    "windows-reactor: animation: skipping CenterPoint — element not yet laid out"
                );
            }
        }
        // Preserve current Scale.Z; cfg.scale is a uniform X/Y scalar.
        let current_z = iv.get_Scale().map_or(1.0, |v| v.z);
        let a = icomp.CreateVector3KeyFrameAnimation()?;
        let ia = a.cast::<bindings::IVector3KeyFrameAnimation>()?;
        a.cast::<bindings::IKeyFrameAnimation>()?
            .put_Duration(anim_duration_to_timespan(cfg.duration))?;
        let easing = easing_for(&icomp, cfg.easing)?;
        let s = scale as f32;
        ia.InsertKeyFrameWithEasingFunction(
            1.0,
            windows_numerics::Vector3 {
                x: s,
                y: s,
                z: current_z,
            },
            &easing,
        )?;
        let anim = a.cast::<bindings::CompositionAnimation>()?;
        visual_obj.StartAnimation("Scale", &anim)?;
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
            handle.as_framework_element().put_Margin(*t)?;
            Ok(true)
        }
        (Prop::Margin, PropValue::Unset) => {
            handle
                .as_framework_element()
                .put_Margin(Thickness::default())?;
            Ok(true)
        }
        (Prop::Width, PropValue::F64(v)) => {
            handle.as_framework_element().put_Width(*v)?;
            Ok(true)
        }
        (Prop::Width, PropValue::Unset) => {
            handle.as_framework_element().put_Width(f64::NAN)?;
            Ok(true)
        }
        (Prop::Height, PropValue::F64(v)) => {
            handle.as_framework_element().put_Height(*v)?;
            Ok(true)
        }
        (Prop::Height, PropValue::Unset) => {
            handle.as_framework_element().put_Height(f64::NAN)?;
            Ok(true)
        }
        (Prop::MinWidth, PropValue::F64(v)) => {
            handle.as_framework_element().put_MinWidth(*v)?;
            Ok(true)
        }
        (Prop::MinWidth, PropValue::Unset) => {
            handle.as_framework_element().put_MinWidth(0.0)?;
            Ok(true)
        }
        (Prop::MaxWidth, PropValue::F64(v)) => {
            handle.as_framework_element().put_MaxWidth(*v)?;
            Ok(true)
        }
        (Prop::MaxWidth, PropValue::Unset) => {
            handle.as_framework_element().put_MaxWidth(f64::INFINITY)?;
            Ok(true)
        }
        (Prop::MinHeight, PropValue::F64(v)) => {
            handle.as_framework_element().put_MinHeight(*v)?;
            Ok(true)
        }
        (Prop::MinHeight, PropValue::Unset) => {
            handle.as_framework_element().put_MinHeight(0.0)?;
            Ok(true)
        }
        (Prop::MaxHeight, PropValue::F64(v)) => {
            handle.as_framework_element().put_MaxHeight(*v)?;
            Ok(true)
        }
        (Prop::MaxHeight, PropValue::Unset) => {
            handle.as_framework_element().put_MaxHeight(f64::INFINITY)?;
            Ok(true)
        }
        (Prop::HorizontalAlignment, PropValue::I32(v)) => {
            handle
                .as_framework_element()
                .put_HorizontalAlignment(HorizontalAlignment(*v))?;
            Ok(true)
        }
        (Prop::HorizontalAlignment, PropValue::Unset) => {
            handle
                .as_framework_element()
                .put_HorizontalAlignment(HorizontalAlignment::Stretch)?;
            Ok(true)
        }
        (Prop::VerticalAlignment, PropValue::I32(v)) => {
            handle
                .as_framework_element()
                .put_VerticalAlignment(VerticalAlignment(*v))?;
            Ok(true)
        }
        (Prop::VerticalAlignment, PropValue::Unset) => {
            handle
                .as_framework_element()
                .put_VerticalAlignment(VerticalAlignment::Stretch)?;
            Ok(true)
        }
        (Prop::Opacity, PropValue::F64(v)) => {
            handle.as_ui_element().put_Opacity(*v)?;
            Ok(true)
        }
        (Prop::Opacity, PropValue::Unset) => {
            handle.as_ui_element().put_Opacity(1.0)?;
            Ok(true)
        }
        (Prop::AllowDrop, PropValue::Bool(v)) => {
            handle.as_ui_element().put_AllowDrop(*v)?;
            Ok(true)
        }
        (Prop::AllowDrop, PropValue::Unset) => {
            handle.as_ui_element().put_AllowDrop(false)?;
            Ok(true)
        }
        (Prop::IsEnabled, PropValue::Unset) => {
            handle
                .as_ui_element()
                .cast::<bindings::IControl>()?
                .put_IsEnabled(true)?;
            Ok(true)
        }
        (Prop::Resources, PropValue::Resources(map)) => {
            let rd = handle.as_framework_element().get_Resources()?;
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
                .put_Fill(&solid_brush(*b)?)?;
            Ok(true)
        }
        (Prop::Fill, PropValue::Unset) => {
            handle.cast_inner::<bindings::IShape>()?.put_Fill(None)?;
            Ok(true)
        }
        (Prop::Stroke, PropValue::Color(b)) => {
            handle
                .cast_inner::<bindings::IShape>()?
                .put_Stroke(&solid_brush(*b)?)?;
            Ok(true)
        }
        (Prop::Stroke, PropValue::Unset) => {
            handle.cast_inner::<bindings::IShape>()?.put_Stroke(None)?;
            Ok(true)
        }
        (Prop::StrokeThickness, PropValue::F64(v)) => {
            handle
                .cast_inner::<bindings::IShape>()?
                .put_StrokeThickness(*v)?;
            Ok(true)
        }
        (Prop::StrokeThickness, PropValue::Unset) => {
            handle
                .cast_inner::<bindings::IShape>()?
                .put_StrokeThickness(0.0)?;
            Ok(true)
        }
        _ => Ok(false),
    }
}

fn set_padding(handle: &Handle, thickness: Thickness) -> Result<bool> {
    if let Ok(border) = handle.cast_inner::<bindings::IBorder>() {
        border.put_Padding(thickness)?;
    } else if let Ok(ctl) = handle.cast_inner::<bindings::IControl>() {
        ctl.put_Padding(thickness)?;
    } else {
        diag::unhandled_modifier("set_prop", Prop::Padding, handle);
    }
    Ok(true)
}

fn set_background(
    handle: &Handle,
    brush: impl windows_core::Param<bindings::Brush>,
) -> Result<bool> {
    if let Ok(panel) = handle.cast_inner::<bindings::IPanel>() {
        panel.put_Background(brush)?;
    } else if let Ok(ctl) = handle.cast_inner::<bindings::IControl>() {
        ctl.put_Background(brush)?;
    } else if let Ok(border) = handle.cast_inner::<bindings::IBorder>() {
        border.put_Background(brush)?;
    } else {
        diag::unhandled_modifier("set_prop", Prop::Background, handle);
    }
    Ok(true)
}

fn set_foreground(
    handle: &Handle,
    brush: impl windows_core::Param<bindings::Brush>,
) -> Result<bool> {
    if let Ok(ctl) = handle.cast_inner::<bindings::IControl>() {
        ctl.put_Foreground(brush)?;
    } else if let Ok(tb) = handle.cast_inner::<bindings::ITextBlock>() {
        tb.put_Foreground(brush)?;
    } else if let Ok(rtb) = handle.cast_inner::<bindings::IRichTextBlock>() {
        rtb.put_Foreground(brush)?;
    } else {
        diag::unhandled_modifier("set_prop", Prop::Foreground, handle);
    }
    Ok(true)
}

fn set_font_f64(handle: &Handle, v: f64) -> Result<bool> {
    if let Ok(ctrl) = handle.cast_inner::<bindings::IControl>() {
        ctrl.put_FontSize(v)?;
    } else if let Ok(tb) = handle.cast_inner::<bindings::ITextBlock>() {
        tb.put_FontSize(v)?;
    } else if let Ok(rtb) = handle.cast_inner::<bindings::IRichTextBlock>() {
        rtb.put_FontSize(v)?;
    } else {
        diag::unhandled_modifier("set_prop", Prop::FontSize, handle);
    }
    Ok(true)
}

fn set_font_weight(handle: &Handle, fw: bindings::FontWeight) -> Result<bool> {
    if let Ok(ctrl) = handle.cast_inner::<bindings::IControl>() {
        ctrl.put_FontWeight(fw)?;
    } else if let Ok(tb) = handle.cast_inner::<bindings::ITextBlock>() {
        tb.put_FontWeight(fw)?;
    } else {
        diag::unhandled_modifier("set_prop", Prop::FontWeight, handle);
    }
    Ok(true)
}

fn set_font_family(handle: &Handle, ff: &bindings::FontFamily) -> Result<bool> {
    if let Ok(ctrl) = handle.cast_inner::<bindings::IControl>() {
        ctrl.put_FontFamily(ff)?;
    } else if let Ok(tb) = handle.cast_inner::<bindings::ITextBlock>() {
        tb.put_FontFamily(ff)?;
    } else if let Ok(rtb) = handle.cast_inner::<bindings::IRichTextBlock>() {
        rtb.put_FontFamily(ff)?;
    } else {
        diag::unhandled_modifier("set_prop", Prop::FontFamily, handle);
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

impl Backend for WinUIBackend {
    fn create(&mut self, kind: ControlKind) -> ControlId {
        let id = self.alloc_id();
        let handle = Self::make_handle_for_kind(kind);
        self.controls.borrow_mut().insert(id, handle);
        id
    }
    #[allow(clippy::match_same_arms)] // large dispatch table with semantically distinct no-op arms
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
                    tb.put_IsTextSelectionEnabled(*v)
                }
                (Prop::IsTextSelectionEnabled, PropValue::Unset, Handle::RichTextBlock(tb)) => {
                    tb.put_IsTextSelectionEnabled(false)
                }
                (Prop::TextWrappingWrap, PropValue::I32(v), Handle::RichTextBlock(tb)) => {
                    tb.put_TextWrapping(TextWrapping(*v))
                }
                (Prop::Content, PropValue::Str(s), Handle::Button(b)) => {
                    let cc = b.cast::<bindings::IContentControl>()?;
                    // If the button has an icon+text layout (StackPanel from
                    // Icon), update just the TextBlock child so the icon
                    // is preserved when only the label changes.
                    if let Ok(existing) = cc.get_Content()
                        && let Ok(panel) = existing.cast::<bindings::IPanel>()
                    {
                        let children = panel.get_Children()?;
                        if children.Size()? >= 2
                            && let Ok(tb) = children.GetAt(1)?.cast::<bindings::ITextBlock>()
                        {
                            return tb.put_Text(s);
                        }
                    }
                    let tb = string_as_textblock(s)?;
                    cc.put_Content(&tb)
                }
                (Prop::Icon, PropValue::I32(v), Handle::Button(b)) => {
                    let icon_elem = bindings::SymbolIcon::CreateInstanceWithSymbol(Symbol(*v))?;
                    let cc = b.cast::<bindings::IContentControl>()?;
                    // If the button already has an icon+text StackPanel layout,
                    // replace just the icon child (index 0) to preserve the text.
                    if let Ok(existing) = cc.get_Content()
                        && let Ok(panel) = existing.cast::<bindings::IPanel>()
                    {
                        let children = panel.get_Children()?;
                        if children.Size()? >= 2 {
                            children.SetAt(0, &icon_elem.cast::<bindings::UIElement>()?)?;
                            return Ok(());
                        }
                    }
                    let use_icon_only = if let Ok(existing) = cc.get_Content() {
                        // Already in icon-only mode (existing is a SymbolIcon).
                        existing.cast::<bindings::ISymbolIcon>().is_ok()
                            || existing
                                .cast::<bindings::ITextBlock>()
                                .ok()
                                .and_then(|tb| tb.get_Text().ok())
                                .is_some_and(|t| t.is_empty())
                    } else {
                        true
                    };
                    if use_icon_only {
                        cc.put_Content(&icon_elem)
                    } else {
                        let panel = bindings::StackPanel::new()?;
                        panel.put_Orientation(Orientation::Horizontal)?;
                        panel.put_Spacing(8.0)?;
                        let children = panel.cast::<bindings::IPanel>()?.get_Children()?;
                        children.Append(&icon_elem.cast::<bindings::UIElement>()?)?;
                        if let Ok(existing) = cc.get_Content()
                            && let Ok(ui) = existing.cast::<bindings::UIElement>()
                        {
                            children.Append(&ui)?;
                        }
                        cc.put_Content(&panel)
                    }
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
                        let resources = bindings::Application::get_Current()
                            .and_then(|app| app.get_Resources())?;
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
                            fe.put_Style(&s)?;
                        }
                    } else {
                        fe.put_Style(None)?;
                    }
                    Ok(())
                }
                (Prop::Value, PropValue::Str(s), Handle::TextBox(t)) => {
                    if t.get_Text().ok().as_deref() == Some(s.as_str()) {
                        return Ok(());
                    }
                    t.put_Text(s.as_str())
                }
                (Prop::GridRows, PropValue::GridLengths(rows), Handle::Grid(g)) => {
                    let defs = g.get_RowDefinitions()?;
                    defs.cast::<windows_collections::IVector<bindings::RowDefinition>>()?
                        .Clear()?;
                    for r in rows {
                        let rd = bindings::RowDefinition::new()?;
                        rd.cast::<bindings::IRowDefinition>()?
                            .put_Height(to_xaml_gridlength(*r)?)?;
                        defs.cast::<windows_collections::IVector<bindings::RowDefinition>>()?
                            .Append(&rd)?;
                    }
                    Ok(())
                }
                (Prop::GridColumns, PropValue::GridLengths(cols), Handle::Grid(g)) => {
                    let defs = g.get_ColumnDefinitions()?;
                    defs.cast::<windows_collections::IVector<bindings::ColumnDefinition>>()?
                        .Clear()?;
                    for c in cols {
                        let cd = bindings::ColumnDefinition::new()?;
                        cd.cast::<bindings::IColumnDefinition>()?
                            .put_Width(to_xaml_gridlength(*c)?)?;
                        defs.cast::<windows_collections::IVector<bindings::ColumnDefinition>>()?
                            .Append(&cd)?;
                    }
                    Ok(())
                }
                (Prop::Step, PropValue::F64(v), Handle::Slider(s)) => {
                    s.put_StepFrequency(*v)?;
                    s.cast::<bindings::IRangeBase>()?.put_SmallChange(*v)
                }
                (Prop::Step, PropValue::Unset, Handle::Slider(s)) => {
                    s.put_StepFrequency(1.0)?;
                    s.cast::<bindings::IRangeBase>()?.put_SmallChange(1.0)
                }
                (Prop::NavigateUri, PropValue::Str(s), Handle::HyperlinkButton(h)) => {
                    let uri = bindings::Uri::CreateUri(s.as_str())?;
                    h.put_NavigateUri(&uri)
                }
                (Prop::NavigateUri, PropValue::Unset, Handle::HyperlinkButton(h)) => {
                    h.put_NavigateUri(None)
                }
                (Prop::IsClosable, PropValue::Bool(v), Handle::TabViewItem(ti)) => {
                    ti.put_IsClosable(*v)
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
                                    .and_then(|u| u.get_XamlRoot().ok()),
                            })
                            .next();
                        match xroot {
                            Some(root) => {
                                if let Err(e) =
                                    d.cast::<bindings::IUIElement>()?.put_XamlRoot(&root)
                                    && cfg!(debug_assertions)
                                {
                                    eprintln!(
                                        "windows-reactor: ContentDialog.SetXamlRoot failed: {e}"
                                    );
                                }
                                if let Err(e) = d.ShowAsync()
                                    && cfg!(debug_assertions)
                                {
                                    eprintln!(
                                        "windows-reactor: ContentDialog.ShowAsync failed: {e}"
                                    );
                                }
                            }
                            None => {
                                if cfg!(debug_assertions) {
                                    eprintln!(
                                        "windows-reactor: ContentDialog.is_open ignored — no XamlRoot available"
                                    );
                                }
                            }
                        }
                        Ok(())
                    } else {
                        d.Hide()
                    }
                }
                (Prop::Value, PropValue::I32(v), Handle::InfoBadge(ib)) => {
                    if *v < 0 {
                        ib.put_Value(-1)
                    } else {
                        ib.put_Value(*v)
                    }
                }
                (Prop::DisplayName, PropValue::Unset, Handle::PersonPicture(p)) => {
                    p.put_DisplayName("")
                }
                (Prop::Initials, PropValue::Unset, Handle::PersonPicture(p)) => p.put_Initials(""),
                (Prop::CornerRadius, PropValue::F64(v), Handle::Rectangle(r)) => {
                    r.put_RadiusX(*v).and_then(|_| r.put_RadiusY(*v))
                }
                (Prop::CornerRadius, PropValue::Unset, Handle::Rectangle(r)) => {
                    r.put_RadiusX(0.0).and_then(|_| r.put_RadiusY(0.0))
                }
                (Prop::CornerRadius, PropValue::F64(v), Handle::Border(b)) => {
                    b.put_CornerRadius(bindings::CornerRadius {
                        top_left: *v,
                        top_right: *v,
                        bottom_right: *v,
                        bottom_left: *v,
                    })
                }
                (Prop::CornerRadius, PropValue::Unset, Handle::Border(b)) => {
                    b.put_CornerRadius(bindings::CornerRadius::default())
                }
                (Prop::BorderBrush, PropValue::Color(br), Handle::Border(b)) => {
                    b.put_BorderBrush(&solid_brush(*br)?)
                }
                (Prop::BorderBrush, PropValue::Unset, Handle::Border(b)) => b.put_BorderBrush(None),
                (Prop::BorderBrush, _, h) => {
                    diag::unhandled_modifier("set_prop", Prop::BorderBrush, h);
                    Ok(())
                }
                (Prop::BorderThickness, PropValue::Thickness(t), Handle::Border(b)) => {
                    b.put_BorderThickness(*t)
                }
                (Prop::BorderThickness, PropValue::Unset, Handle::Border(b)) => {
                    b.put_BorderThickness(Thickness::default())
                }
                (Prop::BorderThickness, _, h) => {
                    diag::unhandled_modifier("set_prop", Prop::BorderThickness, h);
                    Ok(())
                }
                (Prop::LineEndpoints, PropValue::LineEndpoints(p), Handle::Line(l)) => l
                    .put_X1(p.x1)
                    .and_then(|_| l.put_Y1(p.y1))
                    .and_then(|_| l.put_X2(p.x2))
                    .and_then(|_| l.put_Y2(p.y2)),
                (Prop::ImageSource, PropValue::Str(s), Handle::Image(img)) => {
                    let uri = bindings::Uri::CreateUri(s.as_str())?;
                    let bmp = bindings::BitmapImage::new()?;
                    bmp.cast::<bindings::IBitmapImage>()?.put_UriSource(&uri)?;
                    img.put_Source(&bmp.cast::<bindings::ImageSource>()?)
                }
                (Prop::ImageSource, PropValue::SurfaceImageSource(sis), Handle::Image(img)) => {
                    img.put_Source(&sis.image_source()?)
                }
                (Prop::ImageSource, PropValue::Unset, Handle::Image(img)) => img.put_Source(None),
                (Prop::Header, PropValue::Str(s), Handle::TabViewItem(ti)) => {
                    let tb = string_as_textblock(s)?;
                    ti.put_Header(&tb)
                }
                (Prop::Header, PropValue::Str(s), Handle::Expander(e)) => {
                    let tb = string_as_textblock(s)?;
                    e.put_Header(&tb)
                }
                (Prop::Header, PropValue::Unset, Handle::Expander(e)) => e.put_Header(None),
                (Prop::ItemKey, PropValue::Str(s), Handle::TabViewItem(ti)) => {
                    let tag = windows_reference::IReference::from(s.as_str());
                    ti.cast::<bindings::IFrameworkElement>()?.put_Tag(&tag)
                }
                (Prop::MenuItems, PropValue::NavMenuItems(items), Handle::NavigationView(nv)) => {
                    let menu = nv.get_MenuItems()?;
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
                    nv.put_SelectedItem(None)
                }
                (Prop::AutoSuggestBox, PropValue::Bool(true), Handle::NavigationView(nv)) => {
                    let asb = bindings::AutoSuggestBox::new()?;
                    nv.put_AutoSuggestBox(&asb)
                }
                (Prop::AutoSuggestBox, PropValue::Bool(false), Handle::NavigationView(nv)) => {
                    nv.put_AutoSuggestBox(None)
                }
                (Prop::AutoSuggestPlaceholder, PropValue::Str(s), Handle::NavigationView(nv)) => {
                    if let Ok(asb) = nv.get_AutoSuggestBox() {
                        asb.put_PlaceholderText(s.as_str())?;
                    }
                    Ok(())
                }
                (Prop::AutoSuggestItems, PropValue::StrList(items), Handle::NavigationView(nv)) => {
                    if let Ok(asb) = nv.get_AutoSuggestBox() {
                        asb.cast::<bindings::IItemsControl>()?
                            .put_ItemsSource(&str_list_as_ivector(items))?;
                    }
                    Ok(())
                }
                (Prop::Tall, PropValue::Bool(v), Handle::TitleBar(_)) => {
                    set_titlebar_height(*v);
                    Ok(())
                }
                (Prop::IsBackButtonVisible, PropValue::Bool(v), Handle::NavigationView(nv)) => {
                    let val = if *v {
                        bindings::NavigationViewBackButtonVisible::Auto
                    } else {
                        bindings::NavigationViewBackButtonVisible::Collapsed
                    };
                    nv.cast::<bindings::INavigationView2>()?
                        .put_IsBackButtonVisible(val)
                }
                (Prop::ItemHeader, PropValue::Str(s), Handle::PivotItem(pi)) => {
                    let tb = string_as_textblock(s)?;
                    pi.put_Header(&tb)
                }
                (Prop::Items, PropValue::StrList(items), Handle::BreadcrumbBar(bc)) => {
                    bc.put_ItemsSource(&str_list_as_ivector(items))
                }
                (Prop::Value, PropValue::Str(s), Handle::PasswordBox(p)) => {
                    if p.get_Password().ok().as_deref() == Some(s.as_str()) {
                        return Ok(());
                    }
                    p.put_Password(s.as_str())
                }
                (Prop::Value, PropValue::Unset, Handle::PasswordBox(p)) => p.put_Password(""),
                (Prop::Items, PropValue::StrList(items), Handle::RadioButtons(r)) => {
                    set_str_items(&r.get_Items()?.cast()?, items)
                }
                (Prop::Items, PropValue::StrList(items), Handle::ComboBox(c)) => set_str_items(
                    &c.cast::<bindings::IItemsControl>()?.get_Items()?.cast()?,
                    items,
                ),
                (Prop::ColorValue, PropValue::Color(c), Handle::ColorPicker(cp)) => {
                    cp.put_Color(*c)
                }
                (Prop::Items, PropValue::StrList(items), Handle::ListBox(lb)) => set_str_items(
                    &lb.cast::<bindings::IItemsControl>()?.get_Items()?.cast()?,
                    items,
                ),
                (Prop::Text, PropValue::Str(s), Handle::AutoSuggestBox(asb)) => {
                    // Skip SetText when the control already has this value —
                    // calling SetText during a user-initiated TextChanged
                    // cycle steals focus from the input field.
                    if asb.get_Text().ok().as_deref() == Some(s.as_str()) {
                        return Ok(());
                    }
                    asb.put_Text(s)
                }
                (Prop::Items, PropValue::StrList(items), Handle::AutoSuggestBox(asb)) => asb
                    .cast::<bindings::IItemsControl>()?
                    .put_ItemsSource(&str_list_as_ivector(items)),
                (Prop::DisplayMode, PropValue::I32(m), Handle::SplitView(sv)) => {
                    sv.put_DisplayMode(bindings::SplitViewDisplayMode(*m))
                }
                (Prop::Items, PropValue::MenuBarItems(items), Handle::MenuBar(mb)) => {
                    let winui_items = mb.get_Items()?;
                    winui_items.Clear()?;
                    for bar_item_def in items {
                        let mbi = bindings::MenuBarItem::new()?;
                        mbi.put_Title(&bar_item_def.title)?;
                        let flyout_items = mbi.get_Items()?;
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
                    let flyout_items = flyout.get_Items()?;
                    for def in items {
                        let fi = build_menu_flyout_item_base(def)?;
                        flyout_items.Append(&fi)?;
                    }
                    btn.cast::<bindings::IButton>()?.put_Flyout(&flyout)?;
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
                    let flyout_items = flyout.get_Items()?;
                    for def in items {
                        let fi = build_menu_flyout_item_base(def)?;
                        flyout_items.Append(&fi)?;
                    }
                    btn.put_Flyout(&flyout)?;
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
                    let primary_cmds = flyout.get_PrimaryCommands()?;
                    let secondary_cmds = flyout.get_SecondaryCommands()?;
                    for def in primary {
                        let el = build_command_bar_element(def)?;
                        primary_cmds.Append(&el)?;
                    }
                    for def in secondary {
                        let el = build_command_bar_element(def)?;
                        secondary_cmds.Append(&el)?;
                    }
                    btn.put_Flyout(&flyout)?;
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
                    let root = tv.get_RootNodes()?;
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
                    let primary = cb.get_PrimaryCommands()?;
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
                    let secondary = cb.get_SecondaryCommands()?;
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
                    tt.put_ActionButtonContent(&boxed)
                }
                (Prop::CloseButton, PropValue::Str(s), Handle::TeachingTip(tt)) => {
                    let boxed: windows_core::IInspectable =
                        windows_reference::IReference::<windows_core::HSTRING>::from(
                            windows_core::HSTRING::from(s.as_str()),
                        )
                        .cast()?;
                    tt.put_CloseButtonContent(&boxed)
                }
                (Prop::Items, PropValue::SelectorBarItems(items), Handle::SelectorBar(sb)) => {
                    let vec = sb.get_Items()?;
                    vec.Clear()?;
                    for def in items {
                        let item = bindings::SelectorBarItem::new()?;
                        item.put_Text(&def.text)?;
                        if let Some(sym) = &def.icon {
                            let icon_elem = bindings::SymbolIcon::CreateInstanceWithSymbol(*sym)?;
                            item.put_Icon(&icon_elem)?;
                        }
                        vec.Append(&item)?;
                    }
                    Ok(())
                }
                (Prop::Text, PropValue::Str(s), Handle::RichEditBox(reb)) => {
                    let doc = reb.get_Document()?;
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
                    reb.put_Header(&tb)
                }
                (Prop::Header, PropValue::Unset, Handle::RichEditBox(reb)) => reb.put_Header(None),
                (Prop::FlyoutContent, PropValue::Str(s), Handle::Button(b)) => {
                    let flyout = bindings::Flyout::new()?;
                    let tb = string_as_textblock(s)?;
                    flyout.put_Content(&tb)?;
                    b.put_Flyout(&flyout)?;
                    Ok(())
                }
                (Prop::FlyoutPlacement, PropValue::I32(v), Handle::Button(b)) => {
                    // The flyout must already exist (FlyoutContent set first).
                    if let Ok(fb) = b.get_Flyout() {
                        let _ = fb
                            .cast::<bindings::IFlyoutBase>()?
                            .put_Placement(FlyoutPlacementMode(*v));
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
            diag::com_error("set_prop", id, &e);
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
    fn set_templated_item_count(&mut self, _id: ControlId, _count: usize) {}
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

        let items_control: bindings::IItemsControl = match list_h {
            Handle::ListView(lv) => lv.cast().unwrap(),
            Handle::GridView(gv) => gv.cast().unwrap(),
            Handle::FlipView(fv) => fv.cast().unwrap(),
            other => panic!(
                "set_templated_row_content: {} is not a templated list",
                describe_kind(other)
            ),
        };
        let items = items_control
            .get_Items()
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
        let _ = selector.put_SelectedIndex(index);
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
        let _ = lvb.put_SelectionMode(winui_mode);
    }

    fn set_templated_can_drag_items(&mut self, id: ControlId, value: bool) {
        let map = self.controls.borrow();
        let Some(handle) = map.get(&id) else { return };
        let lvb: bindings::IListViewBase = match handle {
            Handle::ListView(lv) => lv.cast().unwrap(),
            Handle::GridView(gv) => gv.cast().unwrap(),
            _ => return,
        };
        let _ = lvb.put_CanDragItems(value);
    }

    fn set_templated_can_reorder_items(&mut self, id: ControlId, value: bool) {
        let map = self.controls.borrow();
        let Some(handle) = map.get(&id) else { return };
        let lvb: bindings::IListViewBase = match handle {
            Handle::ListView(lv) => lv.cast().unwrap(),
            Handle::GridView(gv) => gv.cast().unwrap(),
            _ => return,
        };
        let _ = lvb.put_CanReorderItems(value);
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
        let _ = ui.put_AllowDrop(value);
    }

    fn set_header_element(&mut self, id: ControlId, header_id: Option<ControlId>) {
        let map = self.controls.borrow();
        let Some(handle) = map.get(&id) else { return };
        if let Handle::Expander(e) = handle {
            if let Some(hdr_id) = header_id {
                if let Some(hdr_handle) = map.get(&hdr_id) {
                    let ui_elem = hdr_handle.as_ui_element();
                    let _ = e.put_Header(&ui_elem);
                }
            } else {
                let _ = e.put_Header(None);
            }
        } else if let Handle::TitleBar(tb) = handle {
            if let Some(hdr_id) = header_id {
                if let Some(hdr_handle) = map.get(&hdr_id) {
                    let ui_elem = hdr_handle.as_ui_element();
                    let _ = tb.put_Content(&ui_elem);
                }
            } else {
                let _ = tb.put_Content(None);
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
                    let _ = sv.put_Pane(&ui_elem);
                }
            } else {
                let _ = sv.put_Pane(None);
            }
        } else if let Handle::TitleBar(tb) = handle {
            if let Some(pid) = pane_id {
                if let Some(pane_handle) = map.get(&pid) {
                    let ui_elem = pane_handle.as_ui_element();
                    let _ = tb.put_RightHeader(&ui_elem);
                }
            } else {
                let _ = tb.put_RightHeader(None);
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
                let _ = fv
                    .cast::<bindings::ISelector>()
                    .unwrap()
                    .put_SelectedIndex(index);
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
            if let Ok(items) = items_control.get_Items()
                && let Ok(coll) =
                    items.cast::<windows_collections::IVector<windows_core::IInspectable>>()
            {
                let len = coll.Size().unwrap_or(0);
                if (index as u32) < len
                    && let Ok(item) = coll.GetAt(index as u32)
                {
                    let _ = lvb.ScrollIntoView(&item);
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
        let revoker = selector
            .SelectionChanged(move |sender, _args| {
                let idx = sender
                    .as_ref()
                    .and_then(|s| s.cast::<bindings::ISelector>().ok())
                    .and_then(|sel| sel.get_SelectedIndex().ok())
                    .unwrap_or(-1);
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
        _id: ControlId,
        _realize: Rc<dyn Fn(usize)>,
        _recycle: Rc<dyn Fn(usize)>,
    ) {
    }
    fn destroy(&mut self, id: ControlId) {
        // Drop per-control revokers for templated selection and pointer/tap handlers.
        self.templated_selection_revokers.borrow_mut().remove(&id);
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
                            .and_then(|a| a.get_Result().ok())
                            .unwrap_or(bindings::ContentDialogResult(0));
                        handler.invoke_i32(result.0);
                    })
                    .unwrap(),
                );
            }
            (Event::SelectionChanged, Handle::TabView(tv)) => {
                revokers.push(
                    tv.SelectionChanged(move |sender, _args| {
                        let idx = sender
                            .as_ref()
                            .and_then(|s| s.cast::<bindings::TabView>().ok())
                            .and_then(|t| t.get_SelectedIndex().ok())
                            .unwrap_or(-1);
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
                            .and_then(|a| a.get_Tab().ok())
                            .and_then(|tab| {
                                tab.cast::<bindings::IFrameworkElement>()
                                    .unwrap()
                                    .get_Tag()
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
                            .and_then(|a| {
                                a.cast::<bindings::INavigationViewSelectionChangedEventArgs>()
                                    .unwrap()
                                    .get_SelectedItem()
                                    .ok()
                            })
                            .and_then(|item| item.cast::<bindings::NavigationViewItem>().ok())
                            .and_then(|nvi| {
                                nvi.cast::<bindings::IFrameworkElement>()
                                    .unwrap()
                                    .get_Tag()
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
                if let Ok(asb) = nv.get_AutoSuggestBox() {
                    revokers.push(
                        asb.QuerySubmitted(move |_sender, args| {
                            let query = args
                                .as_ref()
                                .and_then(|a| a.get_QueryText().ok())
                                .unwrap_or_default();
                            handler.invoke_string(query);
                        })
                        .unwrap(),
                    );
                }
            }
            (Event::TextChanged, Handle::NavigationView(nv)) => {
                if let Ok(asb) = nv.get_AutoSuggestBox() {
                    revokers.push(
                        asb.TextChanged(move |sender, _args| {
                            let text = sender
                                .as_ref()
                                .and_then(|s| s.get_Text().ok())
                                .unwrap_or_default();
                            handler.invoke_string(text);
                        })
                        .unwrap(),
                    );
                }
            }
            (Event::SuggestionChosen, Handle::NavigationView(nv)) => {
                if let Ok(asb) = nv.get_AutoSuggestBox() {
                    revokers.push(
                        asb.SuggestionChosen(move |_sender, args| {
                            let item = args
                                .as_ref()
                                .and_then(|a| a.get_SelectedItem().ok())
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
                revokers.push(
                    p.SelectionChanged(move |sender, _args| {
                        let idx = sender
                            .as_ref()
                            .and_then(|s| s.cast::<bindings::Selector>().ok())
                            .and_then(|sel| {
                                sel.cast::<bindings::ISelector>()
                                    .unwrap()
                                    .get_SelectedIndex()
                                    .ok()
                            })
                            .unwrap_or(-1);
                        if idx >= 0 {
                            handler.invoke_i32(idx);
                        }
                    })
                    .unwrap(),
                );
            }
            (Event::SelectionChanged, Handle::ComboBox(c)) => {
                revokers.push(
                    c.cast::<bindings::ISelector>()
                        .unwrap()
                        .SelectionChanged(move |sender, _args| {
                            let idx = sender
                                .as_ref()
                                .and_then(|s| s.cast::<bindings::Selector>().ok())
                                .and_then(|sel| {
                                    sel.cast::<bindings::ISelector>()
                                        .unwrap()
                                        .get_SelectedIndex()
                                        .ok()
                                })
                                .unwrap_or(-1);
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
                                .and_then(|a| a.get_NewColor().ok())
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
                            && let Ok(dt) = a.get_NewDate()
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
                            && let Ok(ts) = a.get_NewTime()
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
                            && let Ok(dt) = a.get_NewDate()
                        {
                            handler.invoke_datetime(dt);
                        }
                    })
                    .unwrap(),
                );
            }
            (Event::SelectionChanged, Handle::ListBox(lb)) => {
                revokers.push(
                    lb.cast::<bindings::ISelector>()
                        .unwrap()
                        .SelectionChanged(move |_sender, _args| {
                            if let Some(sel) = _sender.as_ref()
                                && let Ok(idx) = sel
                                    .cast::<bindings::ISelector>()
                                    .and_then(|s| s.get_SelectedIndex())
                            {
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
                            .and_then(|a| a.get_Reason().ok())
                            .is_some_and(|r| {
                                r == bindings::AutoSuggestionBoxTextChangeReason::UserInput
                            });
                        if is_user_input {
                            let text = sender
                                .as_ref()
                                .and_then(|s| s.get_Text().ok())
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
                            .and_then(|a| a.get_QueryText().ok())
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
                            .and_then(|a| a.get_SelectedItem().ok())
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
                            .and_then(|a| a.get_InvokedItem().ok())
                            .and_then(|insp| {
                                insp.cast::<bindings::ITreeViewNode>()
                                    .ok()
                                    .and_then(|node| node.get_Content().ok())
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
                if let Ok(primary) = cb.get_PrimaryCommands() {
                    let revs = Self::wire_command_bar_clicks(&primary, &handler);
                    revokers.extend(revs);
                }
                if let Ok(secondary) = cb.get_SecondaryCommands() {
                    let revs = Self::wire_command_bar_clicks(&secondary, &handler);
                    revokers.extend(revs);
                }
            }
            (Event::SelectionChanged, Handle::SelectorBar(sb)) => {
                let sb2 = sb.clone();
                revokers.push(
                    sb.SelectionChanged(move |_sender, _args| {
                        if let Ok(selected) = sb2.get_SelectedItem()
                            && let Ok(text) = selected.get_Text()
                        {
                            handler.invoke_string(text);
                        }
                    })
                    .unwrap(),
                );
            }
            (Event::TextChanged, Handle::RichEditBox(reb)) => {
                revokers.push(
                    reb.TextChanged(move |sender, _args| {
                        let text = sender
                            .as_ref()
                            .and_then(|s| s.cast::<bindings::RichEditBox>().ok())
                            .and_then(|reb| {
                                let doc = reb.get_Document().ok()?;
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
                let _ = fe.put_Style(None);
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
        let _ = bindings::AutomationProperties::SetName(
            &dep,
            accessibility.automation_name.as_deref().unwrap_or(""),
        );
        let _ = bindings::AutomationProperties::SetAutomationId(
            &dep,
            accessibility.automation_id.as_deref().unwrap_or(""),
        );
        let _ = bindings::AutomationProperties::SetHelpText(
            &dep,
            accessibility.help_text.as_deref().unwrap_or(""),
        );
        let live = accessibility
            .live_setting
            .unwrap_or(AutomationLiveSetting::Off);
        let _ = bindings::AutomationProperties::SetLiveSetting(&dep, live);
        let heading = accessibility
            .heading_level
            .unwrap_or(AutomationHeadingLevel::None);
        let _ = bindings::AutomationProperties::SetHeadingLevel(&dep, heading);
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
            match iue.get_KeyboardAccelerators() {
                Ok(v) => v,
                Err(_) => return,
            };
        let _ = vec.Clear();

        // Suppress the default accelerator tooltip that WinUI would otherwise show.
        let _ = iue.put_KeyboardAcceleratorPlacementMode(
            bindings::KeyboardAcceleratorPlacementMode::Hidden,
        );

        for accel in accelerators {
            let Ok(ka) = bindings::KeyboardAccelerator::new() else {
                continue;
            };
            let Ok(ika) = ka.cast::<bindings::IKeyboardAccelerator>() else {
                continue;
            };
            let _ = ika.put_Key(accel.key);
            let _ = ika.put_Modifiers(accel.modifiers);
            let cb = accel.on_invoked.clone();
            let _ = ika
                .Invoked(move |_sender, args| {
                    if let Some(a) = args.as_ref()
                        && let Ok(ia) = a.cast::<bindings::IKeyboardAcceleratorInvokedEventArgs>()
                    {
                        let _ = ia.put_Handled(true);
                    }
                    cb.invoke(());
                })
                .ok()
                .map(|r| r.into_token());
            let _ = vec.Append(&ka);
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
        if let Err(e) = apply_implicit_transitions(&ui, transitions)
            && cfg!(debug_assertions)
        {
            eprintln!("windows-reactor: set_implicit_transitions failed: {e:?}");
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
        if let Err(e) = run_property_animation_inner(&ui, cfg)
            && cfg!(debug_assertions)
        {
            eprintln!("windows-reactor: run_property_animation failed: {e:?}");
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
        let Ok(blocks) = rtb.get_Blocks() else { return };
        let _ = blocks.Clear();
        for para_def in paragraphs {
            let Ok(para) = bindings::Paragraph::new() else {
                continue;
            };
            let Ok(inlines) = para.get_Inlines() else {
                continue;
            };
            for inline in &para_def.inlines {
                match inline {
                    RichTextInline::Run(r) => {
                        let Ok(run) = bindings::Run::new() else {
                            continue;
                        };
                        let _ = run.put_Text(&r.text);
                        if r.is_bold {
                            let _ = run.cast::<bindings::ITextElement>().and_then(|te| {
                                te.put_FontWeight(bindings::FontWeight { weight: 700 })
                            });
                        }
                        let _ = run
                            .cast::<bindings::Inline>()
                            .and_then(|i| inlines.Append(&i));
                    }
                    RichTextInline::LineBreak => {
                        // LineBreak inline — use a Run with newline.
                        let Ok(run) = bindings::Run::new() else {
                            continue;
                        };
                        let _ = run.put_Text("\n");
                        let _ = run
                            .cast::<bindings::Inline>()
                            .and_then(|i| inlines.Append(&i));
                    }
                    RichTextInline::Hyperlink(h) => {
                        // Hyperlink rendered as plain text (no navigation support yet).
                        let Ok(run) = bindings::Run::new() else {
                            continue;
                        };
                        let _ = run.put_Text(&h.text);
                        let _ = run
                            .cast::<bindings::Inline>()
                            .and_then(|i| inlines.Append(&i));
                    }
                }
            }
            let _ = para
                .cast::<bindings::Block>()
                .and_then(|b| blocks.Append(&b));
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
                            if cfg!(debug_assertions) {
                                eprintln!("windows-reactor: ToolTip::new failed: {e:?}");
                            }
                            return;
                        }
                    };
                    if let Some(ui) = mount_static_tooltip_element(elem)
                        && let Ok(cc) = tt.cast::<bindings::IContentControl>()
                    {
                        let _ = cc.put_Content(&ui);
                    }
                    Some(tt.into())
                }
            },
        };
        let _ = bindings::ToolTipService::SetToolTip(&dep, inspectable.as_ref());

        // Fall back to Top so cleared placements actually reset the slot.
        let placement = tooltip
            .and_then(|t| t.placement)
            .map_or(bindings::PlacementMode::Top, map_placement);
        let _ = bindings::ToolTipService::SetPlacement(&dep, placement);
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
        let iue: bindings::IUIElement = match ui.cast() {
            Ok(i) => i,
            Err(_) => return,
        };
        drop(prev);

        let Some(handlers) = handlers else {
            return;
        };
        let mut tokens = PointerRevokerSet::default();

        if let Some(cb) = handlers.on_tapped.clone() {
            tokens.tapped = iue
                .Tapped(move |_sender, _args| {
                    cb.invoke(());
                })
                .ok();
        }

        if let Some(cb) = handlers.on_right_tapped.clone() {
            tokens.right_tapped = iue
                .RightTapped(move |_sender, _args| {
                    cb.invoke(());
                })
                .ok();
        }

        if let Some(cb) = handlers.on_pointer_pressed.clone() {
            tokens.pressed = iue
                .PointerPressed(move |sender, args| {
                    let info = pointer_event_info(sender, args);
                    cb.invoke(info);
                })
                .ok();
        }

        if let Some(cb) = handlers.on_pointer_released.clone() {
            tokens.released = iue
                .PointerReleased(move |sender, args| {
                    let info = pointer_event_info(sender, args);
                    cb.invoke(info);
                })
                .ok();
        }

        if let Some(cb) = handlers.on_pointer_exited.clone() {
            tokens.exited = iue
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
        let ui_element: bindings::IUIElement = match ui.cast() {
            Ok(i) => i,
            Err(_) => return,
        };
        drop(prev);

        let Some(handlers) = handlers else {
            return;
        };
        let mut tokens = DragRevokerSet::default();

        if let Some(callback) = handlers.on_drag_enter.clone() {
            let marshaller = WinUIDispatcher::for_current_thread()
                .map(|dispatcher| dispatcher.marshaller())
                .ok();

            tokens.enter = ui_element
                .DragEnter(move |_sender, args| {
                    let Some(args) = args.as_ref() else { return };
                    let Ok(drag_event_args) = args.cast::<bindings::IDragEventArgs>() else {
                        return;
                    };

                    let formats = drag_event_args
                        .get_DataView()
                        .ok()
                        .and_then(|dv| dv.cast::<bindings::IDataPackageView>().ok())
                        .map(|data_package_view| read_available_formats(&data_package_view))
                        .unwrap_or_default();

                    let agile_deferral = drag_event_args
                        .GetDeferral()
                        .ok()
                        .and_then(|deferral| {
                            deferral.cast::<bindings::IDragOperationDeferral>().ok()
                        })
                        .and_then(|deferral| windows_core::AgileReference::new(&deferral).ok());

                    let agile_args = windows_core::AgileReference::new(&drag_event_args).ok();

                    let callback = callback.clone();
                    let marshaller = marshaller.clone();
                    windows_threading::submit(move || {
                        let Some(marshaller) = marshaller else {
                            if let Some(deferral) =
                                agile_deferral.and_then(|agile_ref| agile_ref.resolve().ok())
                            {
                                let _ = deferral.Complete();
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
            tokens.leave = ui_element
                .DragLeave(move |_sender, args| {
                    let ctx = build_drag_context(args.as_ref());
                    cb.call(&ctx);
                })
                .ok();
        }

        if let Some(cb) = handlers.on_drag_over.clone() {
            tokens.over = ui_element
                .DragOver(move |_sender, args| {
                    accept_or_reject(&cb, args.as_ref());
                })
                .ok();
        }

        if let Some(callback) = handlers.on_drag_drop.clone() {
            let marshaller = WinUIDispatcher::for_current_thread()
                .map(|dispatcher| dispatcher.marshaller())
                .ok();

            tokens.drop = ui_element
                .Drop(move |_sender, args| {
                    let Some(args) = args.as_ref() else { return };
                    let Ok(drag_event_args) = args.cast::<bindings::IDragEventArgs>() else {
                        return;
                    };

                    let data_view =
                        drag_event_args
                            .get_DataView()
                            .ok()
                            .and_then(|data_package_view| {
                                data_package_view.cast::<bindings::IDataPackageView>().ok()
                            });

                    let formats = data_view
                        .as_ref()
                        .map(read_available_formats)
                        .unwrap_or_default();

                    let agile_deferral = drag_event_args
                        .GetDeferral()
                        .ok()
                        .and_then(|deferral| {
                            deferral.cast::<bindings::IDragOperationDeferral>().ok()
                        })
                        .and_then(|deferral| windows_core::AgileReference::new(&deferral).ok());

                    let agile_data_view = data_view.and_then(|data_package_view| {
                        windows_core::AgileReference::new(&data_package_view).ok()
                    });

                    let agile_args = windows_core::AgileReference::new(&drag_event_args).ok();

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
                                            path: item.get_Path().unwrap_or_default(),
                                            name: item.get_Name().unwrap_or_default(),
                                            is_folder: item.get_Attributes().is_ok_and(|attrs| {
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
                                let _ = deferral.Complete();
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

fn read_available_formats(data_package_view: &bindings::IDataPackageView) -> AvailableFormats {
    let mut available_formats = AvailableFormats::default();
    let Ok(formats) = data_package_view.get_AvailableFormats() else {
        return available_formats;
    };

    let size = formats.Size().unwrap_or(0);
    for i in 0..size {
        if let Ok(s) = formats.GetAt(i) {
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
    let Ok(iargs) = a.cast::<bindings::IDragEventArgs>() else {
        return ctx;
    };
    let Ok(dv) = iargs.get_DataView() else {
        return ctx;
    };
    let Ok(idv) = dv.cast::<bindings::IDataPackageView>() else {
        return ctx;
    };

    let formats = read_available_formats(&idv);
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
        let size = items.Size().unwrap_or(0);
        (0..size)
            .filter_map(|i| items.GetAt(i).ok())
            .map(|item| DroppedItem {
                path: item.get_Path().unwrap_or_default(),
                name: item.get_Name().unwrap_or_default(),
                is_folder: item
                    .get_Attributes()
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
    iargs_agile: Option<windows_core::AgileReference<bindings::IDragEventArgs>>,
    deferral_agile: Option<windows_core::AgileReference<bindings::IDragOperationDeferral>>,
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
            let _ = iargs.put_AcceptedOperation(accepted);
            if (ctx.caption.is_some()
                || ctx.glyph_visible.is_some()
                || ctx.content_visible.is_some())
                && let Ok(ui) = iargs.get_DragUIOverride()
            {
                if let Some(v) = ctx.caption {
                    let _ = ui.put_Caption(&v);
                }
                if let Some(v) = ctx.glyph_visible {
                    let _ = ui.put_IsGlyphVisible(v);
                }
                if let Some(v) = ctx.content_visible {
                    let _ = ui.put_IsContentVisible(v);
                }
            }
        }
        if let Some(d) = deferral_agile.and_then(|a| a.resolve().ok()) {
            let _ = d.Complete();
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
    let Ok(iargs) = a.cast::<bindings::IDragEventArgs>() else {
        return;
    };

    let mut ctx = build_drag_context(Some(a));

    let result = cb.call(&mut ctx);

    let accepted = match result {
        DragOperation::None => bindings::DataPackageOperation::None,
        DragOperation::Copy => bindings::DataPackageOperation::Copy,
        DragOperation::Move => bindings::DataPackageOperation::Move,
        DragOperation::Link => bindings::DataPackageOperation::Link,
    };
    let _ = iargs.put_AcceptedOperation(accepted);

    if (ctx.caption.is_some() || ctx.glyph_visible.is_some() || ctx.content_visible.is_some())
        && let Ok(ui) = iargs.get_DragUIOverride()
    {
        if let Some(v) = ctx.caption {
            let _ = ui.put_Caption(&v);
        }
        if let Some(v) = ctx.glyph_visible {
            let _ = ui.put_IsGlyphVisible(v);
        }
        if let Some(v) = ctx.content_visible {
            let _ = ui.put_IsContentVisible(v);
        }
    }
}

/// Extract the button state for a `PointerPressed` / `PointerReleased`
/// callback; falls back to all-`false` on any null hop.
fn pointer_event_info(
    sender: windows_core::InRef<'_, windows_core::IInspectable>,
    args: windows_core::InRef<'_, bindings::PointerRoutedEventArgs>,
) -> PointerEventInfo {
    let mut info = PointerEventInfo::default();
    let Some(args) = args.as_ref() else {
        return info;
    };
    let Ok(iargs) = args.cast::<bindings::IPointerRoutedEventArgs>() else {
        return info;
    };
    let relative: Option<bindings::UIElement> = sender
        .as_ref()
        .and_then(|s| s.cast::<bindings::UIElement>().ok());
    let point = match relative.as_ref() {
        Some(ue) => iargs.GetCurrentPoint(ue),
        None => iargs.GetCurrentPoint(None),
    };
    let Ok(point) = point else {
        return info;
    };
    let Ok(ipoint) = point.cast::<bindings::IPointerPoint>() else {
        return info;
    };
    let Ok(props) = ipoint.get_Properties() else {
        return info;
    };
    let Ok(iprops) = props.cast::<bindings::IPointerPointProperties>() else {
        return info;
    };
    info.is_left_button_pressed = iprops.get_IsLeftButtonPressed().unwrap_or(false);
    info.is_right_button_pressed = iprops.get_IsRightButtonPressed().unwrap_or(false);
    info.is_middle_button_pressed = iprops.get_IsMiddleButtonPressed().unwrap_or(false);
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
            tb.put_Text(t.text.as_str()).ok()?;
            tb.cast::<bindings::UIElement>().ok()
        }
        Element::StackPanel(s) => {
            let sp = bindings::StackPanel::new().ok()?;
            sp.put_Orientation(s.orientation).ok()?;
            sp.put_Spacing(s.spacing).ok()?;
            let children = sp
                .cast::<bindings::IPanel>()
                .ok()?
                .get_Children()
                .ok()?
                .cast::<windows_collections::IVector<bindings::UIElement>>()
                .ok()?;
            for child in &s.children {
                if let Some(cui) = mount_static_tooltip_element(child) {
                    let _ = children.Append(&cui);
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
                        if let Ok(ibmp) = bmp.cast::<bindings::IBitmapImage>() {
                            let _ = ibmp.put_UriSource(&uri);
                        }
                        if let Ok(src) = bmp.cast::<bindings::ImageSource>() {
                            let _ = i.put_Source(&src);
                        }
                    }
                }
                ImageSource::Surface(sis) => {
                    if let Ok(src) = sis.image_source() {
                        let _ = i.put_Source(&src);
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
            tb.put_Text(el.kind_name()).ok()?;
            tb.cast::<bindings::UIElement>().ok()
        }
    }
}
