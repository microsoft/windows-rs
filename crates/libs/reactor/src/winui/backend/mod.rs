use std::cell::RefCell;

use rustc_hash::FxHashMap;
use windows_core::Interface;

use super::*;
use crate::bindings as Xaml;
use Xaml::FontWeight as WinFontWeight;

mod convert;
mod diag;
mod elements;
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
    theme_brush_registry: RefCell<FxHashMap<ControlId, Vec<(Prop, crate::core::theme::ThemeRef)>>>,
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
        match parent_h {
            Handle::StackPanel(_) | Handle::Grid(_) | Handle::Canvas(_) => {
                let vec = panel_children_vec(parent_h).unwrap();
                vec.InsertAt(v_index as u32, &child_ui).unwrap();
            }
            Handle::Border(b) => {
                b.put_Child(&child_ui).unwrap();
            }
            Handle::Viewbox(v) => {
                v.put_Child(&child_ui).unwrap();
            }
            Handle::ScrollViewer(_)
            | Handle::Expander(_)
            | Handle::TabViewItem(_)
            | Handle::NavigationView(_)
            | Handle::PivotItem(_) => {
                let cc = content_control_for(parent_h).unwrap();
                cc.put_Content(&child_ui).unwrap();
            }
            Handle::ScrollView(sv) => {
                sv.put_Content(&child_ui).unwrap();
            }
            Handle::TabView(tv) => {
                let items = tv.get_TabItems().unwrap();
                let insp: windows_core::IInspectable = child_ui.cast().unwrap();
                items.InsertAt(v_index as u32, &insp).unwrap();
            }
            Handle::Pivot(p) => {
                let items = p
                    .cast::<Xaml::IItemsControl>()
                    .unwrap()
                    .get_Items()
                    .unwrap()
                    .cast::<windows_collections::IVector<windows_core::IInspectable>>()
                    .unwrap();
                let insp: windows_core::IInspectable = child_ui.cast().unwrap();
                items.InsertAt(v_index as u32, &insp).unwrap();
            }
            _ => panic!("WinUIBackend::visual_insert_at: {parent} is not a container"),
        }
    }
    /// Remove the child at visual index `v_index`. Logical mirror is NOT touched.
    fn visual_remove_at(&self, parent: ControlId, v_index: usize) {
        let map = self.controls.borrow();
        let parent_h = map
            .get(&parent)
            .unwrap_or_else(|| panic!("WinUIBackend::visual_remove_at: unknown parent {parent}"));
        match parent_h {
            Handle::StackPanel(_) | Handle::Grid(_) | Handle::Canvas(_) => {
                if let Some(vec) = panel_children_vec(parent_h) {
                    vec.RemoveAt(v_index as u32).unwrap();
                }
            }
            Handle::Border(b) => {
                debug_assert_eq!(v_index, 0);
                b.put_Child(None).unwrap();
            }
            Handle::Viewbox(v) => {
                debug_assert_eq!(v_index, 0);
                v.put_Child(None).unwrap();
            }
            Handle::ScrollViewer(_)
            | Handle::Expander(_)
            | Handle::TabViewItem(_)
            | Handle::NavigationView(_)
            | Handle::PivotItem(_) => {
                debug_assert_eq!(v_index, 0);
                if let Some(cc) = content_control_for(parent_h) {
                    cc.put_Content(None).unwrap();
                }
            }
            Handle::ScrollView(sv) => {
                debug_assert_eq!(v_index, 0);
                sv.put_Content(None).unwrap();
            }
            Handle::TabView(tv) => {
                let items = tv.get_TabItems().unwrap();
                items.RemoveAt(v_index as u32).unwrap();
            }
            Handle::Pivot(p) => {
                let items = p
                    .cast::<Xaml::IItemsControl>()
                    .unwrap()
                    .get_Items()
                    .unwrap()
                    .cast::<windows_collections::IVector<windows_core::IInspectable>>()
                    .unwrap();
                items.RemoveAt(v_index as u32).unwrap();
            }
            _ => panic!("WinUIBackend::visual_remove_at: {parent} is not a container"),
        }
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
        match parent_h {
            Handle::StackPanel(_) | Handle::Grid(_) | Handle::Canvas(_) => {
                let vec = panel_children_vec(parent_h).unwrap();
                vec.SetAt(v_index as u32, &new_ui).unwrap();
            }
            Handle::Border(b) => {
                b.put_Child(&new_ui).unwrap();
            }
            Handle::Viewbox(v) => {
                v.put_Child(&new_ui).unwrap();
            }
            Handle::ScrollViewer(_)
            | Handle::Expander(_)
            | Handle::TabViewItem(_)
            | Handle::NavigationView(_)
            | Handle::PivotItem(_) => {
                let cc = content_control_for(parent_h).unwrap();
                cc.put_Content(&new_ui).unwrap();
            }
            Handle::ScrollView(sv) => {
                sv.put_Content(&new_ui).unwrap();
            }
            Handle::TabView(tv) => {
                let items = tv.get_TabItems().unwrap();
                let insp: windows_core::IInspectable = new_ui.cast().unwrap();
                items.SetAt(v_index as u32, &insp).unwrap();
            }
            Handle::Pivot(p) => {
                let items = p
                    .cast::<Xaml::IItemsControl>()
                    .unwrap()
                    .get_Items()
                    .unwrap()
                    .cast::<windows_collections::IVector<windows_core::IInspectable>>()
                    .unwrap();
                let insp: windows_core::IInspectable = new_ui.cast().unwrap();
                items.SetAt(v_index as u32, &insp).unwrap();
            }
            _ => panic!("WinUIBackend::visual_set_at: {parent} is not a container"),
        }
    }
}

fn panel_children_vec(parent: &Handle) -> Option<windows_collections::IVector<Xaml::UIElement>> {
    let panel = match parent {
        Handle::StackPanel(s) => s.cast::<Xaml::IPanel>().ok()?,
        Handle::Grid(g) => g.cast::<Xaml::IPanel>().ok()?,
        Handle::Canvas(c) => c.cast::<Xaml::IPanel>().ok()?,
        Handle::RelativePanel(r) => r.cast::<Xaml::IPanel>().ok()?,
        _ => return None,
    };
    panel
        .get_Children()
        .ok()?
        .cast::<windows_collections::IVector<Xaml::UIElement>>()
        .ok()
}

fn content_control_for(parent: &Handle) -> Option<Xaml::IContentControl> {
    match parent {
        Handle::ScrollViewer(s) => s.cast().ok(),
        Handle::Expander(e) => e.cast().ok(),
        Handle::TabViewItem(ti) => ti.cast().ok(),
        Handle::NavigationView(nv) => nv.cast().ok(),
        Handle::PivotItem(pi) => pi.cast().ok(),
        _ => None,
    }
}

/// Build and apply a XAML Style with {ThemeResource} setters to an element.
/// WinUI handles theme-reactive resolution natively (Light ↔ Dark).
fn apply_theme_resource_style(handle: &Handle, bindings: &[(Prop, crate::core::theme::ThemeRef)]) {
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

fn anim_duration_to_timespan(d: std::time::Duration) -> windows_time::TimeSpan {
    windows_time::TimeSpan::try_from(d).unwrap_or(windows_time::TimeSpan::MAX)
}

fn easing_for(
    compositor: &Xaml::ICompositor,
    easing: crate::core::animation::Easing,
) -> windows_core::Result<Xaml::CompositionEasingFunction> {
    use crate::core::animation::Easing;
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
    transitions: Option<crate::core::animation::ImplicitTransitions>,
) -> windows_core::Result<()> {
    use crate::core::animation::Easing;
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
    let insert = |target: &str,
                  duration: std::time::Duration,
                  is_scalar: bool|
     -> windows_core::Result<()> {
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

fn run_property_animation_inner(
    ui: &Xaml::UIElement,
    cfg: crate::core::animation::AnimationConfig,
) -> windows_core::Result<()> {
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

impl Backend for WinUIBackend {
    fn create(&mut self, kind: ControlKind) -> ControlId {
        let id = self.alloc_id();
        let handle = Self::make_handle_for_kind(kind);
        self.controls.borrow_mut().insert(id, handle);
        id
    }
    #[allow(clippy::match_same_arms)] // large dispatch table with semantically distinct no-op arms
    fn set_prop(&mut self, id: ControlId, prop: Prop, value: PropValue) {
        let map = self.controls.borrow();
        let handle = map
            .get(&id)
            .unwrap_or_else(|| panic!("WinUIBackend::set_prop: unknown control {id}"));
        let result: windows_core::Result<()> = (|| -> windows_core::Result<()> {
            // Dispatch element-specific props to per-element modules.
            // Returns early if the element module handled the prop.
            let element_result = match handle {
                Handle::TextBlock(tb) => elements::text_block::set_prop(tb, prop, &value),
                Handle::RichTextBlock(tb) => elements::rich_text_block::set_prop(tb, prop, &value),
                Handle::Button(b) => elements::button::set_prop(b, prop, &value),
                Handle::CheckBox(c) => elements::check_box::set_prop(c, prop, &value),
                Handle::ToggleButton(tb) => elements::toggle_button::set_prop(tb, prop, &value),
                Handle::TextBox(t) => elements::text_box::set_prop(t, prop, &value),
                Handle::Grid(g) => elements::grid::set_prop(g, prop, &value),
                Handle::ScrollViewer(s) => elements::scroll_viewer::set_prop(s, prop, &value),
                Handle::StackPanel(s) => elements::stack_panel::set_prop(s, prop, &value),
                Handle::Border(b) => elements::border::set_prop(b, prop, &value),
                Handle::Canvas(c) => elements::canvas::set_prop(c, prop, &value),
                Handle::Rectangle(r) => elements::rectangle::set_prop(r, prop, &value),
                Handle::Ellipse(e) => elements::ellipse::set_prop(e, prop, &value),
                Handle::Line(l) => elements::line::set_prop(l, prop, &value),
                Handle::Image(img) => elements::image::set_prop(img, prop, &value),
                Handle::Viewbox(vb) => elements::viewbox::set_prop(vb, prop, &value),
                Handle::TabView(tv) => elements::tab_view::set_prop(tv, prop, &value),
                Handle::ToggleSwitch(ts) => elements::toggle_switch::set_prop(ts, prop, &value),
                Handle::Slider(s) => elements::slider::set_prop(s, prop, &value),
                Handle::NumberBox(n) => elements::number_box::set_prop(n, prop, &value),
                Handle::ProgressBar(p) => elements::progress_bar::set_prop(p, prop, &value),
                Handle::ProgressRing(p) => elements::progress_ring::set_prop(p, prop, &value),
                Handle::RadioButton(r) => elements::radio_button::set_prop(r, prop, &value),
                Handle::Expander(e) => elements::expander::set_prop(e, prop, &value),
                Handle::HyperlinkButton(h) => elements::hyperlink_button::set_prop(h, prop, &value),
                Handle::InfoBar(ib) => elements::info_bar::set_prop(ib, prop, &value),
                Handle::TabViewItem(ti) => elements::tab_view_item::set_prop(ti, prop, &value),
                Handle::ContentDialog(d) => elements::content_dialog::set_prop(d, prop, &value),
                Handle::InfoBadge(ib) => elements::info_badge::set_prop(ib, prop, &value),
                Handle::PersonPicture(p) => elements::person_picture::set_prop(p, prop, &value),
                Handle::NavigationView(nv) => elements::navigation_view::set_prop(nv, prop, &value),
                Handle::TitleBar(tb) => elements::title_bar::set_prop(tb, prop, &value),
                Handle::Pivot(p) => elements::pivot::set_prop(p, prop, &value),
                Handle::PivotItem(pi) => elements::pivot_item::set_prop(pi, prop, &value),
                Handle::BreadcrumbBar(bc) => elements::breadcrumb_bar::set_prop(bc, prop, &value),
                Handle::PasswordBox(p) => elements::password_box::set_prop(p, prop, &value),
                Handle::RadioButtons(r) => elements::radio_buttons::set_prop(r, prop, &value),
                Handle::ComboBox(c) => elements::combo_box::set_prop(c, prop, &value),
                Handle::RepeatButton(b) => elements::repeat_button::set_prop(b, prop, &value),
                Handle::RatingControl(r) => elements::rating_control::set_prop(r, prop, &value),
                Handle::ColorPicker(cp) => elements::color_picker::set_prop(cp, prop, &value),
                Handle::DatePicker(dp) => elements::date_picker::set_prop(dp, prop, &value),
                Handle::TimePicker(tp) => elements::time_picker::set_prop(tp, prop, &value),
                Handle::CalendarDatePicker(cdp) => {
                    elements::calendar_date_picker::set_prop(cdp, prop, &value)
                }
                Handle::CalendarView(cv) => elements::calendar_view::set_prop(cv, prop, &value),
                Handle::ListBox(lb) => elements::list_box::set_prop(lb, prop, &value),
                Handle::DropDownButton(ddb) => {
                    elements::drop_down_button::set_prop(ddb, prop, &value)
                }
                Handle::SplitButton(sb) => elements::split_button::set_prop(sb, prop, &value),
                Handle::AutoSuggestBox(asb) => {
                    elements::auto_suggest_box::set_prop(asb, prop, &value)
                }
                Handle::SplitView(sv) => elements::split_view::set_prop(sv, prop, &value),
                Handle::ScrollView(sv) => elements::scroll_view::set_prop(sv, prop, &value),
                Handle::TreeView(tv) => elements::tree_view::set_prop(tv, prop, &value),
                Handle::CommandBar(cb) => elements::command_bar::set_prop(cb, prop, &value),
                Handle::TeachingTip(tt) => elements::teaching_tip::set_prop(tt, prop, &value),
                Handle::RichEditBox(reb) => elements::rich_edit_box::set_prop(reb, prop, &value),
                _ => None,
            };
            if let Some(r) = element_result {
                return r;
            }
            match (prop, &value, handle) {
<<<<<<< HEAD
                (Prop::Text, PropValue::Str(s), Handle::TextBlock(tb)) => tb.put_Text(s.as_str()),
                (Prop::Text, PropValue::Unset, Handle::TextBlock(tb)) => tb.put_Text(""),
=======
>>>>>>> 8433bc0a2ef9610aa1d218df8e3ef3676a4601b7
                (Prop::FontSize, PropValue::F64(v), h) => {
                    if let Ok(ctrl) = h.cast_inner::<Xaml::IControl>() {
                        ctrl.put_FontSize(*v)
                    } else if let Ok(tb) = h.cast_inner::<Xaml::ITextBlock>() {
                        tb.put_FontSize(*v)
                    } else if let Ok(rtb) = h.cast_inner::<Xaml::IRichTextBlock>() {
                        rtb.put_FontSize(*v)
                    } else {
                        Ok(())
                    }
                }
                (Prop::FontSize, PropValue::Unset, h) => {
                    if let Ok(ctrl) = h.cast_inner::<Xaml::IControl>() {
                        ctrl.put_FontSize(14.0)
                    } else if let Ok(tb) = h.cast_inner::<Xaml::ITextBlock>() {
                        tb.put_FontSize(14.0)
                    } else if let Ok(rtb) = h.cast_inner::<Xaml::IRichTextBlock>() {
                        rtb.put_FontSize(14.0)
                    } else {
                        Ok(())
                    }
                }
                (Prop::FontWeight, PropValue::U16(w), h) => {
                    let fw = WinFontWeight { Weight: *w };
                    if let Ok(ctrl) = h.cast_inner::<Xaml::IControl>() {
                        ctrl.put_FontWeight(fw)
                    } else if let Ok(tb) = h.cast_inner::<Xaml::ITextBlock>() {
                        tb.put_FontWeight(fw)
                    } else {
                        Ok(())
                    }
                }
                (Prop::FontWeight, PropValue::Unset, h) => {
                    let fw = WinFontWeight { Weight: 400 };
                    if let Ok(ctrl) = h.cast_inner::<Xaml::IControl>() {
                        ctrl.put_FontWeight(fw)
                    } else if let Ok(tb) = h.cast_inner::<Xaml::ITextBlock>() {
                        tb.put_FontWeight(fw)
                    } else {
                        Ok(())
                    }
                }
                (Prop::FontFamily, PropValue::Str(s), h) => {
                    let ff = Xaml::FontFamily::CreateInstanceWithName(s)?;
                    if let Ok(ctrl) = h.cast_inner::<Xaml::IControl>() {
                        ctrl.put_FontFamily(&ff)
                    } else if let Ok(tb) = h.cast_inner::<Xaml::ITextBlock>() {
                        tb.put_FontFamily(&ff)
                    } else if let Ok(rtb) = h.cast_inner::<Xaml::IRichTextBlock>() {
                        rtb.put_FontFamily(&ff)
                    } else {
                        Ok(())
                    }
                }
                (Prop::FontFamily, PropValue::Unset, h) => {
                    let ff = Xaml::FontFamily::CreateInstanceWithName("Segoe UI")?;
                    if let Ok(ctrl) = h.cast_inner::<Xaml::IControl>() {
                        ctrl.put_FontFamily(&ff)
                    } else if let Ok(tb) = h.cast_inner::<Xaml::ITextBlock>() {
                        tb.put_FontFamily(&ff)
                    } else if let Ok(rtb) = h.cast_inner::<Xaml::IRichTextBlock>() {
                        rtb.put_FontFamily(&ff)
                    } else {
                        Ok(())
                    }
                }
<<<<<<< HEAD
                (Prop::IsTextSelectionEnabled, PropValue::Bool(v), Handle::RichTextBlock(tb)) => {
                    tb.put_IsTextSelectionEnabled(*v)
                }
                (Prop::IsTextSelectionEnabled, PropValue::Unset, Handle::RichTextBlock(tb)) => {
                    tb.put_IsTextSelectionEnabled(false)
                }
                (Prop::IsTextSelectionEnabled, PropValue::Bool(v), Handle::TextBlock(tb)) => {
                    tb.put_IsTextSelectionEnabled(*v)
                }
                (Prop::IsTextSelectionEnabled, PropValue::Unset, Handle::TextBlock(tb)) => {
                    tb.put_IsTextSelectionEnabled(false)
                }
                (Prop::TextWrappingWrap, PropValue::Bool(v), Handle::TextBlock(tb)) => {
                    let mode = if *v {
                        Xaml::TextWrapping::Wrap
                    } else {
                        Xaml::TextWrapping::NoWrap
                    };
                    tb.put_TextWrapping(mode)
                }
                (Prop::TextWrappingWrap, PropValue::Unset, Handle::TextBlock(tb)) => {
                    tb.put_TextWrapping(Xaml::TextWrapping::NoWrap)
                }
                (Prop::TextWrappingWrap, PropValue::Bool(v), Handle::RichTextBlock(tb)) => {
                    let mode = if *v {
                        Xaml::TextWrapping::Wrap
                    } else {
                        Xaml::TextWrapping::NoWrap
                    };
                    tb.put_TextWrapping(mode)
                }
                (Prop::TextWrappingWrap, PropValue::Unset, Handle::RichTextBlock(tb)) => {
                    tb.put_TextWrapping(Xaml::TextWrapping::NoWrap)
                }
                (Prop::ButtonContent, PropValue::Str(s), Handle::Button(b)) => {
                    let cc = b.cast::<Xaml::IContentControl>()?;
                    // If the button has an icon+text layout (StackPanel from
                    // ButtonIcon), update just the TextBlock child so the icon
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
                (Prop::ButtonContent, PropValue::Unset, Handle::Button(b)) => {
                    let cc = b.cast::<Xaml::IContentControl>()?;
                    if let Ok(existing) = cc.get_Content()
                        && let Ok(panel) = existing.cast::<Xaml::IPanel>()
                    {
                        let children = panel.get_Children()?;
                        if children.Size()? >= 2
                            && let Ok(tb) = children.GetAt(1)?.cast::<Xaml::ITextBlock>()
                        {
                            return tb.put_Text("");
                        }
                    }
                    cc.put_Content(None)
                }
                (Prop::ButtonIcon, PropValue::SymbolIcon(sym), Handle::Button(b)) => {
                    let icon_elem =
                        Xaml::SymbolIcon::CreateInstanceWithSymbol(Xaml::Symbol(sym.to_raw()))?;
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
                        panel.put_Orientation(Xaml::Orientation::Horizontal)?;
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
                (Prop::ButtonIcon, PropValue::Unset, Handle::Button(b)) => {
                    let cc = b.cast::<Xaml::IContentControl>()?;
                    if let Ok(existing) = cc.get_Content()
                        && let Ok(panel) = existing.cast::<Xaml::IPanel>()
                    {
                        let children = panel.get_Children()?;
                        if children.Size()? >= 2 {
                            let content = children.GetAt(1)?;
                            return cc.put_Content(&content);
                        }
                    }
                    if let Ok(existing) = cc.get_Content()
                        && existing.cast::<Xaml::ISymbolIcon>().is_ok()
                    {
                        cc.put_Content(None)
                    } else {
                        Ok(())
                    }
                }
                (Prop::ButtonStyleVariant, PropValue::ButtonStyle(style), Handle::Button(b)) => {
                    use crate::core::widgets::ButtonStyle;
                    let fe = b.cast::<Xaml::IFrameworkElement>()?;
                    let style_key = match style {
                        ButtonStyle::Accent => Some("AccentButtonStyle"),
                        ButtonStyle::Subtle => Some("SubtleButtonStyle"),
                        ButtonStyle::TextLink => Some("TextBlockButtonStyle"),
                        ButtonStyle::Default => None,
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
                (Prop::ButtonStyleVariant, PropValue::Unset, Handle::Button(b)) => {
                    let fe = b.cast::<Xaml::IFrameworkElement>()?;
                    fe.put_Style(None)?;
                    Ok(())
                }
                (Prop::IsEnabled, PropValue::Bool(v), Handle::Button(b)) => {
                    b.cast::<Xaml::IControl>()?.put_IsEnabled(*v)
                }
                (Prop::IsEnabled, PropValue::Unset, Handle::Button(b)) => {
                    b.cast::<Xaml::IControl>()?.put_IsEnabled(true)
                }
                (Prop::IsEnabled, PropValue::Bool(v), Handle::CheckBox(c)) => {
                    c.cast::<Xaml::IControl>()?.put_IsEnabled(*v)
                }
                (Prop::IsEnabled, PropValue::Unset, Handle::CheckBox(c)) => {
                    c.cast::<Xaml::IControl>()?.put_IsEnabled(true)
                }
                (Prop::IsEnabled, PropValue::Bool(v), Handle::TextBox(t)) => {
                    t.cast::<Xaml::IControl>()?.put_IsEnabled(*v)
                }
                (Prop::IsEnabled, PropValue::Unset, Handle::TextBox(t)) => {
                    t.cast::<Xaml::IControl>()?.put_IsEnabled(true)
                }
=======
>>>>>>> 8433bc0a2ef9610aa1d218df8e3ef3676a4601b7
                (Prop::IsEnabled, PropValue::Unset, _) => handle
                    .as_ui_element()
                    .cast::<Xaml::IControl>()?
                    .put_IsEnabled(true),
<<<<<<< HEAD
                (Prop::IsChecked, PropValue::Bool(v), Handle::CheckBox(c)) => {
                    c.cast::<Xaml::IToggleButton>()?.put_IsChecked(Some(*v))
                }
                (Prop::IsChecked, PropValue::Unset, Handle::CheckBox(c)) => {
                    c.cast::<Xaml::IToggleButton>()?.put_IsChecked(None)
                }
                (Prop::CheckBoxLabel, PropValue::Str(s), Handle::CheckBox(c)) => {
                    let tb = string_as_textblock(s)?;
                    c.cast::<Xaml::IContentControl>()?.put_Content(&tb)
                }
                (Prop::CheckBoxLabel, PropValue::Unset, Handle::CheckBox(c)) => {
                    c.cast::<Xaml::IContentControl>()?.put_Content(None)
                }
                // ── ToggleButton ─────────────────────────────────────────────
                (Prop::IsChecked, PropValue::Bool(v), Handle::ToggleButton(tb)) => {
                    tb.put_IsChecked(Some(*v))
                }
                (Prop::IsChecked, PropValue::Unset, Handle::ToggleButton(tb)) => {
                    tb.put_IsChecked(None)
                }
                (Prop::CheckBoxLabel, PropValue::Str(s), Handle::ToggleButton(tb)) => {
                    let txt = string_as_textblock(s)?;
                    tb.cast::<Xaml::IContentControl>()?.put_Content(&txt)
                }
                (Prop::CheckBoxLabel, PropValue::Unset, Handle::ToggleButton(tb)) => {
                    tb.cast::<Xaml::IContentControl>()?.put_Content(None)
                }
                (Prop::TextBoxValue, PropValue::Str(s), Handle::TextBox(t)) => {
                    if t.get_Text().ok().as_deref() == Some(s.as_str()) {
                        return Ok(());
                    }
                    t.put_Text(s.as_str())
                }
                (Prop::TextBoxValue, PropValue::Unset, Handle::TextBox(t)) => {
                    if t.get_Text().ok().as_deref() == Some("") {
                        return Ok(());
                    }
                    t.put_Text("")
                }
                (Prop::Placeholder, PropValue::Str(s), Handle::TextBox(t)) => {
                    t.put_PlaceholderText(s.as_str())
                }
                (Prop::Placeholder, PropValue::Unset, Handle::TextBox(t)) => {
                    t.put_PlaceholderText("")
                }
                (Prop::Header, PropValue::Str(s), Handle::TextBox(t)) => {
                    let tb = string_as_textblock(s)?;
                    t.put_Header(&tb)
                }
                (Prop::Header, PropValue::Unset, Handle::TextBox(t)) => t.put_Header(None),
                (Prop::AcceptsReturn, PropValue::Bool(v), Handle::TextBox(t)) => {
                    t.put_AcceptsReturn(*v)
                }
                (Prop::AcceptsReturn, PropValue::Unset, Handle::TextBox(t)) => {
                    t.put_AcceptsReturn(false)
                }
                (Prop::TextWrappingWrap, PropValue::Bool(v), Handle::TextBox(t)) => {
                    let mode = if *v {
                        Xaml::TextWrapping::Wrap
                    } else {
                        Xaml::TextWrapping::NoWrap
                    };
                    t.put_TextWrapping(mode)
                }
                (Prop::TextWrappingWrap, PropValue::Unset, Handle::TextBox(t)) => {
                    t.put_TextWrapping(Xaml::TextWrapping::NoWrap)
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
                (Prop::GridRows, PropValue::Unset, Handle::Grid(g)) => {
                    let defs = g.get_RowDefinitions()?;
                    defs.cast::<windows_collections::IVector<Xaml::RowDefinition>>()?
                        .Clear()
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
                (Prop::GridColumns, PropValue::Unset, Handle::Grid(g)) => {
                    let defs = g.get_ColumnDefinitions()?;
                    defs.cast::<windows_collections::IVector<Xaml::ColumnDefinition>>()?
                        .Clear()
                }
                (Prop::GridRowSpacing, PropValue::F64(v), Handle::Grid(g)) => g.put_RowSpacing(*v),
                (Prop::GridRowSpacing, PropValue::Unset, Handle::Grid(g)) => g.put_RowSpacing(0.0),
                (Prop::GridColumnSpacing, PropValue::F64(v), Handle::Grid(g)) => {
                    g.put_ColumnSpacing(*v)
                }
                (Prop::GridColumnSpacing, PropValue::Unset, Handle::Grid(g)) => {
                    g.put_ColumnSpacing(0.0)
                }
=======
>>>>>>> 8433bc0a2ef9610aa1d218df8e3ef3676a4601b7
                (Prop::AttachedGridRow, PropValue::I32(v), _) => {
                    Xaml::Grid::SetRow(&handle.as_framework_element(), *v)
                }
                (Prop::AttachedGridRow, PropValue::Unset, _) => {
                    Xaml::Grid::SetRow(&handle.as_framework_element(), 0)
                }
                (Prop::AttachedGridColumn, PropValue::I32(v), _) => {
                    Xaml::Grid::SetColumn(&handle.as_framework_element(), *v)
                }
                (Prop::AttachedGridColumn, PropValue::Unset, _) => {
                    Xaml::Grid::SetColumn(&handle.as_framework_element(), 0)
                }
                (Prop::AttachedGridRowSpan, PropValue::I32(v), _) => {
                    Xaml::Grid::SetRowSpan(&handle.as_framework_element(), *v)
                }
                (Prop::AttachedGridRowSpan, PropValue::Unset, _) => {
                    Xaml::Grid::SetRowSpan(&handle.as_framework_element(), 1)
                }
                (Prop::AttachedGridColumnSpan, PropValue::I32(v), _) => {
                    Xaml::Grid::SetColumnSpan(&handle.as_framework_element(), *v)
                }
<<<<<<< HEAD
                (Prop::AttachedGridColumnSpan, PropValue::Unset, _) => {
                    Xaml::Grid::SetColumnSpan(&handle.as_framework_element(), 1)
                }
                (
                    Prop::HorizontalScrollBarVisibility,
                    PropValue::ScrollVis(v),
                    Handle::ScrollViewer(s),
                ) => s.put_HorizontalScrollBarVisibility(to_xaml_scroll_visibility(*v)),
                (
                    Prop::VerticalScrollBarVisibility,
                    PropValue::ScrollVis(v),
                    Handle::ScrollViewer(s),
                ) => s.put_VerticalScrollBarVisibility(to_xaml_scroll_visibility(*v)),
                (Prop::Orientation, PropValue::Vertical(vert), Handle::StackPanel(s)) => s
                    .put_Orientation(if *vert {
                        Xaml::Orientation::Vertical
                    } else {
                        Xaml::Orientation::Horizontal
                    }),
                (Prop::Orientation, PropValue::Unset, Handle::StackPanel(s)) => {
                    s.put_Orientation(Xaml::Orientation::Horizontal)
                }
                (Prop::Spacing, PropValue::F64(v), Handle::StackPanel(s)) => s.put_Spacing(*v),
                (Prop::Spacing, PropValue::Unset, Handle::StackPanel(s)) => s.put_Spacing(0.0),
=======
>>>>>>> 8433bc0a2ef9610aa1d218df8e3ef3676a4601b7
                (Prop::Margin, PropValue::Thickness(t), _) => handle
                    .as_framework_element()
                    .cast::<Xaml::IFrameworkElement>()?
                    .put_Margin(to_xaml_thickness(*t)),
                (Prop::Margin, PropValue::Unset, _) => handle
                    .as_framework_element()
                    .cast::<Xaml::IFrameworkElement>()?
                    .put_Margin(to_xaml_thickness(Thickness::default())),
                (Prop::Width, PropValue::F64(v), _) => handle
                    .as_framework_element()
                    .cast::<Xaml::IFrameworkElement>()?
                    .put_Width(*v),
                (Prop::Width, PropValue::Unset, _) => handle
                    .as_framework_element()
                    .cast::<Xaml::IFrameworkElement>()?
                    .put_Width(f64::NAN),
                (Prop::Height, PropValue::F64(v), _) => handle
                    .as_framework_element()
                    .cast::<Xaml::IFrameworkElement>()?
                    .put_Height(*v),
                (Prop::Height, PropValue::Unset, _) => handle
                    .as_framework_element()
                    .cast::<Xaml::IFrameworkElement>()?
                    .put_Height(f64::NAN),
                (Prop::MinWidth, PropValue::F64(v), _) => handle
                    .as_framework_element()
                    .cast::<Xaml::IFrameworkElement>()?
                    .put_MinWidth(*v),
                (Prop::MinWidth, PropValue::Unset, _) => handle
                    .as_framework_element()
                    .cast::<Xaml::IFrameworkElement>()?
                    .put_MinWidth(0.0),
                (Prop::MaxWidth, PropValue::F64(v), _) => handle
                    .as_framework_element()
                    .cast::<Xaml::IFrameworkElement>()?
                    .put_MaxWidth(*v),
                (Prop::MaxWidth, PropValue::Unset, _) => handle
                    .as_framework_element()
                    .cast::<Xaml::IFrameworkElement>()?
                    .put_MaxWidth(f64::INFINITY),
                (Prop::MinHeight, PropValue::F64(v), _) => handle
                    .as_framework_element()
                    .cast::<Xaml::IFrameworkElement>()?
                    .put_MinHeight(*v),
                (Prop::MinHeight, PropValue::Unset, _) => handle
                    .as_framework_element()
                    .cast::<Xaml::IFrameworkElement>()?
                    .put_MinHeight(0.0),
                (Prop::MaxHeight, PropValue::F64(v), _) => handle
                    .as_framework_element()
                    .cast::<Xaml::IFrameworkElement>()?
                    .put_MaxHeight(*v),
                (Prop::MaxHeight, PropValue::Unset, _) => handle
                    .as_framework_element()
                    .cast::<Xaml::IFrameworkElement>()?
                    .put_MaxHeight(f64::INFINITY),
                (Prop::HorizontalAlignment, PropValue::HAlign(v), _) => handle
                    .as_framework_element()
                    .cast::<Xaml::IFrameworkElement>()?
                    .put_HorizontalAlignment(to_xaml_halign(*v)),
                (Prop::HorizontalAlignment, PropValue::Unset, _) => handle
                    .as_framework_element()
                    .cast::<Xaml::IFrameworkElement>()?
                    .put_HorizontalAlignment(Xaml::HorizontalAlignment::Stretch),
                (Prop::VerticalAlignment, PropValue::VAlign(v), _) => handle
                    .as_framework_element()
                    .cast::<Xaml::IFrameworkElement>()?
                    .put_VerticalAlignment(to_xaml_valign(*v)),
                (Prop::VerticalAlignment, PropValue::Unset, _) => handle
                    .as_framework_element()
                    .cast::<Xaml::IFrameworkElement>()?
                    .put_VerticalAlignment(Xaml::VerticalAlignment::Stretch),
                (Prop::Opacity, PropValue::F64(v), _) => handle
                    .as_ui_element()
                    .cast::<Xaml::IUIElement>()?
                    .put_Opacity(*v),
                (Prop::Opacity, PropValue::Unset, _) => handle
                    .as_ui_element()
                    .cast::<Xaml::IUIElement>()?
                    .put_Opacity(1.0),
                (Prop::Resources, PropValue::Resources(map), _) => {
                    let rd = handle
                        .as_framework_element()
                        .cast::<Xaml::IFrameworkElement>()?
                        .get_Resources()?;
                    let imap = rd.cast::<windows_collections::IMap<
                        windows_core::IInspectable,
                        windows_core::IInspectable,
                    >>()?;
                    for (k, v) in map {
                        let key = windows_reference::IReference::from(k.as_str());
                        let val = windows_reference::IReference::from(v.as_str());
                        imap.Insert(&key, &val)?;
                    }
                    Ok(())
                }
<<<<<<< HEAD
                (Prop::Resources, PropValue::Unset, _) => {
                    let rd = handle
                        .as_framework_element()
                        .cast::<Xaml::IFrameworkElement>()?
                        .get_Resources()?;
                    let imap = rd.cast::<windows_collections::IMap<
                        windows_core::IInspectable,
                        windows_core::IInspectable,
                    >>()?;
                    imap.Clear()?;
                    Ok(())
                }
                (Prop::Padding, PropValue::Thickness(t), Handle::Button(b)) => b
                    .cast::<Xaml::IControl>()?
                    .put_Padding(to_xaml_thickness(*t)),
                (Prop::Padding, PropValue::Unset, Handle::Button(b)) => b
                    .cast::<Xaml::IControl>()?
                    .put_Padding(to_xaml_thickness(Thickness::default())),
                (Prop::Padding, PropValue::Thickness(t), Handle::Border(br)) => {
                    br.put_Padding(to_xaml_thickness(*t))
                }
                (Prop::Padding, PropValue::Unset, Handle::Border(br)) => {
                    br.put_Padding(to_xaml_thickness(Thickness::default()))
                }
=======
>>>>>>> 8433bc0a2ef9610aa1d218df8e3ef3676a4601b7
                (Prop::Padding, PropValue::Thickness(t), h) => {
                    if let Ok(ctl) = h.as_framework_element().cast::<Xaml::Control>() {
                        ctl.cast::<Xaml::IControl>()?
                            .put_Padding(to_xaml_thickness(*t))
                    } else {
                        diag::unhandled_modifier("set_prop", Prop::Padding, h);
                        Ok(())
                    }
                }
                (Prop::Padding, PropValue::Unset, _) => {
                    if let Ok(ctl) = handle.as_framework_element().cast::<Xaml::Control>() {
                        ctl.cast::<Xaml::IControl>()?
                            .put_Padding(to_xaml_thickness(Thickness::default()))
                    } else {
                        Ok(())
                    }
                }
                (Prop::Foreground, PropValue::Brush(_), h) => {
                    diag::unhandled_modifier("set_prop", Prop::Foreground, h);
                    Ok(())
                }
<<<<<<< HEAD
                (Prop::Foreground, PropValue::Unset, Handle::TextBlock(tb)) => {
                    tb.put_Foreground(None)
                }
                (Prop::Foreground, PropValue::Unset, Handle::RichTextBlock(tb)) => {
                    tb.put_Foreground(None)
                }
                (Prop::Foreground, PropValue::Unset, Handle::Button(b)) => {
                    b.cast::<Xaml::IControl>()?.put_Foreground(None)
                }
                (Prop::Foreground, PropValue::Unset, h) => match h {
                    Handle::TextBlock(tb) => tb.put_Foreground(None),
                    Handle::RichTextBlock(tb) => tb.put_Foreground(None),
                    Handle::Button(b) => b.cast::<Xaml::IControl>()?.put_Foreground(None),
                    other => {
                        diag::unhandled_modifier("set_prop", Prop::Foreground, other);
                        Ok(())
                    }
                },
                (Prop::IsOn, PropValue::Bool(v), Handle::ToggleSwitch(ts)) => ts.put_IsOn(*v),
                (Prop::IsOn, PropValue::Unset, Handle::ToggleSwitch(ts)) => ts.put_IsOn(false),
                (Prop::OnContent, PropValue::Str(s), Handle::ToggleSwitch(ts)) => {
                    let tb = string_as_textblock(s)?;
                    ts.put_OnContent(&tb)
                }
                (Prop::OnContent, PropValue::Unset, Handle::ToggleSwitch(ts)) => {
                    ts.put_OnContent(None)
                }
                (Prop::OffContent, PropValue::Str(s), Handle::ToggleSwitch(ts)) => {
                    let tb = string_as_textblock(s)?;
                    ts.put_OffContent(&tb)
                }
                (Prop::OffContent, PropValue::Unset, Handle::ToggleSwitch(ts)) => {
                    ts.put_OffContent(None)
                }
                (Prop::Header, PropValue::Str(s), Handle::ToggleSwitch(ts)) => {
                    let tb = string_as_textblock(s)?;
                    ts.put_Header(&tb)
                }
                (Prop::Header, PropValue::Unset, Handle::ToggleSwitch(ts)) => ts.put_Header(None),
                (Prop::IsEnabled, PropValue::Bool(v), Handle::ToggleSwitch(ts)) => {
                    ts.cast::<Xaml::IControl>()?.put_IsEnabled(*v)
                }
                (Prop::NumericValue, PropValue::F64(v), Handle::Slider(s)) => {
                    s.cast::<Xaml::IRangeBase>()?.put_Value(*v)
                }
                (Prop::NumericValue, PropValue::Unset, Handle::Slider(s)) => {
                    s.cast::<Xaml::IRangeBase>()?.put_Value(0.0)
                }
                (Prop::Minimum, PropValue::F64(v), Handle::Slider(s)) => {
                    s.cast::<Xaml::IRangeBase>()?.put_Minimum(*v)
                }
                (Prop::Minimum, PropValue::Unset, Handle::Slider(s)) => {
                    s.cast::<Xaml::IRangeBase>()?.put_Minimum(0.0)
                }
                (Prop::Maximum, PropValue::F64(v), Handle::Slider(s)) => {
                    s.cast::<Xaml::IRangeBase>()?.put_Maximum(*v)
                }
                (Prop::Maximum, PropValue::Unset, Handle::Slider(s)) => {
                    s.cast::<Xaml::IRangeBase>()?.put_Maximum(100.0)
                }
                (Prop::Step, PropValue::F64(v), Handle::Slider(s)) => {
                    s.put_StepFrequency(*v)?;
                    s.cast::<Xaml::IRangeBase>()?.put_SmallChange(*v)
                }
                (Prop::Step, PropValue::Unset, Handle::Slider(s)) => {
                    s.put_StepFrequency(1.0)?;
                    s.cast::<Xaml::IRangeBase>()?.put_SmallChange(1.0)
                }
                (Prop::Header, PropValue::Str(s), Handle::Slider(sl)) => {
                    let tb = string_as_textblock(s)?;
                    sl.put_Header(&tb)
                }
                (Prop::Header, PropValue::Unset, Handle::Slider(sl)) => sl.put_Header(None),
                (Prop::IsEnabled, PropValue::Bool(v), Handle::Slider(s)) => {
                    s.cast::<Xaml::IControl>()?.put_IsEnabled(*v)
                }
                (Prop::Orientation, PropValue::Vertical(vert), Handle::Slider(s)) => s
                    .put_Orientation(if *vert {
                        Xaml::Orientation::Vertical
                    } else {
                        Xaml::Orientation::Horizontal
                    }),
                (Prop::Orientation, PropValue::Unset, Handle::Slider(s)) => {
                    s.put_Orientation(Xaml::Orientation::Horizontal)
                }
                (Prop::NumericValue, PropValue::F64(v), Handle::NumberBox(n)) => n.put_Value(*v),
                (Prop::NumericValue, PropValue::Unset, Handle::NumberBox(n)) => n.put_Value(0.0),
                (Prop::Minimum, PropValue::F64(v), Handle::NumberBox(n)) => n.put_Minimum(*v),
                (Prop::Minimum, PropValue::Unset, Handle::NumberBox(n)) => n.put_Minimum(0.0),
                (Prop::Maximum, PropValue::F64(v), Handle::NumberBox(n)) => n.put_Maximum(*v),
                (Prop::Maximum, PropValue::Unset, Handle::NumberBox(n)) => n.put_Maximum(0.0),
                (Prop::Header, PropValue::Str(s), Handle::NumberBox(n)) => {
                    let tb = string_as_textblock(s)?;
                    n.put_Header(&tb)
                }
                (Prop::Header, PropValue::Unset, Handle::NumberBox(n)) => n.put_Header(None),
                (Prop::IsEnabled, PropValue::Bool(v), Handle::NumberBox(n)) => {
                    n.cast::<Xaml::IControl>()?.put_IsEnabled(*v)
                }
                (Prop::NumericValue, PropValue::F64(v), Handle::ProgressBar(p)) => {
                    p.cast::<Xaml::IRangeBase>()?.put_Value(*v)
                }
                (Prop::NumericValue, PropValue::Unset, Handle::ProgressBar(p)) => {
                    p.cast::<Xaml::IRangeBase>()?.put_Value(0.0)
                }
                (Prop::Minimum, PropValue::F64(v), Handle::ProgressBar(p)) => {
                    p.cast::<Xaml::IRangeBase>()?.put_Minimum(*v)
                }
                (Prop::Minimum, PropValue::Unset, Handle::ProgressBar(p)) => {
                    p.cast::<Xaml::IRangeBase>()?.put_Minimum(0.0)
                }
                (Prop::Maximum, PropValue::F64(v), Handle::ProgressBar(p)) => {
                    p.cast::<Xaml::IRangeBase>()?.put_Maximum(*v)
                }
                (Prop::Maximum, PropValue::Unset, Handle::ProgressBar(p)) => {
                    p.cast::<Xaml::IRangeBase>()?.put_Maximum(100.0)
                }
                (Prop::IsIndeterminate, PropValue::Bool(v), Handle::ProgressBar(p)) => {
                    p.put_IsIndeterminate(*v)
                }
                (Prop::IsIndeterminate, PropValue::Unset, Handle::ProgressBar(p)) => {
                    p.put_IsIndeterminate(false)
                }
                (Prop::NumericValue, PropValue::F64(v), Handle::ProgressRing(p)) => {
                    p.cast::<Xaml::IRangeBase>()?.put_Value(*v)
                }
                (Prop::NumericValue, PropValue::Unset, Handle::ProgressRing(p)) => {
                    p.cast::<Xaml::IRangeBase>()?.put_Value(0.0)
                }
                (Prop::Minimum, PropValue::F64(v), Handle::ProgressRing(p)) => p.put_Minimum(*v),
                (Prop::Minimum, PropValue::Unset, Handle::ProgressRing(p)) => p.put_Minimum(0.0),
                (Prop::Maximum, PropValue::F64(v), Handle::ProgressRing(p)) => p.put_Maximum(*v),
                (Prop::Maximum, PropValue::Unset, Handle::ProgressRing(p)) => p.put_Maximum(100.0),
                (Prop::IsIndeterminate, PropValue::Bool(v), Handle::ProgressRing(p)) => {
                    p.put_IsIndeterminate(*v)
                }
                (Prop::IsIndeterminate, PropValue::Unset, Handle::ProgressRing(p)) => {
                    p.put_IsIndeterminate(false)
                }
                (Prop::IsActive, PropValue::Bool(v), Handle::ProgressRing(p)) => p.put_IsActive(*v),
                (Prop::IsActive, PropValue::Unset, Handle::ProgressRing(p)) => p.put_IsActive(true),
                (Prop::RadioLabel, PropValue::Str(s), Handle::RadioButton(r)) => {
                    let tb = string_as_textblock(s)?;
                    r.cast::<Xaml::IContentControl>()?.put_Content(&tb)
                }
                (Prop::RadioLabel, PropValue::Unset, Handle::RadioButton(r)) => {
                    r.cast::<Xaml::IContentControl>()?.put_Content(None)
                }
                (Prop::IsChecked, PropValue::Bool(v), Handle::RadioButton(r)) => {
                    r.cast::<Xaml::IToggleButton>()?.put_IsChecked(Some(*v))
                }
                (Prop::IsChecked, PropValue::Unset, Handle::RadioButton(r)) => {
                    r.cast::<Xaml::IToggleButton>()?.put_IsChecked(Some(false))
                }
                (Prop::GroupName, PropValue::Str(s), Handle::RadioButton(r)) => {
                    r.put_GroupName(s.as_str())
                }
                (Prop::GroupName, PropValue::Unset, Handle::RadioButton(r)) => r.put_GroupName(""),
                (Prop::IsEnabled, PropValue::Bool(v), Handle::RadioButton(r)) => {
                    r.cast::<Xaml::IControl>()?.put_IsEnabled(*v)
                }
                (Prop::Header, PropValue::Str(s), Handle::Expander(e)) => {
                    let tb = string_as_textblock(s)?;
                    e.put_Header(&tb)
                }
                (Prop::Header, PropValue::Unset, Handle::Expander(e)) => e.put_Header(None),
                (Prop::IsExpanded, PropValue::Bool(v), Handle::Expander(e)) => e.put_IsExpanded(*v),
                (Prop::IsExpanded, PropValue::Unset, Handle::Expander(e)) => {
                    e.put_IsExpanded(false)
                }
                (Prop::ButtonContent, PropValue::Str(s), Handle::HyperlinkButton(h)) => {
                    let tb = string_as_textblock(s)?;
                    h.cast::<Xaml::IContentControl>()?.put_Content(&tb)
                }
                (Prop::ButtonContent, PropValue::Unset, Handle::HyperlinkButton(h)) => {
                    h.cast::<Xaml::IContentControl>()?.put_Content(None)
                }
                (Prop::NavigateUri, PropValue::Str(s), Handle::HyperlinkButton(h)) => {
                    let uri = Xaml::Uri::CreateUri(s.as_str())?;
                    h.put_NavigateUri(&uri)
                }
                (Prop::NavigateUri, PropValue::Unset, Handle::HyperlinkButton(h)) => {
                    h.put_NavigateUri(None)
                }
                (Prop::IsEnabled, PropValue::Bool(v), Handle::HyperlinkButton(h)) => {
                    h.cast::<Xaml::IControl>()?.put_IsEnabled(*v)
                }
                (Prop::InfoBarTitle, PropValue::Str(s), Handle::InfoBar(ib)) => {
                    ib.put_Title(s.as_str())
                }
                (Prop::InfoBarTitle, PropValue::Unset, Handle::InfoBar(ib)) => ib.put_Title(""),
                (Prop::InfoBarMessage, PropValue::Str(s), Handle::InfoBar(ib)) => {
                    ib.put_Message(s.as_str())
                }
                (Prop::InfoBarMessage, PropValue::Unset, Handle::InfoBar(ib)) => ib.put_Message(""),
                (Prop::InfoBarSeverity, PropValue::InfoBarSev(v), Handle::InfoBar(ib)) => {
                    ib.put_Severity(to_winui_info_bar_severity(*v))
                }
                (Prop::InfoBarSeverity, PropValue::Unset, Handle::InfoBar(ib)) => {
                    ib.put_Severity(Xaml::InfoBarSeverity::Informational)
                }
                (Prop::InfoBarIsOpen, PropValue::Bool(v), Handle::InfoBar(ib)) => ib.put_IsOpen(*v),
                (Prop::InfoBarIsOpen, PropValue::Unset, Handle::InfoBar(ib)) => {
                    ib.put_IsOpen(false)
                }
                (Prop::IsClosable, PropValue::Bool(v), Handle::InfoBar(ib)) => {
                    ib.put_IsClosable(*v)
                }
                (Prop::IsClosable, PropValue::Unset, Handle::InfoBar(ib)) => {
                    ib.put_IsClosable(false)
                }
                (Prop::IsClosable, PropValue::Bool(v), Handle::TabViewItem(ti)) => {
                    ti.put_IsClosable(*v)
                }
                // ContentDialog (W6 — modal popup hosted via ShowAsync).
                (Prop::IsClosable, PropValue::Unset, Handle::TabViewItem(ti)) => {
                    ti.put_IsClosable(true)
                }
                (Prop::ContentDialogTitle, PropValue::Str(s), Handle::ContentDialog(d)) => {
                    let title = windows_reference::IReference::from(s.as_str());
                    d.put_Title(&title)
                }
                (Prop::ContentDialogTitle, PropValue::Unset, Handle::ContentDialog(d)) => {
                    d.put_Title(None)
                }
                (Prop::ContentDialogBody, PropValue::Str(s), Handle::ContentDialog(d)) => {
                    let tb = string_as_textblock(s)?;
                    d.cast::<Xaml::IContentControl>()?.put_Content(&tb)
                }
                (Prop::ContentDialogBody, PropValue::Unset, Handle::ContentDialog(d)) => {
                    d.cast::<Xaml::IContentControl>()?.put_Content(None)
                }
                (Prop::ContentDialogPrimaryText, PropValue::Str(s), Handle::ContentDialog(d)) => {
                    d.put_PrimaryButtonText(s.as_str())
                }
                (Prop::ContentDialogPrimaryText, PropValue::Unset, Handle::ContentDialog(d)) => {
                    d.put_PrimaryButtonText("")
                }
                (Prop::ContentDialogSecondaryText, PropValue::Str(s), Handle::ContentDialog(d)) => {
                    d.put_SecondaryButtonText(s.as_str())
                }
                (Prop::ContentDialogSecondaryText, PropValue::Unset, Handle::ContentDialog(d)) => {
                    d.put_SecondaryButtonText("")
                }
                (Prop::ContentDialogCloseText, PropValue::Str(s), Handle::ContentDialog(d)) => {
                    d.put_CloseButtonText(s.as_str())
                }
                (Prop::ContentDialogCloseText, PropValue::Unset, Handle::ContentDialog(d)) => {
                    d.put_CloseButtonText("")
                }
                (
                    Prop::ContentDialogPrimaryEnabled,
                    PropValue::Bool(v),
                    Handle::ContentDialog(d),
                ) => d.put_IsPrimaryButtonEnabled(*v),
                (
                    Prop::ContentDialogSecondaryEnabled,
                    PropValue::Bool(v),
                    Handle::ContentDialog(d),
                ) => d.put_IsSecondaryButtonEnabled(*v),
=======
                // ContentDialog.is_open still lives here because it needs backend state
                // to resolve a live XamlRoot before ShowAsync.
>>>>>>> 8433bc0a2ef9610aa1d218df8e3ef3676a4601b7
                (Prop::ContentDialogIsOpen, PropValue::Bool(v), Handle::ContentDialog(d)) => {
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
<<<<<<< HEAD
                (Prop::ContentDialogIsOpen, PropValue::Unset, Handle::ContentDialog(d)) => d.Hide(),
                (Prop::InfoBadgeValue, PropValue::I32(v), Handle::InfoBadge(ib)) => {
                    if *v < 0 {
                        ib.put_Value(-1)
                    } else {
                        ib.put_Value(*v)
                    }
                }
                (Prop::InfoBadgeValue, PropValue::Unset, Handle::InfoBadge(ib)) => ib.put_Value(0),
                (Prop::PersonDisplayName, PropValue::Str(s), Handle::PersonPicture(p)) => {
                    p.put_DisplayName(s.as_str())
                }
                (Prop::PersonDisplayName, PropValue::Unset, Handle::PersonPicture(p)) => {
                    p.put_DisplayName("")
                }
                (Prop::PersonInitials, PropValue::Str(s), Handle::PersonPicture(p)) => {
                    p.put_Initials(s.as_str())
                }
                (Prop::PersonInitials, PropValue::Unset, Handle::PersonPicture(p)) => {
                    p.put_Initials("")
                }
                (Prop::Fill, PropValue::Brush(b), Handle::Rectangle(r)) => {
                    r.cast::<Xaml::IShape>()?.put_Fill(&brush_of(b)?)
                }
                (Prop::Fill, PropValue::Unset, Handle::Rectangle(r)) => {
                    r.cast::<Xaml::IShape>()?.put_Fill(None)
                }
                (Prop::Fill, PropValue::Brush(b), Handle::Ellipse(e)) => {
                    e.cast::<Xaml::IShape>()?.put_Fill(&brush_of(b)?)
                }
                (Prop::Fill, PropValue::Unset, Handle::Ellipse(e)) => {
                    e.cast::<Xaml::IShape>()?.put_Fill(None)
                }
                (Prop::Stroke, PropValue::Brush(b), Handle::Rectangle(r)) => {
                    r.cast::<Xaml::IShape>()?.put_Stroke(&brush_of(b)?)
                }
                (Prop::Stroke, PropValue::Unset, Handle::Rectangle(r)) => {
                    r.cast::<Xaml::IShape>()?.put_Stroke(None)
                }
                (Prop::Stroke, PropValue::Brush(b), Handle::Ellipse(e)) => {
                    e.cast::<Xaml::IShape>()?.put_Stroke(&brush_of(b)?)
                }
                (Prop::Stroke, PropValue::Unset, Handle::Ellipse(e)) => {
                    e.cast::<Xaml::IShape>()?.put_Stroke(None)
                }
                (Prop::Stroke, PropValue::Brush(b), Handle::Line(l)) => {
                    l.cast::<Xaml::IShape>()?.put_Stroke(&brush_of(b)?)
                }
                (Prop::Stroke, PropValue::Unset, Handle::Line(l)) => {
                    l.cast::<Xaml::IShape>()?.put_Stroke(None)
                }
                (Prop::StrokeThickness, PropValue::F64(v), Handle::Rectangle(r)) => {
                    r.cast::<Xaml::IShape>()?.put_StrokeThickness(*v)
                }
                (Prop::StrokeThickness, PropValue::Unset, Handle::Rectangle(r)) => {
                    r.cast::<Xaml::IShape>()?.put_StrokeThickness(0.0)
                }
                (Prop::StrokeThickness, PropValue::F64(v), Handle::Ellipse(e)) => {
                    e.cast::<Xaml::IShape>()?.put_StrokeThickness(*v)
                }
                (Prop::StrokeThickness, PropValue::Unset, Handle::Ellipse(e)) => {
                    e.cast::<Xaml::IShape>()?.put_StrokeThickness(0.0)
                }
                (Prop::StrokeThickness, PropValue::F64(v), Handle::Line(l)) => {
                    l.cast::<Xaml::IShape>()?.put_StrokeThickness(*v)
                }
                (Prop::StrokeThickness, PropValue::Unset, Handle::Line(l)) => {
                    l.cast::<Xaml::IShape>()?.put_StrokeThickness(0.0)
                }
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
=======
>>>>>>> 8433bc0a2ef9610aa1d218df8e3ef3676a4601b7
                (Prop::BorderBrush, _, h) => {
                    diag::unhandled_modifier("set_prop", Prop::BorderBrush, h);
                    Ok(())
                }
                (Prop::BorderThickness, _, h) => {
                    diag::unhandled_modifier("set_prop", Prop::BorderThickness, h);
                    Ok(())
                }
<<<<<<< HEAD
                (Prop::LineEndpoints, PropValue::LineEndpoints(p), Handle::Line(l)) => l
                    .put_X1(p.x1)
                    .and_then(|_| l.put_Y1(p.y1))
                    .and_then(|_| l.put_X2(p.x2))
                    .and_then(|_| l.put_Y2(p.y2)),
                (Prop::LineEndpoints, PropValue::Unset, Handle::Line(l)) => l
                    .put_X1(0.0)
                    .and_then(|_| l.put_Y1(0.0))
                    .and_then(|_| l.put_X2(0.0))
                    .and_then(|_| l.put_Y2(0.0)),
                (Prop::ImageSource, PropValue::Str(s), Handle::Image(img)) => {
                    let uri = Xaml::Uri::CreateUri(s.as_str())?;
                    let bmp = Xaml::BitmapImage::new()?;
                    bmp.cast::<Xaml::IBitmapImage>()?.put_UriSource(&uri)?;
                    img.put_Source(&bmp.cast::<Xaml::ImageSource>()?)
                }
                (Prop::ImageSource, PropValue::Unset, Handle::Image(img)) => img.put_Source(None),
                (Prop::ImageStretch, PropValue::ImageStretch(s), Handle::Image(img)) => {
                    use ImageStretch as E;
                    use Xaml::Stretch as X;
                    let mapped = match s {
                        E::Uniform => X::Uniform,
                        E::UniformToFill => X::UniformToFill,
                        E::Fill => X::Fill,
                        E::None => X::None,
                    };
                    img.put_Stretch(mapped)
                }
                (Prop::ImageStretch, PropValue::Unset, Handle::Image(img)) => {
                    img.put_Stretch(Xaml::Stretch::Uniform)
                }
                (Prop::ImageStretch, PropValue::ImageStretch(s), Handle::Viewbox(vb)) => {
                    use ImageStretch as E;
                    use Xaml::Stretch as X;
                    let mapped = match s {
                        E::Uniform => X::Uniform,
                        E::UniformToFill => X::UniformToFill,
                        E::Fill => X::Fill,
                        E::None => X::None,
                    };
                    vb.put_Stretch(mapped)
                }
                (Prop::ImageStretch, PropValue::Unset, Handle::Viewbox(vb)) => {
                    vb.put_Stretch(Xaml::Stretch::Uniform)
                }
                (Prop::SelectedIndex, PropValue::I32(v), Handle::TabView(tv)) => {
                    tv.put_SelectedIndex(*v)
                }
                (Prop::SelectedIndex, PropValue::Unset, Handle::TabView(tv)) => {
                    tv.put_SelectedIndex(-1)
                }
                (Prop::CanReorderTabs, PropValue::Bool(v), Handle::TabView(tv)) => {
                    tv.put_CanReorderTabs(*v)
                }
                (Prop::CanReorderTabs, PropValue::Unset, Handle::TabView(tv)) => {
                    tv.put_CanReorderTabs(false)
                }
                (Prop::IsAddTabButtonVisible, PropValue::Bool(v), Handle::TabView(tv)) => {
                    tv.put_IsAddTabButtonVisible(*v)
                }
                (Prop::IsAddTabButtonVisible, PropValue::Unset, Handle::TabView(tv)) => {
                    tv.put_IsAddTabButtonVisible(true)
                }
                (Prop::TabHeader, PropValue::Str(s), Handle::TabViewItem(ti)) => {
                    let tb = string_as_textblock(s)?;
                    ti.put_Header(&tb)
                }
                (Prop::TabHeader, PropValue::Unset, Handle::TabViewItem(ti)) => ti.put_Header(None),
                (Prop::TabItemKey, PropValue::Str(s), Handle::TabViewItem(ti)) => {
                    let tag = windows_reference::IReference::from(s.as_str());
                    ti.cast::<Xaml::IFrameworkElement>()?.put_Tag(&tag)
                }
                (Prop::TabItemKey, PropValue::Unset, Handle::TabViewItem(ti)) => {
                    ti.cast::<Xaml::IFrameworkElement>()?.put_Tag(None)
                }
                (
                    Prop::NavMenuItems,
                    PropValue::NavMenuItems(items),
                    Handle::NavigationView(nv),
                ) => {
                    let menu = nv.get_MenuItems()?;
                    menu.Clear()?;
                    for item in items {
                        let nv_item = build_nav_view_item(item)?;
                        menu.Append(&nv_item)?;
                    }
                    Ok(())
                }
                (Prop::IsPaneOpen, PropValue::Bool(v), Handle::NavigationView(nv)) => {
                    nv.put_IsPaneOpen(*v)
                }
                (Prop::IsPaneOpen, PropValue::Unset, Handle::NavigationView(nv)) => {
                    nv.put_IsPaneOpen(true)
                }
                (
                    Prop::PaneDisplayMode,
                    PropValue::NavPaneDisplayMode(mode),
                    Handle::NavigationView(nv),
                ) => {
                    use NavViewPaneDisplayMode as M;
                    use Xaml::NavigationViewPaneDisplayMode as W;
                    let mapped = match mode {
                        M::Auto => W::Auto,
                        M::Left => W::Left,
                        M::Top => W::Top,
                        M::LeftCompact => W::LeftCompact,
                        M::LeftMinimal => W::LeftMinimal,
                    };
                    nv.cast::<Xaml::INavigationView2>()?
                        .put_PaneDisplayMode(mapped)
                }
                (Prop::IsBackEnabled, PropValue::Bool(v), Handle::NavigationView(nv)) => {
                    nv.cast::<Xaml::INavigationView2>()?.put_IsBackEnabled(*v)
                }
                (Prop::IsBackEnabled, PropValue::Unset, Handle::NavigationView(nv)) => nv
                    .cast::<Xaml::INavigationView2>()?
                    .put_IsBackEnabled(false),
                (Prop::IsSettingsVisible, PropValue::Bool(v), Handle::NavigationView(nv)) => {
                    nv.put_IsSettingsVisible(*v)
                }
                (Prop::IsSettingsVisible, PropValue::Unset, Handle::NavigationView(nv)) => {
                    nv.put_IsSettingsVisible(true)
                }
                (Prop::PaneTitle, PropValue::Str(s), Handle::NavigationView(nv)) => nv
                    .cast::<Xaml::INavigationView2>()?
                    .put_PaneTitle(s.as_str()),
                (Prop::PaneTitle, PropValue::Unset, Handle::NavigationView(nv)) => {
                    nv.cast::<Xaml::INavigationView2>()?.put_PaneTitle("")
                }
                (Prop::NavHeaderString, PropValue::Str(s), Handle::NavigationView(nv)) => {
                    let tb = string_as_textblock(s)?;
                    nv.put_Header(&tb)
                }
                (Prop::NavHeaderString, PropValue::Unset, Handle::NavigationView(nv)) => {
                    nv.put_Header(None)
                }
                (Prop::NavSelectedTag, PropValue::Str(tag), Handle::NavigationView(nv)) => {
                    select_nav_item_by_tag(nv, tag)
                }
                (Prop::NavSelectedTag, PropValue::Unset, Handle::NavigationView(nv)) => {
                    nv.put_SelectedItem(None)
                }
                (Prop::NavAutoSuggestBox, PropValue::Bool(true), Handle::NavigationView(nv)) => {
                    // Create an AutoSuggestBox if one isn't already set.
                    let asb = Xaml::AutoSuggestBox::new()?;
                    nv.put_AutoSuggestBox(&asb)
                }
                (Prop::NavAutoSuggestBox, PropValue::Bool(false), Handle::NavigationView(nv)) => {
                    nv.put_AutoSuggestBox(None)
                }
                (Prop::NavAutoSuggestBox, PropValue::Unset, Handle::NavigationView(nv)) => {
                    nv.put_AutoSuggestBox(None)
                }
                (
                    Prop::NavAutoSuggestPlaceholder,
                    PropValue::Str(s),
                    Handle::NavigationView(nv),
                ) => {
                    if let Ok(asb) = nv.get_AutoSuggestBox() {
                        asb.put_PlaceholderText(s.as_str())?;
                    }
                    Ok(())
                }
                (
                    Prop::NavAutoSuggestItems,
                    PropValue::StrList(items),
                    Handle::NavigationView(nv),
                ) => {
                    if let Ok(asb) = nv.get_AutoSuggestBox() {
                        let vec: Vec<Option<windows_core::IInspectable>> = items
                            .iter()
                            .map(|s| {
                                let r = windows_reference::IReference::from(s.as_str());
                                Some(r.into())
                            })
                            .collect();
                        let ivec: windows_collections::IVector<windows_core::IInspectable> =
                            vec.into();
                        asb.cast::<Xaml::IItemsControl>()?.put_ItemsSource(&ivec)?;
                    }
                    Ok(())
                }
                (Prop::TitleBarTitle, PropValue::Str(s), Handle::TitleBar(tb)) => {
                    tb.put_Title(s.as_str())
                }
                (Prop::TitleBarTitle, PropValue::Unset, Handle::TitleBar(tb)) => tb.put_Title(""),
                (Prop::TitleBarSubtitle, PropValue::Str(s), Handle::TitleBar(tb)) => {
                    tb.put_Subtitle(s.as_str())
                }
                (Prop::TitleBarSubtitle, PropValue::Unset, Handle::TitleBar(tb)) => {
                    tb.put_Subtitle("")
                }
                (Prop::TitleBarTall, PropValue::Bool(v), Handle::TitleBar(_)) => {
                    super::host::set_titlebar_height(*v);
                    Ok(())
                }
                (Prop::TitleBarTall, PropValue::Unset, Handle::TitleBar(_)) => {
                    super::host::set_titlebar_height(false);
                    Ok(())
                }
                (Prop::IsBackButtonVisible, PropValue::Bool(v), Handle::TitleBar(tb)) => {
                    tb.put_IsBackButtonVisible(*v)
                }
                (Prop::IsBackButtonVisible, PropValue::Unset, Handle::TitleBar(tb)) => {
                    tb.put_IsBackButtonVisible(false)
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
                (Prop::IsBackButtonVisible, PropValue::Unset, Handle::NavigationView(nv)) => nv
                    .cast::<Xaml::INavigationView2>()?
                    .put_IsBackButtonVisible(Xaml::NavigationViewBackButtonVisible::Collapsed),
                (Prop::IsBackEnabled, PropValue::Bool(v), Handle::TitleBar(tb)) => {
                    tb.put_IsBackButtonEnabled(*v)
                }
                (Prop::IsBackEnabled, PropValue::Unset, Handle::TitleBar(tb)) => {
                    tb.put_IsBackButtonEnabled(false)
                }
                (Prop::IsPaneToggleButtonVisible, PropValue::Bool(v), Handle::TitleBar(tb)) => {
                    tb.put_IsPaneToggleButtonVisible(*v)
                }
                (Prop::IsPaneToggleButtonVisible, PropValue::Unset, Handle::TitleBar(tb)) => {
                    tb.put_IsPaneToggleButtonVisible(false)
                }
                (
                    Prop::IsPaneToggleButtonVisible,
                    PropValue::Bool(v),
                    Handle::NavigationView(nv),
                ) => nv.put_IsPaneToggleButtonVisible(*v),
                (Prop::SelectedIndex, PropValue::I32(v), Handle::Pivot(p)) => {
                    p.put_SelectedIndex(*v)
                }
                (Prop::SelectedIndex, PropValue::Unset, Handle::Pivot(p)) => {
                    p.put_SelectedIndex(-1)
                }
                (Prop::PivotTitle, PropValue::Str(s), Handle::Pivot(p)) => {
                    let tb = string_as_textblock(s)?;
                    p.put_Title(&tb)
                }
                (Prop::PivotTitle, PropValue::Unset, Handle::Pivot(p)) => p.put_Title(None),
                (Prop::PivotItemHeader, PropValue::Str(s), Handle::PivotItem(pi)) => {
                    let tb = string_as_textblock(s)?;
                    pi.put_Header(&tb)
                }
                (Prop::PivotItemHeader, PropValue::Unset, Handle::PivotItem(pi)) => {
                    pi.put_Header(None)
                }
                (Prop::BreadcrumbItems, PropValue::StrList(items), Handle::BreadcrumbBar(bc)) => {
                    let vec: Vec<Option<windows_core::IInspectable>> = items
                        .iter()
                        .map(|s| {
                            let r = windows_reference::IReference::from(s.as_str());
                            Some(r.into())
                        })
                        .collect();
                    let ivec: windows_collections::IVector<windows_core::IInspectable> = vec.into();
                    bc.put_ItemsSource(&ivec)
                }
                // ── W2: PasswordBox ───────────────────────────────────────────
                (Prop::BreadcrumbItems, PropValue::Unset, Handle::BreadcrumbBar(bc)) => {
                    let vec: Vec<Option<windows_core::IInspectable>> = Vec::new();
                    let ivec: windows_collections::IVector<windows_core::IInspectable> = vec.into();
                    bc.put_ItemsSource(&ivec)
                }
                (Prop::PasswordValue, PropValue::Str(s), Handle::PasswordBox(p)) => {
                    if p.get_Password().ok().as_deref() == Some(s.as_str()) {
                        return Ok(());
                    }
                    p.put_Password(s.as_str())
                }
                (Prop::PasswordValue, PropValue::Unset, Handle::PasswordBox(p)) => {
                    p.put_Password("")
                }
                (Prop::Placeholder, PropValue::Str(s), Handle::PasswordBox(p)) => {
                    p.put_PlaceholderText(s.as_str())
                }
                (Prop::Placeholder, PropValue::Unset, Handle::PasswordBox(p)) => {
                    p.put_PlaceholderText("")
                }
                (Prop::Header, PropValue::Str(s), Handle::PasswordBox(p)) => {
                    let tb = string_as_textblock(s)?;
                    p.put_Header(&tb)
                }
                (Prop::Header, PropValue::Unset, Handle::PasswordBox(p)) => p.put_Header(None),
                (Prop::IsEnabled, PropValue::Bool(v), Handle::PasswordBox(p)) => {
                    p.cast::<Xaml::IControl>()?.put_IsEnabled(*v)
                }
                (
                    Prop::PasswordRevealMode,
                    PropValue::PasswordRevealMode(m),
                    Handle::PasswordBox(p),
                ) => {
                    use crate::core::widgets::PasswordRevealMode as M;
                    let mapped = match m {
                        M::Peek => Xaml::PasswordRevealMode::Peek,
                        M::Hidden => Xaml::PasswordRevealMode::Hidden,
                        M::Visible => Xaml::PasswordRevealMode::Visible,
                    };
                    p.put_PasswordRevealMode(mapped)
                }
                (
                    Prop::IsPasswordRevealButtonEnabled,
                    PropValue::Bool(v),
                    Handle::PasswordBox(p),
                ) => p.put_IsPasswordRevealButtonEnabled(*v),
                (Prop::IsPasswordRevealButtonEnabled, PropValue::Unset, Handle::PasswordBox(p)) => {
                    p.put_IsPasswordRevealButtonEnabled(true)
                }
                // ── W3: RadioButtons ──────────────────────────────────────────
                (Prop::RadioButtonsItems, PropValue::StrList(items), Handle::RadioButtons(r)) => {
                    let vec = r.get_Items()?;
                    vec.Clear()?;
                    for s in items {
                        let insp = windows_reference::IReference::from(s.as_str());
                        vec.Append(&insp)?;
                    }
                    Ok(())
                }
                (Prop::RadioButtonsItems, PropValue::Unset, Handle::RadioButtons(r)) => {
                    let vec = r.get_Items()?;
                    vec.Clear()?;
                    Ok(())
                }
                (Prop::SelectedIndex, PropValue::I32(v), Handle::RadioButtons(r)) => {
                    r.put_SelectedIndex(*v)
                }
                (Prop::SelectedIndex, PropValue::Unset, Handle::RadioButtons(r)) => {
                    r.put_SelectedIndex(-1)
                }
                (Prop::Header, PropValue::Str(s), Handle::RadioButtons(r)) => {
                    let tb = string_as_textblock(s)?;
                    r.put_Header(&tb)
                }
                (Prop::Header, PropValue::Unset, Handle::RadioButtons(r)) => r.put_Header(None),
                (Prop::RadioButtonsMaxColumns, PropValue::I32(v), Handle::RadioButtons(r)) => {
                    r.put_MaxColumns(*v)
                }
                // ── W4: ComboBox ──────────────────────────────────────────────
                (Prop::RadioButtonsMaxColumns, PropValue::Unset, Handle::RadioButtons(r)) => {
                    r.put_MaxColumns(1)
                }
                (Prop::ComboBoxItems, PropValue::StrList(items), Handle::ComboBox(c)) => {
                    let coll =
                        c.cast::<Xaml::IItemsControl>()?
                            .get_Items()?
                            .cast::<windows_collections::IVector<windows_core::IInspectable>>()?;
                    coll.Clear()?;
                    for s in items {
                        let insp = windows_reference::IReference::from(s.as_str());
                        coll.Append(&insp)?;
                    }
                    Ok(())
                }
                (Prop::ComboBoxItems, PropValue::Unset, Handle::ComboBox(c)) => {
                    let coll =
                        c.cast::<Xaml::IItemsControl>()?
                            .get_Items()?
                            .cast::<windows_collections::IVector<windows_core::IInspectable>>()?;
                    coll.Clear()?;
                    Ok(())
                }
                (Prop::SelectedIndex, PropValue::I32(v), Handle::ComboBox(c)) => {
                    c.cast::<Xaml::ISelector>()?.put_SelectedIndex(*v)
                }
                (Prop::SelectedIndex, PropValue::Unset, Handle::ComboBox(c)) => {
                    c.cast::<Xaml::ISelector>()?.put_SelectedIndex(-1)
                }
                (Prop::Header, PropValue::Str(s), Handle::ComboBox(c)) => {
                    let tb = string_as_textblock(s)?;
                    c.put_Header(&tb)
                }
                (Prop::Header, PropValue::Unset, Handle::ComboBox(c)) => c.put_Header(None),
                (Prop::Placeholder, PropValue::Str(s), Handle::ComboBox(c)) => {
                    c.put_PlaceholderText(s.as_str())
                }
                (Prop::Placeholder, PropValue::Unset, Handle::ComboBox(c)) => {
                    c.put_PlaceholderText("")
                }
                (Prop::IsEnabled, PropValue::Bool(v), Handle::ComboBox(c)) => {
                    c.cast::<Xaml::IControl>()?.put_IsEnabled(*v)
                }
                (Prop::IsEditable, PropValue::Bool(v), Handle::ComboBox(c)) => c.put_IsEditable(*v),
=======
>>>>>>> 8433bc0a2ef9610aa1d218df8e3ef3676a4601b7
                // ── W5: Canvas attached props ─────────────────────────────────
                (Prop::IsEditable, PropValue::Unset, Handle::ComboBox(c)) => {
                    c.put_IsEditable(false)
                }
                (Prop::AttachedCanvasLeft, PropValue::F64(v), _) => {
                    Xaml::Canvas::SetLeft(&handle.as_ui_element(), *v)
                }
                (Prop::AttachedCanvasLeft, PropValue::Unset, _) => {
                    Xaml::Canvas::SetLeft(&handle.as_ui_element(), 0.0)
                }
                (Prop::AttachedCanvasTop, PropValue::F64(v), _) => {
                    Xaml::Canvas::SetTop(&handle.as_ui_element(), *v)
                }
                (Prop::AttachedCanvasTop, PropValue::Unset, _) => {
                    Xaml::Canvas::SetTop(&handle.as_ui_element(), 0.0)
                }
                (Prop::AttachedCanvasZIndex, PropValue::I32(v), _) => {
                    Xaml::Canvas::SetZIndex(&handle.as_ui_element(), *v)
                }
<<<<<<< HEAD
                // ── W6: RepeatButton ──────────────────────────────────────────
                (Prop::AttachedCanvasZIndex, PropValue::Unset, _) => {
                    Xaml::Canvas::SetZIndex(&handle.as_ui_element(), 0)
                }
                (Prop::ButtonContent, PropValue::Str(s), Handle::RepeatButton(b)) => {
                    let tb = string_as_textblock(s)?;
                    b.cast::<Xaml::IContentControl>()?.put_Content(&tb)
                }
                (Prop::ButtonContent, PropValue::Unset, Handle::RepeatButton(b)) => {
                    b.cast::<Xaml::IContentControl>()?.put_Content(None)
                }
                (Prop::IsEnabled, PropValue::Bool(v), Handle::RepeatButton(b)) => {
                    b.cast::<Xaml::IControl>()?.put_IsEnabled(*v)
                }
                (Prop::RepeatDelay, PropValue::I32(v), Handle::RepeatButton(b)) => b.put_Delay(*v),
                (Prop::RepeatDelay, PropValue::Unset, Handle::RepeatButton(b)) => b.put_Delay(0),
                (Prop::RepeatInterval, PropValue::I32(v), Handle::RepeatButton(b)) => {
                    b.put_Interval(*v)
                }
                // ── W7: RatingControl ─────────────────────────────────────────
                (Prop::RepeatInterval, PropValue::Unset, Handle::RepeatButton(b)) => {
                    b.put_Interval(0)
                }
                (Prop::NumericValue, PropValue::F64(v), Handle::RatingControl(r)) => {
                    r.put_Value(*v)
                }
                (Prop::NumericValue, PropValue::Unset, Handle::RatingControl(r)) => {
                    r.put_Value(0.0)
                }
                (Prop::MaxRating, PropValue::I32(v), Handle::RatingControl(r)) => {
                    r.put_MaxRating(*v)
                }
                (Prop::MaxRating, PropValue::Unset, Handle::RatingControl(r)) => r.put_MaxRating(5),
                (Prop::RatingCaption, PropValue::Str(s), Handle::RatingControl(r)) => {
                    r.put_Caption(s.as_str())
                }
                (Prop::RatingCaption, PropValue::Unset, Handle::RatingControl(r)) => {
                    r.put_Caption("")
                }
                (Prop::PlaceholderValue, PropValue::F64(v), Handle::RatingControl(r)) => {
                    r.put_PlaceholderValue(*v)
                }
                (Prop::PlaceholderValue, PropValue::Unset, Handle::RatingControl(r)) => {
                    r.put_PlaceholderValue(0.0)
                }
                (Prop::IsReadOnly, PropValue::Bool(v), Handle::RatingControl(r)) => {
                    r.put_IsReadOnly(*v)
                }
                // ── W8: ColorPicker ───────────────────────────────────────────
                (Prop::IsReadOnly, PropValue::Unset, Handle::RatingControl(r)) => {
                    r.put_IsReadOnly(false)
                }
                (Prop::ColorValue, PropValue::Color { a, r, g, b }, Handle::ColorPicker(cp)) => cp
                    .put_Color(Xaml::Color {
                        A: *a,
                        R: *r,
                        G: *g,
                        B: *b,
                    }),
                (Prop::IsAlphaEnabled, PropValue::Bool(v), Handle::ColorPicker(cp)) => {
                    cp.put_IsAlphaEnabled(*v)
                }
                (Prop::IsAlphaEnabled, PropValue::Unset, Handle::ColorPicker(cp)) => {
                    cp.put_IsAlphaEnabled(false)
                }
                (Prop::IsHexInputVisible, PropValue::Bool(v), Handle::ColorPicker(cp)) => {
                    cp.put_IsHexInputVisible(*v)
                }
                (Prop::IsHexInputVisible, PropValue::Unset, Handle::ColorPicker(cp)) => {
                    cp.put_IsHexInputVisible(false)
                }
                (Prop::IsColorSliderVisible, PropValue::Bool(v), Handle::ColorPicker(cp)) => {
                    cp.put_IsColorSliderVisible(*v)
                }
                (Prop::IsColorSliderVisible, PropValue::Unset, Handle::ColorPicker(cp)) => {
                    cp.put_IsColorSliderVisible(false)
                }
                (
                    Prop::IsColorChannelTextInputVisible,
                    PropValue::Bool(v),
                    Handle::ColorPicker(cp),
                ) => cp.put_IsColorChannelTextInputVisible(*v),
                // ── W9: DatePicker ────────────────────────────────────────────
                (Prop::Header, PropValue::Str(s), Handle::DatePicker(dp)) => {
                    let tb = string_as_textblock(s)?;
                    dp.put_Header(&tb)
                }
                (Prop::Header, PropValue::Unset, Handle::DatePicker(dp)) => dp.put_Header(None),
                (Prop::DayVisible, PropValue::Bool(v), Handle::DatePicker(dp)) => {
                    dp.put_DayVisible(*v)
                }
                (Prop::DayVisible, PropValue::Unset, Handle::DatePicker(dp)) => {
                    dp.put_DayVisible(false)
                }
                (Prop::MonthVisible, PropValue::Bool(v), Handle::DatePicker(dp)) => {
                    dp.put_MonthVisible(*v)
                }
                (Prop::MonthVisible, PropValue::Unset, Handle::DatePicker(dp)) => {
                    dp.put_MonthVisible(false)
                }
                (Prop::YearVisible, PropValue::Bool(v), Handle::DatePicker(dp)) => {
                    dp.put_YearVisible(*v)
                }
                (Prop::YearVisible, PropValue::Unset, Handle::DatePicker(dp)) => {
                    dp.put_YearVisible(false)
                }
                (Prop::IsEnabled, PropValue::Bool(v), Handle::DatePicker(dp)) => {
                    dp.cast::<Xaml::IControl>()?.put_IsEnabled(*v)
                }
                // ── W10: TimePicker ───────────────────────────────────────────
                (Prop::Header, PropValue::Str(s), Handle::TimePicker(tp)) => {
                    let tb = string_as_textblock(s)?;
                    tp.put_Header(&tb)
                }
                (Prop::Header, PropValue::Unset, Handle::TimePicker(tp)) => tp.put_Header(None),
                (Prop::ClockIdentifier, PropValue::Str(s), Handle::TimePicker(tp)) => {
                    tp.put_ClockIdentifier(s.as_str())
                }
                (Prop::ClockIdentifier, PropValue::Unset, Handle::TimePicker(tp)) => {
                    tp.put_ClockIdentifier("")
                }
                (Prop::MinuteIncrement, PropValue::I32(v), Handle::TimePicker(tp)) => {
                    tp.put_MinuteIncrement(*v)
                }
                (Prop::MinuteIncrement, PropValue::Unset, Handle::TimePicker(tp)) => {
                    tp.put_MinuteIncrement(1)
                }
                (Prop::IsEnabled, PropValue::Bool(v), Handle::TimePicker(tp)) => {
                    tp.cast::<Xaml::IControl>()?.put_IsEnabled(*v)
                }
                // ── W11: CalendarDatePicker ───────────────────────────────────
                (Prop::Header, PropValue::Str(s), Handle::CalendarDatePicker(cdp)) => {
                    let tb = string_as_textblock(s)?;
                    cdp.put_Header(&tb)
                }
                (Prop::Header, PropValue::Unset, Handle::CalendarDatePicker(cdp)) => {
                    cdp.put_Header(None)
                }
                (Prop::Placeholder, PropValue::Str(s), Handle::CalendarDatePicker(cdp)) => {
                    cdp.put_PlaceholderText(s.as_str())
                }
                (Prop::Placeholder, PropValue::Unset, Handle::CalendarDatePicker(cdp)) => {
                    cdp.put_PlaceholderText("")
                }
                (Prop::IsTodayHighlighted, PropValue::Bool(v), Handle::CalendarDatePicker(cdp)) => {
                    cdp.put_IsTodayHighlighted(*v)
                }
                (Prop::IsTodayHighlighted, PropValue::Unset, Handle::CalendarDatePicker(cdp)) => {
                    cdp.put_IsTodayHighlighted(true)
                }
                (Prop::IsCalendarOpen, PropValue::Bool(v), Handle::CalendarDatePicker(cdp)) => {
                    cdp.put_IsCalendarOpen(*v)
                }
                (Prop::IsCalendarOpen, PropValue::Unset, Handle::CalendarDatePicker(cdp)) => {
                    cdp.put_IsCalendarOpen(false)
                }
                (Prop::IsEnabled, PropValue::Bool(v), Handle::CalendarDatePicker(cdp)) => {
                    cdp.cast::<Xaml::IControl>()?.put_IsEnabled(*v)
                }
                // ── W12: CalendarView ─────────────────────────────────────────
                (Prop::IsTodayHighlighted, PropValue::Bool(v), Handle::CalendarView(cv)) => {
                    cv.put_IsTodayHighlighted(*v)
                }
                (Prop::IsTodayHighlighted, PropValue::Unset, Handle::CalendarView(cv)) => {
                    cv.put_IsTodayHighlighted(true)
                }
                (Prop::IsGroupLabelVisible, PropValue::Bool(v), Handle::CalendarView(cv)) => {
                    cv.put_IsGroupLabelVisible(*v)
                }
                (Prop::IsGroupLabelVisible, PropValue::Unset, Handle::CalendarView(cv)) => {
                    cv.put_IsGroupLabelVisible(false)
                }
                (Prop::IsEnabled, PropValue::Bool(v), Handle::CalendarView(cv)) => {
                    cv.cast::<Xaml::IControl>()?.put_IsEnabled(*v)
                }
                // ── W13: ListBox ──────────────────────────────────────────────
                (Prop::ListBoxItems, PropValue::StrList(items), Handle::ListBox(lb)) => {
                    let coll =
                        lb.cast::<Xaml::IItemsControl>()?
                            .get_Items()?
                            .cast::<windows_collections::IVector<windows_core::IInspectable>>()?;
                    coll.Clear()?;
                    for s in items {
                        let insp = windows_reference::IReference::from(s.as_str());
                        coll.Append(&insp)?;
                    }
                    Ok(())
                }
                (Prop::ListBoxItems, PropValue::Unset, Handle::ListBox(lb)) => {
                    let coll =
                        lb.cast::<Xaml::IItemsControl>()?
                            .get_Items()?
                            .cast::<windows_collections::IVector<windows_core::IInspectable>>()?;
                    coll.Clear()?;
                    Ok(())
                }
                (Prop::SelectedIndex, PropValue::I32(v), Handle::ListBox(lb)) => {
                    lb.cast::<Xaml::ISelector>()?.put_SelectedIndex(*v)
                }
                (Prop::SelectedIndex, PropValue::Unset, Handle::ListBox(lb)) => {
                    lb.cast::<Xaml::ISelector>()?.put_SelectedIndex(-1)
                }
                (Prop::IsEnabled, PropValue::Bool(v), Handle::ListBox(lb)) => {
                    lb.cast::<Xaml::IControl>()?.put_IsEnabled(*v)
                }
                // ── W14: DropDownButton ───────────────────────────────────────
                (Prop::ButtonContent, PropValue::Str(s), Handle::DropDownButton(ddb)) => {
                    let insp = windows_reference::IReference::from(s.as_str());
                    ddb.cast::<Xaml::IContentControl>()?.put_Content(&insp)
                }
                (Prop::ButtonContent, PropValue::Unset, Handle::DropDownButton(ddb)) => {
                    ddb.cast::<Xaml::IContentControl>()?.put_Content(None)
                }
                (Prop::IsEnabled, PropValue::Bool(v), Handle::DropDownButton(ddb)) => {
                    ddb.cast::<Xaml::IControl>()?.put_IsEnabled(*v)
                }
                // ── W15: SplitButton ──────────────────────────────────────────
                (Prop::ButtonContent, PropValue::Str(s), Handle::SplitButton(sb)) => {
                    let insp = windows_reference::IReference::from(s.as_str());
                    sb.cast::<Xaml::IContentControl>()?.put_Content(&insp)
                }
                (Prop::ButtonContent, PropValue::Unset, Handle::SplitButton(sb)) => {
                    sb.cast::<Xaml::IContentControl>()?.put_Content(None)
                }
                (Prop::IsEnabled, PropValue::Bool(v), Handle::SplitButton(sb)) => {
                    sb.cast::<Xaml::IControl>()?.put_IsEnabled(*v)
                }
                // ── W16: AutoSuggestBox ───────────────────────────────────────
                (Prop::AutoSuggestText, PropValue::Str(s), Handle::AutoSuggestBox(asb)) => {
                    // Skip SetText when the control already has this value —
                    // calling SetText during a user-initiated TextChanged
                    // cycle steals focus from the input field.
                    if asb.get_Text().ok().as_deref() == Some(s.as_str()) {
                        return Ok(());
                    }
                    asb.put_Text(s)
                }
                (Prop::AutoSuggestText, PropValue::Unset, Handle::AutoSuggestBox(asb)) => {
                    if asb.get_Text().ok().as_deref() == Some("") {
                        return Ok(());
                    }
                    asb.put_Text("")
                }
                (
                    Prop::AutoSuggestItems,
                    PropValue::StrList(items),
                    Handle::AutoSuggestBox(asb),
                ) => {
                    // Build a Rust Vec, wrap into IVector via the stock
                    // implementation, then assign as ItemsSource.
                    let vec: Vec<Option<windows_core::IInspectable>> = items
                        .iter()
                        .map(|s| {
                            let r = windows_reference::IReference::from(s.as_str());
                            Some(r.into())
                        })
                        .collect();
                    let ivec: windows_collections::IVector<windows_core::IInspectable> = vec.into();
                    asb.cast::<Xaml::IItemsControl>()?.put_ItemsSource(&ivec)
                }
                (Prop::Placeholder, PropValue::Str(s), Handle::AutoSuggestBox(asb)) => {
                    asb.put_PlaceholderText(s)
                }
                (Prop::Placeholder, PropValue::Unset, Handle::AutoSuggestBox(asb)) => {
                    asb.put_PlaceholderText("")
                }
                (Prop::Header, PropValue::Str(s), Handle::AutoSuggestBox(asb)) => {
                    let insp = windows_reference::IReference::from(s.as_str());
                    asb.put_Header(&insp)
                }
                (Prop::Header, PropValue::Unset, Handle::AutoSuggestBox(asb)) => {
                    asb.put_Header(None)
                }
                (Prop::IsEnabled, PropValue::Bool(v), Handle::AutoSuggestBox(asb)) => {
                    asb.cast::<Xaml::IControl>()?.put_IsEnabled(*v)
                }
                // ── W17: SplitView ───────────────────────────────────────
                (Prop::SplitViewDisplayMode, PropValue::I32(m), Handle::SplitView(sv)) => {
                    sv.put_DisplayMode(Xaml::SplitViewDisplayMode(*m))
                }
                (Prop::SplitViewDisplayMode, PropValue::Unset, Handle::SplitView(sv)) => {
                    sv.put_DisplayMode(Xaml::SplitViewDisplayMode(0))
                }
                (Prop::SplitViewIsPaneOpen, PropValue::Bool(v), Handle::SplitView(sv)) => {
                    sv.put_IsPaneOpen(*v)
                }
                (Prop::SplitViewIsPaneOpen, PropValue::Unset, Handle::SplitView(sv)) => {
                    sv.put_IsPaneOpen(false)
                }
                (Prop::SplitViewOpenPaneLength, PropValue::F64(v), Handle::SplitView(sv)) => {
                    sv.put_OpenPaneLength(*v)
                }
                (Prop::SplitViewOpenPaneLength, PropValue::Unset, Handle::SplitView(sv)) => {
                    sv.put_OpenPaneLength(320.0)
                }
                (Prop::SplitViewCompactPaneLength, PropValue::F64(v), Handle::SplitView(sv)) => {
                    sv.put_CompactPaneLength(*v)
                }
=======
                // MenuBar items still live here because rebuilding them needs
                // backend-stored click handlers.
>>>>>>> 8433bc0a2ef9610aa1d218df8e3ef3676a4601b7
                // ── W18: MenuBar ─────────────────────────────────────────
                (Prop::SplitViewCompactPaneLength, PropValue::Unset, Handle::SplitView(sv)) => {
                    sv.put_CompactPaneLength(48.0)
                }
                (Prop::MenuBarItems, PropValue::MenuBarItems(items), Handle::MenuBar(mb)) => {
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
                    // Re-wire click handlers if a handler is already stored.
                    let handlers = self.menu_click_handlers.borrow();
                    if let Some(handler) = handlers.get(&id) {
                        let revs = Self::wire_menu_bar_clicks(mb, handler);
                        if !revs.is_empty() {
                            self.event_revokers
                                .borrow_mut()
                                .insert((id, Event::MenuBarItemClicked), revs);
                        }
                    }
                    Ok(())
                }
                // MenuFlyout items still live here because rebuilding them
                // needs backend-stored click handlers.
                // MenuFlyout on Button/DropDownButton
                (Prop::MenuBarItems, PropValue::Unset, Handle::MenuBar(mb)) => {
                    mb.get_Items()?.Clear()?;
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
                    // Wire click handlers if stored.
                    let handlers = self.menu_click_handlers.borrow();
                    if let Some(handler) = handlers.get(&id) {
                        let revs = Self::wire_flyout_clicks(&flyout, handler);
                        if !revs.is_empty() {
                            self.event_revokers
                                .borrow_mut()
                                .insert((id, Event::MenuFlyoutItemClicked), revs);
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
                    // Wire click handlers if stored.
                    let handlers = self.menu_click_handlers.borrow();
                    if let Some(handler) = handlers.get(&id) {
                        let revs = Self::wire_flyout_clicks(&flyout, handler);
                        if !revs.is_empty() {
                            self.event_revokers
                                .borrow_mut()
                                .insert((id, Event::MenuFlyoutItemClicked), revs);
                        }
                    }
                    Ok(())
                }
                // CommandBarFlyout commands still live here because rebuilding
                // them needs backend-stored click handlers.
                // CommandBarFlyout on Button
                (Prop::MenuFlyoutItems, PropValue::Unset, Handle::Button(btn)) => {
                    if let Ok(fb) = btn.get_Flyout()
                        && let Ok(flyout) = fb.cast::<Xaml::MenuFlyout>()
                    {
                        flyout.get_Items()?.Clear()?;
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
                    // Wire click handlers if stored.
                    let handlers = self.command_bar_flyout_handlers.borrow();
                    if let Some(handler) = handlers.get(&id) {
                        let mut revs = Self::wire_command_bar_clicks(&primary_cmds, handler);
                        revs.extend(Self::wire_command_bar_clicks(&secondary_cmds, handler));
                        if !revs.is_empty() {
                            self.event_revokers
                                .borrow_mut()
                                .insert((id, Event::CommandBarFlyoutClick), revs);
                        }
                    }
                    Ok(())
                }
<<<<<<< HEAD
                // ── W19: ScrollView ──────────────────────────────────────
                (
                    Prop::HorizontalScrollBarVisibility,
                    PropValue::ScrollViewScrollBarVis(v),
                    Handle::ScrollView(sv),
                ) => {
                    use ScrollViewScrollBarVisibility as E;
                    use Xaml::ScrollingScrollBarVisibility as W;
                    let mapped = match v {
                        E::Auto => W::Auto,
                        E::Visible => W::Visible,
                        E::Hidden => W::Hidden,
                    };
                    sv.put_HorizontalScrollBarVisibility(mapped)
                }
                (
                    Prop::VerticalScrollBarVisibility,
                    PropValue::ScrollViewScrollBarVis(v),
                    Handle::ScrollView(sv),
                ) => {
                    use ScrollViewScrollBarVisibility as E;
                    use Xaml::ScrollingScrollBarVisibility as W;
                    let mapped = match v {
                        E::Auto => W::Auto,
                        E::Visible => W::Visible,
                        E::Hidden => W::Hidden,
                    };
                    sv.put_VerticalScrollBarVisibility(mapped)
                }
                // ── W20: TreeView ────────────────────────────────────────
                (Prop::TreeViewNodes, PropValue::TreeViewNodes(nodes), Handle::TreeView(tv)) => {
                    let root = tv.get_RootNodes()?;
                    root.Clear()?;
                    for node_def in nodes {
                        let node = build_tree_view_node(node_def)?;
                        root.Append(&node)?;
                    }
                    Ok(())
                }
                (Prop::TreeViewNodes, PropValue::Unset, Handle::TreeView(tv)) => {
                    tv.get_RootNodes()?.Clear()?;
                    Ok(())
                }
                (
                    Prop::TreeViewSelectionMode,
                    PropValue::TreeViewSelectionMode(mode),
                    Handle::TreeView(tv),
                ) => {
                    use TreeSelectionMode as E;
                    use Xaml::TreeViewSelectionMode as W;
                    let mapped = match mode {
                        E::None => W::None,
                        E::Single => W::Single,
                        E::Multiple => W::Multiple,
                    };
                    tv.put_SelectionMode(mapped)
                }
=======
>>>>>>> 8433bc0a2ef9610aa1d218df8e3ef3676a4601b7
                // ── W21: CommandBar ──────────────────────────────────────
                (
                    Prop::CommandBarPrimaryCommands,
                    PropValue::CommandBarCommands(cmds),
                    Handle::CommandBar(cb),
                ) => {
                    let primary = cb.get_PrimaryCommands()?;
                    primary.Clear()?;
                    for def in cmds {
                        let el = build_command_bar_element(def)?;
                        primary.Append(&el)?;
                    }
                    // Wire click handlers.
                    let handlers = self.menu_click_handlers.borrow();
                    if let Some(handler) = handlers.get(&id) {
                        let revs = Self::wire_command_bar_clicks(&primary, handler);
                        if !revs.is_empty() {
                            self.event_revokers
                                .borrow_mut()
                                .insert((id, Event::CommandBarClick), revs);
                        }
                    }
                    Ok(())
                }
                (
                    Prop::CommandBarSecondaryCommands,
                    PropValue::CommandBarCommands(cmds),
                    Handle::CommandBar(cb),
                ) => {
                    let secondary = cb.get_SecondaryCommands()?;
                    secondary.Clear()?;
                    for def in cmds {
                        let el = build_command_bar_element(def)?;
                        secondary.Append(&el)?;
                    }
                    // Wire click handlers for secondary too.
                    let handlers = self.menu_click_handlers.borrow();
                    if let Some(handler) = handlers.get(&id) {
                        let revs = Self::wire_command_bar_clicks(&secondary, handler);
                        if !revs.is_empty() {
                            let mut rev_map = self.event_revokers.borrow_mut();
                            rev_map
                                .entry((id, Event::CommandBarClick))
                                .or_default()
                                .extend(revs);
                        }
                    }
                    Ok(())
                }
                // CommandBar item collections still live here because they need
                // backend-stored click handlers when rebuilt.
                // ── W22: TeachingTip ────────────────────────────────────
<<<<<<< HEAD
                (Prop::TeachingTipTitle, PropValue::Str(s), Handle::TeachingTip(tt)) => {
                    tt.put_Title(s.as_str())
                }
                (Prop::TeachingTipTitle, PropValue::Unset, Handle::TeachingTip(tt)) => {
                    tt.put_Title("")
                }
                (Prop::TeachingTipSubtitle, PropValue::Str(s), Handle::TeachingTip(tt)) => {
                    tt.put_Subtitle(s.as_str())
                }
                (Prop::TeachingTipSubtitle, PropValue::Unset, Handle::TeachingTip(tt)) => {
                    tt.put_Subtitle("")
                }
                (Prop::TeachingTipIsOpen, PropValue::Bool(v), Handle::TeachingTip(tt)) => {
                    tt.put_IsOpen(*v)
                }
                (Prop::TeachingTipIsOpen, PropValue::Unset, Handle::TeachingTip(tt)) => {
                    tt.put_IsOpen(false)
                }
                (Prop::TeachingTipIsLightDismiss, PropValue::Bool(v), Handle::TeachingTip(tt)) => {
                    tt.put_IsLightDismissEnabled(*v)
                }
                (Prop::TeachingTipIsLightDismiss, PropValue::Unset, Handle::TeachingTip(tt)) => {
                    tt.put_IsLightDismissEnabled(false)
                }
                (
                    Prop::TeachingTipPlacement,
                    PropValue::TeachingTipPlacement(p),
                    Handle::TeachingTip(tt),
                ) => {
                    use TeachingTipPlacement as E;
                    use Xaml::TeachingTipPlacementMode as W;
                    let mapped = match p {
                        E::Auto => W::Auto,
                        E::Top => W::Top,
                        E::Bottom => W::Bottom,
                        E::Left => W::Left,
                        E::Right => W::Right,
                        E::TopRight => W::TopRight,
                        E::TopLeft => W::TopLeft,
                        E::BottomRight => W::BottomRight,
                        E::BottomLeft => W::BottomLeft,
                        E::LeftTop => W::LeftTop,
                        E::LeftBottom => W::LeftBottom,
                        E::RightTop => W::RightTop,
                        E::RightBottom => W::RightBottom,
                        E::Center => W::Center,
                    };
                    tt.put_PreferredPlacement(mapped)
                }
                (Prop::TeachingTipActionButton, PropValue::Str(s), Handle::TeachingTip(tt)) => {
                    let boxed: windows_core::IInspectable =
                        windows_reference::IReference::<windows_core::HSTRING>::from(
                            windows_core::HSTRING::from(s.as_str()),
                        )
                        .cast()?;
                    tt.put_ActionButtonContent(&boxed)
                }
                (Prop::TeachingTipActionButton, PropValue::Unset, Handle::TeachingTip(tt)) => {
                    tt.put_ActionButtonContent(None)
                }
                (Prop::TeachingTipCloseButton, PropValue::Str(s), Handle::TeachingTip(tt)) => {
                    let boxed: windows_core::IInspectable =
                        windows_reference::IReference::<windows_core::HSTRING>::from(
                            windows_core::HSTRING::from(s.as_str()),
                        )
                        .cast()?;
                    tt.put_CloseButtonContent(&boxed)
                }
=======
>>>>>>> 8433bc0a2ef9610aa1d218df8e3ef3676a4601b7
                // ── W23: SelectorBar ────────────────────────────────────
                (Prop::TeachingTipCloseButton, PropValue::Unset, Handle::TeachingTip(tt)) => {
                    tt.put_CloseButtonContent(None)
                }
                (
                    Prop::SelectorBarItems,
                    PropValue::SelectorBarItems(items),
                    Handle::SelectorBar(sb),
                ) => {
                    let vec = sb.get_Items()?;
                    vec.Clear()?;
                    for def in items {
                        let item = Xaml::SelectorBarItem::new()?;
                        item.put_Text(&def.text)?;
                        if let Some(sym) = &def.icon {
                            let icon_elem = Xaml::SymbolIcon::CreateInstanceWithSymbol(
                                Xaml::Symbol(sym.to_raw()),
                            )?;
                            item.put_Icon(&icon_elem)?;
                        }
                        vec.Append(&item)?;
                    }
                    Ok(())
                }
<<<<<<< HEAD
                (Prop::RichEditBoxText, PropValue::Str(s), Handle::RichEditBox(reb)) => {
                    let doc = reb.get_Document()?;
                    let mut current = windows_core::HSTRING::default();
                    doc.GetText(Xaml::TextGetOptions::None, &mut current).ok();
                    if current == s.as_str() {
                        return Ok(());
                    }
                    doc.SetText(Xaml::TextSetOptions::None, s.as_str())
                }
                (Prop::RichEditBoxText, PropValue::Unset, Handle::RichEditBox(reb)) => {
                    let doc = reb.get_Document()?;
                    doc.SetText(Xaml::TextSetOptions::None, "")
                }
                (Prop::Placeholder, PropValue::Str(s), Handle::RichEditBox(reb)) => {
                    reb.put_PlaceholderText(s.as_str())
                }
                (Prop::Placeholder, PropValue::Unset, Handle::RichEditBox(reb)) => {
                    reb.put_PlaceholderText("")
                }
                (Prop::Header, PropValue::Str(s), Handle::RichEditBox(reb)) => {
                    let tb = string_as_textblock(s)?;
                    reb.put_Header(&tb)
                }
                (Prop::Header, PropValue::Unset, Handle::RichEditBox(reb)) => reb.put_Header(None),
                (Prop::RichEditBoxIsReadOnly, PropValue::Bool(v), Handle::RichEditBox(reb)) => {
                    reb.put_IsReadOnly(*v)
                }
                // ── Flyout on Button ──────────────────────────────────────────
                (Prop::RichEditBoxIsReadOnly, PropValue::Unset, Handle::RichEditBox(reb)) => {
                    reb.put_IsReadOnly(false)
                }
                (Prop::FlyoutContent, PropValue::Str(s), Handle::Button(b)) => {
                    let flyout = Xaml::Flyout::new()?;
                    let tb = string_as_textblock(s)?;
                    flyout.put_Content(&tb)?;
                    b.put_Flyout(&flyout)?;
                    Ok(())
                }
                (Prop::FlyoutContent, PropValue::Unset, Handle::Button(b)) => b.put_Flyout(None),
                (Prop::FlyoutPlacement, PropValue::FlyoutPlacement(p), Handle::Button(b)) => {
                    // The flyout must already exist (FlyoutContent set first).
                    if let Ok(fb) = b.get_Flyout() {
                        let mode = match p {
                            crate::core::widgets::FlyoutPlacement::Top => {
                                Xaml::FlyoutPlacementMode::Top
                            }
                            crate::core::widgets::FlyoutPlacement::Bottom => {
                                Xaml::FlyoutPlacementMode::Bottom
                            }
                            crate::core::widgets::FlyoutPlacement::Left => {
                                Xaml::FlyoutPlacementMode::Left
                            }
                            crate::core::widgets::FlyoutPlacement::Right => {
                                Xaml::FlyoutPlacementMode::Right
                            }
                            crate::core::widgets::FlyoutPlacement::Full => {
                                Xaml::FlyoutPlacementMode::Full
                            }
                            crate::core::widgets::FlyoutPlacement::TopEdgeAlignedLeft => {
                                Xaml::FlyoutPlacementMode::TopEdgeAlignedLeft
                            }
                            crate::core::widgets::FlyoutPlacement::TopEdgeAlignedRight => {
                                Xaml::FlyoutPlacementMode::TopEdgeAlignedRight
                            }
                            crate::core::widgets::FlyoutPlacement::BottomEdgeAlignedLeft => {
                                Xaml::FlyoutPlacementMode::BottomEdgeAlignedLeft
                            }
                            crate::core::widgets::FlyoutPlacement::BottomEdgeAlignedRight => {
                                Xaml::FlyoutPlacementMode::BottomEdgeAlignedRight
                            }
                            crate::core::widgets::FlyoutPlacement::LeftEdgeAlignedTop => {
                                Xaml::FlyoutPlacementMode::LeftEdgeAlignedTop
                            }
                            crate::core::widgets::FlyoutPlacement::LeftEdgeAlignedBottom => {
                                Xaml::FlyoutPlacementMode::LeftEdgeAlignedBottom
                            }
                            crate::core::widgets::FlyoutPlacement::RightEdgeAlignedTop => {
                                Xaml::FlyoutPlacementMode::RightEdgeAlignedTop
                            }
                            crate::core::widgets::FlyoutPlacement::RightEdgeAlignedBottom => {
                                Xaml::FlyoutPlacementMode::RightEdgeAlignedBottom
                            }
                            crate::core::widgets::FlyoutPlacement::Auto => {
                                Xaml::FlyoutPlacementMode::Auto
                            }
                        };
                        let _ = fb.cast::<Xaml::IFlyoutBase>()?.put_Placement(mode);
                    }
                    Ok(())
                }
=======
>>>>>>> 8433bc0a2ef9610aa1d218df8e3ef3676a4601b7
                // ── RelativePanel attached props ──────────────────────────────
                (Prop::FlyoutPlacement, PropValue::Unset, Handle::Button(_)) => Ok(()),
                (Prop::RelativePanelAlignLeftWithPanel, PropValue::Bool(v), _) => {
                    Xaml::RelativePanel::SetAlignLeftWithPanel(&handle.as_ui_element(), *v)
                }
                (Prop::RelativePanelAlignLeftWithPanel, PropValue::Unset, _) => {
                    Xaml::RelativePanel::SetAlignLeftWithPanel(&handle.as_ui_element(), false)
                }
                (Prop::RelativePanelAlignRightWithPanel, PropValue::Bool(v), _) => {
                    Xaml::RelativePanel::SetAlignRightWithPanel(&handle.as_ui_element(), *v)
                }
                (Prop::RelativePanelAlignRightWithPanel, PropValue::Unset, _) => {
                    Xaml::RelativePanel::SetAlignRightWithPanel(&handle.as_ui_element(), false)
                }
                (Prop::RelativePanelAlignTopWithPanel, PropValue::Bool(v), _) => {
                    Xaml::RelativePanel::SetAlignTopWithPanel(&handle.as_ui_element(), *v)
                }
                (Prop::RelativePanelAlignTopWithPanel, PropValue::Unset, _) => {
                    Xaml::RelativePanel::SetAlignTopWithPanel(&handle.as_ui_element(), false)
                }
                (Prop::RelativePanelAlignBottomWithPanel, PropValue::Bool(v), _) => {
                    Xaml::RelativePanel::SetAlignBottomWithPanel(&handle.as_ui_element(), *v)
                }
                (Prop::RelativePanelAlignBottomWithPanel, PropValue::Unset, _) => {
                    Xaml::RelativePanel::SetAlignBottomWithPanel(&handle.as_ui_element(), false)
                }
                (Prop::RelativePanelAlignHCenterWithPanel, PropValue::Bool(v), _) => {
                    Xaml::RelativePanel::SetAlignHorizontalCenterWithPanel(
                        &handle.as_ui_element(),
                        *v,
                    )
                }
                (Prop::RelativePanelAlignHCenterWithPanel, PropValue::Unset, _) => {
                    Xaml::RelativePanel::SetAlignHorizontalCenterWithPanel(
                        &handle.as_ui_element(),
                        false,
                    )
                }
                (Prop::RelativePanelAlignVCenterWithPanel, PropValue::Bool(v), _) => {
                    Xaml::RelativePanel::SetAlignVerticalCenterWithPanel(
                        &handle.as_ui_element(),
                        *v,
                    )
                }
                (Prop::RelativePanelAlignVCenterWithPanel, PropValue::Unset, _) => {
                    Xaml::RelativePanel::SetAlignVerticalCenterWithPanel(
                        &handle.as_ui_element(),
                        false,
                    )
                }
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
        match parent_h {
            Handle::StackPanel(_)
            | Handle::Grid(_)
            | Handle::Canvas(_)
            | Handle::RelativePanel(_) => {
                let vec = panel_children_vec(parent_h).unwrap();
                vec.Append(&child_ui).unwrap();
            }
            Handle::Border(b) => {
                b.put_Child(&child_ui).unwrap();
            }
            Handle::Viewbox(v) => {
                v.put_Child(&child_ui).unwrap();
            }
            Handle::ScrollViewer(_)
            | Handle::Expander(_)
            | Handle::TabViewItem(_)
            | Handle::NavigationView(_)
            | Handle::PivotItem(_) => {
                let cc = content_control_for(parent_h).unwrap();
                cc.put_Content(&child_ui).unwrap();
            }
            Handle::SplitView(sv) => {
                sv.put_Content(&child_ui).unwrap();
            }
            Handle::ScrollView(sv) => {
                sv.put_Content(&child_ui).unwrap();
            }
            Handle::TabView(tv) => {
                let items = tv.get_TabItems().unwrap();
                let insp: windows_core::IInspectable = child_ui.cast().unwrap();
                items.Append(&insp).unwrap();
            }
            Handle::Pivot(p) => {
                let items = p
                    .cast::<Xaml::IItemsControl>()
                    .unwrap()
                    .get_Items()
                    .unwrap()
                    .cast::<windows_collections::IVector<windows_core::IInspectable>>()
                    .unwrap();
                let insp: windows_core::IInspectable = child_ui.cast().unwrap();
                items.Append(&insp).unwrap();
            }
            other => {
                let kind = describe_kind(other);
                panic!("WinUIBackend::append_child: {kind} ({parent}) is not a container");
            }
        }
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
        match parent_h {
            Handle::StackPanel(_) | Handle::Grid(_) | Handle::Canvas(_) => {
                if let Some(vec) = panel_children_vec(parent_h) {
                    vec.RemoveAt(v_index as u32).unwrap();
                }
            }
            Handle::Border(b) => {
                debug_assert_eq!(v_index, 0);
                b.put_Child(None).unwrap();
            }
            Handle::Viewbox(v) => {
                debug_assert_eq!(v_index, 0);
                v.put_Child(None).unwrap();
            }
            Handle::ScrollViewer(_)
            | Handle::Expander(_)
            | Handle::TabViewItem(_)
            | Handle::NavigationView(_)
            | Handle::PivotItem(_) => {
                debug_assert_eq!(v_index, 0);
                if let Some(cc) = content_control_for(parent_h) {
                    cc.put_Content(None).unwrap();
                }
            }
            Handle::TabView(tv) => {
                let items = tv.get_TabItems().unwrap();
                items.RemoveAt(v_index as u32).unwrap();
            }
            Handle::Pivot(p) => {
                let items = p
                    .cast::<Xaml::IItemsControl>()
                    .unwrap()
                    .get_Items()
                    .unwrap()
                    .cast::<windows_collections::IVector<windows_core::IInspectable>>()
                    .unwrap();
                items.RemoveAt(v_index as u32).unwrap();
            }
            _ => panic!("WinUIBackend::remove_child: {parent} is not a container"),
        }
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
        match parent_h {
            Handle::StackPanel(s) => {
                ui_element_collection_move(
                    &s.cast::<Xaml::IPanel>().unwrap().get_Children().unwrap(),
                    v_from,
                    v_to,
                );
            }
            Handle::Grid(g) => {
                ui_element_collection_move(
                    &g.cast::<Xaml::IPanel>().unwrap().get_Children().unwrap(),
                    v_from,
                    v_to,
                );
            }
            Handle::Canvas(c) => {
                ui_element_collection_move(
                    &c.cast::<Xaml::IPanel>().unwrap().get_Children().unwrap(),
                    v_from,
                    v_to,
                );
            }
            Handle::Border(_)
            | Handle::Viewbox(_)
            | Handle::ScrollViewer(_)
            | Handle::Expander(_)
            | Handle::TabViewItem(_)
            | Handle::NavigationView(_)
            | Handle::PivotItem(_) => {}
            Handle::Pivot(p) => {
                let items = p
                    .cast::<Xaml::IItemsControl>()
                    .unwrap()
                    .get_Items()
                    .unwrap()
                    .cast::<windows_collections::IVector<windows_core::IInspectable>>()
                    .unwrap();
                let item = items.GetAt(v_from as u32).unwrap();
                items.RemoveAt(v_from as u32).unwrap();
                items.InsertAt(v_to as u32, &item).unwrap();
            }
            Handle::TabView(tv) => {
                let items = tv.get_TabItems().unwrap();
                let item = items.GetAt(v_from as u32).unwrap();
                items.RemoveAt(v_from as u32).unwrap();
                items.InsertAt(v_to as u32, &item).unwrap();
            }
            _ => panic!("WinUIBackend::move_child: {parent} is not a container"),
        }
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
        match parent_h {
            Handle::StackPanel(_) | Handle::Grid(_) | Handle::Canvas(_) => {
                let vec = panel_children_vec(parent_h).unwrap();
                vec.InsertAt(v_index as u32, &child_ui).unwrap();
            }
            Handle::Border(b) => {
                b.put_Child(&child_ui).unwrap();
            }
            Handle::Viewbox(v) => {
                v.put_Child(&child_ui).unwrap();
            }
            Handle::ScrollViewer(_)
            | Handle::Expander(_)
            | Handle::TabViewItem(_)
            | Handle::NavigationView(_)
            | Handle::PivotItem(_) => {
                let cc = content_control_for(parent_h).unwrap();
                cc.put_Content(&child_ui).unwrap();
            }
            Handle::TabView(tv) => {
                let items = tv.get_TabItems().unwrap();
                let insp: windows_core::IInspectable = child_ui.cast().unwrap();
                items.InsertAt(v_index as u32, &insp).unwrap();
            }
            Handle::Pivot(p) => {
                let items = p
                    .cast::<Xaml::IItemsControl>()
                    .unwrap()
                    .get_Items()
                    .unwrap()
                    .cast::<windows_collections::IVector<windows_core::IInspectable>>()
                    .unwrap();
                let insp: windows_core::IInspectable = child_ui.cast().unwrap();
                items.InsertAt(v_index as u32, &insp).unwrap();
            }
            _ => panic!("WinUIBackend::insert_child: {parent} is not a container"),
        }
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

    fn set_templated_selection_mode(
        &mut self,
        id: ControlId,
        mode: crate::core::templated_list::SelectionMode,
    ) {
        let map = self.controls.borrow();
        let Some(handle) = map.get(&id) else { return };
        let lvb: Xaml::IListViewBase = match handle {
            Handle::ListView(lv) => lv.cast().unwrap(),
            Handle::GridView(gv) => gv.cast().unwrap(),
            // FlipView doesn't support SelectionMode.
            _ => return,
        };
        use crate::core::templated_list::SelectionMode;
        let winui_mode = match mode {
            SelectionMode::None => Xaml::ListViewSelectionMode::None,
            SelectionMode::Single => Xaml::ListViewSelectionMode::Single,
            SelectionMode::Multiple => Xaml::ListViewSelectionMode::Multiple,
            SelectionMode::Extended => Xaml::ListViewSelectionMode::Extended,
        };
        let _ = lvb.put_SelectionMode(winui_mode);
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
    fn attach_templated_selection_changed(
        &mut self,
        id: ControlId,
        handler: crate::core::callback::Callback<i32>,
    ) {
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
        _realize: std::rc::Rc<dyn Fn(usize)>,
        _recycle: std::rc::Rc<dyn Fn(usize)>,
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
        let mut revokers: Vec<windows_core::EventRevoker> = Vec::new();

        let element_revokers = match handle {
            Handle::Button(b) => elements::button::attach_event(b, event, handler.clone()),
            Handle::CheckBox(c) => elements::check_box::attach_event(c, event, handler.clone()),
            Handle::TextBox(tb) => elements::text_box::attach_event(tb, event, handler.clone()),
            Handle::ToggleSwitch(ts) => {
                elements::toggle_switch::attach_event(ts, event, handler.clone())
            }
            Handle::Slider(s) => elements::slider::attach_event(s, event, handler.clone()),
            Handle::RadioButton(r) => {
                elements::radio_button::attach_event(r, event, handler.clone())
            }
            Handle::NumberBox(n) => elements::number_box::attach_event(n, event, handler.clone()),
            Handle::Expander(e) => elements::expander::attach_event(e, event, handler.clone()),
            Handle::HyperlinkButton(h) => {
                elements::hyperlink_button::attach_event(h, event, handler.clone())
            }
            Handle::InfoBar(ib) => elements::info_bar::attach_event(ib, event, handler.clone()),
            Handle::TabView(tv) => elements::tab_view::attach_event(tv, event, handler.clone()),
            Handle::NavigationView(nv) => {
                elements::navigation_view::attach_event(nv, event, handler.clone())
            }
            Handle::TitleBar(tb) => elements::title_bar::attach_event(tb, event, handler.clone()),
            Handle::Pivot(p) => elements::pivot::attach_event(p, event, handler.clone()),
            Handle::BreadcrumbBar(bc) => {
                elements::breadcrumb_bar::attach_event(bc, event, handler.clone())
            }
            Handle::PasswordBox(p) => {
                elements::password_box::attach_event(p, event, handler.clone())
            }
            Handle::RadioButtons(r) => {
                elements::radio_buttons::attach_event(r, event, handler.clone())
            }
            Handle::ComboBox(c) => elements::combo_box::attach_event(c, event, handler.clone()),
            Handle::ContentDialog(d) => {
                elements::content_dialog::attach_event(d, event, handler.clone())
            }
            Handle::RepeatButton(b) => {
                elements::repeat_button::attach_event(b, event, handler.clone())
            }
            Handle::RatingControl(r) => {
                elements::rating_control::attach_event(r, event, handler.clone())
            }
            Handle::ColorPicker(cp) => {
                elements::color_picker::attach_event(cp, event, handler.clone())
            }
            Handle::DatePicker(dp) => {
                elements::date_picker::attach_event(dp, event, handler.clone())
            }
            Handle::TimePicker(tp) => {
                elements::time_picker::attach_event(tp, event, handler.clone())
            }
            Handle::CalendarDatePicker(cdp) => {
                elements::calendar_date_picker::attach_event(cdp, event, handler.clone())
            }
            Handle::CalendarView(cv) => {
                elements::calendar_view::attach_event(cv, event, handler.clone())
            }
            Handle::ListBox(lb) => elements::list_box::attach_event(lb, event, handler.clone()),
            Handle::DropDownButton(ddb) => {
                elements::drop_down_button::attach_event(ddb, event, handler.clone())
            }
            Handle::SplitButton(sb) => {
                elements::split_button::attach_event(sb, event, handler.clone())
            }
            Handle::AutoSuggestBox(asb) => {
                elements::auto_suggest_box::attach_event(asb, event, handler.clone())
            }
            Handle::SplitView(sv) => elements::split_view::attach_event(sv, event, handler.clone()),
            Handle::TreeView(tv) => elements::tree_view::attach_event(tv, event, handler.clone()),
            Handle::TeachingTip(tt) => {
                elements::teaching_tip::attach_event(tt, event, handler.clone())
            }
            Handle::SelectorBar(sb) => {
                elements::selector_bar::attach_event(sb, event, handler.clone())
            }
            Handle::RichEditBox(reb) => {
                elements::rich_edit_box::attach_event(reb, event, handler.clone())
            }
            Handle::ToggleButton(tb) => {
                elements::toggle_button::attach_event(tb, event, handler.clone())
            }
            _ => None,
        };

        if let Some(r) = element_revokers {
            revokers = r;
        } else {
            match (event, handle) {
                (Event::MenuBarItemClicked, Handle::MenuBar(mb)) => {
                    self.menu_click_handlers
                        .borrow_mut()
                        .insert(id, handler.clone());
                    let revs = Self::wire_menu_bar_clicks(mb, &handler);
                    revokers.extend(revs);
                }
                (Event::MenuFlyoutItemClicked, Handle::DropDownButton(_) | Handle::Button(_)) => {
                    self.menu_click_handlers.borrow_mut().insert(id, handler);
                }
                (Event::CommandBarFlyoutClick, Handle::Button(_)) => {
                    self.command_bar_flyout_handlers
                        .borrow_mut()
                        .insert(id, handler);
                }
                (Event::CommandBarClick, Handle::CommandBar(cb)) => {
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
                (Event::FlyoutOpened, _) | (Event::FlyoutClosed, _) => {}
                (ev, h) => panic!(
                    "WinUIBackend::attach_event: {ev:?} on {} {id}",
                    describe_kind(h)
                ),
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
        bindings: &[(Prop, crate::core::theme::ThemeRef)],
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
        transitions: Option<crate::core::animation::ImplicitTransitions>,
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
    fn set_layout_animation(
        &mut self,
        _id: ControlId,
        _config: Option<crate::core::animation::LayoutAnimationConfig>,
    ) {
        // Layout-driven size/offset animations are not yet wired up
        // (requires SetIsTranslationEnabled + Translation channel).
    }
    fn run_property_animation(
        &mut self,
        id: ControlId,
        config: Option<crate::core::animation::AnimationConfig>,
    ) {
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
    fn set_rich_text_paragraphs(
        &mut self,
        id: ControlId,
        paragraphs: &[crate::core::rich_text::RichTextParagraph],
    ) {
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
                    crate::core::rich_text::RichTextInline::Run(r) => {
                        let Ok(run) = Xaml::Run::new() else { continue };
                        let _ = run.put_Text(&r.text);
                        if r.is_bold {
                            let _ = run
                                .cast::<Xaml::ITextElement>()
                                .and_then(|te| te.put_FontWeight(WinFontWeight { Weight: 700 }));
                        }
                        let _ = run.cast::<Xaml::Inline>().and_then(|i| inlines.Append(&i));
                    }
                    crate::core::rich_text::RichTextInline::LineBreak => {
                        // LineBreak inline — use a Run with newline.
                        let Ok(run) = Xaml::Run::new() else { continue };
                        let _ = run.put_Text("\n");
                        let _ = run.cast::<Xaml::Inline>().and_then(|i| inlines.Append(&i));
                    }
                    crate::core::rich_text::RichTextInline::Hyperlink(h) => {
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

    fn set_tooltip(&mut self, id: ControlId, tooltip: Option<&crate::core::tooltip::Tooltip>) {
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
                crate::core::tooltip::TooltipContent::Text(s) => {
                    let reference = windows_reference::IReference::from(s.as_str());
                    Some(reference.into())
                }
                crate::core::tooltip::TooltipContent::Rich(elem) => {
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

    fn set_pointer_handlers(
        &mut self,
        id: ControlId,
        handlers: Option<&crate::core::pointer::PointerHandlers>,
    ) {
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
) -> crate::core::pointer::PointerEventInfo {
    let mut info = crate::core::pointer::PointerEventInfo::default();
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

fn map_placement(p: crate::core::tooltip::TooltipPlacement) -> Xaml::PlacementMode {
    use crate::core::tooltip::TooltipPlacement;
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
            tb.put_Text(t.content.as_str()).ok()?;
            tb.cast::<Xaml::UIElement>().ok()
        }
        Element::StackPanel(s) => {
            let sp = Xaml::StackPanel::new().ok()?;
            let orientation = if s.vertical {
                Xaml::Orientation::Vertical
            } else {
                Xaml::Orientation::Horizontal
            };
            sp.put_Orientation(orientation).ok()?;
            if let Some(sp_val) = s.spacing {
                sp.put_Spacing(sp_val).ok()?;
            }
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
            if !img.source.is_empty()
                && let Ok(uri) = Xaml::Uri::CreateUri(img.source.as_str())
                && let Ok(bmp) = Xaml::BitmapImage::new()
            {
                if let Ok(ibmp) = bmp.cast::<Xaml::IBitmapImage>() {
                    let _ = ibmp.put_UriSource(&uri);
                }
                if let Ok(src) = bmp.cast::<Xaml::ImageSource>() {
                    let _ = i.put_Source(&src);
                }
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
