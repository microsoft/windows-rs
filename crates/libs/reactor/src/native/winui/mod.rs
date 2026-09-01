use std::any::Any;
use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Write;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use super::*;
use windows_core::Interface;

windows_core::link!("kernel32.dll" "system" fn FindResourceW(module: *mut std::ffi::c_void, name: *const u16, resource_type: *const u16) -> *mut std::ffi::c_void);
windows_core::link!("kernel32.dll" "system" fn GetModuleHandleW(name: *const u16) -> *mut std::ffi::c_void);
windows_core::link!("kernel32.dll" "system" fn LoadResource(module: *mut std::ffi::c_void, resource: *mut std::ffi::c_void) -> *mut std::ffi::c_void);
windows_core::link!("kernel32.dll" "system" fn LockResource(resource: *mut std::ffi::c_void) -> *mut std::ffi::c_void);
windows_core::link!("kernel32.dll" "system" fn SizeofResource(module: *mut std::ffi::c_void, resource: *mut std::ffi::c_void) -> u32);

#[cfg_attr(not(feature = "test"), allow(dead_code))]
#[allow(
    clippy::missing_transmute_annotations,
    clippy::upper_case_acronyms,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals
)]
mod bindings;
pub use bindings::*;
mod app_shim;
pub use app_shim::*;
mod bootstrap;
mod content_dialog;
#[cfg(feature = "test")]
pub(crate) use content_dialog::LiveContentDialogState;
use content_dialog::{ContentDialogAction, ContentDialogScheduler};
mod element_factory;
#[allow(unused_qualifications)]
mod generated;
pub use generated::*;
mod framework;
mod grid;
#[cfg(feature = "test")]
pub(crate) mod test;
#[cfg(feature = "test")]
pub(crate) use test::native_window_handle;

enum PropertyTarget<'a> {
    Framework(UIElement),
    Attached(UIElement),
    GridDefinitions(&'a Handle),
    Generated(&'a Handle),
}

impl<'a> PropertyTarget<'a> {
    fn resolve(
        runtime: &'a WinUiRuntime,
        node: NodeId,
        property: PropertyId,
    ) -> Result<Self, RuntimeError> {
        Ok(match property {
            PropertyId::Width
            | PropertyId::Height
            | PropertyId::MinWidth
            | PropertyId::MaxWidth
            | PropertyId::MinHeight
            | PropertyId::MaxHeight
            | PropertyId::Opacity
            | PropertyId::HorizontalAlignment
            | PropertyId::VerticalAlignment
            | PropertyId::Margin => Self::Framework(runtime.ui_element(node)?),
            PropertyId::GridRow
            | PropertyId::GridColumn
            | PropertyId::GridRowSpan
            | PropertyId::GridColumnSpan
            | PropertyId::RelativeAlignLeft
            | PropertyId::RelativeAlignTop
            | PropertyId::RelativeAlignRight
            | PropertyId::RelativeAlignBottom
            | PropertyId::RelativeAlignHorizontalCenter
            | PropertyId::RelativeAlignVerticalCenter
            | PropertyId::CanvasLeft
            | PropertyId::CanvasTop
            | PropertyId::AutomationName
            | PropertyId::AutomationId
            | PropertyId::AutomationHeadingLevel => Self::Attached(runtime.ui_element(node)?),
            PropertyId::GridRows | PropertyId::GridColumns => Self::GridDefinitions(
                runtime
                    .handles
                    .get(&node)
                    .ok_or(RuntimeError::MissingNode(node))?,
            ),
            _ => Self::Generated(
                runtime
                    .handles
                    .get(&node)
                    .ok_or(RuntimeError::MissingNode(node))?,
            ),
        })
    }

    fn set(&self, property: PropertyId, value: &PropertyValue) -> Result<(), RuntimeError> {
        match self {
            Self::Framework(element) => framework::set(element, property, value),
            Self::Attached(element) => grid::set_attached(element, property, value),
            Self::GridDefinitions(handle) => grid::set_definitions(handle, property, value),
            Self::Generated(handle) => set_property(handle, property, value),
        }
    }

    fn clear(&self, property: PropertyId) -> Result<(), RuntimeError> {
        match self {
            Self::Framework(element) => framework::clear(element, property),
            Self::Attached(element) => grid::clear_attached(element, property),
            Self::GridDefinitions(handle) => grid::clear_definitions(handle, property),
            Self::Generated(handle) => clear_property(handle, property),
        }
    }
}

pub enum NativeSubscription {
    Event {
        _revoker: windows_core::EventRevoker,
        revision: u32,
    },
    Property {
        object: DependencyObject,
        property: DependencyProperty,
        token: i64,
    },
}

impl Drop for NativeSubscription {
    fn drop(&mut self) {
        if let Self::Property {
            object,
            property,
            token,
        } = self
        {
            _ = object.UnregisterPropertyChangedCallback(&*property, *token);
        }
    }
}

#[derive(Default)]
pub struct WinUiRuntime {
    application: Option<(NodeId, Application)>,
    event_errors: Rc<RefCell<Vec<NativeWork<QueuedEventError>>>>,
    handles: HashMap<NodeId, Handle>,
    events: Rc<RefCell<Vec<NativeWork<QueuedEvent>>>>,
    host_events: Rc<RefCell<Vec<NativeWork<HostEvent>>>>,
    async_ingress: Arc<Mutex<AsyncIngressQueue>>,
    async_state: Rc<RefCell<AsyncIngressState>>,
    encoded_image_nodes: Rc<RefCell<HashSet<NodeId>>>,
    feedback: Rc<RefCell<HashMap<(NodeId, EventId), FeedbackExpectation>>>,
    controlled_collection_indices: HashMap<NodeId, i32>,
    content_dialogs: Rc<RefCell<ContentDialogScheduler>>,
    drop_policies: Rc<RefCell<HashMap<NodeId, DragDropPolicy>>>,
    flyouts: HashMap<NodeId, (bindings::Flyout, NodeId)>,
    owned_menus: HashMap<NodeId, NativeOwnedMenu>,
    pointer_capture: Rc<RefCell<HashMap<NodeId, bool>>>,
    resource_override_keys: HashMap<NodeId, HashSet<String>>,
    command_bar_flyouts: HashMap<NodeId, NativeCommandBarFlyout>,
    identity: Rc<Cell<Option<WindowToken>>>,
    selection_items: Rc<RefCell<Vec<(NodeId, windows_core::IInspectable)>>>,
    selection_owners: HashMap<NodeId, (NodeId, SlotId)>,
    realizations: Rc<RefCell<Vec<NativeWork<RealizationRequest>>>>,
    retained_subtrees: HashMap<NodeId, NativeRetainedSubtree>,
    scheduler: Rc<RefCell<SchedulerState>>,
    image_decode_tickets: HashMap<NodeId, u64>,
    observation_subscriptions: HashMap<(NodeId, u64), ObservationSubscription>,
    subscriptions: HashMap<(NodeId, EventId), NativeSubscription>,
    theme_styles: HashMap<(MountedKind, ThemeStyle), Style>,
    virtuals: HashMap<NodeId, element_factory::VirtualHandle>,
    window_closed: Rc<Cell<bool>>,
    window_observations: HashMap<NodeId, WindowObservationFlags>,
    window_hosts: HashMap<NodeId, bindings::Grid>,
    window_observation_subscriptions: HashMap<NodeId, Vec<windows_core::EventRevoker>>,
    window_subscriptions: HashMap<NodeId, windows_core::EventRevoker>,
    window_title_bars: HashMap<NodeId, (NodeId, WindowTitleBarHeight)>,
    window_title_revisions: Rc<RefCell<HashMap<NodeId, u64>>>,
    window_visuals: HashMap<NodeId, WindowVisuals>,
    webview_initializations: Rc<RefCell<HashMap<NodeId, WebViewInitialization>>>,
    windows: HashMap<NodeId, Window>,
    pending_application: Option<Application>,
    #[cfg(feature = "test")]
    native_apply_times_us: Vec<f64>,
}

struct WebViewInitialization {
    _action: Option<windows_future::IAsyncAction>,
    _loaded: Option<windows_core::EventRevoker>,
    _initialized: windows_core::EventRevoker,
    completions: Vec<Callback<Result<windows_core::IUnknown, RuntimeError>>>,
}

fn complete_webview_initialization(
    initializations: &Rc<RefCell<HashMap<NodeId, WebViewInitialization>>>,
    node: NodeId,
    result: Result<windows_core::IUnknown, RuntimeError>,
) {
    let Some(initialization) = initializations.borrow_mut().remove(&node) else {
        return;
    };
    for completion in initialization.completions {
        _ = completion.call(result.clone());
    }
}

struct XamlRootScaleSubscriptions {
    _changed: Rc<RefCell<Option<windows_core::EventRevoker>>>,
    _loaded: windows_core::EventRevoker,
}

enum ObservationSubscription {
    SwapChainPanel {
        _rendering: windows_core::EventRevoker,
        _scale: windows_core::EventRevoker,
        _size: windows_core::EventRevoker,
    },
    ImageScale {
        _root: XamlRootScaleSubscriptions,
    },
    CompositionHost {
        _root: XamlRootScaleSubscriptions,
        _size: windows_core::EventRevoker,
    },
}

struct NativeRetainedSubtree {
    nodes: Vec<NodeId>,
    parent: NodeId,
    slot: Option<SlotId>,
    ticket: u64,
    _timer: DispatcherQueueTimer,
    _tick: windows_core::EventRevoker,
}

#[derive(Default)]
struct AsyncIngressQueue {
    identity: Option<WindowToken>,
    completions: VecDeque<AsyncIngressCompletion>,
}

struct AsyncIngressCompletion {
    identity: WindowToken,
    ticket: u64,
    payload: Box<dyn Any + Send>,
}

#[derive(Default)]
struct AsyncIngressState {
    next_ticket: u64,
    pending: HashMap<u64, PendingAsync>,
}

struct PendingAsync {
    node: NodeId,
    cancel: Box<dyn FnOnce()>,
    finalize: Box<dyn FnOnce(&mut WinUiRuntime, Box<dyn Any + Send>)>,
}

#[derive(Clone)]
struct AsyncIngressSender<T> {
    identity: WindowToken,
    ticket: u64,
    ingress: Arc<Mutex<AsyncIngressQueue>>,
    wake: Arc<dyn Fn() -> bool + Send + Sync>,
    marker: PhantomData<fn(T)>,
}

impl<T: Send + 'static> AsyncIngressSender<T> {
    fn complete(self, payload: T) -> bool {
        let completion = AsyncIngressCompletion {
            identity: self.identity,
            ticket: self.ticket,
            payload: Box::new(payload),
        };
        self.enqueue(completion)
    }

    fn enqueue(self, completion: AsyncIngressCompletion) -> bool {
        {
            let mut ingress = self.ingress.lock().unwrap();
            if ingress.identity != Some(self.identity) {
                return false;
            }
            ingress.completions.push_back(completion);
        }
        if (self.wake)() {
            true
        } else {
            self.ingress
                .lock()
                .unwrap()
                .completions
                .retain(|completion| completion.ticket != self.ticket);
            false
        }
    }
}

struct NativeOwnedMenu {
    target: NodeId,
    kind: OwnedMenuKind,
    _flyout: Option<MenuFlyout>,
    _revokers: Vec<windows_core::EventRevoker>,
}

struct NativeCommandBarFlyout {
    target: NodeId,
    _flyout: bindings::CommandBarFlyout,
    _revokers: Vec<windows_core::EventRevoker>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct WindowVisualChanges {
    backdrop: bool,
    client_size: bool,
    constraints: bool,
    icon: bool,
    theme: bool,
}

fn set_resource_style(
    element: &IFrameworkElement,
    resource: Option<&str>,
) -> Result<(), RuntimeError> {
    let Some(resource) = resource else {
        return element.SetStyle(None::<&Style>).map_err(native_error);
    };
    element
        .SetStyle(&lookup_resource_style(resource)?)
        .map_err(native_error)
}

pub(crate) fn validate_native_image_uri(value: &str) -> windows_core::Result<()> {
    Uri::CreateUri(value).map(drop)
}

pub(crate) fn validate_native_uri(value: &str) -> windows_core::Result<()> {
    Uri::CreateUri(value).map(drop)
}

fn lookup_resource_style(resource: &str) -> Result<Style, RuntimeError> {
    let resources = Application::Current()
        .and_then(|application| application.Resources())
        .map_err(native_error)?;
    let resources = resources
        .cast::<windows_collections::IMap<windows_core::IInspectable, windows_core::IInspectable>>()
        .map_err(native_error)?;
    let key = windows_reference::IReference::from(windows_core::HSTRING::from(resource));
    let style = resources
        .Lookup(&key)
        .and_then(|style| style.cast::<Style>())
        .map_err(native_error)?;
    Ok(style)
}

fn build_command_bar_element(
    command: &CommandBarCommand,
    owner: NodeId,
    revision: u32,
    sink: &EventSink,
) -> Result<(ICommandBarElement, Option<windows_core::EventRevoker>), RuntimeError> {
    match command {
        CommandBarCommand::Button {
            label,
            icon,
            enabled,
            ..
        } => {
            let button = bindings::AppBarButton::new().map_err(native_error)?;
            button.SetLabel(label).map_err(native_error)?;
            button
                .cast::<IControl>()
                .and_then(|control| control.SetIsEnabled(*enabled))
                .map_err(native_error)?;
            if let Some(icon) = icon {
                let icon_element = bindings::SymbolIcon::new().map_err(native_error)?;
                set_property(
                    &Handle::SymbolIcon(icon_element.clone()),
                    PropertyId::SymbolIconSymbol,
                    &PropertyValue::Symbol(*icon),
                )?;
                button.SetIcon(&icon_element).map_err(native_error)?;
            }
            let label = label.clone();
            let sink = sink.clone();
            let revoker = button
                .cast::<IButtonBase>()
                .and_then(|button| {
                    button.Click(move |_, _| {
                        sink.enqueue(
                            owner,
                            EventId::OwnedCommandInvoked,
                            revision,
                            EventPayload::String(label.clone()),
                        );
                    })
                })
                .map_err(native_error)?;
            Ok((button.cast().map_err(native_error)?, Some(revoker)))
        }
        CommandBarCommand::Separator { .. } => Ok((
            bindings::AppBarSeparator::new()
                .and_then(|separator| separator.cast())
                .map_err(native_error)?,
            None,
        )),
    }
}

fn build_menu_items(
    items: &[MenuItem],
    owner: NodeId,
    revision: u32,
    sink: &EventSink,
    output: &windows_collections::IVector<MenuFlyoutItemBase>,
    revokers: &mut Vec<windows_core::EventRevoker>,
) -> Result<(), RuntimeError> {
    for item in items {
        let native: MenuFlyoutItemBase = match item {
            MenuItem::Item { label, enabled, .. } => {
                let item = MenuFlyoutItem::new().map_err(native_error)?;
                item.SetText(label).map_err(native_error)?;
                item.cast::<IControl>()
                    .and_then(|control| control.SetIsEnabled(*enabled))
                    .map_err(native_error)?;
                let label = label.clone();
                let sink = sink.clone();
                revokers.push(
                    item.Click(move |_, _| {
                        sink.enqueue(
                            owner,
                            EventId::OwnedMenuItemInvoked,
                            revision,
                            EventPayload::String(label.clone()),
                        );
                    })
                    .map_err(native_error)?,
                );
                item.cast().map_err(native_error)?
            }
            MenuItem::Separator { .. } => MenuFlyoutSeparator::new()
                .and_then(|separator| separator.cast())
                .map_err(native_error)?,
            MenuItem::Submenu { label, items, .. } => {
                let submenu = MenuFlyoutSubItem::new().map_err(native_error)?;
                submenu.SetText(label).map_err(native_error)?;
                build_menu_items(
                    items,
                    owner,
                    revision,
                    sink,
                    &submenu.Items().map_err(native_error)?,
                    revokers,
                )?;
                submenu.cast().map_err(native_error)?
            }
        };
        output.Append(&native).map_err(native_error)?;
    }
    Ok(())
}

fn set_rich_edit_text(control: &bindings::RichEditBox, value: &str) -> Result<(), RuntimeError> {
    let read_only = control.IsReadOnly().map_err(native_error)?;
    let document = control.Document().map_err(native_error)?;
    let mut current = windows_core::HSTRING::new();
    document
        .GetText(TextGetOptions::None, &mut current)
        .map_err(native_error)?;
    if current == value {
        return Ok(());
    }
    if read_only {
        control.SetIsReadOnly(false).map_err(native_error)?;
    }
    let write = document
        .SetText(TextSetOptions::None, value)
        .map_err(native_error);
    let restore = if read_only {
        control.SetIsReadOnly(true).map_err(native_error)
    } else {
        Ok(())
    };
    write.and(restore)
}

fn build_tree_node(definition: &TreeNode) -> Result<TreeViewNode, RuntimeError> {
    let node = TreeViewNode::new().map_err(native_error)?;
    let content: windows_core::IInspectable =
        windows_reference::IReference::from(windows_core::HSTRING::from(&definition.text)).into();
    node.SetContent(&content).map_err(native_error)?;
    node.SetIsExpanded(definition.expanded)
        .map_err(native_error)?;
    let children = node.Children().map_err(native_error)?;
    for child in &definition.children {
        let child = build_tree_node(child)?;
        children.Append(&child).map_err(native_error)?;
    }
    Ok(node)
}

fn window_visual_changes(
    previous: Option<WindowVisuals>,
    next: WindowVisuals,
) -> WindowVisualChanges {
    WindowVisualChanges {
        backdrop: previous.is_none_or(|previous| previous.backdrop != next.backdrop),
        client_size: previous.is_none_or(|previous| previous.client_size != next.client_size),
        constraints: previous.is_none_or(|previous| previous.constraints != next.constraints),
        icon: next.icon.is_some() && previous.is_none_or(|previous| previous.icon != next.icon),
        theme: previous.is_none_or(|previous| previous.theme != next.theme),
    }
}

#[cfg(test)]
mod window_visual_tests {
    use super::*;

    #[test]
    fn theme_change_does_not_reapply_client_size_or_backdrop() {
        let previous = WindowVisuals::new()
            .theme(WindowTheme::System)
            .backdrop(WindowBackdrop::Mica)
            .client_size(1400.0, 900.0);
        let next = WindowVisuals::new()
            .theme(WindowTheme::Dark)
            .backdrop(WindowBackdrop::Mica)
            .client_size(1400.0, 900.0);

        assert_eq!(
            window_visual_changes(Some(previous), next),
            WindowVisualChanges {
                backdrop: false,
                client_size: false,
                constraints: false,
                icon: false,
                theme: true,
            }
        );
    }

    #[test]
    fn constraints_change_independently_and_can_be_cleared() {
        let constrained =
            WindowVisuals::new()
                .client_size(800.0, 600.0)
                .constraints(WindowConstraints {
                    min_width: Some(400.0),
                    ..Default::default()
                });
        let resized =
            WindowVisuals::new()
                .client_size(900.0, 700.0)
                .constraints(WindowConstraints {
                    min_width: Some(400.0),
                    ..Default::default()
                });
        let cleared = WindowVisuals::new().client_size(900.0, 700.0);

        assert_eq!(
            window_visual_changes(Some(constrained), resized),
            WindowVisualChanges {
                backdrop: false,
                client_size: true,
                constraints: false,
                icon: false,
                theme: false,
            }
        );
        assert_eq!(
            window_visual_changes(Some(resized), cleared),
            WindowVisualChanges {
                backdrop: false,
                client_size: false,
                constraints: true,
                icon: false,
                theme: false,
            }
        );
    }
}

impl WinUiRuntime {
    pub fn with_application(application: Application) -> Self {
        Self {
            pending_application: Some(application),
            ..Default::default()
        }
    }

    #[cfg(feature = "test")]
    pub fn live_select_list_box_item(&self, node: NodeId, index: u32) -> Result<(), RuntimeError> {
        let Some(handle @ Handle::ListBox(_)) = self.handles.get(&node) else {
            return Err(RuntimeError::UnsupportedKind);
        };
        let item = slot_collection(handle, SlotId::ListBoxItems)?.GetAt(index)?;
        set_selected_item(
            handle,
            selection_for_slot(SlotId::ListBoxItems).unwrap(),
            &item,
        )
    }

    #[cfg(feature = "test")]
    pub fn live_range_value(&self, node: NodeId) -> Result<f64, RuntimeError> {
        match self.handles.get(&node) {
            Some(Handle::NumberBox(control)) => control.Value().map_err(native_error),
            Some(Handle::Slider(control)) => control
                .cast::<IRangeBase>()
                .and_then(|control| control.Value())
                .map_err(native_error),
            _ => Err(RuntimeError::UnsupportedKind),
        }
    }

    #[cfg(feature = "test")]
    pub fn live_checked_value(&self, node: NodeId) -> Result<bool, RuntimeError> {
        match self.handles.get(&node) {
            Some(Handle::CheckBox(control)) => control
                .cast::<IToggleButton>()
                .and_then(|control| control.IsChecked())
                .map_err(native_error),
            Some(Handle::ToggleButton(control)) => control.IsChecked().map_err(native_error),
            _ => Err(RuntimeError::UnsupportedKind),
        }
    }

    #[cfg(feature = "test")]
    pub fn live_set_checked(&self, node: NodeId, value: bool) -> Result<(), RuntimeError> {
        let property = match self.handles.get(&node).map(Handle::kind) {
            Some(MountedKind::CheckBox) => PropertyId::CheckBoxIsChecked,
            Some(MountedKind::ToggleButton) => PropertyId::ToggleButtonIsChecked,
            _ => return Err(RuntimeError::UnsupportedKind),
        };
        self.live_write_property(node, property, &PropertyValue::Bool(value))
    }

    #[cfg(feature = "test")]
    fn live_write_property(
        &self,
        node: NodeId,
        property: PropertyId,
        value: &PropertyValue,
    ) -> Result<(), RuntimeError> {
        let handle = self
            .handles
            .get(&node)
            .ok_or(RuntimeError::MissingNode(node))?;
        set_property(handle, property, value)
    }

    #[cfg(feature = "test")]
    pub(crate) fn live_write_test_property(
        &self,
        node: NodeId,
        property: PropertyId,
        value: &PropertyValue,
    ) -> Result<(), RuntimeError> {
        self.live_write_property(node, property, value)
    }

    #[cfg(feature = "test")]
    pub(crate) fn live_set_test_date(
        &self,
        node: NodeId,
        value: DateTime,
    ) -> Result<(), RuntimeError> {
        match self.handles.get(&node) {
            Some(Handle::DatePicker(control)) => {
                control.SetSelectedDate(Some(value)).map_err(native_error)
            }
            Some(Handle::CalendarDatePicker(control)) => {
                control.SetDate(Some(value)).map_err(native_error)
            }
            _ => Err(RuntimeError::UnsupportedKind),
        }
    }

    #[cfg(feature = "test")]
    pub(crate) fn live_set_test_time(
        &self,
        node: NodeId,
        value: TimeSpan,
    ) -> Result<(), RuntimeError> {
        let Some(Handle::TimePicker(control)) = self.handles.get(&node) else {
            return Err(RuntimeError::UnsupportedKind);
        };
        control.SetSelectedTime(Some(value)).map_err(native_error)
    }

    #[cfg(feature = "test")]
    pub fn take_live_native_apply_times(&mut self) -> Vec<f64> {
        std::mem::take(&mut self.native_apply_times_us)
    }

    #[cfg(feature = "test")]
    pub fn clear_live_native_apply_times(&mut self) {
        self.native_apply_times_us.clear();
    }

    #[cfg(feature = "test")]
    pub fn live_event_subscription_count(&self) -> usize {
        self.subscriptions.len() + self.content_dialogs.borrow().subscription_count()
    }

    #[cfg(feature = "test")]
    pub(crate) fn live_content_dialog_states(&self) -> Vec<LiveContentDialogState> {
        self.content_dialogs.borrow().states()
    }

    #[cfg(feature = "test")]
    pub(crate) fn live_hide_content_dialog(&self, node: NodeId) -> Result<(), RuntimeError> {
        let dialog = self.content_dialogs.borrow().dialog(node)?;
        dialog.Hide().map_err(native_error)
    }

    #[cfg(feature = "test")]
    pub fn live_bring_virtual_index(&self, index: usize) -> Result<(), RuntimeError> {
        let virtual_handle = self
            .virtuals
            .values()
            .next()
            .ok_or(RuntimeError::UnsupportedKind)?;
        let index = i32::try_from(index).map_err(|_| RuntimeError::IndexOutOfBounds)?;
        virtual_handle
            .repeater
            .GetOrCreateElement(index)
            .and_then(|element| element.StartBringIntoView())
            .map_err(native_error)
    }

    #[cfg(feature = "test")]
    pub fn live_virtual_shell_counts(&self) -> Result<(usize, usize), RuntimeError> {
        let mut live = 0;
        let mut retired = 0;
        for handle in self.virtuals.values() {
            let counts = handle.shell_counts();
            live += counts.0;
            retired += counts.1;
        }
        if self.virtuals.is_empty() {
            Err(RuntimeError::UnsupportedKind)
        } else {
            Ok((live, retired))
        }
    }

    fn apply_window_visuals(
        &mut self,
        node: NodeId,
        visuals: WindowVisuals,
    ) -> Result<(), RuntimeError> {
        let window = self
            .windows
            .get(&node)
            .ok_or(RuntimeError::MissingNode(node))?;
        let window_2 = window.cast::<IWindow2>().map_err(native_error)?;
        let changes = window_visual_changes(self.window_visuals.get(&node).copied(), visuals);
        let mut hwnd = None;
        let mut window_handle = || {
            if let Some(hwnd) = hwnd {
                return Ok(hwnd);
            }
            let mut value = std::ptr::null_mut();
            unsafe {
                window
                    .cast::<IWindowNative>()
                    .map_err(native_error)?
                    .WindowHandle(&mut value)
                    .ok()
                    .map_err(native_error)?;
            }
            hwnd = Some(value);
            Ok(value)
        };

        if changes.backdrop {
            match visuals.backdrop {
                WindowBackdrop::None => window_2
                    .SetSystemBackdrop(None::<&SystemBackdrop>)
                    .map_err(native_error)?,
                WindowBackdrop::Mica | WindowBackdrop::MicaAlt => {
                    let mica = MicaBackdrop::new().map_err(native_error)?;
                    mica.SetKind(match visuals.backdrop {
                        WindowBackdrop::Mica => MicaKind::Base,
                        WindowBackdrop::MicaAlt => MicaKind::BaseAlt,
                        _ => unreachable!(),
                    })
                    .map_err(native_error)?;
                    let backdrop: SystemBackdrop = mica.cast().map_err(native_error)?;
                    window_2
                        .SetSystemBackdrop(&backdrop)
                        .map_err(native_error)?;
                }
                WindowBackdrop::Acrylic => {
                    let backdrop: SystemBackdrop = DesktopAcrylicBackdrop::new()
                        .and_then(|backdrop| backdrop.cast())
                        .map_err(native_error)?;
                    window_2
                        .SetSystemBackdrop(&backdrop)
                        .map_err(native_error)?;
                }
            }
        }

        if changes.theme {
            let title_bar = window_2
                .AppWindow()
                .and_then(|window| window.TitleBar())
                .and_then(|title_bar| title_bar.cast::<IAppWindowTitleBar3>())
                .map_err(native_error)?;
            title_bar
                .SetPreferredTheme(match visuals.theme {
                    WindowTheme::System => TitleBarTheme::UseDefaultAppMode,
                    WindowTheme::Light => TitleBarTheme::Light,
                    WindowTheme::Dark => TitleBarTheme::Dark,
                })
                .map_err(native_error)?;

            if let Some(host) = self.window_hosts.get(&node) {
                Self::apply_window_theme(host, visuals.theme)?;
            }
        }

        if changes.icon
            && let Some(path) = visuals.icon
        {
            window_2
                .AppWindow()
                .and_then(|window| window.SetIcon(path))
                .map_err(native_error)?;
        }

        if changes.constraints {
            let app_window = window_2.AppWindow().map_err(native_error)?;
            let presenter = app_window
                .Presenter()
                .and_then(|presenter| presenter.cast::<IOverlappedPresenter3>())
                .map_err(native_error)?;
            let (min_width, min_height, max_width, max_height) =
                if let Some(constraints) = visuals.constraints {
                    let hwnd = window_handle()?;
                    let dpi = unsafe { GetDpiForWindow(hwnd) }.max(96);
                    let pixels = |dips: f64| (dips * f64::from(dpi) / 96.0).round() as i32;
                    let client_window = app_window.cast::<IAppWindow2>().map_err(native_error)?;
                    let outer = app_window.Size().map_err(native_error)?;
                    let inner = client_window.ClientSize().map_err(native_error)?;
                    let non_client_width = outer.width.saturating_sub(inner.width);
                    let non_client_height = outer.height.saturating_sub(inner.height);
                    (
                        constraints
                            .min_width
                            .map(|value| pixels(value).saturating_add(non_client_width)),
                        constraints
                            .min_height
                            .map(|value| pixels(value).saturating_add(non_client_height)),
                        constraints
                            .max_width
                            .map(|value| pixels(value).saturating_add(non_client_width)),
                        constraints
                            .max_height
                            .map(|value| pixels(value).saturating_add(non_client_height)),
                    )
                } else {
                    (None, None, None, None)
                };
            presenter
                .SetPreferredMinimumWidth(min_width)
                .and_then(|()| presenter.SetPreferredMinimumHeight(min_height))
                .and_then(|()| presenter.SetPreferredMaximumWidth(max_width))
                .and_then(|()| presenter.SetPreferredMaximumHeight(max_height))
                .map_err(native_error)?;
        }

        if changes.client_size
            && let Some((width, height)) = visuals.client_size
        {
            let hwnd = window_handle()?;
            let dpi = unsafe { GetDpiForWindow(hwnd) }.max(96);
            let pixels = |dips: f64| (dips * f64::from(dpi) / 96.0).round() as i32;
            window_2
                .AppWindow()
                .and_then(|window| window.cast::<IAppWindow2>())
                .and_then(|window| {
                    window.ResizeClient(SizeInt32 {
                        width: pixels(width),
                        height: pixels(height),
                    })
                })
                .map_err(native_error)?;
        }
        self.window_visuals.insert(node, visuals);
        Ok(())
    }

    fn clear_window_title_bar(&mut self, node: NodeId) -> Result<(), RuntimeError> {
        let window = self
            .windows
            .get(&node)
            .ok_or(RuntimeError::MissingNode(node))?;
        window
            .SetTitleBar(None::<&UIElement>)
            .and_then(|()| window.SetExtendsContentIntoTitleBar(false))
            .map_err(native_error)?;
        window
            .cast::<IWindow2>()
            .and_then(|window| window.AppWindow())
            .and_then(|window| window.TitleBar())
            .and_then(|title_bar| title_bar.cast::<IAppWindowTitleBar2>())
            .and_then(|title_bar| {
                title_bar.SetPreferredHeightOption(TitleBarHeightOption::Standard)
            })
            .map_err(native_error)?;
        self.window_title_bars.remove(&node);
        Ok(())
    }

    fn apply_window_title_bar(
        &mut self,
        node: NodeId,
        title_bar: NodeId,
        height: WindowTitleBarHeight,
    ) -> Result<(), RuntimeError> {
        let window = self
            .windows
            .get(&node)
            .ok_or(RuntimeError::MissingNode(node))?;
        let title_bar_element = self.ui_element(title_bar)?;
        window
            .SetExtendsContentIntoTitleBar(true)
            .and_then(|()| window.SetTitleBar(&title_bar_element))
            .map_err(native_error)?;
        let height_option = match height {
            WindowTitleBarHeight::Standard => TitleBarHeightOption::Standard,
            WindowTitleBarHeight::Tall => TitleBarHeightOption::Tall,
        };
        window
            .cast::<IWindow2>()
            .and_then(|window| window.AppWindow())
            .and_then(|window| window.TitleBar())
            .and_then(|title_bar| title_bar.cast::<IAppWindowTitleBar2>())
            .and_then(|title_bar| title_bar.SetPreferredHeightOption(height_option))
            .map_err(native_error)?;
        self.window_title_bars.insert(node, (title_bar, height));
        Ok(())
    }

    fn set_window_title(&mut self, node: NodeId, title: &str) -> Result<(), RuntimeError> {
        let window = self
            .windows
            .get(&node)
            .ok_or(RuntimeError::MissingNode(node))?;
        let revision = {
            let mut revisions = self.window_title_revisions.borrow_mut();
            let revision = revisions
                .get_mut(&node)
                .ok_or(RuntimeError::MissingNode(node))?;
            *revision = revision.wrapping_add(1);
            *revision
        };
        window.SetTitle(title).map_err(native_error)?;
        if !self.window_title_bars.contains_key(&node) {
            return Ok(());
        }

        let window = window.clone();
        let title = title.to_string();
        let title_revisions = Rc::clone(&self.window_title_revisions);
        let identity = Rc::clone(&self.identity);
        let window_identity = identity.get().unwrap();
        let sink = self.event_sink()?;
        let handler = DispatcherQueueHandler::new(move || {
            if identity.get() != Some(window_identity)
                || title_revisions.borrow().get(&node) != Some(&revision)
            {
                return;
            }
            if let Err(error) = window.SetTitle(&title).map_err(native_error) {
                sink.enqueue_host(HostEvent::Error(error));
            }
        });
        let accepted = DispatcherQueue::GetForCurrentThread()
            .and_then(|dispatcher| {
                dispatcher.TryEnqueueWithPriority(DispatcherQueuePriority::Low, &handler)
            })
            .map_err(native_error)?;
        if accepted {
            Ok(())
        } else {
            Err(RuntimeError::DispatcherRejected)
        }
    }

    fn apply_theme_style(&mut self, node: NodeId, style: ThemeStyle) -> Result<(), RuntimeError> {
        let handle = self
            .handles
            .get(&node)
            .ok_or(RuntimeError::MissingNode(node))?;
        let framework_element = handle
            .ui_element()
            .and_then(|element| element.cast::<IFrameworkElement>())
            .map_err(native_error)?;
        if style.is_empty() {
            return framework_element
                .SetStyle(None::<&Style>)
                .map_err(native_error);
        }

        let kind = handle.kind();
        let compiled = if let Some(compiled) = self.theme_styles.get(&(kind, style)) {
            compiled.clone()
        } else {
            let (target, properties) =
                theme_style_info(kind).ok_or(RuntimeError::UnsupportedKind)?;
            let mut xaml = format!(
                "<Style xmlns='http://schemas.microsoft.com/winfx/2006/xaml/presentation' \
                 TargetType='{target}'>"
            );
            for (property, brush) in properties.iter().zip(style.values()) {
                if let Some(brush) = brush {
                    write!(
                        xaml,
                        "<Setter Property='{property}' Value='{{ThemeResource {}}}'/>",
                        brush.resource_key()
                    )
                    .unwrap();
                }
            }
            xaml.push_str("</Style>");
            let compiled = XamlReader::Load(&xaml)
                .and_then(|value| value.cast::<Style>())
                .map_err(native_error)?;
            self.theme_styles.insert((kind, style), compiled.clone());
            compiled
        };
        framework_element.SetStyle(&compiled).map_err(native_error)
    }

    fn apply_window_theme(host: &bindings::Grid, theme: WindowTheme) -> Result<(), RuntimeError> {
        host.cast::<FrameworkElement>()
            .and_then(|root| {
                root.SetRequestedTheme(match theme {
                    WindowTheme::System => ElementTheme::Default,
                    WindowTheme::Light => ElementTheme::Light,
                    WindowTheme::Dark => ElementTheme::Dark,
                })
            })
            .map_err(native_error)
    }

    fn set_resource_overrides(
        &mut self,
        node: NodeId,
        resources: &ResourceOverrides,
    ) -> Result<(), RuntimeError> {
        let dictionary = self
            .ui_element(node)?
            .cast::<IFrameworkElement>()
            .and_then(|element| element.Resources())
            .map_err(native_error)?;
        let map = dictionary
            .cast::<
                windows_collections::IMap<
                    windows_core::IInspectable,
                    windows_core::IInspectable,
                >,
            >()
            .map_err(native_error)?;
        let desired_keys = resources
            .values()
            .map(|(key, _)| key.to_string())
            .collect::<HashSet<_>>();

        if let Some(previous_keys) = self.resource_override_keys.get(&node) {
            for key in previous_keys.difference(&desired_keys) {
                let key = windows_reference::IReference::from(key.as_str());
                if map.HasKey(&key).map_err(native_error)? {
                    map.Remove(&key).map_err(native_error)?;
                }
            }
        }

        for (key, value) in resources.values() {
            let key = windows_reference::IReference::from(key);
            let value: windows_core::IInspectable = match value {
                ResourceValue::Color(value) => {
                    let brush = SolidColorBrush::new().map_err(native_error)?;
                    brush
                        .SetColor(bindings::Color {
                            a: value.a,
                            r: value.r,
                            g: value.g,
                            b: value.b,
                        })
                        .map_err(native_error)?;
                    brush.into()
                }
                ResourceValue::Thickness(value) => {
                    windows_reference::IReference::from(bindings::Thickness {
                        left: value.left(),
                        top: value.top(),
                        right: value.right(),
                        bottom: value.bottom(),
                    })
                    .into()
                }
                ResourceValue::CornerRadius(value) => {
                    windows_reference::IReference::from(bindings::CornerRadius {
                        top_left: value.top_left(),
                        top_right: value.top_right(),
                        bottom_right: value.bottom_right(),
                        bottom_left: value.bottom_left(),
                    })
                    .into()
                }
            };
            map.Insert(&key, &value).map_err(native_error)?;
        }

        if desired_keys.is_empty() {
            self.resource_override_keys.remove(&node);
        } else {
            self.resource_override_keys.insert(node, desired_keys);
        }
        Ok(())
    }

    fn set_window_observations(
        &mut self,
        node: NodeId,
        observations: WindowObservationFlags,
    ) -> Result<(), RuntimeError> {
        self.window_observations.insert(node, observations);
        self.window_observation_subscriptions.remove(&node);
        if observations == WindowObservationFlags::default() {
            return Ok(());
        }
        let source = self
            .window_hosts
            .get(&node)
            .ok_or(RuntimeError::MissingNode(node))?
            .cast::<IFrameworkElement>()
            .map_err(native_error)?;
        let sink = self.event_sink()?;
        let mut subscriptions = Vec::new();
        if let Some(observation) = observations.window_size {
            let sink = sink.clone();
            subscriptions.push(
                source
                    .SizeChanged(move |_, args| {
                        let result = args
                            .as_ref()
                            .ok_or_else(windows_core::Error::empty)
                            .and_then(|args| args.NewSize());
                        match result {
                            Ok(value) => sink.enqueue_host(HostEvent::WindowSize {
                                observation,
                                size: WindowSize {
                                    width: f64::from(value.width),
                                    height: f64::from(value.height),
                                },
                            }),
                            Err(error) => {
                                sink.enqueue_host(HostEvent::ObservationError {
                                    observation,
                                    error: native_error(error),
                                });
                            }
                        }
                    })
                    .map_err(native_error)?,
            );
        }
        if let Some(observation) = observations.color_scheme {
            let changed_sink = sink.clone();
            subscriptions.push(
                source
                    .ActualThemeChanged(move |sender, _| {
                        let result = sender
                            .as_ref()
                            .ok_or_else(windows_core::Error::empty)
                            .and_then(|sender| sender.cast::<IFrameworkElement>())
                            .and_then(|sender| sender.ActualTheme());
                        match result {
                            Ok(ElementTheme::Dark) => {
                                changed_sink.enqueue_host(HostEvent::ColorScheme {
                                    observation,
                                    scheme: ColorScheme::Dark,
                                });
                            }
                            Ok(_) => {
                                changed_sink.enqueue_host(HostEvent::ColorScheme {
                                    observation,
                                    scheme: ColorScheme::Light,
                                });
                            }
                            Err(error) => {
                                changed_sink.enqueue_host(HostEvent::ObservationError {
                                    observation,
                                    error: native_error(error),
                                });
                            }
                        }
                    })
                    .map_err(native_error)?,
            );
            let scheme = match source.ActualTheme().map_err(native_error)? {
                ElementTheme::Dark => ColorScheme::Dark,
                _ => ColorScheme::Light,
            };
            sink.enqueue_host(HostEvent::ColorScheme {
                observation,
                scheme,
            });
        }
        self.window_observation_subscriptions
            .insert(node, subscriptions);
        Ok(())
    }

    fn clear_resource_overrides(&mut self, node: NodeId) -> Result<(), RuntimeError> {
        let Some(keys) = self.resource_override_keys.remove(&node) else {
            return Ok(());
        };
        let dictionary = self
            .ui_element(node)?
            .cast::<IFrameworkElement>()
            .and_then(|element| element.Resources())
            .map_err(native_error)?;
        let map = dictionary
            .cast::<
                windows_collections::IMap<
                    windows_core::IInspectable,
                    windows_core::IInspectable,
                >,
            >()
            .map_err(native_error)?;
        for key in keys {
            let key = windows_reference::IReference::from(key.as_str());
            if map.HasKey(&key).map_err(native_error)? {
                map.Remove(&key).map_err(native_error)?;
            }
        }
        Ok(())
    }

    fn set_key_accelerators(
        &self,
        node: NodeId,
        accelerators: &KeyAccelerators,
    ) -> Result<(), RuntimeError> {
        let element = self
            .ui_element(node)?
            .cast::<IUIElement>()
            .map_err(native_error)?;
        let values = element.KeyboardAccelerators().map_err(native_error)?;
        values.Clear().map_err(native_error)?;
        element
            .SetKeyboardAcceleratorPlacementMode(KeyboardAcceleratorPlacementMode::Hidden)
            .map_err(native_error)?;

        for accelerator in &accelerators.values {
            let value = KeyboardAccelerator::new().map_err(native_error)?;
            value
                .SetKey(match accelerator.key {
                    AcceleratorKey::R => VirtualKey::R,
                    AcceleratorKey::NumberPad0 => VirtualKey::NumberPad0,
                    AcceleratorKey::NumberPad1 => VirtualKey::NumberPad1,
                    AcceleratorKey::NumberPad2 => VirtualKey::NumberPad2,
                    AcceleratorKey::NumberPad3 => VirtualKey::NumberPad3,
                    AcceleratorKey::NumberPad4 => VirtualKey::NumberPad4,
                    AcceleratorKey::NumberPad5 => VirtualKey::NumberPad5,
                    AcceleratorKey::NumberPad6 => VirtualKey::NumberPad6,
                    AcceleratorKey::NumberPad7 => VirtualKey::NumberPad7,
                    AcceleratorKey::NumberPad8 => VirtualKey::NumberPad8,
                    AcceleratorKey::NumberPad9 => VirtualKey::NumberPad9,
                    AcceleratorKey::Divide => VirtualKey::Divide,
                    AcceleratorKey::Multiply => VirtualKey::Multiply,
                    AcceleratorKey::Subtract => VirtualKey::Subtract,
                    AcceleratorKey::Add => VirtualKey::Add,
                    AcceleratorKey::Decimal => VirtualKey::Decimal,
                    AcceleratorKey::Enter => VirtualKey::Enter,
                })
                .map_err(native_error)?;
            value
                .SetModifiers(match accelerator.modifiers {
                    AcceleratorModifiers::None => VirtualKeyModifiers::None,
                    AcceleratorModifiers::Control => VirtualKeyModifiers::Control,
                })
                .map_err(native_error)?;
            let callback = accelerator.callback.clone();
            value
                .Invoked(move |_, args| {
                    if let Some(args) = args.as_ref() {
                        _ = args.SetHandled(true);
                    }
                    _ = callback.call(());
                })
                .map(windows_core::EventRevoker::into_token)
                .map_err(native_error)?;
            values.Append(&value).map_err(native_error)?;
        }
        Ok(())
    }

    fn apply_one(&mut self, command: &Command) -> Result<(), RuntimeError> {
        match command {
            Command::CreateApplication { node } => {
                if self.contains(*node) || self.application.is_some() {
                    return Err(RuntimeError::DuplicateNode(*node));
                }
                let application = self
                    .pending_application
                    .take()
                    .ok_or(RuntimeError::MissingApplication)?;
                self.application = Some((*node, application));
            }
            Command::CreateWindow { node } => {
                if self.contains(*node) {
                    return Err(RuntimeError::DuplicateNode(*node));
                }
                let window = Window::new().map_err(native_error)?;
                let host = bindings::Grid::new().map_err(native_error)?;
                window
                    .SetContent(&host.cast::<UIElement>().map_err(native_error)?)
                    .map_err(native_error)?;
                let closed = Rc::clone(&self.window_closed);
                let identity = self.identity.get().unwrap();
                let subscription = window
                    .Closed(move |_, _| {
                        closed.set(true);
                        dispatch_window_closed(identity);
                    })
                    .map_err(native_error)?;
                self.window_subscriptions.insert(*node, subscription);
                self.window_hosts.insert(*node, host);
                self.window_title_revisions.borrow_mut().insert(*node, 0);
                self.windows.insert(*node, window);
            }
            Command::ActivateWindow { node } => {
                self.windows
                    .get(node)
                    .ok_or(RuntimeError::MissingNode(*node))?
                    .Activate()
                    .map_err(native_error)?;
            }
            Command::CloseWindow { node } => {
                self.windows
                    .get(node)
                    .ok_or(RuntimeError::MissingNode(*node))?
                    .Close()
                    .map_err(native_error)?;
            }
            Command::SetWindowTitle { node, title } => {
                self.set_window_title(*node, title)?;
            }
            Command::ClearWindowTitleBar { node } => {
                self.clear_window_title_bar(*node)?;
            }
            Command::SetWindowTitleBar {
                node,
                title_bar,
                height,
            } => {
                self.apply_window_title_bar(*node, *title_bar, *height)?;
            }
            Command::SetWindowVisuals { node, visuals } => {
                self.apply_window_visuals(*node, *visuals)?;
            }
            Command::SetWindowObservations { node, observations } => {
                self.set_window_observations(*node, *observations)?;
            }
            Command::SetThemeStyle { node, style } => {
                self.apply_theme_style(*node, *style)?;
            }
            Command::Create { node, kind } => {
                if self.contains(*node) {
                    return Err(RuntimeError::DuplicateNode(*node));
                }
                let handle = Handle::create(*kind)?;
                if let Handle::ContentDialog(dialog) = &handle {
                    self.content_dialogs
                        .borrow_mut()
                        .create(*node, dialog.clone());
                }
                self.handles.insert(*node, handle);
            }
            Command::CreateVirtualCollection {
                node,
                item_count,
                source_revision,
            } => {
                if self.contains(*node) {
                    return Err(RuntimeError::DuplicateNode(*node));
                }
                let handle = element_factory::VirtualHandle::create(
                    self.identity.get().unwrap(),
                    *node,
                    *item_count,
                    *source_revision,
                    Rc::clone(&self.realizations),
                    self.event_sink()?,
                )
                .map_err(native_error)?;
                self.virtuals.insert(*node, handle);
            }
            Command::ResetVirtualCollection {
                node,
                item_count,
                source_revision,
            } => {
                self.virtuals
                    .get(node)
                    .ok_or(RuntimeError::MissingNode(*node))?
                    .reset(*item_count, *source_revision)
                    .map_err(native_error)?;
            }
            Command::AttachRealized {
                collection,
                container,
                child,
            } => {
                let child = self.ui_element(*child)?;
                self.virtuals
                    .get(collection)
                    .ok_or(RuntimeError::MissingNode(*collection))?
                    .set_content(*container, Some(&child))
                    .map_err(native_error)?;
            }
            Command::DetachRealized {
                collection,
                container,
                ..
            } => {
                self.virtuals
                    .get(collection)
                    .ok_or(RuntimeError::MissingNode(*collection))?
                    .clear_content(*container)
                    .map_err(native_error)?;
            }
            Command::AcknowledgeRecycle {
                collection,
                container,
            } => {
                self.virtuals
                    .get(collection)
                    .ok_or(RuntimeError::MissingNode(*collection))?
                    .acknowledge_recycle(*container)
                    .map_err(native_error)?;
            }
            Command::Destroy { node } => {
                if !self.contains(*node) {
                    return Err(RuntimeError::MissingNode(*node));
                }
                self.cancel_retirements_for_node(*node)?;
                if self.flyouts.contains_key(node)
                    || self.flyouts.values().any(|(_, content)| content == node)
                    || self.owned_menus.values().any(|menu| menu.target == *node)
                    || self
                        .command_bar_flyouts
                        .values()
                        .any(|flyout| flyout.target == *node)
                {
                    return Err(RuntimeError::StillParented(*node));
                }
                if self
                    .window_title_bars
                    .values()
                    .any(|(title_bar, _)| title_bar == node)
                {
                    return Err(RuntimeError::StillParented(*node));
                }
                return self.remove_node_state(*node);
            }
            Command::RetireSubtree {
                root,
                nodes,
                parent,
                slot,
                transition,
            } => {
                self.start_retirement(*root, nodes.clone(), *parent, *slot, *transition)?;
            }

            Command::Focus { node, completion } => {
                let result = self.ui_element(*node).and_then(|element| {
                    element
                        .Focus(FocusState::Programmatic)
                        .map_err(native_error)
                });
                _ = completion.call(result);
            }
            Command::InitializeWebView2 { node, completion } => {
                let Some(Handle::WebView2(control)) = self.handles.get(node) else {
                    _ = completion.call(Err(if self.contains(*node) {
                        RuntimeError::UnsupportedKind
                    } else {
                        RuntimeError::MissingNode(*node)
                    }));
                    return Ok(());
                };
                if let Ok(core) = control.CoreWebView2() {
                    let result = Ok(core.into());
                    let pending = {
                        let mut initializations = self.webview_initializations.borrow_mut();
                        if let Some(initialization) = initializations.get_mut(node) {
                            initialization.completions.push(completion.clone());
                            true
                        } else {
                            false
                        }
                    };
                    if pending {
                        complete_webview_initialization(
                            &self.webview_initializations,
                            *node,
                            result,
                        );
                    } else {
                        _ = completion.call(result);
                    }
                    return Ok(());
                }
                if let Some(initialization) =
                    self.webview_initializations.borrow_mut().get_mut(node)
                {
                    initialization.completions.push(completion.clone());
                    return Ok(());
                }
                let initializations = Rc::clone(&self.webview_initializations);
                let initialized_node = *node;
                let revoker = match control.CoreWebView2Initialized(move |sender, _| {
                    let result = sender
                        .as_ref()
                        .ok_or_else(windows_core::Error::empty)
                        .and_then(|control| control.CoreWebView2())
                        .map(Into::into)
                        .map_err(native_error);
                    complete_webview_initialization(&initializations, initialized_node, result);
                }) {
                    Ok(revoker) => revoker,
                    Err(error) => {
                        _ = completion.call(Err(native_error(error)));
                        return Ok(());
                    }
                };
                let element = match control.cast::<IFrameworkElement>() {
                    Ok(element) => element,
                    Err(error) => {
                        _ = completion.call(Err(native_error(error)));
                        return Ok(());
                    }
                };
                let mut action = None;
                let loaded = if element.IsLoaded().unwrap_or(false) {
                    match control.EnsureCoreWebView2Async() {
                        Ok(pending) => action = Some(pending),
                        Err(error) => {
                            _ = completion.call(Err(native_error(error)));
                            return Ok(());
                        }
                    }
                    None
                } else {
                    let initializations = Rc::clone(&self.webview_initializations);
                    let loaded_node = *node;
                    let control = control.clone();
                    match element.Loaded(move |_, _| match control.EnsureCoreWebView2Async() {
                        Ok(value) => {
                            if let Some(initialization) =
                                initializations.borrow_mut().get_mut(&loaded_node)
                            {
                                initialization._action = Some(value);
                            }
                        }
                        Err(error) => complete_webview_initialization(
                            &initializations,
                            loaded_node,
                            Err(native_error(error)),
                        ),
                    }) {
                        Ok(loaded) => Some(loaded),
                        Err(error) => {
                            _ = completion.call(Err(native_error(error)));
                            return Ok(());
                        }
                    }
                };
                self.webview_initializations.borrow_mut().insert(
                    *node,
                    WebViewInitialization {
                        _action: action,
                        _loaded: loaded,
                        _initialized: revoker,
                        completions: vec![completion.clone()],
                    },
                );
            }
            Command::ObserveSwapChainPanel {
                node,
                observation,
                callback,
            } => {
                let Some(Handle::SwapChainPanel(control)) = self.handles.get(node) else {
                    return Err(if self.contains(*node) {
                        RuntimeError::UnsupportedKind
                    } else {
                        RuntimeError::MissingNode(*node)
                    });
                };
                let element = control.cast::<IFrameworkElement>().map_err(native_error)?;
                let emit_metrics = |width: f64, height: f64| {
                    let event = SwapChainPanelEvent::Metrics {
                        width,
                        height,
                        scale_x: control.CompositionScaleX().unwrap_or(1.0),
                        scale_y: control.CompositionScaleY().unwrap_or(1.0),
                    };
                    invoke_callback(callback, event);
                };
                emit_metrics(
                    element.ActualWidth().unwrap_or(0.0),
                    element.ActualHeight().unwrap_or(0.0),
                );

                let size_callback = callback.clone();
                let size_control = control.clone();
                let size = element
                    .SizeChanged(move |_, args| {
                        if let Some(args) = args.as_ref()
                            && let Ok(value) = args.NewSize()
                        {
                            invoke_callback(
                                &size_callback,
                                SwapChainPanelEvent::Metrics {
                                    width: f64::from(value.width),
                                    height: f64::from(value.height),
                                    scale_x: size_control.CompositionScaleX().unwrap_or(1.0),
                                    scale_y: size_control.CompositionScaleY().unwrap_or(1.0),
                                },
                            );
                        }
                    })
                    .map_err(native_error)?;
                let scale_callback = callback.clone();
                let scale_element = element;
                let scale = control
                    .CompositionScaleChanged(move |sender, _| {
                        if let Some(sender) = sender.as_ref() {
                            invoke_callback(
                                &scale_callback,
                                SwapChainPanelEvent::Metrics {
                                    width: scale_element.ActualWidth().unwrap_or(0.0),
                                    height: scale_element.ActualHeight().unwrap_or(0.0),
                                    scale_x: sender.CompositionScaleX().unwrap_or(1.0),
                                    scale_y: sender.CompositionScaleY().unwrap_or(1.0),
                                },
                            );
                        }
                    })
                    .map_err(native_error)?;
                let rendering_callback = callback.clone();
                let rendering = CompositionTarget::Rendering(move |_, _| {
                    invoke_callback(&rendering_callback, SwapChainPanelEvent::Rendering);
                })
                .map_err(native_error)?;
                self.observation_subscriptions.insert(
                    (*node, *observation),
                    ObservationSubscription::SwapChainPanel {
                        _rendering: rendering,
                        _scale: scale,
                        _size: size,
                    },
                );
            }
            Command::SetSwapChain {
                node,
                swap_chain,
                completion,
            } => {
                let result = match self.handles.get(node) {
                    Some(Handle::SwapChainPanel(control)) => control
                        .cast::<ISwapChainPanelNative>()
                        .map_err(native_error)
                        .and_then(|native| {
                            let raw = swap_chain
                                .as_ref()
                                .map_or(std::ptr::null_mut(), Interface::as_raw);
                            unsafe { native.SetSwapChain(raw).ok().map_err(native_error) }
                        }),
                    Some(_) => Err(RuntimeError::UnsupportedKind),
                    None => Err(RuntimeError::MissingNode(*node)),
                };
                _ = completion.call(result);
            }
            Command::SetNativeImageSource {
                node,
                source,
                completion,
            } => {
                let control = match self.handles.get(node) {
                    Some(Handle::Image(control)) => Ok(control.clone()),
                    Some(_) => Err(RuntimeError::UnsupportedKind),
                    None => Err(RuntimeError::MissingNode(*node)),
                };
                let source = source
                    .as_ref()
                    .map(|source| source.cast::<ImageSource>().map_err(native_error))
                    .transpose();
                let result = control.and_then(|control| {
                    source.and_then(|source| {
                        self.release_encoded_image_source(*node);
                        control.SetSource(source.as_ref()).map_err(native_error)
                    })
                });
                _ = completion.call(result);
            }
            Command::ObserveImageScale {
                node,
                observation,
                callback,
            } => {
                let control = match self.handles.get(node) {
                    Some(Handle::Image(control)) => control,
                    Some(_) => return Err(RuntimeError::UnsupportedKind),
                    None => return Err(RuntimeError::MissingNode(*node)),
                };
                let element = control.cast::<UIElement>().map_err(native_error)?;
                let framework = control.cast::<IFrameworkElement>().map_err(native_error)?;
                let sink = self.event_sink()?;
                let scale_callback = callback.clone();
                let subscriptions =
                    subscribe_xaml_root_scale(&element, &framework, &sink, move |scale| {
                        invoke_callback(&scale_callback, scale);
                    })?;
                self.observation_subscriptions.insert(
                    (*node, *observation),
                    ObservationSubscription::ImageScale {
                        _root: subscriptions,
                    },
                );
            }
            Command::ObserveCompositionHost {
                node,
                observation,
                callback,
            } => {
                let control = match self.handles.get(node) {
                    Some(Handle::Grid(control)) => control,
                    Some(_) => return Err(RuntimeError::UnsupportedKind),
                    None => return Err(RuntimeError::MissingNode(*node)),
                };
                let element = control.cast::<UIElement>().map_err(native_error)?;
                let framework = control.cast::<IFrameworkElement>().map_err(native_error)?;
                let visual =
                    ElementCompositionPreview::GetElementVisual(&element).map_err(native_error)?;
                let compositor = visual
                    .cast::<ICompositionObject>()
                    .map_err(native_error)?
                    .Compositor()
                    .map_err(native_error)?;
                invoke_callback(
                    callback,
                    CompositionHostEvent::Ready {
                        compositor: compositor.into(),
                        width: framework.ActualWidth().unwrap_or(0.0),
                        height: framework.ActualHeight().unwrap_or(0.0),
                        scale: xaml_scale(&element)?,
                    },
                );

                let size_callback = callback.clone();
                let size_element = element.clone();
                let sink = self.event_sink()?;
                let size_sink = sink.clone();
                let size = framework
                    .SizeChanged(move |_, args| {
                        if let Some(args) = args.as_ref()
                            && let Ok(value) = args.NewSize()
                        {
                            let scale = match xaml_scale(&size_element) {
                                Ok(scale) => scale,
                                Err(error) => {
                                    size_sink.enqueue_host(HostEvent::Error(error));
                                    return;
                                }
                            };
                            invoke_callback(
                                &size_callback,
                                CompositionHostEvent::Metrics {
                                    width: f64::from(value.width),
                                    height: f64::from(value.height),
                                    scale,
                                },
                            );
                        }
                    })
                    .map_err(native_error)?;

                let scale_callback = callback.clone();
                let scale_framework = framework.clone();
                let root = subscribe_xaml_root_scale(&element, &framework, &sink, move |scale| {
                    invoke_callback(
                        &scale_callback,
                        CompositionHostEvent::Metrics {
                            width: scale_framework.ActualWidth().unwrap_or(0.0),
                            height: scale_framework.ActualHeight().unwrap_or(0.0),
                            scale,
                        },
                    );
                })?;
                self.observation_subscriptions.insert(
                    (*node, *observation),
                    ObservationSubscription::CompositionHost {
                        _root: root,
                        _size: size,
                    },
                );
            }
            Command::RevokeObservation { node, observation } => {
                self.observation_subscriptions
                    .remove(&(*node, *observation));
            }
            Command::SetCompositionChildVisual {
                node,
                visual,
                completion,
            } => {
                let result = match self.handles.get(node) {
                    Some(Handle::Grid(control)) => control
                        .cast::<UIElement>()
                        .map_err(native_error)
                        .and_then(|element| {
                            let visual = visual
                                .as_ref()
                                .map(|visual| visual.cast::<Visual>().map_err(native_error))
                                .transpose()?;
                            ElementCompositionPreview::SetElementChildVisual(
                                &element,
                                visual.as_ref(),
                            )
                            .map_err(native_error)
                        }),
                    Some(_) => Err(RuntimeError::UnsupportedKind),
                    None => Err(RuntimeError::MissingNode(*node)),
                };
                _ = completion.call(result);
            }
            Command::SetProperty {
                node,
                property,
                value,
            } => {
                if matches!(
                    property,
                    PropertyId::ImageSource | PropertyId::ImageIconSource
                ) {
                    self.release_encoded_image_source(*node);
                    if let PropertyValue::EncodedImage(value) = value {
                        self.encoded_image_nodes.borrow_mut().insert(*node);
                        let handle = self
                            .handles
                            .get(node)
                            .ok_or(RuntimeError::MissingNode(*node))?;
                        clear_property(handle, *property)?;
                        if let Err(error) = self.start_encoded_image(*node, value) {
                            self.report_image_decode_error(error);
                        }
                        return Ok(());
                    }
                }
                if matches!(
                    property,
                    PropertyId::ButtonKeyboardAccelerators | PropertyId::GridKeyboardAccelerators
                ) {
                    return match value {
                        PropertyValue::KeyAccelerators(value) => {
                            self.set_key_accelerators(*node, value)
                        }
                        _ => Err(RuntimeError::UnsupportedKind),
                    };
                }
                if *property == PropertyId::ButtonResources {
                    return match value {
                        PropertyValue::ResourceOverrides(value) => {
                            self.set_resource_overrides(*node, value)
                        }
                        _ => Err(RuntimeError::UnsupportedKind),
                    };
                }
                let target = PropertyTarget::resolve(self, *node, *property)?;
                let feedback = expected_feedback(*property, Some(value));
                let selection_owner = self
                    .selection_owners
                    .get(node)
                    .copied()
                    .filter(|(_, slot)| selection_for_item_property(*property, *slot).is_some());
                let feedback_event = feedback.as_ref().map(|(event, _)| *event);
                if let Some((event, expectation)) = feedback {
                    self.feedback
                        .borrow_mut()
                        .insert((*node, event), expectation);
                }
                let (result, observation) = self.with_selection_suppressed(selection_owner, || {
                    let result = if *property == PropertyId::BorderCapturePointerOnPress {
                        match value {
                            PropertyValue::Bool(true) => self.ui_element(*node).map(|_| {
                                self.pointer_capture.borrow_mut().insert(*node, true);
                            }),
                            PropertyValue::Bool(false) => self
                                .ui_element(*node)
                                .and_then(|element| {
                                    element.ReleasePointerCaptures().map_err(native_error)
                                })
                                .map(|_| {
                                    self.pointer_capture.borrow_mut().remove(node);
                                }),
                            _ => Err(RuntimeError::UnsupportedKind),
                        }
                    } else if *property == PropertyId::BorderAllowDrop {
                        match value {
                            PropertyValue::DragDropPolicy(policy) => self
                                .ui_element(*node)
                                .and_then(|element| {
                                    element
                                        .SetAllowDrop(
                                            policy.accepts(DragKind::StorageItems)
                                                || policy.accepts(DragKind::Text),
                                        )
                                        .map_err(native_error)
                                })
                                .map(|_| {
                                    self.drop_policies
                                        .borrow_mut()
                                        .insert(*node, policy.clone());
                                }),
                            _ => Err(RuntimeError::UnsupportedKind),
                        }
                    } else {
                        target.set(*property, value)
                    };
                    let observation =
                        feedback_event.and_then(|event| {
                            self.feedback.borrow_mut().remove(&(*node, event)).and_then(
                                |expectation| match expectation {
                                    FeedbackExpectation::Normalized { observation } => observation,
                                    FeedbackExpectation::Exact(_)
                                    | FeedbackExpectation::Suppressed => None,
                                },
                            )
                        });
                    (result, observation)
                });
                result?;
                if controlled_collection_for_property(*property).is_some()
                    && let PropertyValue::I32(value) = value
                {
                    self.controlled_collection_indices.insert(*node, *value);
                }
                if let Some(observation) = observation {
                    self.events.borrow_mut().push(NativeWork {
                        identity: self.identity.get().unwrap(),
                        work: observation,
                    });
                    self.schedule_dispatch()?;
                }
            }
            Command::ClearProperty { node, property } => {
                if matches!(
                    property,
                    PropertyId::ImageSource | PropertyId::ImageIconSource
                ) {
                    self.release_encoded_image_source(*node);
                }
                if matches!(
                    property,
                    PropertyId::ButtonKeyboardAccelerators | PropertyId::GridKeyboardAccelerators
                ) {
                    return self.set_key_accelerators(*node, &KeyAccelerators::default());
                }
                if *property == PropertyId::ButtonResources {
                    return self.clear_resource_overrides(*node);
                }
                let target = PropertyTarget::resolve(self, *node, *property)?;
                let feedback = expected_feedback(*property, None);
                let selection_owner = self
                    .selection_owners
                    .get(node)
                    .copied()
                    .filter(|(_, slot)| selection_for_item_property(*property, *slot).is_some());
                let feedback_event = feedback.as_ref().map(|(event, _)| *event);
                if let Some((event, expectation)) = feedback {
                    self.feedback
                        .borrow_mut()
                        .insert((*node, event), expectation);
                }
                let result = self.with_selection_suppressed(selection_owner, || {
                    let result = if *property == PropertyId::BorderCapturePointerOnPress {
                        self.ui_element(*node)?
                            .ReleasePointerCaptures()
                            .map_err(native_error)?;
                        self.pointer_capture.borrow_mut().remove(node);
                        Ok(())
                    } else if *property == PropertyId::BorderAllowDrop {
                        self.ui_element(*node)?
                            .SetAllowDrop(false)
                            .map_err(native_error)?;
                        self.drop_policies.borrow_mut().remove(node);
                        Ok(())
                    } else {
                        target.clear(*property)
                    };
                    if let Some(event) = feedback_event {
                        self.feedback.borrow_mut().remove(&(*node, event));
                    }
                    result
                });
                result?;
                if controlled_collection_for_property(*property).is_some() {
                    self.controlled_collection_indices.remove(node);
                }
            }
            Command::SubscribeEvent {
                node,
                event,
                revision,
            } => {
                let content_dialog_closed = *event == EventId::ContentDialogClosed
                    && self.content_dialogs.borrow().contains(*node);
                if self.subscriptions.contains_key(&(*node, *event))
                    || content_dialog_closed
                        && self.content_dialogs.borrow().has_subscription(*node)
                {
                    return Err(RuntimeError::DuplicateEvent(*node, *event));
                }
                let handle = self
                    .handles
                    .get(node)
                    .ok_or(RuntimeError::MissingNode(*node))?;
                let sink = self.event_sink()?;
                let revoker = subscribe_event(handle, *node, *event, *revision, sink)?;
                if content_dialog_closed {
                    self.content_dialogs
                        .borrow_mut()
                        .subscribe(*node, *revision, revoker)?;
                } else {
                    self.subscriptions.insert((*node, *event), revoker);
                }
            }
            Command::UnsubscribeEvent { node, event } => {
                if *event == EventId::ContentDialogClosed
                    && self
                        .content_dialogs
                        .borrow_mut()
                        .unsubscribe(*node, *event)?
                {
                    return Ok(());
                }
                let revoker = self
                    .subscriptions
                    .remove(&(*node, *event))
                    .ok_or(RuntimeError::MissingSubscription(*node, *event))?;
                drop(revoker);
            }
            Command::SetSlot {
                parent,
                slot,
                child,
            } => {
                let child = child.map(|child| self.ui_element(child)).transpose()?;
                let handle = self
                    .handles
                    .get(parent)
                    .ok_or(RuntimeError::MissingNode(*parent))?;
                set_slot(handle, *slot, child.as_ref())?;
            }
            Command::SetTooltip {
                target,
                tooltip,
                placement,
            } => {
                let target = self
                    .ui_element(*target)?
                    .cast::<DependencyObject>()
                    .map_err(native_error)?;
                if let Some(tooltip) = tooltip {
                    ToolTipService::SetToolTip(&target, &self.ui_element(*tooltip)?)
                        .map_err(native_error)?;
                } else {
                    ToolTipService::SetToolTip(&target, None::<&windows_core::IInspectable>)
                        .map_err(native_error)?;
                }
                ToolTipService::SetPlacement(
                    &target,
                    match placement {
                        TooltipPlacement::Top => PlacementMode::Top,
                        TooltipPlacement::Bottom => PlacementMode::Bottom,
                        TooltipPlacement::Left => PlacementMode::Left,
                        TooltipPlacement::Right => PlacementMode::Right,
                        TooltipPlacement::Mouse => PlacementMode::Mouse,
                    },
                )
                .map_err(native_error)?;
            }
            Command::SetFlyout {
                target,
                content,
                placement,
            } => {
                let target_handle = self
                    .handles
                    .get(target)
                    .ok_or(RuntimeError::MissingNode(*target))?;
                if let Some(content) = content {
                    if self
                        .flyouts
                        .iter()
                        .any(|(owner, (_, owned))| owner != target && owned == content)
                    {
                        return Err(RuntimeError::AlreadyParented(*content));
                    }
                    let content_element = self.ui_element(*content)?;
                    let flyout = if let Some((flyout, previous_content)) = self.flyouts.get(target)
                    {
                        if previous_content != content {
                            flyout
                                .SetContent(None::<&UIElement>)
                                .map_err(native_error)?;
                        }
                        flyout.clone()
                    } else {
                        let flyout = bindings::Flyout::new().map_err(native_error)?;
                        match &target_handle {
                            Handle::Button(target) => target.SetFlyout(&flyout),
                            Handle::SplitButton(target) => target.SetFlyout(&flyout),
                            _ => return Err(RuntimeError::UnsupportedKind),
                        }
                        .map_err(native_error)?;
                        flyout
                    };
                    flyout.SetContent(&content_element).map_err(native_error)?;
                    flyout
                        .cast::<IFlyoutBase>()
                        .and_then(|flyout| {
                            flyout.SetPlacement(match placement {
                                FlyoutPlacement::Top => FlyoutPlacementMode::Top,
                                FlyoutPlacement::Bottom => FlyoutPlacementMode::Bottom,
                                FlyoutPlacement::Left => FlyoutPlacementMode::Left,
                                FlyoutPlacement::Right => FlyoutPlacementMode::Right,
                                FlyoutPlacement::Full => FlyoutPlacementMode::Full,
                                FlyoutPlacement::TopEdgeAlignedLeft => {
                                    FlyoutPlacementMode::TopEdgeAlignedLeft
                                }
                                FlyoutPlacement::TopEdgeAlignedRight => {
                                    FlyoutPlacementMode::TopEdgeAlignedRight
                                }
                                FlyoutPlacement::BottomEdgeAlignedLeft => {
                                    FlyoutPlacementMode::BottomEdgeAlignedLeft
                                }
                                FlyoutPlacement::BottomEdgeAlignedRight => {
                                    FlyoutPlacementMode::BottomEdgeAlignedRight
                                }
                                FlyoutPlacement::LeftEdgeAlignedTop => {
                                    FlyoutPlacementMode::LeftEdgeAlignedTop
                                }
                                FlyoutPlacement::LeftEdgeAlignedBottom => {
                                    FlyoutPlacementMode::LeftEdgeAlignedBottom
                                }
                                FlyoutPlacement::RightEdgeAlignedTop => {
                                    FlyoutPlacementMode::RightEdgeAlignedTop
                                }
                                FlyoutPlacement::RightEdgeAlignedBottom => {
                                    FlyoutPlacementMode::RightEdgeAlignedBottom
                                }
                                FlyoutPlacement::Auto => FlyoutPlacementMode::Auto,
                            })
                        })
                        .map_err(native_error)?;
                    self.flyouts.insert(*target, (flyout, *content));
                } else {
                    if let Some((flyout, _)) = self.flyouts.get(target) {
                        flyout
                            .SetContent(None::<&UIElement>)
                            .map_err(native_error)?;
                    }
                    match &target_handle {
                        Handle::Button(target) => target.SetFlyout(None::<&FlyoutBase>),
                        Handle::SplitButton(target) => target.SetFlyout(None::<&FlyoutBase>),
                        _ => return Err(RuntimeError::UnsupportedKind),
                    }
                    .map_err(native_error)?;
                    self.flyouts.remove(target);
                }
            }
            Command::SetOwnedMenu {
                owner,
                target,
                kind,
                items,
                revision,
            } => {
                if let Some(previous) = self.owned_menus.remove(owner) {
                    match previous.kind {
                        OwnedMenuKind::ButtonFlyout | OwnedMenuKind::DropDownButtonFlyout => {
                            let button = self
                                .ui_element(previous.target)?
                                .cast::<IButton>()
                                .map_err(native_error)?;
                            button
                                .SetFlyout(None::<&FlyoutBase>)
                                .map_err(native_error)?;
                        }
                        OwnedMenuKind::MenuBarItem => {
                            let Some(Handle::MenuBarItem(item)) =
                                self.handles.get(&previous.target)
                            else {
                                return Err(RuntimeError::UnsupportedKind);
                            };
                            item.Items()
                                .and_then(|items| items.Clear())
                                .map_err(native_error)?;
                        }
                    }
                }
                let Some(items) = items else {
                    return Ok(());
                };
                let sink = self.event_sink()?;
                let mut revokers = Vec::new();
                let flyout = match kind {
                    OwnedMenuKind::ButtonFlyout | OwnedMenuKind::DropDownButtonFlyout => {
                        let flyout = MenuFlyout::new().map_err(native_error)?;
                        build_menu_items(
                            items,
                            *owner,
                            *revision,
                            &sink,
                            &flyout.Items().map_err(native_error)?,
                            &mut revokers,
                        )?;
                        self.ui_element(*target)?
                            .cast::<IButton>()
                            .and_then(|button| button.SetFlyout(&flyout))
                            .map_err(native_error)?;
                        Some(flyout)
                    }
                    OwnedMenuKind::MenuBarItem => {
                        let Some(Handle::MenuBarItem(item)) = self.handles.get(target) else {
                            return Err(RuntimeError::UnsupportedKind);
                        };
                        build_menu_items(
                            items,
                            *owner,
                            *revision,
                            &sink,
                            &item.Items().map_err(native_error)?,
                            &mut revokers,
                        )?;
                        None
                    }
                };
                self.owned_menus.insert(
                    *owner,
                    NativeOwnedMenu {
                        target: *target,
                        kind: *kind,
                        _flyout: flyout,
                        _revokers: revokers,
                    },
                );
            }
            Command::SetCommandBarFlyout {
                owner,
                target,
                primary,
                secondary,
                revision,
            } => {
                if let Some(previous) = self.command_bar_flyouts.remove(owner) {
                    self.ui_element(previous.target)?
                        .cast::<IButton>()
                        .and_then(|button| button.SetFlyout(None::<&FlyoutBase>))
                        .map_err(native_error)?;
                }
                let Some(primary) = primary else {
                    return Ok(());
                };
                let sink = self.event_sink()?;
                let flyout = bindings::CommandBarFlyout::new().map_err(native_error)?;
                let primary_items = flyout.PrimaryCommands().map_err(native_error)?;
                let secondary_items = flyout.SecondaryCommands().map_err(native_error)?;
                let mut revokers = Vec::new();
                for command in primary {
                    let (command, revoker) =
                        build_command_bar_element(command, *owner, *revision, &sink)?;
                    primary_items.Append(&command).map_err(native_error)?;
                    revokers.extend(revoker);
                }
                for command in secondary {
                    let (command, revoker) =
                        build_command_bar_element(command, *owner, *revision, &sink)?;
                    secondary_items.Append(&command).map_err(native_error)?;
                    revokers.extend(revoker);
                }
                self.ui_element(*target)?
                    .cast::<IButton>()
                    .and_then(|button| button.SetFlyout(&flyout))
                    .map_err(native_error)?;
                self.command_bar_flyouts.insert(
                    *owner,
                    NativeCommandBarFlyout {
                        target: *target,
                        _flyout: flyout,
                        _revokers: revokers,
                    },
                );
            }
            Command::SetTreeViewNodes { target, nodes } => {
                let Some(Handle::TreeView(tree)) = self.handles.get(target) else {
                    return Err(RuntimeError::UnsupportedKind);
                };
                let roots = tree.RootNodes().map_err(native_error)?;
                roots.Clear().map_err(native_error)?;
                for node in nodes {
                    let node = build_tree_node(node)?;
                    roots.Append(&node).map_err(native_error)?;
                }
            }
            Command::SetContentDialogOpen { node, owner, open } => {
                let owner = if *open {
                    Some(
                        self.ui_element(*owner)?
                            .cast::<IUIElement>()
                            .map_err(native_error)?,
                    )
                } else {
                    None
                };
                let xaml_root = match owner.as_ref().map(IUIElement::XamlRoot).transpose() {
                    Ok(root) => root,
                    Err(error) if error.code().is_ok() => None,
                    Err(error) => return Err(native_error(error)),
                };
                let action = self
                    .content_dialogs
                    .borrow_mut()
                    .set_open(*node, *open, xaml_root)?;
                match action {
                    ContentDialogAction::None => {}
                    ContentDialogAction::WaitForRoot(generation) => {
                        let sink = self.event_sink()?;
                        let loaded_owner = owner.unwrap();
                        let owner = loaded_owner
                            .cast::<IFrameworkElement>()
                            .map_err(native_error)?;
                        let node_id = *node;
                        let loaded = owner
                            .Loaded(move |_, _| match loaded_owner.XamlRoot() {
                                Ok(root) => {
                                    if let Err(error) =
                                        sink.content_dialog_root_ready(node_id, generation, root)
                                    {
                                        sink.error(node_id, EventId::ContentDialogClosed, 0, error);
                                    }
                                }
                                Err(error) if error.code().is_ok() => {}
                                Err(error) => sink.error(
                                    node_id,
                                    EventId::ContentDialogClosed,
                                    0,
                                    native_error(error),
                                ),
                            })
                            .map_err(native_error)?;
                        self.content_dialogs
                            .borrow_mut()
                            .set_root_loaded(*node, loaded)?;
                    }
                    ContentDialogAction::Hide(dialog) => {
                        dialog.Hide().map_err(native_error)?;
                    }
                }
            }
            Command::InsertChild {
                parent,
                slot,
                child,
                index,
            } => match slot {
                Some(slot) => self.insert_slot_child(*parent, *slot, *child, *index)?,
                None => self.insert_child(*parent, *child, *index)?,
            },
            Command::RemoveChild {
                parent,
                slot,
                child,
            } => match slot {
                Some(slot) => self.remove_slot_child(*parent, *slot, *child)?,
                None => self.remove_child(*parent, *child)?,
            },
            Command::SynchronizeChildren {
                parent,
                slot,
                children,
            } => match slot {
                Some(slot) => self.synchronize_slot_children(*parent, *slot, children)?,
                None => self.synchronize_children(*parent, children)?,
            },
            Command::MoveChild {
                parent,
                slot,
                child,
                index,
            } => match slot {
                Some(slot) => self.move_slot_child(*parent, *slot, *child, *index)?,
                None => self.move_child(*parent, *child, *index)?,
            },
        }
        Ok(())
    }

    fn contains(&self, node: NodeId) -> bool {
        self.handles.contains_key(&node)
            || self.virtuals.contains_key(&node)
            || self.windows.contains_key(&node)
            || self
                .application
                .as_ref()
                .is_some_and(|(application, _)| *application == node)
    }

    fn window_children(
        &self,
        node: NodeId,
    ) -> Result<Option<(&bindings::Grid, UIElementCollection)>, RuntimeError> {
        if !self.windows.contains_key(&node) {
            return Ok(None);
        }
        let host = self
            .window_hosts
            .get(&node)
            .ok_or(RuntimeError::MissingNode(node))?;
        let children = host
            .cast::<IPanel>()
            .and_then(|host| host.Children())
            .map_err(native_error)?;
        Ok(Some((host, children)))
    }

    fn insert_child(
        &mut self,
        parent: NodeId,
        child: NodeId,
        index: usize,
    ) -> Result<(), RuntimeError> {
        let parent_id = parent;
        let child = self.ui_element(child)?;
        if self.windows.contains_key(&parent) {
            if index != 0 {
                return Err(RuntimeError::IndexOutOfBounds);
            }
            let (host, children) = self.window_children(parent)?.unwrap();
            children.Append(&child).map_err(native_error)?;
            if let Some(visuals) = self.window_visuals.get(&parent) {
                Self::apply_window_theme(host, visuals.theme)?;
            }
            if let Some(observations) = self.window_observations.get(&parent).copied() {
                self.set_window_observations(parent, observations)?;
            }
            Ok(())
        } else {
            let parent = self
                .handles
                .get(&parent)
                .ok_or(RuntimeError::MissingNode(parent))?;
            if parent.is_content() {
                if index != 0 {
                    return Err(RuntimeError::IndexOutOfBounds);
                }
                set_content(parent, Some(&child))
            } else if let Some(children) = parent.child_collection()? {
                let retained = self.retained_identities(parent_id, None)?;
                let index = if retained.is_empty() {
                    index
                } else {
                    let current = (0..children.Size().map_err(native_error)?)
                        .map(|index| {
                            children
                                .GetAt(index)
                                .map_err(native_error)
                                .and_then(|child| com_identity(&child))
                        })
                        .collect::<Result<Vec<_>, RuntimeError>>()?;
                    physical_retained_index(&current, &retained, index)?
                };
                children
                    .InsertAt(index32(index)?, &child)
                    .map_err(native_error)
            } else {
                Err(RuntimeError::UnsupportedKind)
            }
        }
    }

    fn insert_slot_child(
        &mut self,
        parent: NodeId,
        slot: SlotId,
        child: NodeId,
        index: usize,
    ) -> Result<(), RuntimeError> {
        let parent_id = parent;
        let child_id = child;
        let parent = self
            .handles
            .get(&parent)
            .ok_or(RuntimeError::MissingNode(parent))?;
        let collection = slot_collection(parent, slot)?;
        let child: windows_core::IInspectable = self.ui_element(child)?.into();
        let selection = selection_for_slot(slot);
        let retained = self.retained_identities(parent_id, Some(slot))?;
        let current = (0..collection.Size()?)
            .map(|index| {
                let child = collection.GetAt(index)?;
                com_identity(&child)
            })
            .collect::<Result<Vec<_>, RuntimeError>>()?;
        let index = physical_retained_index(&current, &retained, index)?;
        let result = self.with_controlled_collection_preserved(parent_id, parent, slot, || {
            self.with_selection_suppressed(selection.map(|_| (parent_id, slot)), || {
                collection.InsertAt(index32(index)?, &child)?;
                if let Some(selection) = selection
                    && selection_item_is_selected(selection, &child)?
                {
                    set_selected_item(parent, selection, &child)?;
                }
                Ok(())
            })
        });
        if result.is_ok() && selection.is_some() {
            self.selection_owners.insert(child_id, (parent_id, slot));
            self.selection_items.borrow_mut().push((child_id, child));
        }
        result
    }

    fn remove_slot_child(
        &mut self,
        parent: NodeId,
        slot: SlotId,
        child: NodeId,
    ) -> Result<(), RuntimeError> {
        let child_id = child;
        let parent_id = parent;
        let parent = self
            .handles
            .get(&parent)
            .ok_or(RuntimeError::MissingNode(parent))?;
        let collection = slot_collection(parent, slot)?;
        let child: windows_core::IInspectable = self.ui_element(child)?.into();
        let selection = selection_for_slot(slot);
        let result = self.with_controlled_collection_preserved(parent_id, parent, slot, || {
            self.with_selection_suppressed(selection.map(|_| (parent_id, slot)), || {
                let index = inspectable_child_index(&collection, child_id, &child)?;
                collection.RemoveAt(index)
            })
        });
        if result.is_ok() {
            self.selection_owners.remove(&child_id);
            self.selection_items
                .borrow_mut()
                .retain(|(node, _)| *node != child_id);
        }
        result
    }

    fn move_slot_child(
        &mut self,
        parent: NodeId,
        slot: SlotId,
        child: NodeId,
        index: usize,
    ) -> Result<(), RuntimeError> {
        let child_id = child;
        let parent_id = parent;
        let parent = self
            .handles
            .get(&parent)
            .ok_or(RuntimeError::MissingNode(parent))?;
        let selection = selection_for_slot(slot);
        let selected = selection
            .map(|selection| selected_item(parent, selection))
            .transpose()?
            .flatten();
        let collection = slot_collection(parent, slot)?;
        let child: windows_core::IInspectable = self.ui_element(child)?.into();
        let restore_selection = match selection {
            Some(selection) => {
                selected.as_ref() == Some(&child) || selection_item_is_selected(selection, &child)?
            }
            None => false,
        };
        let child_identity = com_identity(&child)?;
        let retained = self.retained_identities(parent_id, Some(slot))?;
        let current = (0..collection.Size()?)
            .map(|index| {
                let item = collection.GetAt(index)?;
                com_identity(&item)
            })
            .collect::<Result<Vec<_>, RuntimeError>>()?
            .into_iter()
            .filter(|identity| *identity != child_identity)
            .collect::<Vec<_>>();
        let index = physical_retained_index(&current, &retained, index)?;
        self.with_controlled_collection_preserved(parent_id, parent, slot, || {
            self.with_selection_suppressed(selection.map(|_| (parent_id, slot)), || {
                let from = inspectable_child_index(&collection, child_id, &child)?;
                collection.RemoveAt(from)?;
                collection.InsertAt(index32(index)?, &child)?;
                if restore_selection {
                    set_selected_item(parent, selection.unwrap(), &child)?;
                }
                Ok(())
            })
        })
    }

    fn synchronize_slot_children(
        &mut self,
        parent: NodeId,
        slot: SlotId,
        children: &[NodeId],
    ) -> Result<(), RuntimeError> {
        let parent_id = parent;
        let parent = self
            .handles
            .get(&parent)
            .ok_or(RuntimeError::MissingNode(parent))?;
        let selection = selection_for_slot(slot);
        let selected = selection
            .map(|selection| selected_item(parent, selection))
            .transpose()?
            .flatten();
        let collection = slot_collection(parent, slot)?;
        let desired = children
            .iter()
            .map(|child| {
                let item: windows_core::IInspectable = self.ui_element(*child)?.into();
                Ok((*child, com_identity(&item)?, item))
            })
            .collect::<Result<Vec<_>, RuntimeError>>()?;
        let current = (0..collection.Size()?)
            .map(|index| {
                let item = collection.GetAt(index)?;
                Ok((com_identity(&item)?, item))
            })
            .collect::<Result<Vec<_>, RuntimeError>>()?;
        let current_ids = current
            .iter()
            .map(|(identity, _)| *identity)
            .collect::<Vec<_>>();
        let desired_ids = desired
            .iter()
            .map(|(_, identity, _)| *identity)
            .collect::<Vec<_>>();
        let retiring = self.retained_identities(parent_id, Some(slot))?;
        let target_ids = merge_retained_identities(&current_ids, &desired_ids, &retiring);
        let target = target_ids
            .iter()
            .map(|identity| {
                desired
                    .iter()
                    .find(|(_, candidate, _)| candidate == identity)
                    .map(|(_, _, item)| (*identity, item.clone()))
                    .or_else(|| {
                        current
                            .iter()
                            .find(|(candidate, _)| candidate == identity)
                            .map(|(_, item)| (*identity, item.clone()))
                    })
                    .unwrap()
            })
            .collect::<Vec<_>>();
        let retained = retained_subsequence(&current_ids, &target_ids);
        let restore_selection = selected.as_ref().is_some_and(|selected| {
            current
                .iter()
                .any(|(identity, item)| !retained.contains(identity) && item == selected)
                && desired.iter().any(|(_, _, item)| item == selected)
        });

        let result = self.with_controlled_collection_preserved(parent_id, parent, slot, || {
            self.with_selection_suppressed(selection.map(|_| (parent_id, slot)), || {
                for (index, (identity, _)) in current.iter().enumerate().rev() {
                    if !retained.contains(identity) {
                        collection.RemoveAt(index32(index)?)?;
                    }
                }
                for (index, (identity, child)) in target.iter().enumerate() {
                    if !retained.contains(identity) {
                        collection.InsertAt(index32(index)?, child)?;
                    }
                }
                if restore_selection && let Some(selected) = selected.as_ref() {
                    set_selected_item(parent, selection.unwrap(), selected)?;
                } else {
                    for (_, _, child) in &desired {
                        if let Some(selection) = selection
                            && selection_item_is_selected(selection, child)?
                        {
                            set_selected_item(parent, selection, child)?;
                            break;
                        }
                    }
                }
                Ok(())
            })
        });
        if result.is_ok() && selection.is_some() {
            let desired_nodes = children.iter().copied().collect::<HashSet<_>>();
            self.selection_items.borrow_mut().retain(|(child, _)| {
                self.selection_owners.get(child) != Some(&(parent_id, slot))
                    || desired_nodes.contains(child)
            });
            self.selection_owners.retain(|child, owner| {
                *owner != (parent_id, slot) || desired_nodes.contains(child)
            });
            let mut known = self
                .selection_items
                .borrow()
                .iter()
                .map(|(node, _)| *node)
                .collect::<HashSet<_>>();
            for (child, _, item) in desired {
                self.selection_owners.insert(child, (parent_id, slot));
                if known.insert(child) {
                    self.selection_items.borrow_mut().push((child, item));
                }
            }
        }
        result
    }

    fn remove_child(&mut self, parent: NodeId, child: NodeId) -> Result<(), RuntimeError> {
        let child_id = child;
        let child = self.ui_element(child)?;
        if let Some((_, children)) = self.window_children(parent)? {
            children.Clear().map_err(native_error)?;
            Ok(())
        } else {
            let parent = self
                .handles
                .get(&parent)
                .ok_or(RuntimeError::MissingNode(parent))?;
            if parent.is_content() {
                set_content(parent, None)
            } else if let Some(children) = parent.child_collection()? {
                let index = child_index(&children, child_id, &child)?;
                children.RemoveAt(index).map_err(native_error)
            } else {
                Err(RuntimeError::UnsupportedKind)
            }
        }
    }

    fn reset_children(&mut self, parent: NodeId) -> Result<(), RuntimeError> {
        if let Some((_, children)) = self.window_children(parent)? {
            children.Clear().map_err(native_error)?;
            Ok(())
        } else {
            let parent = self
                .handles
                .get(&parent)
                .ok_or(RuntimeError::MissingNode(parent))?;
            if parent.is_content() {
                set_content(parent, None)
            } else if let Some(children) = parent.child_collection()? {
                children.Clear().map_err(native_error)
            } else {
                Err(RuntimeError::UnsupportedKind)
            }
        }
    }

    fn synchronize_children(
        &mut self,
        parent: NodeId,
        children: &[NodeId],
    ) -> Result<(), RuntimeError> {
        if self.windows.contains_key(&parent) {
            return match children {
                [] => self.reset_children(parent),
                [child] => {
                    self.reset_children(parent)?;
                    self.insert_child(parent, *child, 0)
                }
                _ => Err(RuntimeError::IndexOutOfBounds),
            };
        }
        if self.handles.get(&parent).is_some_and(Handle::is_content) {
            return match children {
                [] => self.reset_children(parent),
                [child] => self.insert_child(parent, *child, 0),
                _ => Err(RuntimeError::IndexOutOfBounds),
            };
        }

        let parent_handle = self
            .handles
            .get(&parent)
            .ok_or(RuntimeError::MissingNode(parent))?;
        let collection = parent_handle
            .child_collection()?
            .ok_or(RuntimeError::UnsupportedKind)?;
        let desired = children
            .iter()
            .map(|child| {
                let item = self.ui_element(*child)?;
                Ok((com_identity(&item)?, item))
            })
            .collect::<Result<Vec<_>, RuntimeError>>()?;
        let current = (0..collection.Size().map_err(native_error)?)
            .map(|index| {
                let item = collection.GetAt(index).map_err(native_error)?;
                Ok((com_identity(&item)?, item))
            })
            .collect::<Result<Vec<_>, RuntimeError>>()?;
        let current_ids = current
            .iter()
            .map(|(identity, _)| *identity)
            .collect::<Vec<_>>();
        let desired_ids = desired
            .iter()
            .map(|(identity, _)| *identity)
            .collect::<Vec<_>>();
        let retiring = self.retained_identities(parent, None)?;
        let target_ids = merge_retained_identities(&current_ids, &desired_ids, &retiring);
        let target = target_ids
            .iter()
            .map(|identity| {
                desired
                    .iter()
                    .find(|(candidate, _)| candidate == identity)
                    .or_else(|| current.iter().find(|(candidate, _)| candidate == identity))
                    .cloned()
                    .unwrap()
            })
            .collect::<Vec<_>>();
        let retained = retained_subsequence(&current_ids, &target_ids);
        for (index, (identity, _)) in current.iter().enumerate().rev() {
            if !retained.contains(identity) {
                collection.RemoveAt(index32(index)?).map_err(native_error)?;
            }
        }
        for (index, (identity, child)) in target.iter().enumerate() {
            if !retained.contains(identity) {
                collection
                    .InsertAt(index32(index)?, child)
                    .map_err(native_error)?;
            }
        }
        Ok(())
    }

    fn move_child(&self, parent: NodeId, child: NodeId, index: usize) -> Result<(), RuntimeError> {
        let parent_id = parent;
        let child_id = child;
        let child = self.ui_element(child)?;
        if self.windows.contains_key(&parent) {
            if index != 0 {
                return Err(RuntimeError::IndexOutOfBounds);
            }
            Ok(())
        } else {
            let parent = self
                .handles
                .get(&parent)
                .ok_or(RuntimeError::MissingNode(parent))?;
            if parent.is_content() {
                if index == 0 {
                    Ok(())
                } else {
                    Err(RuntimeError::IndexOutOfBounds)
                }
            } else if let Some(children) = parent.child_collection()? {
                let from = child_index(&children, child_id, &child)?;
                let retained = self.retained_identities(parent_id, None)?;
                let index = if retained.is_empty() {
                    index
                } else {
                    let child_identity = com_identity(&child)?;
                    let current = (0..children.Size().map_err(native_error)?)
                        .map(|current| {
                            children
                                .GetAt(current)
                                .map_err(native_error)
                                .and_then(|item| com_identity(&item))
                        })
                        .collect::<Result<Vec<_>, RuntimeError>>()?
                        .into_iter()
                        .filter(|identity| *identity != child_identity)
                        .collect::<Vec<_>>();
                    physical_retained_index(&current, &retained, index)?
                };
                let index = index32(index)?;
                children
                    .cast::<IUIElementCollection>()
                    .and_then(|children| children.Move(from, index))
                    .map_err(native_error)
            } else {
                Err(RuntimeError::UnsupportedKind)
            }
        }
    }

    fn ui_element(&self, node: NodeId) -> Result<UIElement, RuntimeError> {
        if let Some(handle) = self.handles.get(&node) {
            handle.ui_element().map_err(native_error)
        } else if let Some(handle) = self.virtuals.get(&node) {
            handle.ui_element().map_err(native_error)
        } else {
            Err(RuntimeError::MissingNode(node))
        }
    }

    fn retained_identities(
        &self,
        parent: NodeId,
        slot: Option<SlotId>,
    ) -> Result<HashSet<usize>, RuntimeError> {
        self.retained_subtrees
            .iter()
            .filter(|(_, retained)| retained.parent == parent && retained.slot == slot)
            .map(|(root, _)| self.ui_element(*root).and_then(|root| com_identity(&root)))
            .collect()
    }

    fn event_sink(&self) -> Result<EventSink, RuntimeError> {
        let dispatcher = DispatcherQueue::GetForCurrentThread().map_err(native_error)?;
        let identity = self.identity.get().unwrap();
        Ok(EventSink {
            queue: Rc::clone(&self.events),
            errors: Rc::clone(&self.event_errors),
            host_events: Rc::clone(&self.host_events),
            async_ingress: Arc::clone(&self.async_ingress),
            async_state: Rc::clone(&self.async_state),
            drop_policies: Rc::clone(&self.drop_policies),
            encoded_image_nodes: Rc::clone(&self.encoded_image_nodes),
            feedback: Rc::clone(&self.feedback),
            content_dialogs: Rc::clone(&self.content_dialogs),
            pointer_capture: Rc::clone(&self.pointer_capture),
            selection_items: Rc::clone(&self.selection_items),
            dispatcher,
            identity,
            current_identity: Rc::clone(&self.identity),
            scheduler: Rc::clone(&self.scheduler),
        })
    }

    pub fn schedule_dispatch(&self) -> Result<(), RuntimeError> {
        self.event_sink()?.request(WorkPriority::Low)
    }

    pub fn close_scheduler(&self) {
        self.scheduler.borrow_mut().close();
    }

    fn with_controlled_collection_preserved(
        &self,
        owner: NodeId,
        handle: &Handle,
        slot: SlotId,
        apply: impl FnOnce() -> Result<(), RuntimeError>,
    ) -> Result<(), RuntimeError> {
        let Some(collection) = controlled_collection_for_slot(slot) else {
            return apply();
        };
        self.feedback
            .borrow_mut()
            .insert((owner, collection.event), FeedbackExpectation::Suppressed);
        let result = apply().and_then(|()| {
            let Some(index) = self.controlled_collection_indices.get(&owner).copied() else {
                return Ok(());
            };
            if index >= 0 {
                let size = slot_collection(handle, slot)?.Size()?;
                if u32::try_from(index).unwrap() >= size {
                    return Ok(());
                }
            }
            set_property(handle, collection.property, &PropertyValue::I32(index))
        });
        self.feedback
            .borrow_mut()
            .remove(&(owner, collection.event));
        result
    }

    fn with_selection_suppressed<T>(
        &self,
        owner: Option<(NodeId, SlotId)>,
        apply: impl FnOnce() -> T,
    ) -> T {
        let selection = if let Some((owner, slot)) = owner
            && let Some(selection) = selection_for_slot(slot)
        {
            self.feedback
                .borrow_mut()
                .insert((owner, selection.event), FeedbackExpectation::Suppressed);
            Some((owner, selection.event))
        } else {
            None
        };
        let result = apply();
        if let Some(selection) = selection {
            self.feedback.borrow_mut().remove(&selection);
        }
        result
    }
}

#[derive(Clone)]
pub struct EventSink {
    queue: Rc<RefCell<Vec<NativeWork<QueuedEvent>>>>,
    errors: Rc<RefCell<Vec<NativeWork<QueuedEventError>>>>,
    host_events: Rc<RefCell<Vec<NativeWork<HostEvent>>>>,
    async_ingress: Arc<Mutex<AsyncIngressQueue>>,
    async_state: Rc<RefCell<AsyncIngressState>>,
    drop_policies: Rc<RefCell<HashMap<NodeId, DragDropPolicy>>>,
    encoded_image_nodes: Rc<RefCell<HashSet<NodeId>>>,
    feedback: Rc<RefCell<HashMap<(NodeId, EventId), FeedbackExpectation>>>,
    content_dialogs: Rc<RefCell<ContentDialogScheduler>>,
    pointer_capture: Rc<RefCell<HashMap<NodeId, bool>>>,
    selection_items: Rc<RefCell<Vec<(NodeId, windows_core::IInspectable)>>>,
    dispatcher: DispatcherQueue,
    identity: WindowToken,
    current_identity: Rc<Cell<Option<WindowToken>>>,
    scheduler: Rc<RefCell<SchedulerState>>,
}

impl EventSink {
    fn enqueue_host(&self, event: HostEvent) {
        self.host_events.borrow_mut().push(NativeWork {
            identity: self.identity,
            work: event,
        });
        self.schedule();
    }

    fn drag_action(&self, node: NodeId, kind: DragKind) -> Option<DragDropAction> {
        self.drop_policies
            .borrow()
            .get(&node)
            .and_then(|policy| match kind {
                DragKind::StorageItems => policy.storage_items.clone(),
                DragKind::Text => policy.text.clone(),
                DragKind::Unsupported => None,
            })
    }

    fn begin_async_event(
        &self,
        node: NodeId,
        event: EventId,
        revision: u32,
        deferral: DragOperationDeferral,
    ) -> AsyncIngressSender<Result<DroppedData, RuntimeError>> {
        let ticket = {
            let mut state = self.async_state.borrow_mut();
            assert_ne!(state.next_ticket, u64::MAX, "async event ticket exhausted");
            let ticket = state.next_ticket;
            state.next_ticket += 1;
            let cancellation = deferral.clone();
            state.pending.insert(
                ticket,
                PendingAsync {
                    node,
                    cancel: Box::new(move || {
                        _ = cancellation.Complete();
                    }),
                    finalize: Box::new(move |runtime, payload| {
                        let result = *payload
                            .downcast::<Result<DroppedData, RuntimeError>>()
                            .unwrap();
                        let result = deferral.Complete().map_err(native_error).and(result);
                        let identity = runtime.identity.get().unwrap();
                        match result {
                            Ok(value) => runtime.events.borrow_mut().push(NativeWork {
                                identity,
                                work: QueuedEvent::new(
                                    node,
                                    event,
                                    revision,
                                    EventPayload::DroppedData(value),
                                ),
                            }),
                            Err(error) => runtime.event_errors.borrow_mut().push(NativeWork {
                                identity,
                                work: QueuedEventError {
                                    node,
                                    event,
                                    revision,
                                    error,
                                },
                            }),
                        }
                    }),
                },
            );
            ticket
        };
        AsyncIngressSender {
            identity: self.identity,
            ticket,
            ingress: Arc::clone(&self.async_ingress),
            wake: background_dispatch_waker(self.dispatcher.clone(), self.identity),
            marker: PhantomData,
        }
    }

    pub fn capture_pointer_on_press(
        &self,
        node: NodeId,
        element: &UIElement,
        args: windows_core::InRef<'_, PointerRoutedEventArgs>,
    ) -> Result<bool, RuntimeError> {
        if !self.pointer_capture.borrow().contains_key(&node) {
            return Ok(false);
        }
        let Some(args) = args.as_ref() else {
            return Ok(false);
        };
        let pointer = args.Pointer().map_err(native_error)?;
        element.CapturePointer(&pointer).map_err(native_error)
    }

    pub fn release_pointer_after_event(
        &self,
        node: NodeId,
        element: &UIElement,
        args: windows_core::InRef<'_, PointerRoutedEventArgs>,
    ) -> Result<(), RuntimeError> {
        if !self.pointer_capture.borrow().contains_key(&node) {
            return Ok(());
        }
        let Some(args) = args.as_ref() else {
            return Ok(());
        };
        let pointer = args.Pointer().map_err(native_error)?;
        element
            .ReleasePointerCapture(&pointer)
            .map_err(native_error)
    }

    fn content_dialog_root_ready(
        &self,
        node: NodeId,
        generation: u64,
        xaml_root: XamlRoot,
    ) -> Result<(), RuntimeError> {
        self.content_dialogs
            .borrow_mut()
            .root_ready(node, generation, xaml_root)
    }

    pub fn content_dialog_closed(
        &self,
        node: NodeId,
        _revision: u32,
    ) -> Result<bool, RuntimeError> {
        content_dialog::closed(&self.content_dialogs, self, node)
    }

    pub fn selection_item(&self, selected: &windows_core::IInspectable) -> Option<NodeId> {
        self.selection_items
            .borrow()
            .iter()
            .find_map(|(node, item)| (item == selected).then_some(*node))
    }

    pub fn enqueue(&self, node: NodeId, event: EventId, revision: u32, payload: EventPayload) {
        if self.encoded_image_nodes.borrow().contains(&node)
            && matches!(event, EventId::ImageImageOpened | EventId::ImageImageFailed)
        {
            return;
        }
        {
            let mut feedback = self.feedback.borrow_mut();
            if let Some(expected) = feedback.get_mut(&(node, event)) {
                match expected {
                    // Keep the expectation active until the setter returns so every synchronous
                    // echo from the same native mutation is covered.
                    FeedbackExpectation::Exact(expected) if expected == &payload => return,
                    FeedbackExpectation::Normalized { observation } => {
                        *observation =
                            Some(QueuedEvent::observation(node, event, revision, payload));
                        return;
                    }
                    FeedbackExpectation::Suppressed => return,
                    FeedbackExpectation::Exact(_) => {}
                }
            }
        }
        self.queue.borrow_mut().push(NativeWork {
            identity: self.identity,
            work: QueuedEvent::new(node, event, revision, payload),
        });
        self.schedule();
    }

    pub fn observe(&self, node: NodeId, event: EventId, revision: u32, payload: EventPayload) {
        self.queue.borrow_mut().push(NativeWork {
            identity: self.identity,
            work: QueuedEvent::observation(node, event, revision, payload),
        });
        self.schedule();
    }

    pub fn enqueue_or_observe(
        &self,
        node: NodeId,
        event: EventId,
        revision: u32,
        payload: EventPayload,
        invoke_callback: bool,
    ) {
        if invoke_callback {
            self.enqueue(node, event, revision, payload);
        } else {
            self.observe(node, event, revision, payload);
        }
    }

    pub fn error(&self, node: NodeId, event: EventId, revision: u32, error: RuntimeError) {
        self.errors.borrow_mut().push(NativeWork {
            identity: self.identity,
            work: QueuedEventError {
                node,
                event,
                revision,
                error,
            },
        });
        self.schedule();
    }

    pub fn wake(&self) {
        self.schedule();
    }

    fn schedule(&self) {
        match self.request(WorkPriority::Normal) {
            Ok(()) | Err(RuntimeError::SchedulerClosed) => {}
            Err(error) => fail_native_scheduler(error),
        }
    }

    fn request(&self, priority: WorkPriority) -> Result<(), RuntimeError> {
        let action = self.scheduler.borrow_mut().request(priority);
        Self::perform(
            action,
            self.identity,
            &self.current_identity,
            &self.scheduler,
            &self.dispatcher,
        )
    }

    fn perform(
        action: ScheduleAction,
        identity: WindowToken,
        current_identity: &Rc<Cell<Option<WindowToken>>>,
        scheduler: &Rc<RefCell<SchedulerState>>,
        dispatcher: &DispatcherQueue,
    ) -> Result<(), RuntimeError> {
        let ScheduleAction::Enqueue(ticket) = action else {
            return match action {
                ScheduleAction::Closed => Err(RuntimeError::SchedulerClosed),
                _ => Ok(()),
            };
        };
        let current_identity_capture = Rc::clone(current_identity);
        let scheduler_capture = Rc::clone(scheduler);
        let dispatcher_capture = dispatcher.clone();
        let handler = DispatcherQueueHandler::new(move || {
            if current_identity_capture.get() != Some(identity) {
                let action = {
                    let mut scheduler = scheduler_capture.borrow_mut();
                    if !scheduler.begin_dispatch(ticket) {
                        return;
                    }
                    _ = scheduler.request(WorkPriority::Normal);
                    scheduler.finish_dispatch()
                };
                if let Some(identity) = current_identity_capture.get()
                    && let Err(error) = Self::perform(
                        action,
                        identity,
                        &current_identity_capture,
                        &scheduler_capture,
                        &dispatcher_capture,
                    )
                    && error != RuntimeError::SchedulerClosed
                {
                    fail_native_scheduler(error);
                }
                return;
            }
            if !scheduler_capture.borrow_mut().begin_dispatch(ticket) {
                return;
            }
            dispatch_native_events(identity);
            let action = scheduler_capture.borrow_mut().finish_dispatch();
            if let Some(identity) = current_identity_capture.get()
                && let Err(error) = Self::perform(
                    action,
                    identity,
                    &current_identity_capture,
                    &scheduler_capture,
                    &dispatcher_capture,
                )
                && error != RuntimeError::SchedulerClosed
            {
                fail_native_scheduler(error);
            }
        });
        let priority = match ticket.priority {
            WorkPriority::Low => DispatcherQueuePriority::Low,
            WorkPriority::Normal => DispatcherQueuePriority::Normal,
        };
        match dispatcher.TryEnqueueWithPriority(priority, &handler) {
            Ok(true) => Ok(()),
            Ok(false) => {
                scheduler.borrow_mut().enqueue_failed(ticket);
                Err(RuntimeError::DispatcherRejected)
            }
            Err(error) => {
                scheduler.borrow_mut().enqueue_failed(ticket);
                Err(native_error(error))
            }
        }
    }
}

fn com_identity(value: &impl Interface) -> Result<usize, RuntimeError> {
    value
        .cast::<windows_core::IUnknown>()
        .map(|identity| identity.as_raw() as usize)
        .map_err(native_error)
}

fn physical_retained_index(
    current: &[usize],
    retained: &HashSet<usize>,
    semantic_index: usize,
) -> Result<usize, RuntimeError> {
    let mut live = 0;
    for (physical, identity) in current.iter().enumerate() {
        if retained.contains(identity) {
            continue;
        }
        if live == semantic_index {
            return Ok(physical);
        }
        live += 1;
    }
    if live == semantic_index {
        Ok(current.len())
    } else {
        Err(RuntimeError::IndexOutOfBounds)
    }
}

fn merge_retained_identities(
    current: &[usize],
    desired: &[usize],
    retained: &HashSet<usize>,
) -> Vec<usize> {
    let desired_set = desired.iter().copied().collect::<HashSet<_>>();
    let mut before = HashMap::<usize, Vec<usize>>::new();
    let mut trailing = Vec::new();
    for (index, identity) in current.iter().copied().enumerate() {
        if !retained.contains(&identity) {
            continue;
        }
        if let Some(successor) = current[index + 1..]
            .iter()
            .copied()
            .find(|candidate| desired_set.contains(candidate))
        {
            before.entry(successor).or_default().push(identity);
        } else {
            trailing.push(identity);
        }
    }
    let mut merged = Vec::with_capacity(desired.len() + retained.len());
    for identity in desired.iter().copied() {
        if let Some(retained) = before.remove(&identity) {
            merged.extend(retained);
        }
        merged.push(identity);
    }
    merged.extend(trailing);
    merged
}

fn retained_subsequence(current: &[usize], desired: &[usize]) -> HashSet<usize> {
    let positions = current
        .iter()
        .copied()
        .enumerate()
        .map(|(index, identity)| (identity, index))
        .collect::<HashMap<_, _>>();
    let sequence = desired
        .iter()
        .filter_map(|identity| {
            positions
                .get(identity)
                .map(|position| (*identity, *position))
        })
        .collect::<Vec<_>>();
    let mut tails = Vec::<usize>::new();
    let mut previous = vec![None; sequence.len()];
    for (index, (_, position)) in sequence.iter().enumerate() {
        let slot = tails.partition_point(|tail| sequence[*tail].1 < *position);
        if slot > 0 {
            previous[index] = Some(tails[slot - 1]);
        }
        if slot == tails.len() {
            tails.push(index);
        } else {
            tails[slot] = index;
        }
    }
    let mut retained = HashSet::new();
    let Some(mut index) = tails.last().copied() else {
        return retained;
    };
    loop {
        retained.insert(sequence[index].0);
        let Some(next) = previous[index] else {
            break;
        };
        index = next;
    }
    retained
}

fn child_index(
    children: &UIElementCollection,
    child_id: NodeId,
    child: &UIElement,
) -> Result<u32, RuntimeError> {
    let mut index = 0;
    children
        .IndexOf(child, &mut index)
        .map_err(native_error)?
        .then_some(index)
        .ok_or(RuntimeError::ChildNotFound(child_id))
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;

fn inspectable_child_index(
    children: &SlotCollection,
    child_id: NodeId,
    child: &windows_core::IInspectable,
) -> Result<u32, RuntimeError> {
    let size = children.Size()?;
    (0..size)
        .find(|index| children.GetAt(*index).as_ref() == Ok(child))
        .ok_or(RuntimeError::ChildNotFound(child_id))
}

fn index32(index: usize) -> Result<u32, RuntimeError> {
    index.try_into().map_err(|_| RuntimeError::IndexOutOfBounds)
}

fn native_selection_index(value: Option<usize>) -> Result<i32, RuntimeError> {
    value.map_or(Ok(-1), |value| {
        value.try_into().map_err(|_| RuntimeError::IndexOutOfBounds)
    })
}

fn selection_index(value: i32) -> Result<Option<usize>, RuntimeError> {
    match value {
        -1 => Ok(None),
        0.. => Ok(Some(value as usize)),
        _ => Err(RuntimeError::IndexOutOfBounds),
    }
}

fn native_number_box_value(value: Option<f64>) -> f64 {
    value.unwrap_or(f64::NAN)
}

fn number_box_value(value: f64) -> Option<f64> {
    (!value.is_nan()).then_some(value)
}

fn native_rating_value(value: Option<f64>) -> f64 {
    value.unwrap_or(-1.0)
}

fn rating_value(value: f64) -> Option<f64> {
    (value != -1.0).then_some(value)
}

fn is_internal_detach(command: &Command, destroyed: &HashSet<NodeId>) -> bool {
    match command {
        Command::RemoveChild { parent, child, .. } => {
            destroyed.contains(parent) && destroyed.contains(child)
        }
        Command::SetSlot {
            parent,
            child: None,
            ..
        } => destroyed.contains(parent),
        _ => false,
    }
}

impl NativeRuntime for WinUiRuntime {
    fn apply(&mut self, commands: &[Command]) -> Result<(), NativeApplyError> {
        #[cfg(feature = "test")]
        let started = std::time::Instant::now();
        // Detach only the external edge of a subtree so WinUI cannot run deferred callbacks
        // against a control whose internal children have already been removed.
        let destroyed = commands
            .iter()
            .filter_map(|command| match command {
                Command::Destroy { node } => Some(*node),
                _ => None,
            })
            .collect::<HashSet<_>>();
        for (index, command) in commands.iter().enumerate() {
            if is_internal_detach(command, &destroyed) {
                continue;
            }
            if let Err(error) = self.apply_one(command) {
                eprintln!("windows-reactor failed command {index}: {command:?}: {error:?}");
                return Err(NativeApplyError {
                    command: index,
                    error,
                });
            }
        }
        #[cfg(feature = "test")]
        self.native_apply_times_us
            .push(started.elapsed().as_secs_f64() * 1_000_000.0);
        Ok(())
    }

    fn open_windows(&mut self, roots: Vec<View>) -> Result<(), RuntimeError> {
        open_live_windows(roots)
    }

    fn reset(&mut self) {
        for root in self.retained_subtrees.keys().copied().collect::<Vec<_>>() {
            _ = self.cancel_retirement(root);
        }
        self.clear_async_ingress();
        for node in self
            .webview_initializations
            .borrow()
            .keys()
            .copied()
            .collect::<Vec<_>>()
        {
            complete_webview_initialization(
                &self.webview_initializations,
                node,
                Err(RuntimeError::MissingNode(node)),
            );
        }
        self.subscriptions.clear();
        self.encoded_image_nodes.borrow_mut().clear();
        self.image_decode_tickets.clear();
        self.observation_subscriptions.clear();
        self.owned_menus.clear();
        self.command_bar_flyouts.clear();
        content_dialog::reset(&self.content_dialogs);
        for (target, (flyout, _)) in self.flyouts.drain() {
            _ = flyout.SetContent(None::<&UIElement>);
            match self.handles.get(&target) {
                Some(Handle::Button(button)) => {
                    _ = button.SetFlyout(None::<&FlyoutBase>);
                }
                Some(Handle::SplitButton(button)) => {
                    _ = button.SetFlyout(None::<&FlyoutBase>);
                }
                _ => {}
            }
        }
        if self.window_closed.get() {
            for (_, subscription) in self.window_subscriptions.drain() {
                subscription.into_token();
            }
        } else {
            self.window_subscriptions.clear();
            for window in self.windows.values() {
                _ = window.Close();
            }
        }
        self.windows.clear();
        self.window_observations.clear();
        self.window_hosts.clear();
        self.window_observation_subscriptions.clear();
        self.window_title_bars.clear();
        self.window_title_revisions.borrow_mut().clear();
        self.window_visuals.clear();
        self.theme_styles.clear();
        self.handles.clear();
        self.virtuals.clear();
        self.application = None;
        self.pending_application = None;
        self.event_errors.borrow_mut().clear();
        self.events.borrow_mut().clear();
        self.host_events.borrow_mut().clear();
        self.feedback.borrow_mut().clear();
        self.drop_policies.borrow_mut().clear();
        self.pointer_capture.borrow_mut().clear();
        self.resource_override_keys.clear();
        self.controlled_collection_indices.clear();
        self.selection_owners.clear();
        self.selection_items.borrow_mut().clear();
        self.realizations.borrow_mut().clear();
        self.window_closed.set(false);
    }

    fn native_window_closed(&mut self) {
        for root in self.retained_subtrees.keys().copied().collect::<Vec<_>>() {
            _ = self.cancel_retirement(root);
        }
        self.clear_async_ingress();
        self.window_observation_subscriptions.clear();
        self.window_title_revisions.borrow_mut().clear();
        for (_, subscription) in self.window_subscriptions.drain() {
            subscription.into_token();
        }
    }

    fn component_waker(&self) -> Option<Rc<dyn Fn()>> {
        let sink = self.event_sink().ok()?;
        Some(Rc::new(move || sink.wake()))
    }

    fn component_background_waker(&self) -> Option<Arc<dyn Fn() -> bool + Send + Sync>> {
        let dispatcher = DispatcherQueue::GetForCurrentThread().ok()?;
        let identity = self.identity.get()?;
        Some(background_dispatch_waker(dispatcher, identity))
    }

    fn set_identity(&mut self, identity: WindowToken) {
        if self
            .identity
            .get()
            .is_none_or(|current| current != identity)
        {
            self.scheduler.borrow_mut().open();
        }
        self.identity.set(Some(identity));
        let mut ingress = self.async_ingress.lock().unwrap();
        ingress.identity = Some(identity);
        ingress.completions.clear();
    }

    fn drain_events(&mut self) -> Vec<NativeWork<QueuedEvent>> {
        self.flush_async_ingress();
        self.events.borrow_mut().drain(..).collect()
    }

    fn drain_event_errors(&mut self) -> Vec<NativeWork<QueuedEventError>> {
        self.event_errors.borrow_mut().drain(..).collect()
    }

    fn drain_host_events(&mut self) -> Vec<NativeWork<HostEvent>> {
        self.host_events.borrow_mut().drain(..).collect()
    }

    fn drain_realizations(&mut self) -> Vec<NativeWork<RealizationRequest>> {
        self.realizations.borrow_mut().drain(..).collect()
    }
}

impl WinUiRuntime {
    fn register_async_completion<T: Send + 'static>(
        &mut self,
        node: NodeId,
        cancel: impl FnOnce() + 'static,
        finalize: impl FnOnce(&mut Self, T) + 'static,
    ) -> Result<(u64, AsyncIngressSender<T>), RuntimeError> {
        let identity = self
            .identity
            .get()
            .ok_or(RuntimeError::MissingApplication)?;
        let ticket = {
            let mut state = self.async_state.borrow_mut();
            if state.next_ticket == u64::MAX {
                return Err(RuntimeError::UnsupportedKind);
            }
            let ticket = state.next_ticket;
            state.next_ticket += 1;
            state.pending.insert(
                ticket,
                PendingAsync {
                    node,
                    cancel: Box::new(cancel),
                    finalize: Box::new(move |runtime, payload| {
                        let payload = payload.downcast::<T>().unwrap();
                        finalize(runtime, *payload);
                    }),
                },
            );
            ticket
        };
        let dispatcher = DispatcherQueue::GetForCurrentThread().map_err(native_error)?;
        Ok((
            ticket,
            AsyncIngressSender {
                identity,
                ticket,
                ingress: Arc::clone(&self.async_ingress),
                wake: background_dispatch_waker(dispatcher, identity),
                marker: PhantomData,
            },
        ))
    }

    fn start_encoded_image(
        &mut self,
        node: NodeId,
        image: &EncodedImage,
    ) -> Result<(), RuntimeError> {
        match self.handles.get(&node) {
            Some(Handle::Image(_) | Handle::ImageIcon(_)) => {}
            Some(_) => return Err(RuntimeError::UnsupportedKind),
            None => return Err(RuntimeError::MissingNode(node)),
        }

        let stream = InMemoryRandomAccessStream::new().map_err(native_error)?;
        let output = stream.GetOutputStreamAt(0).map_err(native_error)?;
        let writer = DataWriter::CreateDataWriter(&output).map_err(native_error)?;
        writer.WriteBytes(image.as_bytes()).map_err(native_error)?;
        let operation = writer.StoreAsync().map_err(native_error)?;
        let cancel_operation = operation.clone();
        let (ticket, sender) = self.register_async_completion(
            node,
            move || {
                _ = cancel_operation.Cancel();
            },
            move |runtime, result: Result<u32, RuntimeError>| {
                runtime.image_decode_tickets.remove(&node);
                if let Err(error) = result {
                    runtime.report_image_decode_error(error);
                    return;
                }
                if let Err(error) = runtime.start_bitmap_decode(node, stream, writer) {
                    runtime.report_image_decode_error(error);
                }
            },
        )?;
        self.image_decode_tickets.insert(node, ticket);
        if let Err(error) = operation.when(move |result| {
            _ = sender.complete(result.map_err(native_error));
        }) {
            self.cancel_image_decode(node);
            return Err(native_error(error));
        }
        Ok(())
    }

    fn start_bitmap_decode(
        &mut self,
        node: NodeId,
        stream: InMemoryRandomAccessStream,
        writer: DataWriter,
    ) -> Result<(), RuntimeError> {
        _ = writer.DetachStream().map_err(native_error)?;
        stream.Seek(0).map_err(native_error)?;
        let bitmap = BitmapImage::new().map_err(native_error)?;
        let operation = bitmap
            .cast::<IBitmapSource>()
            .map_err(native_error)?
            .SetSourceAsync(&stream)
            .map_err(native_error)?;
        let cancel_operation = operation.clone();
        let (ticket, sender) = self.register_async_completion(
            node,
            move || {
                _ = cancel_operation.Cancel();
            },
            move |runtime, result: Result<(), RuntimeError>| {
                runtime.image_decode_tickets.remove(&node);
                match result {
                    Ok(()) => {
                        if let Err(error) = runtime.set_decoded_image(node, &bitmap) {
                            runtime.report_image_decode_error(error);
                        } else {
                            runtime.enqueue_image_decode_event(node, EventId::ImageImageOpened);
                        }
                    }
                    Err(_) => {
                        runtime.enqueue_image_decode_event(node, EventId::ImageImageFailed);
                    }
                }
                drop(stream);
            },
        )?;
        self.image_decode_tickets.insert(node, ticket);
        if let Err(error) = operation.when(move |result| {
            _ = sender.complete(result.map_err(native_error));
        }) {
            self.cancel_image_decode(node);
            return Err(native_error(error));
        }
        Ok(())
    }

    fn set_decoded_image(&self, node: NodeId, bitmap: &BitmapImage) -> Result<(), RuntimeError> {
        let source = bitmap.cast::<ImageSource>().map_err(native_error)?;
        match self.handles.get(&node) {
            Some(Handle::Image(control)) => control.SetSource(&source).map_err(native_error),
            Some(Handle::ImageIcon(control)) => control.SetSource(&source).map_err(native_error),
            Some(_) => Err(RuntimeError::UnsupportedKind),
            None => Err(RuntimeError::MissingNode(node)),
        }
    }

    fn cancel_image_decode(&mut self, node: NodeId) {
        let Some(ticket) = self.image_decode_tickets.remove(&node) else {
            return;
        };
        let pending = self.async_state.borrow_mut().pending.remove(&ticket);
        if let Some(pending) = pending {
            (pending.cancel)();
        }
    }

    fn release_encoded_image_source(&mut self, node: NodeId) {
        self.cancel_image_decode(node);
        self.purge_image_events(node);
        self.encoded_image_nodes.borrow_mut().remove(&node);
    }

    fn report_image_decode_error(&mut self, error: RuntimeError) {
        let Some(identity) = self.identity.get() else {
            return;
        };
        self.host_events.borrow_mut().push(NativeWork {
            identity,
            work: HostEvent::Error(error),
        });
    }

    fn enqueue_image_decode_event(&mut self, node: NodeId, event: EventId) -> bool {
        let Some(NativeSubscription::Event { revision, .. }) =
            self.subscriptions.get(&(node, event))
        else {
            return false;
        };
        self.events.borrow_mut().push(NativeWork {
            identity: self.identity.get().unwrap(),
            work: QueuedEvent::new(node, event, *revision, EventPayload::Unit),
        });
        true
    }

    fn purge_image_events(&mut self, node: NodeId) {
        self.events.borrow_mut().retain(|event| {
            event.work.node != node
                || !matches!(
                    event.work.event,
                    EventId::ImageImageOpened | EventId::ImageImageFailed
                )
        });
    }

    fn start_retirement(
        &mut self,
        root: NodeId,
        nodes: Vec<NodeId>,
        parent: NodeId,
        slot: Option<SlotId>,
        transition: ExitTransition,
    ) -> Result<(), RuntimeError> {
        if self.retained_subtrees.contains_key(&root)
            || nodes.iter().any(|node| !self.contains(*node))
        {
            return Err(RuntimeError::MissingNode(root));
        }
        for node in &nodes {
            self.cancel_image_decode(*node);
            self.purge_image_events(*node);
        }
        let duration =
            TimeSpan::try_from(transition.duration()).map_err(|_| RuntimeError::UnsupportedKind)?;
        let root_element = self.ui_element(root)?;
        let opacity_transition = ScalarTransition::new().map_err(native_error)?;
        opacity_transition
            .SetDuration(duration)
            .map_err(native_error)?;
        root_element
            .SetOpacityTransition(&opacity_transition)
            .map_err(native_error)?;

        let timer = DispatcherQueue::GetForCurrentThread()
            .and_then(|dispatcher| dispatcher.CreateTimer())
            .map_err(native_error)?;
        timer.SetInterval(duration).map_err(native_error)?;
        timer.SetIsRepeating(false).map_err(native_error)?;
        let identity = self
            .identity
            .get()
            .ok_or(RuntimeError::MissingApplication)?;
        let (ticket, sender) = self.register_async_completion(
            root,
            || {},
            move |runtime, ()| {
                if let Err(error) = runtime.finish_retirement(root) {
                    runtime.host_events.borrow_mut().push(NativeWork {
                        identity,
                        work: HostEvent::Error(error),
                    });
                }
            },
        )?;
        let sender = Rc::new(RefCell::new(Some(sender)));
        let tick_sender = Rc::clone(&sender);
        let tick = timer
            .Tick(move |_, _| {
                if let Some(sender) = tick_sender.borrow_mut().take() {
                    _ = sender.complete(());
                }
            })
            .map_err(native_error)?;
        self.retained_subtrees.insert(
            root,
            NativeRetainedSubtree {
                nodes,
                parent,
                slot,
                ticket,
                _timer: timer.clone(),
                _tick: tick,
            },
        );
        if let Err(error) = root_element.SetOpacity(0.0).map_err(native_error) {
            self.cancel_retirement(root)?;
            return Err(error);
        }
        if let Err(error) = timer.Start().map_err(native_error) {
            self.cancel_retirement(root)?;
            return Err(error);
        }
        Ok(())
    }

    fn finish_retirement(&mut self, root: NodeId) -> Result<(), RuntimeError> {
        let Some(retained) = self.retained_subtrees.remove(&root) else {
            return Ok(());
        };
        retained._timer.Stop().map_err(native_error)?;
        match retained.slot {
            Some(slot) => self.remove_slot_child(retained.parent, slot, root)?,
            None => self.remove_child(retained.parent, root)?,
        }
        for node in retained.nodes {
            self.remove_node_state(node)?;
        }
        Ok(())
    }

    fn cancel_retirement(&mut self, root: NodeId) -> Result<(), RuntimeError> {
        let ticket = self
            .retained_subtrees
            .get(&root)
            .map(|retained| retained.ticket);
        let pending =
            ticket.and_then(|ticket| self.async_state.borrow_mut().pending.remove(&ticket));
        if let Some(pending) = pending {
            (pending.cancel)();
        }
        self.finish_retirement(root)
    }

    fn cancel_retirements_for_node(&mut self, node: NodeId) -> Result<(), RuntimeError> {
        let roots = self
            .retained_subtrees
            .iter()
            .filter_map(|(root, retained)| {
                (retained.parent == node || retained.nodes.contains(&node)).then_some(*root)
            })
            .collect::<Vec<_>>();
        for root in roots {
            self.cancel_retirement(root)?;
        }
        Ok(())
    }

    fn remove_node_state(&mut self, node: NodeId) -> Result<(), RuntimeError> {
        self.release_encoded_image_source(node);
        self.subscriptions
            .retain(|(subscription_node, _), _| *subscription_node != node);
        self.cancel_async_for_node(node);
        self.window_subscriptions.remove(&node);
        self.window_observation_subscriptions.remove(&node);
        self.window_observations.remove(&node);
        self.window_hosts.remove(&node);
        self.window_title_bars.remove(&node);
        self.window_title_revisions.borrow_mut().remove(&node);
        self.window_visuals.remove(&node);
        complete_webview_initialization(
            &self.webview_initializations,
            node,
            Err(RuntimeError::MissingNode(node)),
        );
        self.controlled_collection_indices.remove(&node);
        self.observation_subscriptions
            .retain(|(subscription_node, _), _| *subscription_node != node);
        self.drop_policies.borrow_mut().remove(&node);
        self.pointer_capture.borrow_mut().remove(&node);
        if self.resource_override_keys.contains_key(&node) {
            self.clear_resource_overrides(node)?;
        }
        content_dialog::retire(&self.content_dialogs, node);
        self.selection_owners
            .retain(|child, (parent, _)| *child != node && *parent != node);
        self.selection_items
            .borrow_mut()
            .retain(|(child, _)| *child != node);
        if self.handles.remove(&node).is_some()
            || self.virtuals.remove(&node).is_some()
            || self.windows.remove(&node).is_some()
        {
            return Ok(());
        }
        if self
            .application
            .as_ref()
            .is_some_and(|(application, _)| *application == node)
        {
            self.application = None;
            return Ok(());
        }
        Err(RuntimeError::MissingNode(node))
    }

    fn cancel_async_for_node(&mut self, node: NodeId) {
        let mut state = self.async_state.borrow_mut();
        let tickets = state
            .pending
            .iter()
            .filter_map(|(ticket, pending)| (pending.node == node).then_some(*ticket))
            .collect::<Vec<_>>();
        let cancellations = tickets
            .into_iter()
            .filter_map(|ticket| state.pending.remove(&ticket))
            .map(|pending| pending.cancel)
            .collect::<Vec<_>>();
        drop(state);
        for cancel in cancellations {
            cancel();
        }
    }

    fn clear_async_ingress(&mut self) {
        {
            let mut ingress = self.async_ingress.lock().unwrap();
            ingress.identity = None;
            ingress.completions.clear();
        }
        let pending = self
            .async_state
            .borrow_mut()
            .pending
            .drain()
            .map(|(_, pending)| pending)
            .collect::<Vec<_>>();
        for pending in pending {
            (pending.cancel)();
        }
        self.image_decode_tickets.clear();
        self.encoded_image_nodes.borrow_mut().clear();
    }

    fn flush_async_ingress(&mut self) {
        let completions: Vec<_> = self
            .async_ingress
            .lock()
            .unwrap()
            .completions
            .drain(..)
            .collect();
        for completion in completions {
            if self.identity.get() != Some(completion.identity) {
                continue;
            }
            let Some(pending) = self
                .async_state
                .borrow_mut()
                .pending
                .remove(&completion.ticket)
            else {
                continue;
            };
            (pending.finalize)(self, completion.payload);
        }
    }

    #[cfg(feature = "test")]
    pub(crate) fn live_window(&self) -> Result<Window, RuntimeError> {
        self.windows
            .values()
            .next()
            .cloned()
            .ok_or(RuntimeError::MissingApplication)
    }
}

fn background_dispatch_waker(
    dispatcher: DispatcherQueue,
    identity: WindowToken,
) -> Arc<dyn Fn() -> bool + Send + Sync> {
    let queued = Arc::new(AtomicBool::new(false));
    Arc::new(move || {
        if queued.swap(true, Ordering::AcqRel) {
            return true;
        }

        let handler_queued = Arc::clone(&queued);
        let handler = DispatcherQueueHandler::new(move || {
            handler_queued.store(false, Ordering::Release);
            dispatch_native_events(identity);
        });
        if matches!(
            dispatcher.TryEnqueueWithPriority(DispatcherQueuePriority::Normal, &handler),
            Ok(true)
        ) {
            true
        } else {
            queued.store(false, Ordering::Release);
            false
        }
    })
}

fn native_drag_operation(operation: DragDropOperation) -> DataPackageOperation {
    match operation {
        DragDropOperation::Copy => DataPackageOperation::Copy,
        DragDropOperation::Move => DataPackageOperation::Move,
        DragDropOperation::Link => DataPackageOperation::Link,
    }
}

fn native_error(error: windows_core::Error) -> RuntimeError {
    RuntimeError::Native(error.code().0)
}

fn invoke_callback<T>(callback: &Callback<T>, value: T) {
    if std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| callback.call(value))).is_err() {
        std::process::abort();
    }
}

fn subscribe_xaml_root_scale<F>(
    element: &UIElement,
    framework: &IFrameworkElement,
    sink: &EventSink,
    callback: F,
) -> Result<XamlRootScaleSubscriptions, RuntimeError>
where
    F: Clone + Fn(f64) + 'static,
{
    let changed = Rc::new(RefCell::new(None));
    observe_xaml_root_scale(element, &changed, sink, callback.clone())?;
    let loaded_element = element.clone();
    let loaded_changed = changed.clone();
    let loaded_sink = sink.clone();
    let loaded = framework
        .Loaded(move |_, _| {
            if let Err(error) = observe_xaml_root_scale(
                &loaded_element,
                &loaded_changed,
                &loaded_sink,
                callback.clone(),
            ) {
                loaded_sink.enqueue_host(HostEvent::Error(error));
            }
        })
        .map_err(native_error)?;
    Ok(XamlRootScaleSubscriptions {
        _changed: changed,
        _loaded: loaded,
    })
}

fn observe_xaml_root_scale<F>(
    element: &UIElement,
    changed: &Rc<RefCell<Option<windows_core::EventRevoker>>>,
    sink: &EventSink,
    callback: F,
) -> Result<(), RuntimeError>
where
    F: Fn(f64) + 'static,
{
    let root = match element.XamlRoot() {
        Ok(root) => root,
        Err(error) if error.code().is_ok() => return Ok(()),
        Err(error) => return Err(native_error(error)),
    };
    let scale = root.RasterizationScale().map_err(native_error)?;
    callback(scale);
    let changed_sink = sink.clone();
    let revoker = root
        .Changed(move |sender, _| {
            if let Some(sender) = sender.as_ref() {
                match sender.RasterizationScale() {
                    Ok(scale) => callback(scale),
                    Err(error) => {
                        changed_sink.enqueue_host(HostEvent::Error(native_error(error)));
                    }
                }
            }
        })
        .map_err(native_error)?;
    changed.replace(Some(revoker));
    Ok(())
}

fn xaml_scale(element: &UIElement) -> Result<f64, RuntimeError> {
    match element.XamlRoot() {
        Ok(root) => root.RasterizationScale().map_err(native_error),
        Err(error) if error.code().is_ok() => Ok(1.0),
        Err(error) => Err(native_error(error)),
    }
}

pub fn bootstrap_runtime() -> windows_core::Result<()> {
    if self_contained_manifest_present() {
        return if self_contained_runtime_present() {
            Ok(())
        } else {
            Err(windows_core::Error::new(
                windows_core::HRESULT(0x8007007e_u32 as i32),
                "self-contained Windows App Runtime files are missing",
            ))
        };
    }

    bootstrap::bootstrap()
}

fn self_contained_runtime_present() -> bool {
    std::env::current_exe()
        .ok()
        .and_then(|path| path.parent().map(|parent| parent.to_path_buf()))
        .is_some_and(|parent| parent.join("Microsoft.WindowsAppRuntime.dll").is_file())
}

#[allow(clippy::manual_dangling_ptr)] // FindResource uses low pointer values for resource ordinals.
fn self_contained_manifest_present() -> bool {
    const SELF_CONTAINED_MARKER: &str = "windows-reactor-self-contained";
    let marker = SELF_CONTAINED_MARKER.as_bytes();

    unsafe {
        let module = GetModuleHandleW(std::ptr::null());
        if module.is_null() {
            return false;
        }
        let resource = FindResourceW(module, 1usize as *const u16, 24usize as *const u16);
        if resource.is_null() {
            return false;
        }
        let size = SizeofResource(module, resource) as usize;
        let loaded = LoadResource(module, resource);
        if loaded.is_null() {
            return false;
        }
        let data = LockResource(loaded).cast::<u8>();
        if data.is_null() {
            return false;
        }
        std::slice::from_raw_parts(data, size)
            .windows(marker.len())
            .any(|window| window == marker)
    }
}

pub fn initialize_ui_thread() -> windows_core::Result<()> {
    unsafe {
        _ = SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);
    }
    let result = unsafe { CoInitializeEx(std::ptr::null(), COINIT_APARTMENTTHREADED as u32) };
    if result == RPC_E_CHANGED_MODE {
        return Err(windows_core::Error::new(
            RPC_E_CHANGED_MODE,
            "WinUI requires an STA thread",
        ));
    }
    result.ok()
}

pub fn exit_ui_thread() {
    unsafe {
        PostQuitMessage(0);
    }
}
