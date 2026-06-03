use std::cell::RefCell;

use rustc_hash::FxHashMap;
use windows_core::Interface;

use super::*;
use crate::bindings as Xaml;
use Xaml::FontWeight as WinFontWeight;

mod convert;
mod diag;
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
            match (prop, &value, handle) {
                (Prop::Text, PropValue::Str(s), Handle::TextBlock(tb)) => tb.put_Text(s.as_str()),
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
                (Prop::IsTextSelectionEnabled, PropValue::Bool(v), Handle::RichTextBlock(tb)) => {
                    tb.put_IsTextSelectionEnabled(*v)
                }
                (Prop::IsTextSelectionEnabled, PropValue::Bool(v), Handle::TextBlock(tb)) => {
                    tb.put_IsTextSelectionEnabled(*v)
                }
                (Prop::TextWrappingWrap, PropValue::Bool(v), Handle::TextBlock(tb)) => {
                    let mode = if *v {
                        Xaml::TextWrapping::Wrap
                    } else {
                        Xaml::TextWrapping::NoWrap
                    };
                    tb.put_TextWrapping(mode)
                }
                (Prop::TextWrappingWrap, PropValue::Bool(v), Handle::RichTextBlock(tb)) => {
                    let mode = if *v {
                        Xaml::TextWrapping::Wrap
                    } else {
                        Xaml::TextWrapping::NoWrap
                    };
                    tb.put_TextWrapping(mode)
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
                (Prop::IsEnabled, PropValue::Bool(v), Handle::Button(b)) => {
                    b.cast::<Xaml::IControl>()?.put_IsEnabled(*v)
                }
                (Prop::IsEnabled, PropValue::Bool(v), Handle::CheckBox(c)) => {
                    c.cast::<Xaml::IControl>()?.put_IsEnabled(*v)
                }
                (Prop::IsEnabled, PropValue::Bool(v), Handle::TextBox(t)) => {
                    t.cast::<Xaml::IControl>()?.put_IsEnabled(*v)
                }
                (Prop::IsEnabled, PropValue::Unset, _) => handle
                    .as_ui_element()
                    .cast::<Xaml::IControl>()?
                    .put_IsEnabled(true),
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
                (Prop::GridRowSpacing, PropValue::F64(v), Handle::Grid(g)) => g.put_RowSpacing(*v),
                (Prop::GridRowSpacing, PropValue::Unset, Handle::Grid(g)) => g.put_RowSpacing(0.0),
                (Prop::GridColumnSpacing, PropValue::F64(v), Handle::Grid(g)) => {
                    g.put_ColumnSpacing(*v)
                }
                (Prop::GridColumnSpacing, PropValue::Unset, Handle::Grid(g)) => {
                    g.put_ColumnSpacing(0.0)
                }
                (Prop::AttachedGridRow, PropValue::I32(v), _) => {
                    Xaml::Grid::SetRow(&handle.as_framework_element(), *v)
                }
                (Prop::AttachedGridColumn, PropValue::I32(v), _) => {
                    Xaml::Grid::SetColumn(&handle.as_framework_element(), *v)
                }
                (Prop::AttachedGridRowSpan, PropValue::I32(v), _) => {
                    Xaml::Grid::SetRowSpan(&handle.as_framework_element(), *v)
                }
                (Prop::AttachedGridColumnSpan, PropValue::I32(v), _) => {
                    Xaml::Grid::SetColumnSpan(&handle.as_framework_element(), *v)
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
                (Prop::Spacing, PropValue::F64(v), Handle::StackPanel(s)) => s.put_Spacing(*v),
                (Prop::Spacing, PropValue::Unset, Handle::StackPanel(s)) => s.put_Spacing(0.0),
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
                (Prop::Padding, PropValue::Thickness(t), _) => {
                    if let Ok(ctl) = handle.as_framework_element().cast::<Xaml::Control>() {
                        ctl.cast::<Xaml::IControl>()?
                            .put_Padding(to_xaml_thickness(*t))
                    } else {
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
                (Prop::Background, PropValue::Brush(br), Handle::StackPanel(s)) => {
                    s.cast::<Xaml::IPanel>()?.put_Background(&brush_of(br)?)
                }
                (Prop::Background, PropValue::Brush(br), Handle::Grid(g)) => {
                    g.cast::<Xaml::IPanel>()?.put_Background(&brush_of(br)?)
                }
                (Prop::Background, PropValue::Brush(br), Handle::Canvas(c)) => {
                    c.cast::<Xaml::IPanel>()?.put_Background(&brush_of(br)?)
                }
                (Prop::Background, PropValue::Brush(br), Handle::Border(b)) => {
                    b.put_Background(&brush_of(br)?)
                }
                (Prop::Background, PropValue::Brush(br), Handle::Button(b)) => {
                    b.cast::<Xaml::IControl>()?.put_Background(&brush_of(br)?)
                }
                (
                    Prop::Background,
                    PropValue::Brush(_),
                    Handle::TextBlock(_) | Handle::RichTextBlock(_),
                ) => Ok(()),
                (Prop::Background, PropValue::Unset, Handle::StackPanel(s)) => {
                    s.cast::<Xaml::IPanel>()?.put_Background(None)
                }
                (Prop::Background, PropValue::Unset, Handle::Grid(g)) => {
                    g.cast::<Xaml::IPanel>()?.put_Background(None)
                }
                (Prop::Background, PropValue::Unset, Handle::Canvas(c)) => {
                    c.cast::<Xaml::IPanel>()?.put_Background(None)
                }
                (Prop::Background, PropValue::Unset, Handle::Border(b)) => b.put_Background(None),
                (Prop::Background, PropValue::Unset, Handle::Button(b)) => {
                    b.cast::<Xaml::IControl>()?.put_Background(None)
                }
                (Prop::Foreground, PropValue::Brush(br), Handle::TextBlock(tb)) => {
                    tb.put_Foreground(&brush_of(br)?)
                }
                (Prop::Foreground, PropValue::Brush(br), Handle::RichTextBlock(tb)) => {
                    tb.put_Foreground(&brush_of(br)?)
                }
                (Prop::Foreground, PropValue::Brush(br), Handle::Button(b)) => {
                    b.cast::<Xaml::IControl>()?.put_Foreground(&brush_of(br)?)
                }
                (Prop::Foreground, PropValue::Brush(_), h) => {
                    diag::unhandled_modifier("set_prop", Prop::Foreground, h);
                    Ok(())
                }
                (Prop::Foreground, PropValue::Unset, Handle::TextBlock(tb)) => {
                    tb.put_Foreground(None)
                }
                (Prop::Foreground, PropValue::Unset, Handle::RichTextBlock(tb)) => {
                    tb.put_Foreground(None)
                }
                (Prop::Foreground, PropValue::Unset, Handle::Button(b)) => {
                    b.cast::<Xaml::IControl>()?.put_Foreground(None)
                }
                (Prop::IsOn, PropValue::Bool(v), Handle::ToggleSwitch(ts)) => ts.put_IsOn(*v),
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
                (Prop::Minimum, PropValue::F64(v), Handle::Slider(s)) => {
                    s.cast::<Xaml::IRangeBase>()?.put_Minimum(*v)
                }
                (Prop::Maximum, PropValue::F64(v), Handle::Slider(s)) => {
                    s.cast::<Xaml::IRangeBase>()?.put_Maximum(*v)
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
                (Prop::NumericValue, PropValue::F64(v), Handle::NumberBox(n)) => n.put_Value(*v),
                (Prop::Minimum, PropValue::F64(v), Handle::NumberBox(n)) => n.put_Minimum(*v),
                (Prop::Maximum, PropValue::F64(v), Handle::NumberBox(n)) => n.put_Maximum(*v),
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
                (Prop::Minimum, PropValue::F64(v), Handle::ProgressBar(p)) => {
                    p.cast::<Xaml::IRangeBase>()?.put_Minimum(*v)
                }
                (Prop::Maximum, PropValue::F64(v), Handle::ProgressBar(p)) => {
                    p.cast::<Xaml::IRangeBase>()?.put_Maximum(*v)
                }
                (Prop::IsIndeterminate, PropValue::Bool(v), Handle::ProgressBar(p)) => {
                    p.put_IsIndeterminate(*v)
                }
                (Prop::NumericValue, PropValue::F64(v), Handle::ProgressRing(p)) => {
                    p.cast::<Xaml::IRangeBase>()?.put_Value(*v)
                }
                (Prop::Minimum, PropValue::F64(v), Handle::ProgressRing(p)) => p.put_Minimum(*v),
                (Prop::Maximum, PropValue::F64(v), Handle::ProgressRing(p)) => p.put_Maximum(*v),
                (Prop::IsIndeterminate, PropValue::Bool(v), Handle::ProgressRing(p)) => {
                    p.put_IsIndeterminate(*v)
                }
                (Prop::IsActive, PropValue::Bool(v), Handle::ProgressRing(p)) => p.put_IsActive(*v),
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
                (Prop::ButtonContent, PropValue::Str(s), Handle::HyperlinkButton(h)) => {
                    let tb = string_as_textblock(s)?;
                    h.cast::<Xaml::IContentControl>()?.put_Content(&tb)
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
                (Prop::InfoBarIsOpen, PropValue::Bool(v), Handle::InfoBar(ib)) => ib.put_IsOpen(*v),
                (Prop::IsClosable, PropValue::Bool(v), Handle::InfoBar(ib)) => {
                    ib.put_IsClosable(*v)
                }
                (Prop::IsClosable, PropValue::Bool(v), Handle::TabViewItem(ti)) => {
                    ti.put_IsClosable(*v)
                }
                // ContentDialog (W6 — modal popup hosted via ShowAsync).
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
                (Prop::InfoBadgeValue, PropValue::I32(v), Handle::InfoBadge(ib)) => {
                    if *v < 0 {
                        ib.put_Value(-1)
                    } else {
                        ib.put_Value(*v)
                    }
                }
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
                (Prop::Fill, PropValue::Brush(b), Handle::Ellipse(e)) => {
                    e.cast::<Xaml::IShape>()?.put_Fill(&brush_of(b)?)
                }
                (Prop::Stroke, PropValue::Brush(b), Handle::Rectangle(r)) => {
                    r.cast::<Xaml::IShape>()?.put_Stroke(&brush_of(b)?)
                }
                (Prop::Stroke, PropValue::Brush(b), Handle::Ellipse(e)) => {
                    e.cast::<Xaml::IShape>()?.put_Stroke(&brush_of(b)?)
                }
                (Prop::Stroke, PropValue::Brush(b), Handle::Line(l)) => {
                    l.cast::<Xaml::IShape>()?.put_Stroke(&brush_of(b)?)
                }
                (Prop::StrokeThickness, PropValue::F64(v), Handle::Rectangle(r)) => {
                    r.cast::<Xaml::IShape>()?.put_StrokeThickness(*v)
                }
                (Prop::StrokeThickness, PropValue::F64(v), Handle::Ellipse(e)) => {
                    e.cast::<Xaml::IShape>()?.put_StrokeThickness(*v)
                }
                (Prop::StrokeThickness, PropValue::F64(v), Handle::Line(l)) => {
                    l.cast::<Xaml::IShape>()?.put_StrokeThickness(*v)
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
                (Prop::SelectedIndex, PropValue::I32(v), Handle::TabView(tv)) => {
                    tv.put_SelectedIndex(*v)
                }
                (Prop::CanReorderTabs, PropValue::Bool(v), Handle::TabView(tv)) => {
                    tv.put_CanReorderTabs(*v)
                }
                (Prop::IsAddTabButtonVisible, PropValue::Bool(v), Handle::TabView(tv)) => {
                    tv.put_IsAddTabButtonVisible(*v)
                }
                (Prop::TabHeader, PropValue::Str(s), Handle::TabViewItem(ti)) => {
                    let tb = string_as_textblock(s)?;
                    ti.put_Header(&tb)
                }
                (Prop::TabItemKey, PropValue::Str(s), Handle::TabViewItem(ti)) => {
                    let tag = windows_reference::IReference::from(s.as_str());
                    ti.cast::<Xaml::IFrameworkElement>()?.put_Tag(&tag)
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
                (Prop::IsSettingsVisible, PropValue::Bool(v), Handle::NavigationView(nv)) => {
                    nv.put_IsSettingsVisible(*v)
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
                (Prop::IsBackButtonVisible, PropValue::Bool(v), Handle::TitleBar(tb)) => {
                    tb.put_IsBackButtonVisible(*v)
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
                (Prop::IsBackEnabled, PropValue::Bool(v), Handle::TitleBar(tb)) => {
                    tb.put_IsBackButtonEnabled(*v)
                }
                (Prop::IsPaneToggleButtonVisible, PropValue::Bool(v), Handle::TitleBar(tb)) => {
                    tb.put_IsPaneToggleButtonVisible(*v)
                }
                (
                    Prop::IsPaneToggleButtonVisible,
                    PropValue::Bool(v),
                    Handle::NavigationView(nv),
                ) => nv.put_IsPaneToggleButtonVisible(*v),
                (Prop::SelectedIndex, PropValue::I32(v), Handle::Pivot(p)) => {
                    p.put_SelectedIndex(*v)
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
                (Prop::SelectedIndex, PropValue::I32(v), Handle::RadioButtons(r)) => {
                    r.put_SelectedIndex(*v)
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
                (Prop::SelectedIndex, PropValue::I32(v), Handle::ComboBox(c)) => {
                    c.cast::<Xaml::ISelector>()?.put_SelectedIndex(*v)
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
                // ── W5: Canvas attached props ─────────────────────────────────
                (Prop::AttachedCanvasLeft, PropValue::F64(v), _) => {
                    Xaml::Canvas::SetLeft(&handle.as_ui_element(), *v)
                }
                (Prop::AttachedCanvasTop, PropValue::F64(v), _) => {
                    Xaml::Canvas::SetTop(&handle.as_ui_element(), *v)
                }
                (Prop::AttachedCanvasZIndex, PropValue::I32(v), _) => {
                    Xaml::Canvas::SetZIndex(&handle.as_ui_element(), *v)
                }
                // ── W6: RepeatButton ──────────────────────────────────────────
                (Prop::ButtonContent, PropValue::Str(s), Handle::RepeatButton(b)) => {
                    let tb = string_as_textblock(s)?;
                    b.cast::<Xaml::IContentControl>()?.put_Content(&tb)
                }
                (Prop::IsEnabled, PropValue::Bool(v), Handle::RepeatButton(b)) => {
                    b.cast::<Xaml::IControl>()?.put_IsEnabled(*v)
                }
                (Prop::RepeatDelay, PropValue::I32(v), Handle::RepeatButton(b)) => b.put_Delay(*v),
                (Prop::RepeatInterval, PropValue::I32(v), Handle::RepeatButton(b)) => {
                    b.put_Interval(*v)
                }
                // ── W7: RatingControl ─────────────────────────────────────────
                (Prop::NumericValue, PropValue::F64(v), Handle::RatingControl(r)) => {
                    r.put_Value(*v)
                }
                (Prop::MaxRating, PropValue::I32(v), Handle::RatingControl(r)) => {
                    r.put_MaxRating(*v)
                }
                (Prop::RatingCaption, PropValue::Str(s), Handle::RatingControl(r)) => {
                    r.put_Caption(s.as_str())
                }
                (Prop::RatingCaption, PropValue::Unset, Handle::RatingControl(r)) => {
                    r.put_Caption("")
                }
                (Prop::PlaceholderValue, PropValue::F64(v), Handle::RatingControl(r)) => {
                    r.put_PlaceholderValue(*v)
                }
                (Prop::IsReadOnly, PropValue::Bool(v), Handle::RatingControl(r)) => {
                    r.put_IsReadOnly(*v)
                }
                // ── W8: ColorPicker ───────────────────────────────────────────
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
                (Prop::IsHexInputVisible, PropValue::Bool(v), Handle::ColorPicker(cp)) => {
                    cp.put_IsHexInputVisible(*v)
                }
                (Prop::IsColorSliderVisible, PropValue::Bool(v), Handle::ColorPicker(cp)) => {
                    cp.put_IsColorSliderVisible(*v)
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
                (Prop::MonthVisible, PropValue::Bool(v), Handle::DatePicker(dp)) => {
                    dp.put_MonthVisible(*v)
                }
                (Prop::YearVisible, PropValue::Bool(v), Handle::DatePicker(dp)) => {
                    dp.put_YearVisible(*v)
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
                (Prop::MinuteIncrement, PropValue::I32(v), Handle::TimePicker(tp)) => {
                    tp.put_MinuteIncrement(*v)
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
                (Prop::IsCalendarOpen, PropValue::Bool(v), Handle::CalendarDatePicker(cdp)) => {
                    cdp.put_IsCalendarOpen(*v)
                }
                (Prop::IsEnabled, PropValue::Bool(v), Handle::CalendarDatePicker(cdp)) => {
                    cdp.cast::<Xaml::IControl>()?.put_IsEnabled(*v)
                }
                // ── W12: CalendarView ─────────────────────────────────────────
                (Prop::IsTodayHighlighted, PropValue::Bool(v), Handle::CalendarView(cv)) => {
                    cv.put_IsTodayHighlighted(*v)
                }
                (Prop::IsGroupLabelVisible, PropValue::Bool(v), Handle::CalendarView(cv)) => {
                    cv.put_IsGroupLabelVisible(*v)
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
                (Prop::SelectedIndex, PropValue::I32(v), Handle::ListBox(lb)) => {
                    lb.cast::<Xaml::ISelector>()?.put_SelectedIndex(*v)
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
                (Prop::Header, PropValue::Str(s), Handle::AutoSuggestBox(asb)) => {
                    let insp = windows_reference::IReference::from(s.as_str());
                    asb.put_Header(&insp)
                }
                (Prop::IsEnabled, PropValue::Bool(v), Handle::AutoSuggestBox(asb)) => {
                    asb.cast::<Xaml::IControl>()?.put_IsEnabled(*v)
                }
                // ── W17: SplitView ───────────────────────────────────────
                (Prop::SplitViewDisplayMode, PropValue::I32(m), Handle::SplitView(sv)) => {
                    sv.put_DisplayMode(Xaml::SplitViewDisplayMode(*m))
                }
                (Prop::SplitViewIsPaneOpen, PropValue::Bool(v), Handle::SplitView(sv)) => {
                    sv.put_IsPaneOpen(*v)
                }
                (Prop::SplitViewOpenPaneLength, PropValue::F64(v), Handle::SplitView(sv)) => {
                    sv.put_OpenPaneLength(*v)
                }
                (Prop::SplitViewCompactPaneLength, PropValue::F64(v), Handle::SplitView(sv)) => {
                    sv.put_CompactPaneLength(*v)
                }
                // ── W18: MenuBar ─────────────────────────────────────────
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
                // MenuFlyout on Button/DropDownButton
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
                // CommandBarFlyout on Button
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
                (
                    Prop::CommandBarDefaultLabelPosition,
                    PropValue::CommandBarLabelPosition(pos),
                    Handle::CommandBar(cb),
                ) => {
                    use CommandBarLabelPos as E;
                    use Xaml::CommandBarDefaultLabelPosition as W;
                    let mapped = match pos {
                        E::Bottom => W::Bottom,
                        E::Right => W::Right,
                        E::Collapsed => W::Collapsed,
                    };
                    cb.put_DefaultLabelPosition(mapped)
                }
                // ── W22: TeachingTip ────────────────────────────────────
                (Prop::TeachingTipTitle, PropValue::Str(s), Handle::TeachingTip(tt)) => {
                    tt.put_Title(s.as_str())
                }
                (Prop::TeachingTipSubtitle, PropValue::Str(s), Handle::TeachingTip(tt)) => {
                    tt.put_Subtitle(s.as_str())
                }
                (Prop::TeachingTipIsOpen, PropValue::Bool(v), Handle::TeachingTip(tt)) => {
                    tt.put_IsOpen(*v)
                }
                (Prop::TeachingTipIsLightDismiss, PropValue::Bool(v), Handle::TeachingTip(tt)) => {
                    tt.put_IsLightDismissEnabled(*v)
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
                (Prop::TeachingTipCloseButton, PropValue::Str(s), Handle::TeachingTip(tt)) => {
                    let boxed: windows_core::IInspectable =
                        windows_reference::IReference::<windows_core::HSTRING>::from(
                            windows_core::HSTRING::from(s.as_str()),
                        )
                        .cast()?;
                    tt.put_CloseButtonContent(&boxed)
                }
                // ── W23: SelectorBar ────────────────────────────────────
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
                (Prop::RichEditBoxText, PropValue::Str(s), Handle::RichEditBox(reb)) => {
                    let doc = reb.get_Document()?;
                    let mut current = windows_core::HSTRING::default();
                    doc.GetText(Xaml::TextGetOptions::None, &mut current).ok();
                    if current == s.as_str() {
                        return Ok(());
                    }
                    doc.SetText(Xaml::TextSetOptions::None, s.as_str())
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
                (Prop::FlyoutContent, PropValue::Str(s), Handle::Button(b)) => {
                    let flyout = Xaml::Flyout::new()?;
                    let tb = string_as_textblock(s)?;
                    flyout.put_Content(&tb)?;
                    b.put_Flyout(&flyout)?;
                    Ok(())
                }
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
                // ── RelativePanel attached props ──────────────────────────────
                (Prop::RelativePanelAlignLeftWithPanel, PropValue::Bool(v), _) => {
                    Xaml::RelativePanel::SetAlignLeftWithPanel(&handle.as_ui_element(), *v)
                }
                (Prop::RelativePanelAlignRightWithPanel, PropValue::Bool(v), _) => {
                    Xaml::RelativePanel::SetAlignRightWithPanel(&handle.as_ui_element(), *v)
                }
                (Prop::RelativePanelAlignTopWithPanel, PropValue::Bool(v), _) => {
                    Xaml::RelativePanel::SetAlignTopWithPanel(&handle.as_ui_element(), *v)
                }
                (Prop::RelativePanelAlignBottomWithPanel, PropValue::Bool(v), _) => {
                    Xaml::RelativePanel::SetAlignBottomWithPanel(&handle.as_ui_element(), *v)
                }
                (Prop::RelativePanelAlignHCenterWithPanel, PropValue::Bool(v), _) => {
                    Xaml::RelativePanel::SetAlignHorizontalCenterWithPanel(
                        &handle.as_ui_element(),
                        *v,
                    )
                }
                (Prop::RelativePanelAlignVCenterWithPanel, PropValue::Bool(v), _) => {
                    Xaml::RelativePanel::SetAlignVerticalCenterWithPanel(
                        &handle.as_ui_element(),
                        *v,
                    )
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
        match (event, handle) {
            (Event::Click, Handle::Button(b)) => {
                revokers.push(
                    b.cast::<Xaml::IButtonBase>()
                        .unwrap()
                        .add_Click(move |_sender, _args| {
                            handler.invoke();
                        })
                        .unwrap(),
                );
            }
            (Event::Click, Handle::HyperlinkButton(h)) => {
                revokers.push(
                    h.cast::<Xaml::IButtonBase>()
                        .unwrap()
                        .add_Click(move |_sender, _args| {
                            handler.invoke();
                        })
                        .unwrap(),
                );
            }
            (Event::Click, Handle::RepeatButton(b)) => {
                revokers.push(
                    b.cast::<Xaml::IButtonBase>()
                        .unwrap()
                        .add_Click(move |_sender, _args| {
                            handler.invoke();
                        })
                        .unwrap(),
                );
            }
            (Event::Click, Handle::DropDownButton(ddb)) => {
                revokers.push(
                    ddb.cast::<Xaml::IButtonBase>()
                        .unwrap()
                        .add_Click(move |_sender, _args| {
                            handler.invoke();
                        })
                        .unwrap(),
                );
            }
            (Event::Click, _) => {
                panic!("WinUIBackend::attach_event: Click on non-Button control {id}")
            }
            (Event::CheckedChanged, Handle::CheckBox(c)) => {
                let checked_handler = handler.clone();
                revokers.push(
                    c.cast::<Xaml::IToggleButton>()
                        .unwrap()
                        .add_Checked(move |sender, _args| {
                            let is_checked = sender_is_checked(sender);
                            checked_handler.invoke_bool(is_checked);
                        })
                        .unwrap(),
                );
                revokers.push(
                    c.cast::<Xaml::IToggleButton>()
                        .unwrap()
                        .add_Unchecked(move |sender, _args| {
                            let is_checked = sender_is_checked(sender);
                            handler.invoke_bool(is_checked);
                        })
                        .unwrap(),
                );
            }
            (Event::CheckedChanged, Handle::ToggleButton(tb)) => {
                let checked_handler = handler.clone();
                revokers.push(
                    tb.add_Checked(move |sender, _args| {
                        let is_checked = sender_is_checked(sender);
                        checked_handler.invoke_bool(is_checked);
                    })
                    .unwrap(),
                );
                revokers.push(
                    tb.add_Unchecked(move |sender, _args| {
                        let is_checked = sender_is_checked(sender);
                        handler.invoke_bool(is_checked);
                    })
                    .unwrap(),
                );
            }
            (Event::CheckedChanged, _) => {
                panic!(
                    "WinUIBackend::attach_event: CheckedChanged on non-CheckBox/ToggleButton {id}"
                )
            }
            (Event::TextChanged, Handle::TextBox(tb)) => {
                revokers.push(
                    tb.add_TextChanged(move |sender, _args| {
                        let text = sender_text(sender);
                        handler.invoke_string(text);
                    })
                    .unwrap(),
                );
            }
            (Event::TextChanged, _) => {
                panic!("WinUIBackend::attach_event: TextChanged on non-TextBox {id}")
            }
            (Event::Toggled, Handle::ToggleSwitch(ts)) => {
                revokers.push(
                    ts.add_Toggled(move |sender, _args| {
                        let is_on = sender
                            .as_ref()
                            .and_then(|s| s.cast::<Xaml::ToggleSwitch>().ok())
                            .and_then(|ts| ts.get_IsOn().ok())
                            .unwrap_or(false);
                        handler.invoke_bool(is_on);
                    })
                    .unwrap(),
                );
            }
            (Event::Toggled, _) => {
                panic!("WinUIBackend::attach_event: Toggled on non-ToggleSwitch {id}")
            }
            (Event::ValueChanged, Handle::Slider(s)) => {
                revokers.push(
                    s.cast::<Xaml::IRangeBase>()
                        .unwrap()
                        .add_ValueChanged(move |_sender, args| {
                            if let Some(a) = args.as_ref()
                                && let Some(v) = a
                                    .cast::<Xaml::IRangeBaseValueChangedEventArgs>()
                                    .ok()
                                    .and_then(|args| args.get_NewValue().ok())
                            {
                                handler.invoke_f64(v);
                            }
                        })
                        .unwrap(),
                );
            }
            (Event::ValueChanged, Handle::NumberBox(n)) => {
                revokers.push(
                    n.add_ValueChanged(move |_sender, args| {
                        if let Some(a) = args.as_ref()
                            && let Ok(v) = a.get_NewValue()
                        {
                            handler.invoke_f64(v);
                        }
                    })
                    .unwrap(),
                );
            }
            (Event::ValueChanged, _) => {
                panic!("WinUIBackend::attach_event: ValueChanged on unsupported control {id}")
            }
            (Event::RadioChecked, Handle::RadioButton(r)) => {
                revokers.push(
                    r.cast::<Xaml::IToggleButton>()
                        .unwrap()
                        .add_Checked(move |_sender, _args| {
                            handler.invoke();
                        })
                        .unwrap(),
                );
            }
            (Event::RadioChecked, _) => {
                panic!("WinUIBackend::attach_event: RadioChecked on non-RadioButton {id}")
            }
            (Event::ExpandedChanged, Handle::Expander(e)) => {
                let expanding_handler = handler.clone();
                revokers.push(
                    e.add_Expanding(move |_sender, _args| {
                        expanding_handler.invoke_bool(true);
                    })
                    .unwrap(),
                );
                revokers.push(
                    e.add_Collapsed(move |_sender, _args| {
                        handler.invoke_bool(false);
                    })
                    .unwrap(),
                );
            }
            (Event::ExpandedChanged, _) => {
                panic!("WinUIBackend::attach_event: ExpandedChanged on non-Expander {id}")
            }
            (Event::InfoBarClosed, Handle::InfoBar(ib)) => {
                revokers.push(
                    ib.add_Closed(move |_sender, _args| {
                        handler.invoke();
                    })
                    .unwrap(),
                );
            }
            (Event::InfoBarClosed, _) => {
                panic!("WinUIBackend::attach_event: InfoBarClosed on non-InfoBar {id}")
            }
            (Event::ContentDialogClosed, Handle::ContentDialog(d)) => {
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
            (Event::ContentDialogClosed, _) => {
                panic!("WinUIBackend::attach_event: ContentDialogClosed on non-ContentDialog {id}")
            }
            (Event::TabSelectionChanged, Handle::TabView(tv)) => {
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
            (Event::TabSelectionChanged, _) => {
                panic!("WinUIBackend::attach_event: TabSelectionChanged on non-TabView {id}")
            }
            (Event::TabCloseRequested, Handle::TabView(tv)) => {
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
            (Event::TabCloseRequested, _) => {
                panic!("WinUIBackend::attach_event: TabCloseRequested on non-TabView {id}")
            }
            (Event::AddTabButtonClick, Handle::TabView(tv)) => {
                revokers.push(
                    tv.add_AddTabButtonClick(move |_sender, _args| {
                        handler.invoke();
                    })
                    .unwrap(),
                );
            }
            (Event::AddTabButtonClick, _) => {
                panic!("WinUIBackend::attach_event: AddTabButtonClick on non-TabView {id}")
            }
            (Event::NavSelectionChanged, Handle::NavigationView(nv)) => {
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
            (Event::NavSelectionChanged, _) => {
                panic!("WinUIBackend::attach_event: NavSelectionChanged on non-NavigationView {id}")
            }
            (Event::NavBackRequested, Handle::NavigationView(nv)) => {
                revokers.push(
                    nv.cast::<Xaml::INavigationView2>()
                        .unwrap()
                        .add_BackRequested(move |_sender, _args| {
                            handler.invoke();
                        })
                        .unwrap(),
                );
            }
            (Event::NavBackRequested, _) => {
                panic!("WinUIBackend::attach_event: NavBackRequested on non-NavigationView {id}")
            }
            (Event::NavSearchQuerySubmitted, Handle::NavigationView(nv)) => {
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
            (Event::NavSearchQuerySubmitted, _) => {
                panic!(
                    "WinUIBackend::attach_event: NavSearchQuerySubmitted on non-NavigationView {id}"
                )
            }
            (Event::NavSearchTextChanged, Handle::NavigationView(nv)) => {
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
            (Event::NavSearchTextChanged, _) => {
                panic!(
                    "WinUIBackend::attach_event: NavSearchTextChanged on non-NavigationView {id}"
                )
            }
            (Event::NavSearchSuggestionChosen, Handle::NavigationView(nv)) => {
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
            (Event::NavSearchSuggestionChosen, _) => {
                panic!(
                    "WinUIBackend::attach_event: NavSearchSuggestionChosen on non-NavigationView {id}"
                )
            }
            (Event::TitleBarBackRequested, Handle::TitleBar(tb)) => {
                revokers.push(
                    tb.add_BackRequested(move |_sender, _args| {
                        handler.invoke();
                    })
                    .unwrap(),
                );
            }
            (Event::TitleBarBackRequested, _) => {
                panic!("WinUIBackend::attach_event: TitleBarBackRequested on non-TitleBar {id}")
            }
            (Event::TitleBarPaneToggle, Handle::TitleBar(tb)) => {
                revokers.push(
                    tb.add_PaneToggleRequested(move |_sender, _args| {
                        handler.invoke();
                    })
                    .unwrap(),
                );
            }
            (Event::TitleBarPaneToggle, _) => {
                panic!("WinUIBackend::attach_event: TitleBarPaneToggle on non-TitleBar {id}")
            }
            (Event::PivotSelectionChanged, Handle::Pivot(p)) => {
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
            (Event::PivotSelectionChanged, _) => {
                panic!("WinUIBackend::attach_event: PivotSelectionChanged on non-Pivot {id}")
            }
            (Event::BreadcrumbItemClicked, Handle::BreadcrumbBar(bc)) => {
                revokers.push(
                    bc.add_ItemClicked(move |_sender, args| {
                        if let Some(idx) = args.as_ref().and_then(|a| a.get_Index().ok()) {
                            handler.invoke_i32(idx);
                        }
                    })
                    .unwrap(),
                );
            }
            (Event::BreadcrumbItemClicked, _) => {
                panic!(
                    "WinUIBackend::attach_event: BreadcrumbItemClicked on non-BreadcrumbBar {id}"
                )
            }
            (Event::PasswordChanged, Handle::PasswordBox(p)) => {
                revokers.push(
                    p.add_PasswordChanged(move |sender, _args| {
                        let text = sender
                            .as_ref()
                            .and_then(|s| s.cast::<Xaml::PasswordBox>().ok())
                            .and_then(|pb| pb.get_Password().ok())
                            .unwrap_or_default();
                        handler.invoke_string(text);
                    })
                    .unwrap(),
                );
            }
            (Event::PasswordChanged, _) => {
                panic!("WinUIBackend::attach_event: PasswordChanged on non-PasswordBox {id}")
            }
            (Event::RadioButtonsSelectionChanged, Handle::RadioButtons(r)) => {
                revokers.push(
                    r.add_SelectionChanged(move |sender, _args| {
                        let idx = sender
                            .as_ref()
                            .and_then(|s| s.cast::<Xaml::RadioButtons>().ok())
                            .and_then(|rb| rb.get_SelectedIndex().ok())
                            .unwrap_or(-1);
                        handler.invoke_i32(idx);
                    })
                    .unwrap(),
                );
            }
            (Event::RadioButtonsSelectionChanged, _) => {
                panic!(
                    "WinUIBackend::attach_event: RadioButtonsSelectionChanged on non-RadioButtons {id}"
                )
            }
            (Event::ComboSelectionChanged, Handle::ComboBox(c)) => {
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
            (Event::ComboSelectionChanged, _) => {
                panic!("WinUIBackend::attach_event: ComboSelectionChanged on non-ComboBox {id}")
            }
            // ── RatingControl ValueChanged ────────────────────────────────
            (Event::RatingValueChanged, Handle::RatingControl(r)) => {
                revokers.push(
                    r.add_ValueChanged(move |sender, _args| {
                        let v = sender
                            .as_ref()
                            .and_then(|s| s.cast::<Xaml::RatingControl>().ok())
                            .and_then(|rc| rc.get_Value().ok())
                            .unwrap_or(-1.0);
                        handler.invoke_f64(v);
                    })
                    .unwrap(),
                );
            }
            (Event::RatingValueChanged, _) => {
                panic!("WinUIBackend::attach_event: RatingValueChanged on non-RatingControl {id}")
            }
            // ── ColorPicker ColorChanged ──────────────────────────────────
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
            (Event::ColorChanged, _) => {
                panic!("WinUIBackend::attach_event: ColorChanged on non-ColorPicker {id}")
            }
            // ── W9: DatePicker ────────────────────────────────────────────
            (Event::DateSelected, Handle::DatePicker(dp)) => {
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
            (Event::DateSelected, _) => {
                panic!("WinUIBackend::attach_event: DateSelected on non-DatePicker {id}")
            }
            // ── W10: TimePicker ───────────────────────────────────────────
            (Event::TimeSelected, Handle::TimePicker(tp)) => {
                revokers.push(
                    tp.add_SelectedTimeChanged(move |_sender, args| {
                        if let Some(a) = args.as_ref()
                            && let Ok(ts) = a.get_NewTime()
                        {
                            handler
                                .invoke_timespan(windows_time::TimeSpan::from_ticks(ts.Duration));
                        }
                    })
                    .unwrap(),
                );
            }
            (Event::TimeSelected, _) => {
                panic!("WinUIBackend::attach_event: TimeSelected on non-TimePicker {id}")
            }
            // ── W11: CalendarDatePicker ───────────────────────────────────
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
            (Event::CalendarDateSelected, _) => {
                panic!(
                    "WinUIBackend::attach_event: CalendarDateSelected on non-CalendarDatePicker {id}"
                )
            }
            // ── W12: CalendarView ─────────────────────────────────────────
            (Event::CalendarViewSelectionChanged, Handle::CalendarView(cv)) => {
                revokers.push(
                    cv.add_SelectedDatesChanged(move |_sender, _args| {
                        handler.invoke();
                    })
                    .unwrap(),
                );
            }
            (Event::CalendarViewSelectionChanged, _) => {
                panic!(
                    "WinUIBackend::attach_event: CalendarViewSelectionChanged on non-CalendarView {id}"
                )
            }
            // ── W13: ListBox ──────────────────────────────────────────────
            (Event::ListBoxSelectionChanged, Handle::ListBox(lb)) => {
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
            (Event::ListBoxSelectionChanged, _) => {
                panic!("WinUIBackend::attach_event: ListBoxSelectionChanged on non-ListBox {id}")
            }
            // ── W15: SplitButton ──────────────────────────────────────────
            (Event::SplitButtonClick, Handle::SplitButton(sb)) => {
                revokers.push(
                    sb.add_Click(move |_sender, _args| {
                        handler.invoke();
                    })
                    .unwrap(),
                );
            }
            (Event::SplitButtonClick, _) => {
                panic!("WinUIBackend::attach_event: SplitButtonClick on non-SplitButton {id}")
            }
            // ── W16: AutoSuggestBox ───────────────────────────────────────
            (Event::AutoSuggestTextChanged, Handle::AutoSuggestBox(asb)) => {
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
            (Event::AutoSuggestTextChanged, _) => {
                panic!(
                    "WinUIBackend::attach_event: AutoSuggestTextChanged on non-AutoSuggestBox {id}"
                )
            }
            (Event::AutoSuggestQuerySubmitted, Handle::AutoSuggestBox(asb)) => {
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
            (Event::AutoSuggestQuerySubmitted, _) => {
                panic!(
                    "WinUIBackend::attach_event: AutoSuggestQuerySubmitted on non-AutoSuggestBox {id}"
                )
            }
            (Event::AutoSuggestSuggestionChosen, Handle::AutoSuggestBox(asb)) => {
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
            (Event::AutoSuggestSuggestionChosen, _) => {
                panic!(
                    "WinUIBackend::attach_event: AutoSuggestSuggestionChosen on non-AutoSuggestBox {id}"
                )
            }
            // ── W17: SplitView ───────────────────────────────────────
            (Event::SplitViewPaneClosed, Handle::SplitView(sv)) => {
                revokers.push(
                    sv.add_PaneClosed(move |_sender, _args| {
                        handler.invoke();
                    })
                    .unwrap(),
                );
            }
            (Event::SplitViewPaneClosed, _) => {
                panic!("WinUIBackend::attach_event: SplitViewPaneClosed on non-SplitView {id}")
            }
            // ── W18: MenuBar ─────────────────────────────────────────
            (Event::MenuBarItemClicked, Handle::MenuBar(mb)) => {
                // Store the handler so set_prop can re-wire on item rebuild.
                self.menu_click_handlers
                    .borrow_mut()
                    .insert(id, handler.clone());
                // Wire click on all existing items.
                let revs = Self::wire_menu_bar_clicks(mb, &handler);
                revokers.extend(revs);
            }
            (Event::MenuBarItemClicked, _) => {
                panic!("WinUIBackend::attach_event: MenuBarItemClicked on non-MenuBar {id}")
            }
            (Event::MenuFlyoutItemClicked, Handle::DropDownButton(_) | Handle::Button(_)) => {
                // Store the handler so set_prop can wire when flyout is built.
                self.menu_click_handlers.borrow_mut().insert(id, handler);
            }
            (Event::MenuFlyoutItemClicked, _) => {
                panic!(
                    "WinUIBackend::attach_event: MenuFlyoutItemClicked on non-Button/DropDownButton {id}"
                )
            }
            (Event::CommandBarFlyoutClick, Handle::Button(_)) => {
                // Store the handler so set_prop can wire when flyout is built.
                self.command_bar_flyout_handlers
                    .borrow_mut()
                    .insert(id, handler);
            }
            (Event::CommandBarFlyoutClick, _) => {
                panic!("WinUIBackend::attach_event: CommandBarFlyoutClick on non-Button {id}")
            }
            // ── W20: TreeView ─────────────────────────────────────────
            (Event::TreeViewItemInvoked, Handle::TreeView(tv)) => {
                revokers.push(
                    tv.add_ItemInvoked(move |_sender, args| {
                        let text = args
                            .as_ref()
                            .and_then(|a| a.get_InvokedItem().ok())
                            .and_then(|insp| {
                                insp.cast::<super::super::bindings::ITreeViewNode>()
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
            (Event::TreeViewItemInvoked, _) => {
                panic!("WinUIBackend::attach_event: TreeViewItemInvoked on non-TreeView {id}")
            }
            // ── W21: CommandBar ─────────────────────────────────────────
            (Event::CommandBarClick, Handle::CommandBar(cb)) => {
                // Store the handler so set_prop can re-wire on rebuild.
                self.menu_click_handlers
                    .borrow_mut()
                    .insert(id, handler.clone());
                // Wire click on all existing primary + secondary commands.
                if let Ok(primary) = cb.get_PrimaryCommands() {
                    let revs = Self::wire_command_bar_clicks(&primary, &handler);
                    revokers.extend(revs);
                }
                if let Ok(secondary) = cb.get_SecondaryCommands() {
                    let revs = Self::wire_command_bar_clicks(&secondary, &handler);
                    revokers.extend(revs);
                }
            }
            (Event::CommandBarClick, _) => {
                panic!("WinUIBackend::attach_event: CommandBarClick on non-CommandBar {id}")
            }
            // ── W22: TeachingTip ─────────────────────────────────────────
            (Event::TeachingTipClosed, Handle::TeachingTip(tt)) => {
                revokers.push(
                    tt.add_Closed(move |_sender, _args| {
                        handler.invoke();
                    })
                    .unwrap(),
                );
            }
            (Event::TeachingTipClosed, _) => {
                panic!("WinUIBackend::attach_event: TeachingTipClosed on non-TeachingTip {id}")
            }
            (Event::TeachingTipActionClick, Handle::TeachingTip(tt)) => {
                revokers.push(
                    tt.add_ActionButtonClick(move |_sender, _args| {
                        handler.invoke();
                    })
                    .unwrap(),
                );
            }
            (Event::TeachingTipActionClick, _) => {
                panic!("WinUIBackend::attach_event: TeachingTipActionClick on non-TeachingTip {id}")
            }
            // ── W23: SelectorBar ─────────────────────────────────────────
            (Event::SelectorBarSelectionChanged, Handle::SelectorBar(sb)) => {
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
            (Event::SelectorBarSelectionChanged, _) => {
                panic!(
                    "WinUIBackend::attach_event: SelectorBarSelectionChanged on non-SelectorBar {id}"
                )
            }
            (Event::RichEditBoxTextChanged, Handle::RichEditBox(reb)) => {
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
            (Event::RichEditBoxTextChanged, _) => {
                panic!("WinUIBackend::attach_event: RichEditBoxTextChanged on non-RichEditBox {id}")
            }
            (Event::FlyoutOpened, _) | (Event::FlyoutClosed, _) => {
                // Flyout open/close events are not yet wired.
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
