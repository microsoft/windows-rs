use std::cell::RefCell;

use rustc_hash::FxHashMap;
use windows_core::Interface;

use super::*;
use crate::bindings as Xaml;
use Xaml::FontWeight as WinFontWeight;

mod convert;
mod diag;
mod generated_attach_event;
mod generated_set_prop;
use convert::*;

/// Single source of truth for the `Handle` enum, its casts, the
/// `ControlKind`→`Handle` constructor, and `describe_kind`. Each variant name
/// matches its backing `Xaml::Variant` class and must correspond to a
/// `ControlKind` variant; the class must be activatable.
macro_rules! define_handles {
    ( $( $variant:ident ),* $(,)? ) => {
        enum Handle {
            $( $variant(Xaml::$variant), )*
        }

        impl Handle {
            fn cast_inner<T: windows_core::Interface>(&self) -> windows_core::Result<T> {
                match self {
                    $( Handle::$variant(v) => v.cast::<T>(), )*
                }
            }
            fn as_framework_element(&self) -> Xaml::FrameworkElement {
                self.cast_inner().unwrap()
            }
            fn as_ui_element(&self) -> Xaml::UIElement {
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
                            <Xaml::$variant>::new().unwrap(),
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
    Image,
    TabView,
    TabViewItem,
    NavigationView,
    TitleBar,
    Pivot,
    PivotItem,
    BreadcrumbBar,
    PasswordBox,
    RadioButtons,
    ComboBox,
    Canvas,
    Rectangle,
    Ellipse,
    Line,
    ListView,
    GridView,
    FlipView,
    ContentDialog,
    RichTextBlock,
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

/// [`Backend`] implementation that creates real `Microsoft.UI.Xaml`
/// controls and drives them on the WinUI thread.
pub struct WinUIBackend {
    controls: RefCell<FxHashMap<ControlId, Handle>>,
    event_revokers: RefCell<FxHashMap<(ControlId, Event), Vec<windows_core::EventRevoker>>>,
    templated_selection_revokers: RefCell<FxHashMap<ControlId, windows_core::EventRevoker>>,
    /// Pointer-handler revokers (separate from `event_revokers` because
    /// pointer events use the universal `IUIElement` event surface).
    pointer_revokers: RefCell<FxHashMap<ControlId, PointerRevokerSet>>,
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
    pub fn find_titlebar(&self) -> Option<Xaml::TitleBar> {
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
        mb: &Xaml::MenuBar,
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
        flyout: &Xaml::MenuFlyout,
        handler: &EventHandler,
    ) -> Vec<windows_core::EventRevoker> {
        let mut revokers = Vec::new();
        if let Ok(items) = flyout.get_Items() {
            Self::wire_flyout_items_click(&items, handler, &mut revokers);
        }
        revokers
    }

    fn wire_flyout_items_click(
        items: &windows_collections::IVector<Xaml::MenuFlyoutItemBase>,
        handler: &EventHandler,
        revokers: &mut Vec<windows_core::EventRevoker>,
    ) {
        for i in 0..items.Size().unwrap_or(0) {
            let Ok(base) = items.GetAt(i) else { continue };
            if let Ok(item) = base.cast::<Xaml::MenuFlyoutItem>() {
                let text = item.get_Text().unwrap_or_default().clone();
                let handler = handler.clone();
                if let Ok(rev) = item.add_Click(move |_s, _a| {
                    handler.invoke_string(text.clone());
                }) {
                    revokers.push(rev);
                }
            } else if let Ok(sub) = base.cast::<Xaml::MenuFlyoutSubItem>()
                && let Ok(sub_items) = sub.get_Items()
            {
                Self::wire_flyout_items_click(&sub_items, handler, revokers);
            }
        }
    }

    fn wire_command_bar_clicks(
        commands: &windows_collections::IObservableVector<Xaml::ICommandBarElement>,
        handler: &EventHandler,
    ) -> Vec<windows_core::EventRevoker> {
        let mut revokers = Vec::new();
        for i in 0..commands.Size().unwrap_or(0) {
            let Ok(el) = commands.GetAt(i) else { continue };
            if let Ok(btn) = el.cast::<Xaml::AppBarButton>() {
                let label = btn.get_Label().unwrap_or_default().clone();
                let handler = handler.clone();
                if let Ok(rev) = btn.cast::<Xaml::ButtonBase>().and_then(|bb| {
                    bb.add_Click(move |_s, _a| {
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
    Panel(windows_collections::IVector<Xaml::UIElement>),
    /// Single-child container (Border, Viewbox) that uses `put_Child`.
    SingleChild(&'a Handle),
    /// Single-child `IContentControl` (ScrollViewer, Expander,
    /// TabViewItem, NavigationView, PivotItem).
    ContentControl(Xaml::IContentControl),
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
            s.cast::<Xaml::IPanel>()
                .ok()?
                .get_Children()
                .ok()?
                .cast()
                .ok()?,
        )),
        Handle::Grid(g) => Some(ContainerChildren::Panel(
            g.cast::<Xaml::IPanel>()
                .ok()?
                .get_Children()
                .ok()?
                .cast()
                .ok()?,
        )),
        Handle::Canvas(c) => Some(ContainerChildren::Panel(
            c.cast::<Xaml::IPanel>()
                .ok()?
                .get_Children()
                .ok()?
                .cast()
                .ok()?,
        )),
        Handle::RelativePanel(r) => Some(ContainerChildren::Panel(
            r.cast::<Xaml::IPanel>()
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
            p.cast::<Xaml::IItemsControl>()
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
fn container_append(cc: &ContainerChildren<'_>, child: &Xaml::UIElement) {
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
fn container_insert(cc: &ContainerChildren<'_>, index: usize, child: &Xaml::UIElement) {
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
fn container_set(cc: &ContainerChildren<'_>, index: usize, child: &Xaml::UIElement) {
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

fn put_single_child(h: &Handle, child: Option<&Xaml::UIElement>) {
    match h {
        Handle::Border(b) => b.put_Child(child).unwrap(),
        Handle::Viewbox(v) => v.put_Child(child).unwrap(),
        _ => unreachable!(),
    }
}

fn put_direct_content(h: &Handle, child: Option<&Xaml::UIElement>) {
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

    match Xaml::XamlReader::Load(&xaml) {
        Ok(obj) => {
            if let Ok(style) = obj.cast::<Xaml::Style>() {
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
fn style_target_for_handle(handle: &Handle) -> Option<(&'static str, Xaml::IFrameworkElement)> {
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
    compositor: &Xaml::ICompositor,
    easing: Easing,
) -> Result<Xaml::CompositionEasingFunction> {
    use Easing;
    // Control points match the CSS-standard ease-{out,in,in-out} curves.
    let (p1, p2) = match easing {
        Easing::Linear => {
            let lin = compositor.CreateLinearEasingFunction()?;
            return lin.cast::<Xaml::CompositionEasingFunction>();
        }
        Easing::EaseOut => (
            windows_numerics::Vector2 { X: 0.0, Y: 0.0 },
            windows_numerics::Vector2 { X: 0.58, Y: 1.0 },
        ),
        Easing::EaseIn => (
            windows_numerics::Vector2 { X: 0.42, Y: 0.0 },
            windows_numerics::Vector2 { X: 1.0, Y: 1.0 },
        ),
        Easing::EaseInOut => (
            windows_numerics::Vector2 { X: 0.42, Y: 0.0 },
            windows_numerics::Vector2 { X: 0.58, Y: 1.0 },
        ),
    };
    let cubic = compositor.CreateCubicBezierEasingFunction(p1, p2)?;
    cubic.cast::<Xaml::CompositionEasingFunction>()
}

fn apply_implicit_transitions(
    ui: &Xaml::UIElement,
    transitions: Option<ImplicitTransitions>,
) -> Result<()> {
    use Easing;
    let visual = Xaml::ElementCompositionPreview::GetElementVisual(ui)?;
    let obj2 = visual.cast::<Xaml::ICompositionObject2>()?;
    // No transitions, or every slot empty: clear the implicit-animation collection.
    let Some(t) = transitions.filter(|t| !t.is_empty()) else {
        obj2.put_ImplicitAnimations(None)?;
        return Ok(());
    };
    let compositor = visual
        .cast::<Xaml::ICompositionObject>()?
        .get_Compositor()?;
    let icomp = compositor.cast::<Xaml::ICompositor>()?;
    let icomp2 = compositor.cast::<Xaml::ICompositor2>()?;
    let collection = icomp2.CreateImplicitAnimationCollection()?;
    let map = collection
        .cast::<windows_collections::IMap<windows_core::HSTRING, Xaml::ICompositionAnimationBase>>(
        )?;

    // Animate from `this.StartingValue` to `this.FinalValue` over `duration`.
    // The DSL transition types only expose duration, so easing is fixed to
    // EaseOut — the standard XAML implicit-transition curve.
    let insert = |target: &str, duration: std::time::Duration, is_scalar: bool| -> Result<()> {
        let easing = easing_for(&icomp, Easing::EaseOut)?;
        let target_hs = windows_core::HSTRING::from(target);
        let final_expr = "this.FinalValue";
        let anim_base: Xaml::ICompositionAnimationBase = if is_scalar {
            let a = icomp.CreateScalarKeyFrameAnimation()?;
            let kfa = a.cast::<Xaml::IKeyFrameAnimation>()?;
            kfa.put_Duration(anim_duration_to_timespan(duration))?;
            kfa.InsertExpressionKeyFrameWithEasingFunction(1.0, final_expr, &easing)?;
            let anim2 = a.cast::<Xaml::ICompositionAnimation2>()?;
            anim2.put_Target(target)?;
            a.cast::<Xaml::ICompositionAnimationBase>()?
        } else {
            let a = icomp.CreateVector3KeyFrameAnimation()?;
            let kfa = a.cast::<Xaml::IKeyFrameAnimation>()?;
            kfa.put_Duration(anim_duration_to_timespan(duration))?;
            kfa.InsertExpressionKeyFrameWithEasingFunction(1.0, final_expr, &easing)?;
            let anim2 = a.cast::<Xaml::ICompositionAnimation2>()?;
            anim2.put_Target(target)?;
            a.cast::<Xaml::ICompositionAnimationBase>()?
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

fn run_property_animation_inner(ui: &Xaml::UIElement, cfg: AnimationConfig) -> Result<()> {
    let visual = Xaml::ElementCompositionPreview::GetElementVisual(ui)?;
    let visual_obj = visual.cast::<Xaml::ICompositionObject>()?;
    let compositor = visual_obj.get_Compositor()?;
    let icomp = compositor.cast::<Xaml::ICompositor>()?;

    if let Some(opacity) = cfg.opacity {
        let a = icomp.CreateScalarKeyFrameAnimation()?;
        let ia = a.cast::<Xaml::IScalarKeyFrameAnimation>()?;
        a.cast::<Xaml::IKeyFrameAnimation>()?
            .put_Duration(anim_duration_to_timespan(cfg.duration))?;
        let easing = easing_for(&icomp, cfg.easing)?;
        ia.InsertKeyFrameWithEasingFunction(1.0, opacity as f32, &easing)?;
        let anim = a.cast::<Xaml::CompositionAnimation>()?;
        visual_obj.StartAnimation("Opacity", &anim)?;
    }
    if let Some(scale) = cfg.scale {
        // Pivot scale around the element's centre. KNOWN LIMITATION:
        // before the first layout pass ActualWidth/Height are 0 and the
        // previous CenterPoint is reused.
        let iv = visual.cast::<Xaml::IVisual>()?;
        if let Ok(fe) = ui.cast::<Xaml::IFrameworkElement>() {
            let w = fe.get_ActualWidth().unwrap_or(0.0) as f32;
            let h = fe.get_ActualHeight().unwrap_or(0.0) as f32;
            if w > 0.0 && h > 0.0 {
                let _ = iv.put_CenterPoint(windows_numerics::Vector3 {
                    X: w / 2.0,
                    Y: h / 2.0,
                    Z: 0.0,
                });
            } else if cfg!(debug_assertions) {
                eprintln!(
                    "windows-reactor: animation: skipping CenterPoint — element not yet laid out"
                );
            }
        }
        // Preserve current Scale.Z; cfg.scale is a uniform X/Y scalar.
        let current_z = iv.get_Scale().map_or(1.0, |v| v.Z);
        let a = icomp.CreateVector3KeyFrameAnimation()?;
        let ia = a.cast::<Xaml::IVector3KeyFrameAnimation>()?;
        a.cast::<Xaml::IKeyFrameAnimation>()?
            .put_Duration(anim_duration_to_timespan(cfg.duration))?;
        let easing = easing_for(&icomp, cfg.easing)?;
        let s = scale as f32;
        ia.InsertKeyFrameWithEasingFunction(
            1.0,
            windows_numerics::Vector3 {
                X: s,
                Y: s,
                Z: current_z,
            },
            &easing,
        )?;
        let anim = a.cast::<Xaml::CompositionAnimation>()?;
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
            set_font_weight(handle, WinFontWeight { Weight: *w })
        }
        (Prop::FontWeight, PropValue::Unset) => {
            set_font_weight(handle, WinFontWeight { Weight: 400 })
        }
        (Prop::FontFamily, PropValue::Str(s)) => {
            set_font_family(handle, &Xaml::FontFamily::CreateInstanceWithName(s)?)
        }
        (Prop::FontFamily, PropValue::Unset) => set_font_family(
            handle,
            &Xaml::FontFamily::CreateInstanceWithName("Segoe UI")?,
        ),
        (Prop::Margin, PropValue::Thickness(t)) => {
            handle
                .as_framework_element()
                .put_Margin(to_xaml_thickness(*t))?;
            Ok(true)
        }
        (Prop::Margin, PropValue::Unset) => {
            handle
                .as_framework_element()
                .put_Margin(to_xaml_thickness(Thickness::default()))?;
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
        (Prop::IsEnabled, PropValue::Unset) => {
            handle
                .as_ui_element()
                .cast::<Xaml::IControl>()?
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
            Xaml::Grid::SetRow(&handle.as_framework_element(), *v)?;
            Ok(true)
        }
        (Prop::AttachedGridColumn, PropValue::I32(v)) => {
            Xaml::Grid::SetColumn(&handle.as_framework_element(), *v)?;
            Ok(true)
        }
        (Prop::AttachedGridRowSpan, PropValue::I32(v)) => {
            Xaml::Grid::SetRowSpan(&handle.as_framework_element(), *v)?;
            Ok(true)
        }
        (Prop::AttachedGridColumnSpan, PropValue::I32(v)) => {
            Xaml::Grid::SetColumnSpan(&handle.as_framework_element(), *v)?;
            Ok(true)
        }
        (Prop::AttachedCanvasLeft, PropValue::F64(v)) => {
            Xaml::Canvas::SetLeft(&handle.as_ui_element(), *v)?;
            Ok(true)
        }
        (Prop::AttachedCanvasTop, PropValue::F64(v)) => {
            Xaml::Canvas::SetTop(&handle.as_ui_element(), *v)?;
            Ok(true)
        }
        (Prop::AttachedCanvasZIndex, PropValue::I32(v)) => {
            Xaml::Canvas::SetZIndex(&handle.as_ui_element(), *v)?;
            Ok(true)
        }
        (Prop::AlignLeftWithPanel, PropValue::Bool(v)) => {
            Xaml::RelativePanel::SetAlignLeftWithPanel(&handle.as_ui_element(), *v)?;
            Ok(true)
        }
        (Prop::AlignRightWithPanel, PropValue::Bool(v)) => {
            Xaml::RelativePanel::SetAlignRightWithPanel(&handle.as_ui_element(), *v)?;
            Ok(true)
        }
        (Prop::AlignTopWithPanel, PropValue::Bool(v)) => {
            Xaml::RelativePanel::SetAlignTopWithPanel(&handle.as_ui_element(), *v)?;
            Ok(true)
        }
        (Prop::AlignBottomWithPanel, PropValue::Bool(v)) => {
            Xaml::RelativePanel::SetAlignBottomWithPanel(&handle.as_ui_element(), *v)?;
            Ok(true)
        }
        (Prop::AlignHCenterWithPanel, PropValue::Bool(v)) => {
            Xaml::RelativePanel::SetAlignHorizontalCenterWithPanel(&handle.as_ui_element(), *v)?;
            Ok(true)
        }
        (Prop::AlignVCenterWithPanel, PropValue::Bool(v)) => {
            Xaml::RelativePanel::SetAlignVerticalCenterWithPanel(&handle.as_ui_element(), *v)?;
            Ok(true)
        }
        (Prop::Padding, PropValue::Thickness(t)) => set_padding(handle, to_xaml_thickness(*t)),
        (Prop::Padding, PropValue::Unset) => {
            set_padding(handle, to_xaml_thickness(Thickness::default()))
        }
        (Prop::Background, PropValue::Brush(br)) => set_background(handle, &brush_of(br)?),
        (Prop::Background, PropValue::Unset) => set_background(handle, None::<&Xaml::Brush>),
        (Prop::Foreground, PropValue::Brush(br)) => set_foreground(handle, &brush_of(br)?),
        (Prop::Foreground, PropValue::Unset) => set_foreground(handle, None::<&Xaml::Brush>),
        (Prop::Fill, PropValue::Brush(b)) => {
            handle
                .cast_inner::<Xaml::IShape>()?
                .put_Fill(&brush_of(b)?)?;
            Ok(true)
        }
        (Prop::Fill, PropValue::Unset) => {
            handle.cast_inner::<Xaml::IShape>()?.put_Fill(None)?;
            Ok(true)
        }
        (Prop::Stroke, PropValue::Brush(b)) => {
            handle
                .cast_inner::<Xaml::IShape>()?
                .put_Stroke(&brush_of(b)?)?;
            Ok(true)
        }
        (Prop::Stroke, PropValue::Unset) => {
            handle.cast_inner::<Xaml::IShape>()?.put_Stroke(None)?;
            Ok(true)
        }
        (Prop::StrokeThickness, PropValue::F64(v)) => {
            handle
                .cast_inner::<Xaml::IShape>()?
                .put_StrokeThickness(*v)?;
            Ok(true)
        }
        (Prop::StrokeThickness, PropValue::Unset) => {
            handle
                .cast_inner::<Xaml::IShape>()?
                .put_StrokeThickness(0.0)?;
            Ok(true)
        }
        _ => Ok(false),
    }
}

fn set_padding(handle: &Handle, thickness: Xaml::Thickness) -> Result<bool> {
    if let Ok(border) = handle.cast_inner::<Xaml::IBorder>() {
        border.put_Padding(thickness)?;
    } else if let Ok(ctl) = handle.cast_inner::<Xaml::IControl>() {
        ctl.put_Padding(thickness)?;
    } else {
        diag::unhandled_modifier("set_prop", Prop::Padding, handle);
    }
    Ok(true)
}

fn set_background(handle: &Handle, brush: impl windows_core::Param<Xaml::Brush>) -> Result<bool> {
    if let Ok(panel) = handle.cast_inner::<Xaml::IPanel>() {
        panel.put_Background(brush)?;
    } else if let Ok(ctl) = handle.cast_inner::<Xaml::IControl>() {
        ctl.put_Background(brush)?;
    } else if let Ok(border) = handle.cast_inner::<Xaml::IBorder>() {
        border.put_Background(brush)?;
    } else {
        diag::unhandled_modifier("set_prop", Prop::Background, handle);
    }
    Ok(true)
}

fn set_foreground(handle: &Handle, brush: impl windows_core::Param<Xaml::Brush>) -> Result<bool> {
    if let Ok(ctl) = handle.cast_inner::<Xaml::IControl>() {
        ctl.put_Foreground(brush)?;
    } else if let Ok(tb) = handle.cast_inner::<Xaml::ITextBlock>() {
        tb.put_Foreground(brush)?;
    } else if let Ok(rtb) = handle.cast_inner::<Xaml::IRichTextBlock>() {
        rtb.put_Foreground(brush)?;
    } else {
        diag::unhandled_modifier("set_prop", Prop::Foreground, handle);
    }
    Ok(true)
}

fn set_font_f64(handle: &Handle, v: f64) -> Result<bool> {
    if let Ok(ctrl) = handle.cast_inner::<Xaml::IControl>() {
        ctrl.put_FontSize(v)?;
    } else if let Ok(tb) = handle.cast_inner::<Xaml::ITextBlock>() {
        tb.put_FontSize(v)?;
    } else if let Ok(rtb) = handle.cast_inner::<Xaml::IRichTextBlock>() {
        rtb.put_FontSize(v)?;
    } else {
        diag::unhandled_modifier("set_prop", Prop::FontSize, handle);
    }
    Ok(true)
}

fn set_font_weight(handle: &Handle, fw: WinFontWeight) -> Result<bool> {
    if let Ok(ctrl) = handle.cast_inner::<Xaml::IControl>() {
        ctrl.put_FontWeight(fw)?;
    } else if let Ok(tb) = handle.cast_inner::<Xaml::ITextBlock>() {
        tb.put_FontWeight(fw)?;
    } else {
        diag::unhandled_modifier("set_prop", Prop::FontWeight, handle);
    }
    Ok(true)
}

fn set_font_family(handle: &Handle, ff: &Xaml::FontFamily) -> Result<bool> {
    if let Ok(ctrl) = handle.cast_inner::<Xaml::IControl>() {
        ctrl.put_FontFamily(ff)?;
    } else if let Ok(tb) = handle.cast_inner::<Xaml::ITextBlock>() {
        tb.put_FontFamily(ff)?;
    } else if let Ok(rtb) = handle.cast_inner::<Xaml::IRichTextBlock>() {
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
                    let cc = b.cast::<Xaml::IContentControl>()?;
                    // If the button has an icon+text layout (StackPanel from
                    // Icon), update just the TextBlock child so the icon
                    // is preserved when only the label changes.
                    if let Ok(existing) = cc.get_Content()
                        && let Ok(panel) = existing.cast::<Xaml::IPanel>()
                    {
                        let children = panel.get_Children()?;
                        if children.Size()? >= 2
                            && let Ok(tb) = children.GetAt(1)?.cast::<Xaml::ITextBlock>()
                        {
                            return tb.put_Text(s);
                        }
                    }
                    let tb = string_as_textblock(s)?;
                    cc.put_Content(&tb)
                }
                (Prop::Icon, PropValue::I32(v), Handle::Button(b)) => {
                    let icon_elem = Xaml::SymbolIcon::CreateInstanceWithSymbol(Symbol(*v))?;
                    let cc = b.cast::<Xaml::IContentControl>()?;
                    // If the button already has an icon+text StackPanel layout,
                    // replace just the icon child (index 0) to preserve the text.
                    if let Ok(existing) = cc.get_Content()
                        && let Ok(panel) = existing.cast::<Xaml::IPanel>()
                    {
                        let children = panel.get_Children()?;
                        if children.Size()? >= 2 {
                            children.SetAt(0, &icon_elem.cast::<Xaml::UIElement>()?)?;
                            return Ok(());
                        }
                    }
                    let use_icon_only = if let Ok(existing) = cc.get_Content() {
                        // Already in icon-only mode (existing is a SymbolIcon).
                        existing.cast::<Xaml::ISymbolIcon>().is_ok()
                            || existing
                                .cast::<Xaml::ITextBlock>()
                                .ok()
                                .and_then(|tb| tb.get_Text().ok())
                                .is_some_and(|t| t.is_empty())
                    } else {
                        true
                    };
                    if use_icon_only {
                        cc.put_Content(&icon_elem)
                    } else {
                        let panel = Xaml::StackPanel::new()?;
                        panel.put_Orientation(Orientation::Horizontal)?;
                        panel.put_Spacing(8.0)?;
                        let children = panel.cast::<Xaml::IPanel>()?.get_Children()?;
                        children.Append(&icon_elem.cast::<Xaml::UIElement>()?)?;
                        if let Ok(existing) = cc.get_Content()
                            && let Ok(ui) = existing.cast::<Xaml::UIElement>()
                        {
                            children.Append(&ui)?;
                        }
                        cc.put_Content(&panel)
                    }
                }
                (Prop::StyleVariant, PropValue::I32(v), Handle::Button(b)) => {
                    let fe = b.cast::<Xaml::IFrameworkElement>()?;
                    let style_key = match *v {
                        1 => Some("AccentButtonStyle"),
                        2 => Some("SubtleButtonStyle"),
                        3 => Some("TextBlockButtonStyle"),
                        _ => None, // 0 = Default
                    };
                    if let Some(key_str) = style_key {
                        let resources =
                            Xaml::Application::get_Current().and_then(|app| app.get_Resources())?;
                        let key = windows_reference::IReference::from(windows_core::HSTRING::from(
                            key_str,
                        ));
                        let map = resources.cast::<windows_collections::IMap<
                            windows_core::IInspectable,
                            windows_core::IInspectable,
                        >>()?;
                        if let Ok(style_obj) = map.Lookup(&key)
                            && let Ok(s) = style_obj.cast::<Xaml::Style>()
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
                    defs.cast::<windows_collections::IVector<Xaml::RowDefinition>>()?
                        .Clear()?;
                    for r in rows {
                        let rd = Xaml::RowDefinition::new()?;
                        rd.cast::<Xaml::IRowDefinition>()?
                            .put_Height(to_xaml_gridlength(*r)?)?;
                        defs.cast::<windows_collections::IVector<Xaml::RowDefinition>>()?
                            .Append(&rd)?;
                    }
                    Ok(())
                }
                (Prop::GridColumns, PropValue::GridLengths(cols), Handle::Grid(g)) => {
                    let defs = g.get_ColumnDefinitions()?;
                    defs.cast::<windows_collections::IVector<Xaml::ColumnDefinition>>()?
                        .Clear()?;
                    for c in cols {
                        let cd = Xaml::ColumnDefinition::new()?;
                        cd.cast::<Xaml::IColumnDefinition>()?
                            .put_Width(to_xaml_gridlength(*c)?)?;
                        defs.cast::<windows_collections::IVector<Xaml::ColumnDefinition>>()?
                            .Append(&cd)?;
                    }
                    Ok(())
                }
                (Prop::Step, PropValue::F64(v), Handle::Slider(s)) => {
                    s.put_StepFrequency(*v)?;
                    s.cast::<Xaml::IRangeBase>()?.put_SmallChange(*v)
                }
                (Prop::Step, PropValue::Unset, Handle::Slider(s)) => {
                    s.put_StepFrequency(1.0)?;
                    s.cast::<Xaml::IRangeBase>()?.put_SmallChange(1.0)
                }
                (Prop::NavigateUri, PropValue::Str(s), Handle::HyperlinkButton(h)) => {
                    let uri = Xaml::Uri::CreateUri(s.as_str())?;
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
                                    .cast::<Xaml::IUIElement>()
                                    .ok()
                                    .and_then(|u| u.get_XamlRoot().ok()),
                            })
                            .next();
                        match xroot {
                            Some(root) => {
                                if let Err(e) = d.cast::<Xaml::IUIElement>()?.put_XamlRoot(&root)
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
                    b.put_CornerRadius(Xaml::CornerRadius {
                        TopLeft: *v,
                        TopRight: *v,
                        BottomRight: *v,
                        BottomLeft: *v,
                    })
                }
                (Prop::CornerRadius, PropValue::Unset, Handle::Border(b)) => {
                    b.put_CornerRadius(Xaml::CornerRadius::default())
                }
                (Prop::BorderBrush, PropValue::Brush(br), Handle::Border(b)) => {
                    b.put_BorderBrush(&brush_of(br)?)
                }
                (Prop::BorderBrush, PropValue::Unset, Handle::Border(b)) => b.put_BorderBrush(None),
                (Prop::BorderBrush, _, h) => {
                    diag::unhandled_modifier("set_prop", Prop::BorderBrush, h);
                    Ok(())
                }
                (Prop::BorderThickness, PropValue::Thickness(t), Handle::Border(b)) => {
                    b.put_BorderThickness(to_xaml_thickness(*t))
                }
                (Prop::BorderThickness, PropValue::Unset, Handle::Border(b)) => {
                    b.put_BorderThickness(to_xaml_thickness(Thickness::default()))
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
                    let uri = Xaml::Uri::CreateUri(s.as_str())?;
                    let bmp = Xaml::BitmapImage::new()?;
                    bmp.cast::<Xaml::IBitmapImage>()?.put_UriSource(&uri)?;
                    img.put_Source(&bmp.cast::<Xaml::ImageSource>()?)
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
                    ti.cast::<Xaml::IFrameworkElement>()?.put_Tag(&tag)
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
                    let asb = Xaml::AutoSuggestBox::new()?;
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
                        asb.cast::<Xaml::IItemsControl>()?
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
                        Xaml::NavigationViewBackButtonVisible::Auto
                    } else {
                        Xaml::NavigationViewBackButtonVisible::Collapsed
                    };
                    nv.cast::<Xaml::INavigationView2>()?
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
                    &c.cast::<Xaml::IItemsControl>()?.get_Items()?.cast()?,
                    items,
                ),
                (Prop::ColorValue, PropValue::Color { a, r, g, b }, Handle::ColorPicker(cp)) => cp
                    .put_Color(Xaml::Color {
                        A: *a,
                        R: *r,
                        G: *g,
                        B: *b,
                    }),
                (Prop::Items, PropValue::StrList(items), Handle::ListBox(lb)) => set_str_items(
                    &lb.cast::<Xaml::IItemsControl>()?.get_Items()?.cast()?,
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
                    .cast::<Xaml::IItemsControl>()?
                    .put_ItemsSource(&str_list_as_ivector(items)),
                (Prop::DisplayMode, PropValue::I32(m), Handle::SplitView(sv)) => {
                    sv.put_DisplayMode(Xaml::SplitViewDisplayMode(*m))
                }
                (Prop::Items, PropValue::MenuBarItems(items), Handle::MenuBar(mb)) => {
                    let winui_items = mb.get_Items()?;
                    winui_items.Clear()?;
                    for bar_item_def in items {
                        let mbi = Xaml::MenuBarItem::new()?;
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
                    let flyout = Xaml::MenuFlyout::new()?;
                    let flyout_items = flyout.get_Items()?;
                    for def in items {
                        let fi = build_menu_flyout_item_base(def)?;
                        flyout_items.Append(&fi)?;
                    }
                    btn.cast::<Xaml::IButton>()?.put_Flyout(&flyout)?;
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
                    let flyout = Xaml::MenuFlyout::new()?;
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
                    let flyout = Xaml::CommandBarFlyout::new()?;
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
                        let item = Xaml::SelectorBarItem::new()?;
                        item.put_Text(&def.text)?;
                        if let Some(sym) = &def.icon {
                            let icon_elem = Xaml::SymbolIcon::CreateInstanceWithSymbol(*sym)?;
                            item.put_Icon(&icon_elem)?;
                        }
                        vec.Append(&item)?;
                    }
                    Ok(())
                }
                (Prop::Text, PropValue::Str(s), Handle::RichEditBox(reb)) => {
                    let doc = reb.get_Document()?;
                    let mut current = windows_core::HSTRING::default();
                    doc.GetText(Xaml::TextGetOptions::None, &mut current).ok();
                    if current == s.as_str() {
                        return Ok(());
                    }
                    doc.SetText(Xaml::TextSetOptions::None, s.as_str())
                }
                (Prop::Header, PropValue::Str(s), Handle::RichEditBox(reb)) => {
                    let tb = string_as_textblock(s)?;
                    reb.put_Header(&tb)
                }
                (Prop::Header, PropValue::Unset, Handle::RichEditBox(reb)) => reb.put_Header(None),
                (Prop::FlyoutContent, PropValue::Str(s), Handle::Button(b)) => {
                    let flyout = Xaml::Flyout::new()?;
                    let tb = string_as_textblock(s)?;
                    flyout.put_Content(&tb)?;
                    b.put_Flyout(&flyout)?;
                    Ok(())
                }
                (Prop::FlyoutPlacement, PropValue::I32(v), Handle::Button(b)) => {
                    // The flyout must already exist (FlyoutContent set first).
                    if let Ok(fb) = b.get_Flyout() {
                        let _ = fb
                            .cast::<Xaml::IFlyoutBase>()?
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

        let items_control: Xaml::IItemsControl = match list_h {
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
                            Xaml::TextBlock::new().unwrap().cast().unwrap();
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
        let selector: Xaml::ISelector = match handle {
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
        let lvb: Xaml::IListViewBase = match handle {
            Handle::ListView(lv) => lv.cast().unwrap(),
            Handle::GridView(gv) => gv.cast().unwrap(),
            // FlipView doesn't support SelectionMode.
            _ => return,
        };
        use SelectionMode;
        let winui_mode = match mode {
            SelectionMode::None => Xaml::ListViewSelectionMode::None,
            SelectionMode::Single => Xaml::ListViewSelectionMode::Single,
            SelectionMode::Multiple => Xaml::ListViewSelectionMode::Multiple,
            SelectionMode::Extended => Xaml::ListViewSelectionMode::Extended,
        };
        let _ = lvb.put_SelectionMode(winui_mode);
    }

    fn set_templated_can_drag_items(&mut self, id: ControlId, value: bool) {
        let map = self.controls.borrow();
        let Some(handle) = map.get(&id) else { return };
        let lvb: Xaml::IListViewBase = match handle {
            Handle::ListView(lv) => lv.cast().unwrap(),
            Handle::GridView(gv) => gv.cast().unwrap(),
            _ => return,
        };
        let _ = lvb.put_CanDragItems(value);
    }

    fn set_templated_can_reorder_items(&mut self, id: ControlId, value: bool) {
        let map = self.controls.borrow();
        let Some(handle) = map.get(&id) else { return };
        let lvb: Xaml::IListViewBase = match handle {
            Handle::ListView(lv) => lv.cast().unwrap(),
            Handle::GridView(gv) => gv.cast().unwrap(),
            _ => return,
        };
        let _ = lvb.put_CanReorderItems(value);
    }

    fn set_templated_allow_drop(&mut self, id: ControlId, value: bool) {
        let map = self.controls.borrow();
        let Some(handle) = map.get(&id) else { return };
        let ui: Xaml::IUIElement = match handle {
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
        let lvb: Option<Xaml::IListViewBase> = match handle {
            Handle::ListView(lv) => lv.cast().ok(),
            Handle::GridView(gv) => gv.cast().ok(),
            Handle::FlipView(fv) => {
                let _ = fv
                    .cast::<Xaml::ISelector>()
                    .unwrap()
                    .put_SelectedIndex(index);
                None
            }
            _ => return,
        };
        if let Some(lvb) = lvb {
            let items_control: Xaml::IItemsControl = match handle {
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

        let selector: Xaml::ISelector = match handle {
            Handle::ListView(lv) => lv.cast().unwrap(),
            Handle::GridView(gv) => gv.cast().unwrap(),
            Handle::FlipView(fv) => fv.cast().unwrap(),
            _ => return,
        };
        // Drop any prior registration so repeated attaches don't
        // accumulate handlers.
        self.templated_selection_revokers.borrow_mut().remove(&id);
        let revoker = selector
            .add_SelectionChanged(move |sender, _args| {
                let idx = sender
                    .as_ref()
                    .and_then(|s| s.cast::<Xaml::ISelector>().ok())
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
                    d.add_Closed(move |_sender, args| {
                        let result = args
                            .as_ref()
                            .and_then(|a| a.get_Result().ok())
                            .unwrap_or(Xaml::ContentDialogResult(0));
                        handler.invoke_i32(result.0);
                    })
                    .unwrap(),
                );
            }
            (Event::SelectionChanged, Handle::TabView(tv)) => {
                revokers.push(
                    tv.add_SelectionChanged(move |sender, _args| {
                        let idx = sender
                            .as_ref()
                            .and_then(|s| s.cast::<Xaml::TabView>().ok())
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
                    tv.add_TabCloseRequested(move |_sender, args| {
                        let key = args
                            .as_ref()
                            .and_then(|a| a.get_Tab().ok())
                            .and_then(|tab| {
                                tab.cast::<Xaml::IFrameworkElement>()
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
                    nv.add_SelectionChanged(move |_sender, args| {
                        let tag = args
                            .as_ref()
                            .and_then(|a| {
                                a.cast::<Xaml::INavigationViewSelectionChangedEventArgs>()
                                    .unwrap()
                                    .get_SelectedItem()
                                    .ok()
                            })
                            .and_then(|item| item.cast::<Xaml::NavigationViewItem>().ok())
                            .and_then(|nvi| {
                                nvi.cast::<Xaml::IFrameworkElement>()
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
                        asb.add_QuerySubmitted(move |_sender, args| {
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
                        asb.add_TextChanged(move |sender, _args| {
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
                        asb.add_SuggestionChosen(move |_sender, args| {
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
                    p.add_SelectionChanged(move |sender, _args| {
                        let idx = sender
                            .as_ref()
                            .and_then(|s| s.cast::<Xaml::Selector>().ok())
                            .and_then(|sel| {
                                sel.cast::<Xaml::ISelector>()
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
                    c.cast::<Xaml::ISelector>()
                        .unwrap()
                        .add_SelectionChanged(move |sender, _args| {
                            let idx = sender
                                .as_ref()
                                .and_then(|s| s.cast::<Xaml::Selector>().ok())
                                .and_then(|sel| {
                                    sel.cast::<Xaml::ISelector>()
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
                    cp.add_ColorChanged(move |_sender, args| {
                        let color = args.as_ref().and_then(|a| a.get_NewColor().ok()).unwrap_or(
                            Xaml::Color {
                                A: 255,
                                R: 0,
                                G: 0,
                                B: 0,
                            },
                        );
                        handler.invoke_color((color.A, color.R, color.G, color.B));
                    })
                    .unwrap(),
                );
            }
            (Event::SelectedDateChanged, Handle::DatePicker(dp)) => {
                revokers.push(
                    dp.add_SelectedDateChanged(move |_sender, args| {
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
                    tp.add_SelectedTimeChanged(move |_sender, args| {
                        if let Some(a) = args.as_ref()
                            && let Ok(ts) = a.get_NewTime()
                        {
                            handler.invoke_timespan(TimeSpan::from_ticks(ts.Duration));
                        }
                    })
                    .unwrap(),
                );
            }
            (Event::CalendarDateSelected, Handle::CalendarDatePicker(cdp)) => {
                revokers.push(
                    cdp.add_DateChanged(move |_sender, args| {
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
                    lb.cast::<Xaml::ISelector>()
                        .unwrap()
                        .add_SelectionChanged(move |_sender, _args| {
                            if let Some(sel) = _sender.as_ref()
                                && let Ok(idx) = sel
                                    .cast::<Xaml::ISelector>()
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
                    asb.add_TextChanged(move |sender, args| {
                        // Only fire for user input, not programmatic changes.
                        let is_user_input = args
                            .as_ref()
                            .and_then(|a| a.get_Reason().ok())
                            .is_some_and(|r| {
                                r == Xaml::AutoSuggestionBoxTextChangeReason::UserInput
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
                    asb.add_QuerySubmitted(move |_sender, args| {
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
                    asb.add_SuggestionChosen(move |_sender, args| {
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
                    tv.add_ItemInvoked(move |_sender, args| {
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
                    sb.add_SelectionChanged(move |_sender, _args| {
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
                    reb.add_TextChanged(move |sender, _args| {
                        let text = sender
                            .as_ref()
                            .and_then(|s| s.cast::<Xaml::RichEditBox>().ok())
                            .and_then(|reb| {
                                let doc = reb.get_Document().ok()?;
                                let mut buf = windows_core::HSTRING::default();
                                doc.GetText(Xaml::TextGetOptions::None, &mut buf).ok()?;
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
        let dep: Xaml::DependencyObject = match fe.cast() {
            Ok(d) => d,
            Err(_) => return,
        };
        let _ = Xaml::AutomationProperties::SetName(
            &dep,
            accessibility.automation_name.as_deref().unwrap_or(""),
        );
        let _ = Xaml::AutomationProperties::SetAutomationId(
            &dep,
            accessibility.automation_id.as_deref().unwrap_or(""),
        );
        let _ = Xaml::AutomationProperties::SetHelpText(
            &dep,
            accessibility.help_text.as_deref().unwrap_or(""),
        );
        let live = match accessibility.live_setting {
            Some(LiveSetting::Off) | None => Xaml::AutomationLiveSetting::Off,
            Some(LiveSetting::Polite) => Xaml::AutomationLiveSetting::Polite,
            Some(LiveSetting::Assertive) => Xaml::AutomationLiveSetting::Assertive,
        };
        let _ = Xaml::AutomationProperties::SetLiveSetting(&dep, live);
        let heading = match accessibility.heading_level {
            None => Xaml::AutomationHeadingLevel::None,
            Some(HeadingLevel::Level1) => Xaml::AutomationHeadingLevel::Level1,
            Some(HeadingLevel::Level2) => Xaml::AutomationHeadingLevel::Level2,
            Some(HeadingLevel::Level3) => Xaml::AutomationHeadingLevel::Level3,
            Some(HeadingLevel::Level4) => Xaml::AutomationHeadingLevel::Level4,
            Some(HeadingLevel::Level5) => Xaml::AutomationHeadingLevel::Level5,
            Some(HeadingLevel::Level6) => Xaml::AutomationHeadingLevel::Level6,
            Some(HeadingLevel::Level7) => Xaml::AutomationHeadingLevel::Level7,
            Some(HeadingLevel::Level8) => Xaml::AutomationHeadingLevel::Level8,
            Some(HeadingLevel::Level9) => Xaml::AutomationHeadingLevel::Level9,
        };
        let _ = Xaml::AutomationProperties::SetHeadingLevel(&dep, heading);
    }
    fn set_keyboard_accelerators(&mut self, id: ControlId, accelerators: &[KeyboardAccelerator]) {
        let map = self.controls.borrow();
        let Some(handle) = map.get(&id) else {
            return;
        };
        let fe = handle.as_framework_element();
        let iue: Xaml::IUIElement = match fe.cast() {
            Ok(i) => i,
            Err(_) => return,
        };
        let vec: windows_collections::IVector<Xaml::KeyboardAccelerator> =
            match iue.get_KeyboardAccelerators() {
                Ok(v) => v,
                Err(_) => return,
            };
        let _ = vec.Clear();

        // Suppress the default accelerator tooltip that WinUI would otherwise show.
        let _ = iue
            .put_KeyboardAcceleratorPlacementMode(Xaml::KeyboardAcceleratorPlacementMode::Hidden);

        for accel in accelerators {
            let Ok(ka) = Xaml::KeyboardAccelerator::new() else {
                continue;
            };
            let Ok(ika) = ka.cast::<Xaml::IKeyboardAccelerator>() else {
                continue;
            };
            let _ = ika.put_Key(reactor_key_to_virtual_key(accel.key));
            let _ = ika.put_Modifiers(Xaml::VirtualKeyModifiers(accel.modifiers.0));
            let cb = accel.on_invoked.clone();
            let _ = ika
                .add_Invoked(move |_sender, args| {
                    if let Some(a) = args.as_ref()
                        && let Ok(ia) = a.cast::<Xaml::IKeyboardAcceleratorInvokedEventArgs>()
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
        let ui: Xaml::UIElement = handle.as_ui_element();
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
        let ui: Xaml::UIElement = handle.as_ui_element();
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
            let Ok(para) = Xaml::Paragraph::new() else {
                continue;
            };
            let Ok(inlines) = para.get_Inlines() else {
                continue;
            };
            for inline in &para_def.inlines {
                match inline {
                    RichTextInline::Run(r) => {
                        let Ok(run) = Xaml::Run::new() else { continue };
                        let _ = run.put_Text(&r.text);
                        if r.is_bold {
                            let _ = run
                                .cast::<Xaml::ITextElement>()
                                .and_then(|te| te.put_FontWeight(WinFontWeight { Weight: 700 }));
                        }
                        let _ = run.cast::<Xaml::Inline>().and_then(|i| inlines.Append(&i));
                    }
                    RichTextInline::LineBreak => {
                        // LineBreak inline — use a Run with newline.
                        let Ok(run) = Xaml::Run::new() else { continue };
                        let _ = run.put_Text("\n");
                        let _ = run.cast::<Xaml::Inline>().and_then(|i| inlines.Append(&i));
                    }
                    RichTextInline::Hyperlink(h) => {
                        // Hyperlink rendered as plain text (no navigation support yet).
                        let Ok(run) = Xaml::Run::new() else { continue };
                        let _ = run.put_Text(&h.text);
                        let _ = run.cast::<Xaml::Inline>().and_then(|i| inlines.Append(&i));
                    }
                }
            }
            let _ = para.cast::<Xaml::Block>().and_then(|b| blocks.Append(&b));
        }
    }

    fn set_tooltip(&mut self, id: ControlId, tooltip: Option<&Tooltip>) {
        let map = self.controls.borrow();
        let Some(handle) = map.get(&id) else {
            return;
        };
        let fe = handle.as_framework_element();
        let dep: Xaml::DependencyObject = match fe.cast() {
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
                    let tt = match Xaml::ToolTip::new() {
                        Ok(t) => t,
                        Err(e) => {
                            if cfg!(debug_assertions) {
                                eprintln!("windows-reactor: ToolTip::new failed: {e:?}");
                            }
                            return;
                        }
                    };
                    if let Some(ui) = mount_static_tooltip_element(elem)
                        && let Ok(cc) = tt.cast::<Xaml::IContentControl>()
                    {
                        let _ = cc.put_Content(&ui);
                    }
                    Some(tt.into())
                }
            },
        };
        let _ = Xaml::ToolTipService::SetToolTip(&dep, inspectable.as_ref());

        // Fall back to Top so cleared placements actually reset the slot.
        let placement = tooltip
            .and_then(|t| t.placement)
            .map_or(Xaml::PlacementMode::Top, map_placement);
        let _ = Xaml::ToolTipService::SetPlacement(&dep, placement);
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
        let iue: Xaml::IUIElement = match ui.cast() {
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
                .add_Tapped(move |_sender, _args| {
                    cb.invoke(());
                })
                .ok();
        }

        if let Some(cb) = handlers.on_right_tapped.clone() {
            tokens.right_tapped = iue
                .add_RightTapped(move |_sender, _args| {
                    cb.invoke(());
                })
                .ok();
        }

        if let Some(cb) = handlers.on_pointer_pressed.clone() {
            tokens.pressed = iue
                .add_PointerPressed(move |sender, args| {
                    let info = pointer_event_info(sender, args);
                    cb.invoke(info);
                })
                .ok();
        }

        if let Some(cb) = handlers.on_pointer_released.clone() {
            tokens.released = iue
                .add_PointerReleased(move |sender, args| {
                    let info = pointer_event_info(sender, args);
                    cb.invoke(info);
                })
                .ok();
        }

        if let Some(cb) = handlers.on_pointer_exited.clone() {
            tokens.exited = iue
                .add_PointerExited(move |_sender, _args| {
                    cb.invoke(());
                })
                .ok();
        }

        self.pointer_revokers.borrow_mut().insert(id, tokens);
    }

    fn get_native_element(&self, id: ControlId) -> Option<windows_core::IInspectable> {
        self.get_ui_element(id)
    }
}

/// Extract the button state for a `PointerPressed` / `PointerReleased`
/// callback; falls back to all-`false` on any null hop.
fn pointer_event_info(
    sender: windows_core::InRef<'_, windows_core::IInspectable>,
    args: windows_core::InRef<'_, Xaml::PointerRoutedEventArgs>,
) -> PointerEventInfo {
    let mut info = PointerEventInfo::default();
    let Some(args) = args.as_ref() else {
        return info;
    };
    let Ok(iargs) = args.cast::<Xaml::IPointerRoutedEventArgs>() else {
        return info;
    };
    let relative: Option<Xaml::UIElement> = sender
        .as_ref()
        .and_then(|s| s.cast::<Xaml::UIElement>().ok());
    let point = match relative.as_ref() {
        Some(ue) => iargs.GetCurrentPoint(ue),
        None => iargs.GetCurrentPoint(None),
    };
    let Ok(point) = point else {
        return info;
    };
    let Ok(ipoint) = point.cast::<Xaml::IPointerPoint>() else {
        return info;
    };
    let Ok(props) = ipoint.get_Properties() else {
        return info;
    };
    let Ok(iprops) = props.cast::<Xaml::IPointerPointProperties>() else {
        return info;
    };
    info.is_left_button_pressed = iprops.get_IsLeftButtonPressed().unwrap_or(false);
    info.is_right_button_pressed = iprops.get_IsRightButtonPressed().unwrap_or(false);
    info.is_middle_button_pressed = iprops.get_IsMiddleButtonPressed().unwrap_or(false);
    info
}

fn map_placement(p: TooltipPlacement) -> Xaml::PlacementMode {
    use TooltipPlacement;
    match p {
        TooltipPlacement::Top => Xaml::PlacementMode::Top,
        TooltipPlacement::Bottom => Xaml::PlacementMode::Bottom,
        TooltipPlacement::Left => Xaml::PlacementMode::Left,
        TooltipPlacement::Right => Xaml::PlacementMode::Right,
        TooltipPlacement::Mouse => Xaml::PlacementMode::Mouse,
    }
}

/// Best-effort static mount for tooltip content. Supports `TextBlock`,
/// linear `StackPanel`, and `Image`; unsupported kinds fall back to a
/// `TextBlock` showing `kind_name()`.
fn mount_static_tooltip_element(el: &Element) -> Option<Xaml::UIElement> {
    match el {
        Element::TextBlock(t) => {
            let tb = Xaml::TextBlock::new().ok()?;
            tb.put_Text(t.text.as_str()).ok()?;
            tb.cast::<Xaml::UIElement>().ok()
        }
        Element::StackPanel(s) => {
            let sp = Xaml::StackPanel::new().ok()?;
            sp.put_Orientation(s.orientation).ok()?;
            sp.put_Spacing(s.spacing).ok()?;
            let children = sp
                .cast::<Xaml::IPanel>()
                .ok()?
                .get_Children()
                .ok()?
                .cast::<windows_collections::IVector<Xaml::UIElement>>()
                .ok()?;
            for child in &s.children {
                if let Some(cui) = mount_static_tooltip_element(child) {
                    let _ = children.Append(&cui);
                }
            }
            sp.cast::<Xaml::UIElement>().ok()
        }
        Element::Image(img) => {
            let i = Xaml::Image::new().ok()?;
            match &img.source {
                ImageSource::Uri(uri_str) => {
                    if let Ok(uri) = Xaml::Uri::CreateUri(uri_str.as_str())
                        && let Ok(bmp) = Xaml::BitmapImage::new()
                    {
                        if let Ok(ibmp) = bmp.cast::<Xaml::IBitmapImage>() {
                            let _ = ibmp.put_UriSource(&uri);
                        }
                        if let Ok(src) = bmp.cast::<Xaml::ImageSource>() {
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
            i.cast::<Xaml::UIElement>().ok()
        }
        _ => {
            // Fallback: surface the kind_name so the developer sees a
            // hint rather than an empty popup.
            let tb = Xaml::TextBlock::new().ok()?;
            tb.put_Text(el.kind_name()).ok()?;
            tb.cast::<Xaml::UIElement>().ok()
        }
    }
}
