use super::bindings as native;
use crate::reconcile::{FeedbackExpectation, FeedbackState};
use crate::{
    Adapter, Event, EventDispatch, EventId, EventPayload, EventValue, Mutation, ObjectId,
    ObjectType, Observation, Property, PropertyId, PropertyValue, Realization, RelationContract,
    RelationId, SelectionContract, relation_contracts, selection_for_item_property,
    selection_for_relation,
};
use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use windows_core::{HSTRING, IInspectable, Interface};

enum Handle {
    Generated(GeneratedHandle),
    TextBox(NativeTextBox),
    TreeView(native::TreeView),
    TreeNode(NativeTreeNode),
    ListView(native::ListView),
    Data(windows_collections::IObservableMap<HSTRING, IInspectable>),
}

struct NativeTreeNode {
    value: native::TreeViewNode,
    text: HSTRING,
    content: Option<ObjectId>,
}

struct NativeTextBox {
    value: native::TextBox,
    event: Rc<RefCell<NativeTextEvent>>,
    observed_text: Rc<RefCell<Rc<str>>>,
    set_count: Cell<usize>,
    _text_changed: windows_core::EventRevoker,
}

struct NativeValueEvent<T> {
    revision: u64,
    callback: Option<crate::Callback<T>>,
}

impl<T> Default for NativeValueEvent<T> {
    fn default() -> Self {
        Self {
            revision: 0,
            callback: None,
        }
    }
}

type NativeBoolEvent = NativeValueEvent<bool>;
type NativeF64Event = NativeValueEvent<f64>;
type NativeOptionalBoolEvent = NativeValueEvent<Option<bool>>;
type NativeOptionalF64Event = NativeValueEvent<Option<f64>>;
type NativePointerEventInfoEvent = NativeValueEvent<crate::PointerEventInfo>;
type NativeSelectionEvent = NativeValueEvent<Option<Rc<str>>>;
type NativeSelectionIndexEvent = NativeValueEvent<Option<usize>>;
type NativeTextEvent = NativeValueEvent<Rc<str>>;
type NativeStringEvent = NativeTextEvent;
type NativeUnitEvent = NativeValueEvent<()>;

include!("generated.rs");

struct QueuedEvent {
    object: ObjectId,
    event: EventId,
    revision: u64,
    payload: EventPayload,
}

struct NativeSelectionItem {
    owner: ObjectId,
    object: ObjectId,
    value: IInspectable,
}

#[derive(Default)]
struct NativeEventQueue {
    observations: RefCell<Vec<Observation>>,
    events: RefCell<Vec<QueuedEvent>>,
    feedback: RefCell<FeedbackState>,
    selection_items: RefCell<Vec<NativeSelectionItem>>,
    waker: RefCell<Option<Rc<dyn Fn()>>>,
    wake_pending: Cell<bool>,
}

impl NativeEventQueue {
    fn observe(&self, object: ObjectId, event: EventId, observation: Observation) -> bool {
        self.feedback
            .borrow_mut()
            .observe(object, event, observation)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WinUiError {
    DuplicateObject(ObjectId),
    MissingObject(ObjectId),
    InvalidObject(ObjectId),
    InvalidRelation(ObjectId, RelationId),
    InvalidMutation(RelationId),
    StateMismatch(ObjectId),
    MissingContract(ObjectType, RelationId),
    IndexOverflow(usize),
    ChildNotFound(ObjectId),
    StillOwned(ObjectId),
    InvalidReplacement(ObjectId),
    InvalidEventArgs,
    Native(windows_core::Error),
}

impl From<windows_core::Error> for WinUiError {
    fn from(value: windows_core::Error) -> Self {
        Self::Native(value)
    }
}

impl From<WinUiError> for windows_core::Error {
    fn from(value: WinUiError) -> Self {
        match value {
            WinUiError::Native(error) => error,
            error => Self::new(
                windows_core::HRESULT(0x80004005_u32 as i32),
                format!("{error:?}"),
            ),
        }
    }
}

fn solid_color_brush(value: crate::Color) -> Result<native::SolidColorBrush, WinUiError> {
    let brush = native::SolidColorBrush::new()?;
    brush.SetColor(native::Color {
        a: value.a,
        r: value.r,
        g: value.g,
        b: value.b,
    })?;
    Ok(brush)
}

pub struct WinUiAdapter {
    handles: HashMap<ObjectId, Handle>,
    owners: HashMap<ObjectId, (ObjectId, RelationId)>,
    tree_template: Option<native::DataTemplate>,
    list_template: Option<native::DataTemplate>,
    data_text_key: HSTRING,
    event_queue: Rc<NativeEventQueue>,
}

impl Default for WinUiAdapter {
    fn default() -> Self {
        Self {
            handles: HashMap::new(),
            owners: HashMap::new(),
            tree_template: None,
            list_template: None,
            data_text_key: HSTRING::from("Text"),
            event_queue: Rc::new(NativeEventQueue::default()),
        }
    }
}

pub struct NativeWindow {
    window: native::Window,
    closed: Option<windows_core::EventRevoker>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum WindowTheme {
    #[default]
    System,
    Light,
    Dark,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum WindowTitleBarHeight {
    #[default]
    Standard,
    Tall,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct WindowPolicy {
    title: Option<String>,
    theme: WindowTheme,
    client_size: Option<(f64, f64)>,
    minimum_client_size: Option<(f64, f64)>,
    title_bar: Option<(ObjectId, WindowTitleBarHeight)>,
}

impl WindowPolicy {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn theme(mut self, theme: WindowTheme) -> Self {
        self.theme = theme;
        self
    }

    pub fn client_size(mut self, width: f64, height: f64) -> Self {
        validate_window_size(width, height);
        self.client_size = Some((width, height));
        self
    }

    pub fn minimum_client_size(mut self, width: f64, height: f64) -> Self {
        validate_window_size(width, height);
        self.minimum_client_size = Some((width, height));
        self
    }

    pub fn title_bar(mut self, title_bar: ObjectId, height: WindowTitleBarHeight) -> Self {
        self.title_bar = Some((title_bar, height));
        self
    }
}

fn validate_window_size(width: f64, height: f64) {
    assert!(
        width.is_finite() && width > 0.0 && height.is_finite() && height > 0.0,
        "window size must be finite and positive"
    );
}

impl Clone for NativeWindow {
    fn clone(&self) -> Self {
        Self {
            window: self.window.clone(),
            closed: None,
        }
    }
}

impl NativeWindow {
    pub fn set_title(&self, title: &str) -> Result<(), WinUiError> {
        self.window.SetTitle(title).map_err(Into::into)
    }

    pub fn activate(&self) -> Result<(), WinUiError> {
        self.window.Activate().map_err(Into::into)
    }

    pub fn close(&self) -> Result<(), WinUiError> {
        self.window.Close().map_err(Into::into)
    }

    pub fn set_closed(
        &mut self,
        callback: impl Fn() -> windows_core::Result<()> + 'static,
    ) -> Result<(), WinUiError> {
        self.closed = Some(self.window.Closed(move |_, _| {
            if let Err(error) = callback() {
                super::app::report_error(error);
            }
        })?);
        Ok(())
    }
}

impl WinUiAdapter {
    pub fn set_event_waker(&mut self, waker: impl Fn() + 'static) {
        *self.event_queue.waker.borrow_mut() = Some(Rc::new(waker));
    }

    pub fn create_window(&self, root: ObjectId) -> Result<NativeWindow, WinUiError> {
        self.create_window_with_policy(root, &WindowPolicy::new())
    }

    pub fn create_window_with_policy(
        &self,
        root: ObjectId,
        policy: &WindowPolicy,
    ) -> Result<NativeWindow, WinUiError> {
        let window = native::Window::new()?;
        let root = self.ui_element(root)?;
        window.SetContent(&root)?;
        root.cast::<native::IUIElement>()?.UpdateLayout()?;
        self.apply_window_policy(&window, &root, policy)?;
        Ok(NativeWindow {
            window,
            closed: None,
        })
    }

    pub fn open_window(&self, root: ObjectId) -> Result<NativeWindow, WinUiError> {
        let window = self.create_window(root)?;
        window.activate()?;
        Ok(window)
    }

    pub fn open_window_with_policy(
        &self,
        root: ObjectId,
        policy: &WindowPolicy,
    ) -> Result<NativeWindow, WinUiError> {
        let window = self.create_window_with_policy(root, policy)?;
        window.activate()?;
        Ok(window)
    }

    fn apply_window_policy(
        &self,
        window: &native::Window,
        root: &native::UIElement,
        policy: &WindowPolicy,
    ) -> Result<(), WinUiError> {
        if let Some(title) = &policy.title {
            window.SetTitle(title)?;
        }

        let window_2 = window.cast::<native::IWindow2>()?;
        let app_window = window_2.AppWindow()?;
        let title_bar = app_window.TitleBar()?;
        title_bar
            .cast::<native::IAppWindowTitleBar3>()?
            .SetPreferredTheme(match policy.theme {
                WindowTheme::System => native::TitleBarTheme::UseDefaultAppMode,
                WindowTheme::Light => native::TitleBarTheme::Light,
                WindowTheme::Dark => native::TitleBarTheme::Dark,
            })?;
        root.cast::<native::FrameworkElement>()?
            .SetRequestedTheme(match policy.theme {
                WindowTheme::System => native::ElementTheme::Default,
                WindowTheme::Light => native::ElementTheme::Light,
                WindowTheme::Dark => native::ElementTheme::Dark,
            })?;

        if let Some((object, height)) = policy.title_bar {
            if self.kind(object)? != ObjectType::TitleBar {
                return Err(WinUiError::InvalidObject(object));
            }
            let element = self.ui_element(object)?;
            element.cast::<native::IUIElement>()?.SetIsTabStop(false)?;
            window.SetExtendsContentIntoTitleBar(true)?;
            window.SetTitleBar(&element)?;
            title_bar
                .cast::<native::IAppWindowTitleBar2>()?
                .SetPreferredHeightOption(match height {
                    WindowTitleBarHeight::Standard => native::TitleBarHeightOption::Standard,
                    WindowTitleBarHeight::Tall => native::TitleBarHeightOption::Tall,
                })?;
        }

        let needs_metrics = policy.client_size.is_some() || policy.minimum_client_size.is_some();
        if needs_metrics {
            let mut hwnd = std::ptr::null_mut();
            unsafe {
                window
                    .cast::<native::IWindowNative>()?
                    .WindowHandle(&mut hwnd)
                    .ok()?;
            }
            let dpi = unsafe { native::GetDpiForWindow(hwnd.cast()) }.max(96);
            let pixels = |dips: f64| (dips * f64::from(dpi) / 96.0).round() as i32;
            let client_window = app_window.cast::<native::IAppWindow2>()?;

            if let Some((width, height)) = policy.minimum_client_size {
                let outer = app_window.Size()?;
                let inner = client_window.ClientSize()?;
                let non_client_width = outer.width.saturating_sub(inner.width);
                let non_client_height = outer.height.saturating_sub(inner.height);
                let presenter = app_window
                    .Presenter()?
                    .cast::<native::IOverlappedPresenter3>()?;
                presenter.SetPreferredMinimumWidth(Some(
                    pixels(width).saturating_add(non_client_width),
                ))?;
                presenter.SetPreferredMinimumHeight(Some(
                    pixels(height).saturating_add(non_client_height),
                ))?;
            }

            if let Some((width, height)) = policy.client_size {
                client_window.ResizeClient(native::SizeInt32 {
                    width: pixels(width),
                    height: pixels(height),
                })?;
            }
        }
        Ok(())
    }

    pub fn validate_graph(&self, graph: &crate::RetainedGraph) -> Result<(), WinUiError> {
        let Some(root) = graph.root() else {
            return Ok(());
        };
        let mut pending = vec![root];
        let mut visited = HashSet::new();
        while let Some(object) = pending.pop() {
            if !visited.insert(object) {
                continue;
            }
            let kind = graph
                .kind(object)
                .ok_or(WinUiError::MissingObject(object))?;
            if self.kind(object)? != kind {
                return Err(WinUiError::StateMismatch(object));
            }
            if let Handle::Generated(GeneratedHandle::TextBlock(value)) = self
                .handles
                .get(&object)
                .ok_or(WinUiError::MissingObject(object))?
            {
                let expected = graph
                    .properties(object)
                    .unwrap_or_default()
                    .iter()
                    .find_map(|property| match (&property.id, &property.value) {
                        (PropertyId::Text, PropertyValue::String(value)) => Some(value.as_ref()),
                        _ => None,
                    })
                    .unwrap_or_default();
                if value.Text()? != expected {
                    return Err(WinUiError::StateMismatch(object));
                }
            }
            if let Handle::TextBox(value) = self
                .handles
                .get(&object)
                .ok_or(WinUiError::MissingObject(object))?
            {
                let expected = graph
                    .properties(object)
                    .unwrap_or_default()
                    .iter()
                    .find_map(|property| match (&property.id, &property.value) {
                        (PropertyId::Text, PropertyValue::String(value)) => Some(value.as_ref()),
                        _ => None,
                    })
                    .unwrap_or_default();
                if value.value.Text()? != expected {
                    return Err(WinUiError::StateMismatch(object));
                }
            }
            if let Handle::Data(value) = self
                .handles
                .get(&object)
                .ok_or(WinUiError::MissingObject(object))?
            {
                let expected = graph
                    .properties(object)
                    .unwrap_or_default()
                    .iter()
                    .find_map(|property| match (&property.id, &property.value) {
                        (PropertyId::Text, PropertyValue::String(value)) => Some(value.as_ref()),
                        _ => None,
                    })
                    .unwrap_or_default();
                let properties =
                    value.cast::<windows_collections::IMap<HSTRING, IInspectable>>()?;
                let actual = properties.Lookup(&self.data_text_key)?;
                let actual = actual
                    .cast::<windows_reference::IReference<HSTRING>>()?
                    .Value()?;
                if actual != expected {
                    return Err(WinUiError::StateMismatch(object));
                }
            }
            if let Some(selection) = crate::selection_contract(kind) {
                let selected = self.read_selected_item(object, selection)?;
                let selected = selected.as_ref().and_then(|selected| {
                    self.event_queue
                        .selection_items
                        .borrow()
                        .iter()
                        .find_map(|item| {
                            (item.owner == object && item.value == *selected).then_some(item.object)
                        })
                });
                for child in selection.relations.iter().flat_map(|relation| {
                    graph
                        .children(object, *relation)
                        .unwrap_or_default()
                        .iter()
                        .copied()
                }) {
                    if graph.kind(child) != Some(selection.item) {
                        continue;
                    }
                    let expected =
                        graph
                            .properties(child)
                            .unwrap_or_default()
                            .iter()
                            .find_map(|property| match property {
                                Property {
                                    id,
                                    value: PropertyValue::Bool(value),
                                } if *id == selection.selected_property => Some(*value),
                                _ => None,
                            });
                    if let Some(expected) = expected
                        && expected != (selected == Some(child))
                    {
                        return Err(WinUiError::StateMismatch(child));
                    }
                }
            }
            for contract in relation_contracts(kind) {
                match contract.cardinality {
                    crate::Cardinality::One => {
                        if let Some(child) = graph.child(object, contract.id) {
                            if self.owners.get(&child) != Some(&(object, contract.id)) {
                                return Err(WinUiError::StateMismatch(object));
                            }
                            pending.push(child);
                        }
                    }
                    crate::Cardinality::Many => {
                        let children = graph.children(object, contract.id).unwrap_or_default();
                        for child in children {
                            if self.owners.get(child) != Some(&(object, contract.id)) {
                                return Err(WinUiError::StateMismatch(object));
                            }
                            pending.push(*child);
                        }
                        self.validate_native_order(object, contract, children)?;
                    }
                }
            }
        }
        if visited.len() != self.handles.len() {
            return Err(WinUiError::StateMismatch(root));
        }
        Ok(())
    }

    pub fn simulate_text_input(
        &self,
        object: ObjectId,
        text: &str,
        selection_start: i32,
        selection_length: i32,
    ) -> Result<(), WinUiError> {
        let Some(Handle::TextBox(value)) = self.handles.get(&object) else {
            return Err(WinUiError::InvalidObject(object));
        };
        value.value.SetText(text)?;
        value.value.SetSelectionStart(selection_start)?;
        value.value.SetSelectionLength(selection_length)?;
        Self::dispatch_text_changed(
            &value.event,
            &value.observed_text,
            &self.event_queue,
            object,
            Rc::from(value.value.Text()?),
        );
        Ok(())
    }

    pub fn focus_text_box_deferred(
        &self,
        object: ObjectId,
        completion: impl Fn(bool) + 'static,
    ) -> Result<(), WinUiError> {
        let Some(Handle::TextBox(value)) = self.handles.get(&object) else {
            return Err(WinUiError::InvalidObject(object));
        };
        let value = value.value.clone();
        let timer = native::DispatcherQueue::GetForCurrentThread()?.CreateTimer()?;
        timer.SetInterval(windows_time::TimeSpan::from_millis(10))?;
        timer.SetIsRepeating(true)?;
        let state = Rc::new(RefCell::new(
            None::<(
                native::DispatcherQueueTimer,
                windows_core::EventRevoker,
                usize,
            )>,
        ));
        let event_state = Rc::clone(&state);
        let event_timer = timer.clone();
        let revoker = timer.Tick(move |_, _| {
            let focused = value
                .cast::<native::IUIElement>()
                .and_then(|value| value.Focus(native::FocusState::Programmatic))
                .unwrap_or(false);
            let mut state = event_state.borrow_mut();
            let (_, _, attempts) = state.as_mut().unwrap();
            *attempts += 1;
            if focused || *attempts == 100 {
                _ = event_timer.Stop();
                state.take();
                completion(focused);
            }
        });
        *state.borrow_mut() = Some((timer.clone(), revoker?, 0));
        timer.Start()?;
        Ok(())
    }

    pub fn text_box_set_count(&self, object: ObjectId) -> Result<usize, WinUiError> {
        let Some(Handle::TextBox(value)) = self.handles.get(&object) else {
            return Err(WinUiError::InvalidObject(object));
        };
        Ok(value.set_count.get())
    }

    pub fn simulate_click(&self, object: ObjectId) -> Result<(), WinUiError> {
        let Some(Handle::Generated(handle)) = self.handles.get(&object) else {
            return Err(WinUiError::InvalidObject(object));
        };
        let Some(event) = handle.unit_event(EventId::Click) else {
            return Err(WinUiError::InvalidObject(object));
        };
        Self::dispatch_unit(event, &self.event_queue, object, EventId::Click);
        Ok(())
    }

    pub fn simulate_pointer_released(
        &self,
        object: ObjectId,
        value: crate::PointerEventInfo,
    ) -> Result<(), WinUiError> {
        let Some(Handle::Generated(GeneratedHandle::Border(border))) = self.handles.get(&object)
        else {
            return Err(WinUiError::InvalidObject(object));
        };
        Self::dispatch_pointer_event_info(
            &border.pointer_released,
            &self.event_queue,
            object,
            EventId::PointerReleased,
            value,
        );
        Ok(())
    }

    pub fn set_slider_value(&self, object: ObjectId, value: f64) -> Result<(), WinUiError> {
        let Some(Handle::Generated(GeneratedHandle::Slider(slider))) = self.handles.get(&object)
        else {
            return Err(WinUiError::InvalidObject(object));
        };
        slider.value.cast::<native::IRangeBase>()?.SetValue(value)?;
        Ok(())
    }

    pub fn slider_state(&self, object: ObjectId) -> Result<(f64, f64, f64), WinUiError> {
        let Some(Handle::Generated(GeneratedHandle::Slider(slider))) = self.handles.get(&object)
        else {
            return Err(WinUiError::InvalidObject(object));
        };
        let slider = slider.value.cast::<native::IRangeBase>()?;
        Ok((slider.Minimum()?, slider.Maximum()?, slider.Value()?))
    }

    pub fn set_toggle_switch_is_on(&self, object: ObjectId, value: bool) -> Result<(), WinUiError> {
        let Some(Handle::Generated(GeneratedHandle::ToggleSwitch(toggle))) =
            self.handles.get(&object)
        else {
            return Err(WinUiError::InvalidObject(object));
        };
        toggle.value.SetIsOn(value)?;
        Ok(())
    }

    pub fn toggle_switch_state(&self, object: ObjectId) -> Result<bool, WinUiError> {
        let Some(Handle::Generated(GeneratedHandle::ToggleSwitch(toggle))) =
            self.handles.get(&object)
        else {
            return Err(WinUiError::InvalidObject(object));
        };
        Ok(toggle.value.IsOn()?)
    }

    pub fn set_password(&self, object: ObjectId, value: &str) -> Result<(), WinUiError> {
        let Some(Handle::Generated(GeneratedHandle::PasswordBox(password_box))) =
            self.handles.get(&object)
        else {
            return Err(WinUiError::InvalidObject(object));
        };
        self.event_queue.feedback.borrow_mut().begin(
            object,
            EventId::PasswordChanged,
            FeedbackExpectation::Suppressed,
        );
        let result = password_box.value.SetPassword(value);
        self.event_queue
            .feedback
            .borrow_mut()
            .finish(object, EventId::PasswordChanged);
        result?;
        let value = password_box
            .value
            .cast::<native::IPasswordBox>()?
            .Password()
            .map(Rc::<str>::from)?;
        self.event_queue
            .observations
            .borrow_mut()
            .push(Observation::SetProperty {
                object,
                property: Property {
                    id: PropertyId::Password,
                    value: PropertyValue::String(Rc::clone(&value)),
                },
            });
        Self::dispatch_string(
            &password_box.password_changed,
            &self.event_queue,
            object,
            EventId::PasswordChanged,
            value,
        );
        Ok(())
    }

    pub fn set_rating_value(&self, object: ObjectId, value: f64) -> Result<(), WinUiError> {
        let Some(Handle::Generated(GeneratedHandle::RatingControl(rating))) =
            self.handles.get(&object)
        else {
            return Err(WinUiError::InvalidObject(object));
        };
        self.event_queue.feedback.borrow_mut().begin(
            object,
            EventId::ValueChanged,
            FeedbackExpectation::Suppressed,
        );
        let result = rating.value.SetValue(value);
        self.event_queue
            .feedback
            .borrow_mut()
            .finish(object, EventId::ValueChanged);
        result?;
        let value = rating
            .value
            .cast::<native::IRatingControl>()?
            .Value()
            .map(rating_value)?;
        self.event_queue
            .observations
            .borrow_mut()
            .push(Observation::SetProperty {
                object,
                property: Property {
                    id: PropertyId::RatingControlValue,
                    value: PropertyValue::OptionalF64(value),
                },
            });
        Self::dispatch_optional_f64(
            &rating.value_changed,
            &self.event_queue,
            object,
            EventId::ValueChanged,
            value,
        );
        Ok(())
    }

    pub fn set_combo_box_selected_index(
        &self,
        object: ObjectId,
        value: Option<usize>,
    ) -> Result<(), WinUiError> {
        let Some(Handle::Generated(GeneratedHandle::ComboBox(combo_box))) =
            self.handles.get(&object)
        else {
            return Err(WinUiError::InvalidObject(object));
        };
        combo_box
            .value
            .cast::<native::ISelector>()?
            .SetSelectedIndex(native_selection_index(value)?)?;
        Ok(())
    }

    pub fn select_item(
        &self,
        object: ObjectId,
        selected: Option<ObjectId>,
    ) -> Result<(), WinUiError> {
        let selection = crate::selection_contract(self.kind(object)?)
            .ok_or(WinUiError::InvalidObject(object))?;
        let selected = selected
            .map(|selected| {
                self.event_queue
                    .selection_items
                    .borrow()
                    .iter()
                    .find_map(|item| {
                        (item.owner == object && item.object == selected)
                            .then(|| item.value.clone())
                    })
                    .ok_or(WinUiError::ChildNotFound(selected))
            })
            .transpose()?;
        self.write_selected_item(object, selection, selected.as_ref())
    }

    pub fn selected_item(&self, object: ObjectId) -> Result<Option<ObjectId>, WinUiError> {
        let selection = crate::selection_contract(self.kind(object)?)
            .ok_or(WinUiError::InvalidObject(object))?;
        let selected = self.read_selected_item(object, selection)?;
        Ok(selected.as_ref().and_then(|selected| {
            self.event_queue
                .selection_items
                .borrow()
                .iter()
                .find_map(|item| {
                    (item.owner == object && item.value == *selected).then_some(item.object)
                })
        }))
    }

    pub fn check_box_state(&self, object: ObjectId) -> Result<bool, WinUiError> {
        let Some(Handle::Generated(GeneratedHandle::CheckBox(check_box))) =
            self.handles.get(&object)
        else {
            return Err(WinUiError::InvalidObject(object));
        };
        Ok(check_box
            .value
            .cast::<native::IToggleButton>()?
            .IsChecked()?)
    }

    pub fn stack_panel_state(&self, object: ObjectId) -> Result<(f64, bool), WinUiError> {
        let Some(Handle::Generated(GeneratedHandle::StackPanel(panel))) = self.handles.get(&object)
        else {
            return Err(WinUiError::InvalidObject(object));
        };
        let panel = panel.cast::<native::IStackPanel>()?;
        Ok((
            panel.Spacing()?,
            panel.Orientation()? == native::Orientation::Horizontal,
        ))
    }

    pub fn canvas_position(&self, object: ObjectId) -> Result<(f64, f64), WinUiError> {
        let element = self
            .ui_element(object)?
            .cast::<native::FrameworkElement>()?;
        Ok((
            native::Canvas::GetLeft(&element)?,
            native::Canvas::GetTop(&element)?,
        ))
    }

    pub fn text_box_state(&self, object: ObjectId) -> Result<(String, i32, i32), WinUiError> {
        let Some(Handle::TextBox(value)) = self.handles.get(&object) else {
            return Err(WinUiError::InvalidObject(object));
        };
        Ok((
            value.value.Text()?,
            value.value.SelectionStart()?,
            value.value.SelectionLength()?,
        ))
    }

    fn validate_native_order(
        &self,
        parent: ObjectId,
        contract: &RelationContract,
        children: &[ObjectId],
    ) -> Result<(), WinUiError> {
        let matches = match contract.realization {
            Realization::Owned => {
                let values = self.owned_collection(parent, contract.id)?;
                if values.size()? as usize != children.len() {
                    false
                } else {
                    children.iter().enumerate().all(|(index, child)| {
                        values.get_at(index as u32).ok().as_ref()
                            == self
                                .ui_element(*child)
                                .and_then(|value| value.cast().map_err(Into::into))
                                .ok()
                                .as_ref()
                    })
                }
            }
            Realization::Structural => {
                let values = self.tree_nodes(parent)?;
                if values.Size()? as usize != children.len() {
                    false
                } else {
                    children.iter().enumerate().all(|(index, child)| {
                        values.GetAt(index as u32).ok().as_ref()
                            == self.tree_node(*child).ok().as_ref()
                    })
                }
            }
            Realization::Container => {
                let values = self.items(parent)?;
                if values.Size()? as usize != children.len() {
                    false
                } else {
                    children.iter().enumerate().all(|(index, child)| {
                        values.GetAt(index as u32).ok().as_ref()
                            == self.inspectable(*child).ok().as_ref()
                    })
                }
            }
        };
        if matches {
            Ok(())
        } else {
            Err(WinUiError::StateMismatch(parent))
        }
    }

    fn create(&mut self, object: ObjectId, kind: ObjectType) -> Result<(), WinUiError> {
        if self.handles.contains_key(&object) {
            return Err(WinUiError::DuplicateObject(object));
        }
        let handle = if let Some(handle) = GeneratedHandle::create(kind, object, &self.event_queue)?
        {
            Handle::Generated(handle)
        } else {
            match kind {
                ObjectType::TextBox => {
                    let value = native::TextBox::new()?;
                    let event = Rc::new(RefCell::new(NativeTextEvent::default()));
                    let event_for_callback = Rc::clone(&event);
                    let observed_text = Rc::new(RefCell::new(Rc::<str>::from("")));
                    let observed_text_for_event = Rc::clone(&observed_text);
                    let event_queue = Rc::clone(&self.event_queue);
                    let source = value.clone();
                    let text_changed = value.TextChanged(move |_, _| {
                        let text = match source.Text() {
                            Ok(text) => text,
                            Err(error) => {
                                super::app::report_error(error);
                                return;
                            }
                        };
                        Self::dispatch_text_changed(
                            &event_for_callback,
                            &observed_text_for_event,
                            &event_queue,
                            object,
                            Rc::from(text),
                        );
                    })?;
                    Handle::TextBox(NativeTextBox {
                        value,
                        event,
                        observed_text,
                        set_count: Cell::new(0),
                        _text_changed: text_changed,
                    })
                }
                ObjectType::TreeView => {
                    let tree = native::TreeView::new()?;
                    let template = self.tree_template()?;
                    tree.cast::<native::ITreeView2>()?
                        .SetItemTemplate(&template)?;
                    Handle::TreeView(tree)
                }
                ObjectType::TreeNode => Handle::TreeNode(NativeTreeNode {
                    value: native::TreeViewNode::new()?,
                    text: HSTRING::new(),
                    content: None,
                }),
                ObjectType::ListView => {
                    let value = native::ListView::new()?;
                    value
                        .cast::<native::IItemsControl>()?
                        .SetItemTemplate(&self.list_template()?)?;
                    Handle::ListView(value)
                }
                ObjectType::DataItem => {
                    let values = std::collections::BTreeMap::<HSTRING, Option<IInspectable>>::new();
                    Handle::Data(values.into())
                }
                _ => unreachable!("generated object was not created"),
            }
        };
        self.handles.insert(object, handle);
        Ok(())
    }

    fn set_properties(
        &mut self,
        object: ObjectId,
        set: &[Property],
        clear: &[PropertyId],
    ) -> Result<(), WinUiError> {
        let kind = self.kind(object)?;
        for property in clear {
            let feedback = GeneratedHandle::feedback_expectation(kind, *property, None);
            if let Some((event, expectation)) = feedback.clone() {
                self.event_queue
                    .feedback
                    .borrow_mut()
                    .begin(object, event, expectation);
            }
            let selection_feedback = self.selection_feedback(object, *property)?;
            if let Some((owner, event)) = selection_feedback {
                self.event_queue.feedback.borrow_mut().begin(
                    owner,
                    event,
                    FeedbackExpectation::Suppressed,
                );
            }
            let result = 'apply: {
                if let Ok(element) = self.ui_element(object)
                    && let Some(result) =
                        GeneratedHandle::set_attached_property(&element, *property, None)
                {
                    break 'apply result;
                }
                if let Ok(element) = self.ui_element(object)
                    && let Some(result) =
                        GeneratedHandle::set_visual_property(&element, *property, None)
                {
                    break 'apply result;
                }
                if let Some(Handle::Generated(handle)) = self.handles.get(&object)
                    && let Some(result) = handle.set_property(*property, None)
                {
                    break 'apply result;
                }
                if *property == PropertyId::Text
                    && matches!(self.handles.get(&object), Some(Handle::Data(_)))
                {
                    break 'apply self.set_data_text(object, "");
                }
                break 'apply match (self.handle(object)?, property) {
                    (Handle::TextBox(value), PropertyId::Text) => {
                        Self::set_text_box_text(value, "")
                    }
                    (Handle::TreeNode(value), PropertyId::Text) => {
                        value.text = HSTRING::new();
                        if value.content.is_none() {
                            let content: IInspectable =
                                windows_reference::IReference::from(value.text.clone()).into();
                            value.value.SetContent(&content)?;
                        }
                        Ok(())
                    }
                    (Handle::TreeNode(value), PropertyId::Expanded) => {
                        value.value.SetIsExpanded(false).map_err(Into::into)
                    }
                    _ => Err(WinUiError::InvalidObject(object)),
                };
            };
            let observation = feedback.and_then(|(event, _)| {
                self.event_queue.feedback.borrow_mut().finish(object, event)
            });
            if let Some((owner, event)) = selection_feedback {
                self.event_queue.feedback.borrow_mut().finish(owner, event);
            }
            result?;
            if let Some(observation) = observation {
                self.event_queue.observations.borrow_mut().push(observation);
                Self::schedule_event_wake(&self.event_queue);
            }
        }
        for property in set {
            let feedback =
                GeneratedHandle::feedback_expectation(kind, property.id, Some(&property.value));
            if let Some((event, expectation)) = feedback.clone() {
                self.event_queue
                    .feedback
                    .borrow_mut()
                    .begin(object, event, expectation);
            }
            let selection_feedback = self.selection_feedback(object, property.id)?;
            if let Some((owner, event)) = selection_feedback {
                self.event_queue.feedback.borrow_mut().begin(
                    owner,
                    event,
                    FeedbackExpectation::Suppressed,
                );
            }
            let result = 'apply: {
                if let Ok(element) = self.ui_element(object)
                    && let Some(result) = GeneratedHandle::set_attached_property(
                        &element,
                        property.id,
                        Some(&property.value),
                    )
                {
                    break 'apply result;
                }
                if let Ok(element) = self.ui_element(object)
                    && let Some(result) = GeneratedHandle::set_visual_property(
                        &element,
                        property.id,
                        Some(&property.value),
                    )
                {
                    break 'apply result;
                }
                if let Some(Handle::Generated(handle)) = self.handles.get(&object)
                    && let Some(result) = handle.set_property(property.id, Some(&property.value))
                {
                    break 'apply result;
                }
                if property.id == PropertyId::Text
                    && matches!(self.handles.get(&object), Some(Handle::Data(_)))
                    && let PropertyValue::String(text) = &property.value
                {
                    break 'apply self.set_data_text(object, text);
                }
                break 'apply match (self.handle(object)?, property.id, &property.value) {
                    (Handle::TextBox(value), PropertyId::Text, PropertyValue::String(text)) => {
                        Self::set_text_box_text(value, text)
                    }
                    (Handle::TreeNode(value), PropertyId::Text, PropertyValue::String(text)) => {
                        value.text = HSTRING::from(text.as_ref());
                        if value.content.is_none() {
                            let content: IInspectable =
                                windows_reference::IReference::from(value.text.clone()).into();
                            value.value.SetContent(&content)?;
                        }
                        Ok(())
                    }
                    (
                        Handle::TreeNode(value),
                        PropertyId::Expanded,
                        PropertyValue::Bool(expanded),
                    ) => value.value.SetIsExpanded(*expanded).map_err(Into::into),
                    _ => Err(WinUiError::InvalidObject(object)),
                };
            };
            let observation = feedback.and_then(|(event, _)| {
                self.event_queue.feedback.borrow_mut().finish(object, event)
            });
            if let Some((owner, event)) = selection_feedback {
                self.event_queue.feedback.borrow_mut().finish(owner, event);
            }
            result?;
            if let Some(observation) = observation {
                self.event_queue.observations.borrow_mut().push(observation);
                Self::schedule_event_wake(&self.event_queue);
            }
        }
        Ok(())
    }

    fn replace(&mut self, object: ObjectId, kind: ObjectType) -> Result<(), WinUiError> {
        let (parent, relation) = self
            .owners
            .get(&object)
            .copied()
            .ok_or(WinUiError::InvalidReplacement(object))?;
        let contract = relation_contract(self.kind(parent)?, relation)?;
        if contract.realization != Realization::Owned
            || contract.child != crate::object_category(kind)
        {
            return Err(WinUiError::InvalidReplacement(object));
        }
        let previous = self.ui_element(object)?;
        let index = match contract.cardinality {
            crate::Cardinality::One => {
                match (self.handle(parent)?, relation) {
                    (Handle::Generated(parent_handle), relation) => {
                        parent_handle
                            .set_content(relation, None)
                            .ok_or(WinUiError::InvalidRelation(parent, relation))??;
                    }
                    (Handle::TreeNode(parent), RelationId::Content) => {
                        let content: IInspectable =
                            windows_reference::IReference::from(parent.text.clone()).into();
                        parent.value.SetContent(&content)?;
                        parent.content = None;
                    }
                    _ => return Err(WinUiError::InvalidRelation(parent, relation)),
                }
                None
            }
            crate::Cardinality::Many => {
                let values = self.owned_collection(parent, relation)?;
                let previous: IInspectable = previous.cast()?;
                let mut index = 0;
                while index < values.size()? && values.get_at(index)? != previous {
                    index += 1;
                }
                if index == values.size()? {
                    return Err(WinUiError::ChildNotFound(object));
                }
                values.remove_at(index)?;
                Some(index)
            }
        };
        self.handles
            .remove(&object)
            .ok_or(WinUiError::MissingObject(object))?;
        self.create(object, kind)?;
        let replacement = self.ui_element(object)?;
        if let Some(index) = index {
            self.owned_collection(parent, relation)?
                .insert_at(index, &replacement.cast()?)?;
        } else {
            match (self.handle(parent)?, relation) {
                (Handle::Generated(parent_handle), relation) => {
                    parent_handle
                        .set_content(relation, Some(&replacement))
                        .ok_or(WinUiError::InvalidRelation(parent, relation))??;
                }
                (Handle::TreeNode(parent), RelationId::Content) => {
                    parent.value.SetContent(&replacement)?;
                    parent.content = Some(object);
                }
                _ => return Err(WinUiError::InvalidRelation(parent, relation)),
            }
        }
        self.event_queue
            .observations
            .borrow_mut()
            .retain(|observation| {
                !matches!(
                    observation,
                    Observation::SetProperty {
                        object: observed,
                        ..
                    } if *observed == object
                )
            });
        self.event_queue
            .events
            .borrow_mut()
            .retain(|event| event.object != object);
        Ok(())
    }

    fn dispatch_text_changed(
        event: &Rc<RefCell<NativeTextEvent>>,
        observed_text: &Rc<RefCell<Rc<str>>>,
        event_queue: &Rc<NativeEventQueue>,
        object: ObjectId,
        text: Rc<str>,
    ) {
        {
            let mut observed = observed_text.borrow_mut();
            if *observed == text {
                return;
            }
            *observed = Rc::clone(&text);
        }
        let observation = Observation::SetProperty {
            object,
            property: Property {
                id: PropertyId::Text,
                value: PropertyValue::String(Rc::clone(&text)),
            },
        };
        if !event_queue.observe(object, EventId::TextChanged, observation.clone()) {
            return;
        }
        event_queue.observations.borrow_mut().push(observation);
        let event = event.borrow();
        if event.callback.is_some() {
            event_queue.events.borrow_mut().push(QueuedEvent {
                object,
                event: EventId::TextChanged,
                revision: event.revision,
                payload: EventPayload::String(text),
            });
        }
        Self::schedule_event_wake(event_queue);
    }

    fn dispatch_unit(
        event: &Rc<RefCell<NativeUnitEvent>>,
        event_queue: &Rc<NativeEventQueue>,
        object: ObjectId,
        event_id: EventId,
    ) {
        let event = event.borrow();
        if event.callback.is_some() {
            event_queue.events.borrow_mut().push(QueuedEvent {
                object,
                event: event_id,
                revision: event.revision,
                payload: EventPayload::Unit,
            });
            Self::schedule_event_wake(event_queue);
        }
    }

    fn dispatch_bool(
        event: &Rc<RefCell<NativeBoolEvent>>,
        event_queue: &Rc<NativeEventQueue>,
        object: ObjectId,
        event_id: EventId,
        value: bool,
    ) {
        let event = event.borrow();
        if event.callback.is_some() {
            event_queue.events.borrow_mut().push(QueuedEvent {
                object,
                event: event_id,
                revision: event.revision,
                payload: EventPayload::Bool(value),
            });
            Self::schedule_event_wake(event_queue);
        }
    }

    fn dispatch_f64(
        event: &Rc<RefCell<NativeF64Event>>,
        event_queue: &Rc<NativeEventQueue>,
        object: ObjectId,
        event_id: EventId,
        value: f64,
    ) {
        let event = event.borrow();
        if event.callback.is_some() {
            event_queue.events.borrow_mut().push(QueuedEvent {
                object,
                event: event_id,
                revision: event.revision,
                payload: EventPayload::F64(value),
            });
            Self::schedule_event_wake(event_queue);
        }
    }

    fn dispatch_optional_bool(
        event: &Rc<RefCell<NativeOptionalBoolEvent>>,
        event_queue: &Rc<NativeEventQueue>,
        object: ObjectId,
        event_id: EventId,
        value: Option<bool>,
    ) {
        let event = event.borrow();
        if event.callback.is_some() {
            event_queue.events.borrow_mut().push(QueuedEvent {
                object,
                event: event_id,
                revision: event.revision,
                payload: EventPayload::OptionalBool(value),
            });
            Self::schedule_event_wake(event_queue);
        }
    }

    fn dispatch_optional_f64(
        event: &Rc<RefCell<NativeOptionalF64Event>>,
        event_queue: &Rc<NativeEventQueue>,
        object: ObjectId,
        event_id: EventId,
        value: Option<f64>,
    ) {
        let event = event.borrow();
        if event.callback.is_some() {
            event_queue.events.borrow_mut().push(QueuedEvent {
                object,
                event: event_id,
                revision: event.revision,
                payload: EventPayload::OptionalF64(value),
            });
            Self::schedule_event_wake(event_queue);
        }
    }

    fn dispatch_selection(
        event: &Rc<RefCell<NativeSelectionEvent>>,
        event_queue: &Rc<NativeEventQueue>,
        object: ObjectId,
        event_id: EventId,
        item: Option<ObjectId>,
        value: Option<Rc<str>>,
    ) {
        let event = event.borrow();
        if event.callback.is_some() {
            event_queue.events.borrow_mut().push(QueuedEvent {
                object,
                event: event_id,
                revision: event.revision,
                payload: EventPayload::Selection(crate::SelectionChange { item, value }),
            });
            Self::schedule_event_wake(event_queue);
        }
    }

    fn handle_selection_changed(
        event: &Rc<RefCell<NativeSelectionEvent>>,
        event_queue: &Rc<NativeEventQueue>,
        object: ObjectId,
        event_id: EventId,
        selected: windows_core::Result<IInspectable>,
        payload_property: PropertyId,
    ) {
        let selected = match selected {
            Ok(selected) => Some(selected),
            Err(error) if error.code().is_ok() => None,
            Err(error) => {
                super::app::report_error(error);
                return;
            }
        };
        let selected_object = selected.as_ref().and_then(|selected| {
            event_queue
                .selection_items
                .borrow()
                .iter()
                .find_map(|item| {
                    (item.owner == object && item.value == *selected).then_some(item.object)
                })
        });
        let observation = Observation::SetSelection {
            object,
            selected: selected_object,
        };
        if !event_queue.observe(object, event_id, observation.clone()) {
            return;
        }
        event_queue.observations.borrow_mut().push(observation);
        let value = match selected.as_ref() {
            Some(selected) => {
                match GeneratedHandle::selection_payload(payload_property, selected) {
                    Ok(value) => value,
                    Err(error) => {
                        super::app::report_error(error.into());
                        return;
                    }
                }
            }
            None => None,
        };
        Self::dispatch_selection(event, event_queue, object, event_id, selected_object, value);
        Self::schedule_event_wake(event_queue);
    }

    fn dispatch_selection_index(
        event: &Rc<RefCell<NativeSelectionIndexEvent>>,
        event_queue: &Rc<NativeEventQueue>,
        object: ObjectId,
        event_id: EventId,
        value: Option<usize>,
    ) {
        let event = event.borrow();
        if event.callback.is_some() {
            event_queue.events.borrow_mut().push(QueuedEvent {
                object,
                event: event_id,
                revision: event.revision,
                payload: EventPayload::SelectionIndex(value),
            });
            Self::schedule_event_wake(event_queue);
        }
    }

    fn dispatch_string(
        event: &Rc<RefCell<NativeTextEvent>>,
        event_queue: &Rc<NativeEventQueue>,
        object: ObjectId,
        event_id: EventId,
        value: Rc<str>,
    ) {
        let event = event.borrow();
        if event.callback.is_some() {
            event_queue.events.borrow_mut().push(QueuedEvent {
                object,
                event: event_id,
                revision: event.revision,
                payload: EventPayload::String(value),
            });
            Self::schedule_event_wake(event_queue);
        }
    }

    fn dispatch_pointer_event_info(
        event: &Rc<RefCell<NativePointerEventInfoEvent>>,
        event_queue: &Rc<NativeEventQueue>,
        object: ObjectId,
        event_id: EventId,
        value: crate::PointerEventInfo,
    ) {
        if Self::queue_pointer_event_info(event, event_queue, object, event_id, value) {
            Self::schedule_event_wake(event_queue);
        }
    }

    fn queue_pointer_event_info(
        event: &Rc<RefCell<NativePointerEventInfoEvent>>,
        event_queue: &Rc<NativeEventQueue>,
        object: ObjectId,
        event_id: EventId,
        value: crate::PointerEventInfo,
    ) -> bool {
        let event = event.borrow();
        if event.callback.is_some() {
            event_queue.events.borrow_mut().push(QueuedEvent {
                object,
                event: event_id,
                revision: event.revision,
                payload: EventPayload::PointerEventInfo(value),
            });
            true
        } else {
            false
        }
    }

    fn pointer_event_info(
        element: &native::UIElement,
        args: windows_core::Ref<native::PointerRoutedEventArgs>,
    ) -> Result<crate::PointerEventInfo, WinUiError> {
        let Some(args) = args.as_ref() else {
            return Err(WinUiError::InvalidEventArgs);
        };
        let local = args.GetCurrentPoint(element)?;
        let local_position = local.Position()?;
        let window = args.GetCurrentPoint(None::<&native::UIElement>)?;
        let window_position = window.Position()?;
        let properties = local.Properties()?;
        let pointer = args.Pointer()?;
        let mut capture_index = 0;
        let is_captured = element
            .cast::<native::IUIElement>()?
            .PointerCaptures()
            .ok()
            .and_then(|captures| captures.IndexOf(&pointer, &mut capture_index).ok())
            .unwrap_or(false);
        Ok(crate::PointerEventInfo {
            x: f64::from(local_position.x),
            y: f64::from(local_position.y),
            window_x: f64::from(window_position.x),
            window_y: f64::from(window_position.y),
            pointer_id: local.PointerId()?,
            capture_succeeded: None,
            is_captured,
            is_left_button_pressed: properties.IsLeftButtonPressed()?,
            is_right_button_pressed: properties.IsRightButtonPressed()?,
            is_middle_button_pressed: properties.IsMiddleButtonPressed()?,
        })
    }

    fn schedule_event_wake(event_queue: &Rc<NativeEventQueue>) {
        if event_queue.wake_pending.replace(true) {
            return;
        }
        let event_queue = Rc::clone(event_queue);
        let queue = match native::DispatcherQueue::GetForCurrentThread() {
            Ok(queue) => queue,
            Err(error) => {
                event_queue.wake_pending.set(false);
                super::app::report_error(error);
                return;
            }
        };
        let event_queue_for_handler = Rc::clone(&event_queue);
        let handler = native::DispatcherQueueHandler::new(move || {
            event_queue_for_handler.wake_pending.set(false);
            let waker = event_queue_for_handler.waker.borrow().clone();
            if let Some(waker) = waker {
                waker();
            }
        });
        match queue.TryEnqueueWithPriority(native::DispatcherQueuePriority::Normal, &handler) {
            Ok(true) => {}
            Ok(false) => {
                event_queue.wake_pending.set(false);
                super::app::report_error(windows_core::Error::new(
                    native::E_FAIL,
                    "DispatcherQueue rejected the Reactor2 event wake",
                ));
            }
            Err(error) => {
                event_queue.wake_pending.set(false);
                super::app::report_error(error);
            }
        }
    }

    fn set_events(
        &mut self,
        object: ObjectId,
        set: &[Event],
        clear: &[EventId],
    ) -> Result<(), WinUiError> {
        match self.handle(object)? {
            Handle::Generated(value) => {
                return value
                    .set_events(object, set, clear)
                    .unwrap_or(Err(WinUiError::InvalidObject(object)));
            }
            Handle::TextBox(value) => {
                let mut native_event = value.event.borrow_mut();
                if clear.contains(&EventId::TextChanged) {
                    native_event.revision = native_event.revision.wrapping_add(1);
                    native_event.callback = None;
                }
                for event in set {
                    match (event.id, &event.value) {
                        (EventId::TextChanged, EventValue::String(callback)) => {
                            native_event.revision = native_event.revision.wrapping_add(1);
                            native_event.callback = Some(callback.clone());
                        }
                        _ => return Err(WinUiError::InvalidObject(object)),
                    }
                }
            }
            _ => return Err(WinUiError::InvalidObject(object)),
        }
        Ok(())
    }

    fn set_text_box_text(value: &NativeTextBox, text: &str) -> Result<(), WinUiError> {
        if value.value.Text()? == text {
            return Ok(());
        }
        let selection_start = value.value.SelectionStart()?;
        let selection_length = value.value.SelectionLength()?;
        value.value.SetText(text)?;
        *value.observed_text.borrow_mut() = Rc::from(text);
        value.set_count.set(value.set_count.get() + 1);
        let text_length = i32::try_from(text.encode_utf16().count()).unwrap_or(i32::MAX);
        let selection_start = selection_start.clamp(0, text_length);
        let selection_length = selection_length.clamp(0, text_length - selection_start);
        value.value.SetSelectionStart(selection_start)?;
        value.value.SetSelectionLength(selection_length)?;
        Ok(())
    }

    fn attach(
        &mut self,
        parent: ObjectId,
        relation: RelationId,
        child: ObjectId,
    ) -> Result<(), WinUiError> {
        if self.owners.contains_key(&child) {
            return Err(WinUiError::StillOwned(child));
        }
        let child_element = self.ui_element(child)?;
        match (self.handle(parent)?, relation) {
            (Handle::Generated(value), relation) => {
                value
                    .set_content(relation, Some(&child_element))
                    .ok_or(WinUiError::InvalidRelation(parent, relation))??;
            }
            (Handle::TreeNode(parent), RelationId::Content) => {
                parent.value.SetContent(&child_element)?;
                parent.content = Some(child);
            }
            _ => return Err(WinUiError::InvalidRelation(parent, relation)),
        }
        self.owners.insert(child, (parent, relation));
        Ok(())
    }

    fn detach(
        &mut self,
        parent: ObjectId,
        relation: RelationId,
        child: ObjectId,
    ) -> Result<(), WinUiError> {
        if self.owners.get(&child) != Some(&(parent, relation)) {
            return Err(WinUiError::ChildNotFound(child));
        }
        match (self.handle(parent)?, relation) {
            (Handle::Generated(value), relation) => {
                value
                    .set_content(relation, None)
                    .ok_or(WinUiError::InvalidRelation(parent, relation))??;
            }
            (Handle::TreeNode(parent), RelationId::Content) => {
                let content: IInspectable =
                    windows_reference::IReference::from(parent.text.clone()).into();
                parent.value.SetContent(&content)?;
                parent.content = None;
            }
            _ => return Err(WinUiError::InvalidRelation(parent, relation)),
        }
        self.owners.remove(&child);
        Ok(())
    }

    fn insert(
        &mut self,
        parent: ObjectId,
        relation: RelationId,
        child: ObjectId,
        index: usize,
    ) -> Result<(), WinUiError> {
        if self.owners.contains_key(&child) {
            return Err(WinUiError::StillOwned(child));
        }
        let selection = selection_for_relation(self.kind(parent)?, relation);
        let selected = selection
            .map(|selection| self.read_selected_item(parent, selection))
            .transpose()?
            .flatten();
        let selection_item =
            selection.filter(|selection| self.kind(child).is_ok_and(|kind| kind == selection.item));
        let child_value = selection_item
            .map(|_| {
                self.ui_element(child)
                    .and_then(|value| value.cast::<IInspectable>().map_err(Into::into))
            })
            .transpose()?;
        let child_selected = selection_item
            .map(|selection| self.selection_item_is_selected(child, selection))
            .transpose()?
            .unwrap_or(false);
        let feedback = self.begin_selection_feedback(parent, selection);
        let result = (|| {
            match relation_contract(self.kind(parent)?, relation)?.realization {
                Realization::Owned => {
                    self.owned_collection(parent, relation)?
                        .insert_at(index32(index)?, &self.ui_element(child)?.cast()?)?;
                }
                Realization::Structural => {
                    self.tree_nodes(parent)?
                        .InsertAt(index32(index)?, &self.tree_node(child)?)?;
                }
                Realization::Container => {
                    self.items(parent)?
                        .InsertAt(index32(index)?, &self.inspectable(child)?)?;
                }
            }
            self.owners.insert(child, (parent, relation));
            if let (Some(selection), Some(value)) = (selection_item, child_value.as_ref()) {
                self.event_queue
                    .selection_items
                    .borrow_mut()
                    .push(NativeSelectionItem {
                        owner: parent,
                        object: child,
                        value: value.clone(),
                    });
                if child_selected {
                    self.write_selected_item(parent, selection, Some(value))?;
                } else if let Some(selected) = selected.as_ref() {
                    self.write_selected_item(parent, selection, Some(selected))?;
                }
            }
            Ok(())
        })();
        self.finish_selection_feedback(parent, feedback);
        result
    }

    fn remove(
        &mut self,
        parent: ObjectId,
        relation: RelationId,
        child: ObjectId,
        index: usize,
    ) -> Result<(), WinUiError> {
        if self.owners.get(&child) != Some(&(parent, relation)) {
            return Err(WinUiError::ChildNotFound(child));
        }
        let selection = selection_for_relation(self.kind(parent)?, relation);
        let selected = selection
            .map(|selection| self.read_selected_item(parent, selection))
            .transpose()?
            .flatten();
        let selection_item =
            selection.filter(|selection| self.kind(child).is_ok_and(|kind| kind == selection.item));
        let child_value = selection_item
            .map(|_| {
                self.ui_element(child)
                    .and_then(|value| value.cast::<IInspectable>().map_err(Into::into))
            })
            .transpose()?;
        let feedback = self.begin_selection_feedback(parent, selection);
        let result = (|| {
            match relation_contract(self.kind(parent)?, relation)?.realization {
                Realization::Owned => {
                    let values = self.owned_collection(parent, relation)?;
                    if values.get_at(index32(index)?)? != self.ui_element(child)?.cast()? {
                        return Err(WinUiError::ChildNotFound(child));
                    }
                    values.remove_at(index32(index)?)?;
                }
                Realization::Structural => {
                    let values = self.tree_nodes(parent)?;
                    if values.GetAt(index32(index)?)? != self.tree_node(child)? {
                        return Err(WinUiError::ChildNotFound(child));
                    }
                    values.RemoveAt(index32(index)?)?;
                }
                Realization::Container => {
                    let values = self.items(parent)?;
                    if values.GetAt(index32(index)?)? != self.inspectable(child)? {
                        return Err(WinUiError::ChildNotFound(child));
                    }
                    values.RemoveAt(index32(index)?)?;
                }
            }
            self.owners.remove(&child);
            if let Some(selection) = selection {
                self.event_queue
                    .selection_items
                    .borrow_mut()
                    .retain(|item| item.object != child);
                if let Some(selected) = selected.as_ref()
                    && child_value.as_ref() != Some(selected)
                {
                    self.write_selected_item(parent, selection, Some(selected))?;
                }
            }
            Ok(())
        })();
        self.finish_selection_feedback(parent, feedback);
        result
    }

    fn reorder(
        &mut self,
        parent: ObjectId,
        relation: RelationId,
        moves: &[crate::Move],
        children: &[ObjectId],
    ) -> Result<(), WinUiError> {
        let selection = selection_for_relation(self.kind(parent)?, relation);
        let selected = selection
            .map(|selection| self.read_selected_item(parent, selection))
            .transpose()?
            .flatten();
        let feedback = self.begin_selection_feedback(parent, selection);
        let result = (|| {
            match relation_contract(self.kind(parent)?, relation)?.realization {
                Realization::Owned => {
                    let values = self.owned_collection(parent, relation)?;
                    for movement in moves {
                        let child: IInspectable = self.ui_element(movement.child)?.cast()?;
                        let mut from = 0;
                        while from < values.size()? && values.get_at(from)? != child {
                            from += 1;
                        }
                        if from == values.size()? {
                            return Err(WinUiError::ChildNotFound(movement.child));
                        }
                        values.remove_at(from)?;
                        let target = if let Some(before) = movement.before {
                            let before: IInspectable = self.ui_element(before)?.cast()?;
                            let mut target = 0;
                            while target < values.size()? && values.get_at(target)? != before {
                                target += 1;
                            }
                            if target == values.size()? {
                                return Err(WinUiError::ChildNotFound(movement.child));
                            }
                            target
                        } else {
                            values.size()?
                        };
                        values.insert_at(target, &child)?;
                    }
                }
                Realization::Structural => {
                    let tree = self.tree_owner(parent)?;
                    let template = self.tree_template()?;
                    let tree2 = tree.cast::<native::ITreeView2>()?;
                    tree2.SetItemTemplate(None::<&native::DataTemplate>)?;
                    tree.cast::<native::IUIElement>()?.UpdateLayout()?;
                    let values = self.tree_nodes(parent)?;
                    for movement in moves {
                        let child = self.tree_node(movement.child)?;
                        let mut previous = 0;
                        if !values.IndexOf(&child, &mut previous)? {
                            return Err(WinUiError::ChildNotFound(movement.child));
                        }
                        values.RemoveAt(previous)?;
                        let index = if let Some(before) = movement.before {
                            let before = self.tree_node(before)?;
                            let mut index = 0;
                            if !values.IndexOf(&before, &mut index)? {
                                return Err(WinUiError::ChildNotFound(movement.child));
                            }
                            index
                        } else {
                            values.Size()?
                        };
                        values.InsertAt(index, &child)?;
                    }
                    tree2.SetItemTemplate(&template)?;
                }
                Realization::Container => {
                    let values = self.items(parent)?;
                    if moves.len().saturating_add(1) >= children.len() {
                        let children = children
                            .iter()
                            .map(|child| self.inspectable(*child).map(Some))
                            .collect::<Result<Vec<_>, _>>()?;
                        values.ReplaceAll(&children)?;
                    } else {
                        for movement in moves {
                            let child = self.inspectable(movement.child)?;
                            let mut previous = 0;
                            if !values.IndexOf(&child, &mut previous)? {
                                return Err(WinUiError::ChildNotFound(movement.child));
                            }
                            values.RemoveAt(previous)?;
                            let index = if let Some(before) = movement.before {
                                let before = self.inspectable(before)?;
                                let mut index = 0;
                                if !values.IndexOf(&before, &mut index)? {
                                    return Err(WinUiError::ChildNotFound(movement.child));
                                }
                                index
                            } else {
                                values.Size()?
                            };
                            values.InsertAt(index, &child)?;
                        }
                    }
                }
            }
            if let (Some(selection), Some(selected)) = (selection, selected.as_ref()) {
                self.write_selected_item(parent, selection, Some(selected))?;
            }
            Ok(())
        })();
        self.finish_selection_feedback(parent, feedback);
        result?;
        debug_assert_eq!(
            children.len(),
            self.owners
                .values()
                .filter(|owner| **owner == (parent, relation))
                .count()
        );
        Ok(())
    }

    fn selection_feedback(
        &self,
        object: ObjectId,
        property: PropertyId,
    ) -> Result<Option<(ObjectId, EventId)>, WinUiError> {
        let Some((owner, relation)) = self.owners.get(&object).copied() else {
            return Ok(None);
        };
        Ok(
            selection_for_item_property(self.kind(owner)?, relation, self.kind(object)?, property)
                .map(|selection| (owner, selection.event)),
        )
    }

    fn begin_selection_feedback(
        &self,
        owner: ObjectId,
        selection: Option<SelectionContract>,
    ) -> Option<EventId> {
        let event = selection.map(|selection| selection.event)?;
        self.event_queue
            .feedback
            .borrow_mut()
            .begin(owner, event, FeedbackExpectation::Suppressed);
        Some(event)
    }

    fn finish_selection_feedback(&self, owner: ObjectId, event: Option<EventId>) {
        if let Some(event) = event {
            self.event_queue.feedback.borrow_mut().finish(owner, event);
        }
    }

    fn read_selected_item(
        &self,
        object: ObjectId,
        selection: SelectionContract,
    ) -> Result<Option<IInspectable>, WinUiError> {
        let Some(Handle::Generated(handle)) = self.handles.get(&object) else {
            return Err(WinUiError::InvalidObject(object));
        };
        handle
            .selected_item(selection.event)
            .unwrap_or(Err(WinUiError::InvalidObject(object)))
    }

    fn write_selected_item(
        &self,
        object: ObjectId,
        selection: SelectionContract,
        selected: Option<&IInspectable>,
    ) -> Result<(), WinUiError> {
        let Some(Handle::Generated(handle)) = self.handles.get(&object) else {
            return Err(WinUiError::InvalidObject(object));
        };
        handle
            .set_selected_item(selection.event, selected)
            .unwrap_or(Err(WinUiError::InvalidObject(object)))
    }

    fn selection_item_is_selected(
        &self,
        object: ObjectId,
        selection: SelectionContract,
    ) -> Result<bool, WinUiError> {
        let Some(Handle::Generated(handle)) = self.handles.get(&object) else {
            return Err(WinUiError::InvalidObject(object));
        };
        handle
            .selection_item_is_selected(selection.selected_property)
            .unwrap_or(Err(WinUiError::InvalidObject(object)))
    }

    fn handle(&mut self, object: ObjectId) -> Result<&mut Handle, WinUiError> {
        self.handles
            .get_mut(&object)
            .ok_or(WinUiError::MissingObject(object))
    }

    fn kind(&self, object: ObjectId) -> Result<ObjectType, WinUiError> {
        Ok(
            match self
                .handles
                .get(&object)
                .ok_or(WinUiError::MissingObject(object))?
            {
                Handle::Generated(value) => value.kind(),
                Handle::TextBox(_) => ObjectType::TextBox,
                Handle::TreeView(_) => ObjectType::TreeView,
                Handle::TreeNode(_) => ObjectType::TreeNode,
                Handle::ListView(_) => ObjectType::ListView,
                Handle::Data(_) => ObjectType::DataItem,
            },
        )
    }

    fn ui_element(&self, object: ObjectId) -> Result<native::UIElement, WinUiError> {
        match self
            .handles
            .get(&object)
            .ok_or(WinUiError::MissingObject(object))?
        {
            Handle::Generated(value) => value.ui_element(),
            Handle::TextBox(value) => Ok(value.value.cast()?),
            Handle::TreeView(value) => Ok(value.cast()?),
            Handle::ListView(value) => Ok(value.cast()?),
            Handle::TreeNode(_) | Handle::Data(_) => Err(WinUiError::InvalidObject(object)),
        }
    }

    fn inspectable(&self, object: ObjectId) -> Result<IInspectable, WinUiError> {
        match self
            .handles
            .get(&object)
            .ok_or(WinUiError::MissingObject(object))?
        {
            Handle::Data(value) => Ok(value.cast()?),
            _ => Err(WinUiError::InvalidObject(object)),
        }
    }

    fn tree_node(&self, object: ObjectId) -> Result<native::TreeViewNode, WinUiError> {
        match self
            .handles
            .get(&object)
            .ok_or(WinUiError::MissingObject(object))?
        {
            Handle::TreeNode(value) => Ok(value.value.clone()),
            _ => Err(WinUiError::InvalidObject(object)),
        }
    }

    fn owned_collection(
        &self,
        object: ObjectId,
        relation: RelationId,
    ) -> Result<GeneratedCollection, WinUiError> {
        match self
            .handles
            .get(&object)
            .ok_or(WinUiError::MissingObject(object))?
        {
            Handle::Generated(value) => value
                .owned_collection(relation)
                .unwrap_or(Err(WinUiError::InvalidObject(object))),
            _ => Err(WinUiError::InvalidObject(object)),
        }
    }

    fn tree_nodes(
        &self,
        object: ObjectId,
    ) -> Result<windows_collections::IVector<native::TreeViewNode>, WinUiError> {
        match self
            .handles
            .get(&object)
            .ok_or(WinUiError::MissingObject(object))?
        {
            Handle::TreeView(value) => Ok(value.RootNodes()?),
            Handle::TreeNode(value) => Ok(value.value.Children()?),
            _ => Err(WinUiError::InvalidObject(object)),
        }
    }

    fn tree_owner(&self, object: ObjectId) -> Result<native::TreeView, WinUiError> {
        let mut current = object;
        loop {
            match self
                .handles
                .get(&current)
                .ok_or(WinUiError::MissingObject(current))?
            {
                Handle::TreeView(tree) => return Ok(tree.clone()),
                Handle::TreeNode(_) => {
                    current = self
                        .owners
                        .get(&current)
                        .map(|(parent, _)| *parent)
                        .ok_or(WinUiError::StillOwned(current))?;
                }
                _ => return Err(WinUiError::InvalidObject(current)),
            }
        }
    }

    fn items(&self, object: ObjectId) -> Result<native::ItemCollection, WinUiError> {
        match self
            .handles
            .get(&object)
            .ok_or(WinUiError::MissingObject(object))?
        {
            Handle::ListView(value) => Ok(value.cast::<native::IItemsControl>()?.Items()?),
            _ => Err(WinUiError::InvalidObject(object)),
        }
    }

    fn set_data_text(&mut self, object: ObjectId, text: &str) -> Result<(), WinUiError> {
        let key = self.data_text_key.clone();
        let Handle::Data(value) = self.handle(object)? else {
            return Err(WinUiError::InvalidObject(object));
        };
        let properties = value.cast::<windows_collections::IMap<HSTRING, IInspectable>>()?;
        let text: IInspectable = windows_reference::IReference::from(HSTRING::from(text)).into();
        properties.Insert(&key, &text)?;
        Ok(())
    }

    fn tree_template(&mut self) -> Result<native::DataTemplate, WinUiError> {
        if let Some(template) = &self.tree_template {
            return Ok(template.clone());
        }
        let template = native::XamlReader::Load(
            "<DataTemplate xmlns='http://schemas.microsoft.com/winfx/2006/xaml/presentation'>\
             <ContentPresenter Content='{Binding Content}'/>\
             </DataTemplate>",
        )?
        .cast::<native::DataTemplate>()?;
        self.tree_template = Some(template.clone());
        Ok(template)
    }

    fn list_template(&mut self) -> Result<native::DataTemplate, WinUiError> {
        if let Some(template) = &self.list_template {
            return Ok(template.clone());
        }
        let template = native::XamlReader::Load(
            "<DataTemplate xmlns='http://schemas.microsoft.com/winfx/2006/xaml/presentation'>\
             <TextBlock Text='{Binding [Text]}'/>\
             </DataTemplate>",
        )?
        .cast::<native::DataTemplate>()?;
        self.list_template = Some(template.clone());
        Ok(template)
    }
}

impl Adapter for WinUiAdapter {
    type Error = WinUiError;

    fn drain_observations(&mut self, observations: &mut Vec<Observation>) {
        observations.append(&mut self.event_queue.observations.borrow_mut());
    }

    fn drain_events(&mut self, events: &mut Vec<EventDispatch>) {
        for queued in self.event_queue.events.borrow_mut().drain(..) {
            let callback = match (self.handles.get(&queued.object), queued.event) {
                (Some(Handle::TextBox(text_box)), EventId::TextChanged) => {
                    let event = text_box.event.borrow();
                    if event.revision != queued.revision {
                        continue;
                    }
                    event.callback.clone().map(EventValue::String)
                }
                (Some(Handle::Generated(handle)), event) => {
                    handle.event_callback(event, queued.revision)
                }
                _ => None,
            };
            let Some(callback) = callback else {
                continue;
            };
            events.push(EventDispatch::new(
                queued.object,
                queued.event,
                callback,
                queued.payload,
            ));
        }
    }

    fn validate(&self, _mutations: &[Mutation]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn apply(&mut self, mutations: &[Mutation]) -> Result<(), Self::Error> {
        for mutation in mutations {
            match mutation {
                Mutation::Create { object, kind } => self.create(*object, *kind)?,
                Mutation::Replace { object, kind } => self.replace(*object, *kind)?,
                Mutation::SetProperties { object, set, clear } => {
                    self.set_properties(*object, set, clear)?;
                }
                Mutation::SetEvents { object, set, clear } => {
                    self.set_events(*object, set, clear)?;
                }
                Mutation::Attach {
                    parent,
                    relation,
                    child,
                } => self.attach(*parent, *relation, *child)?,
                Mutation::Detach {
                    parent,
                    relation,
                    child,
                } => self.detach(*parent, *relation, *child)?,
                Mutation::Insert {
                    parent,
                    relation,
                    child,
                    index,
                } => self.insert(*parent, *relation, *child, *index)?,
                Mutation::Remove {
                    parent,
                    relation,
                    child,
                    index,
                } => self.remove(*parent, *relation, *child, *index)?,
                Mutation::Reorder {
                    parent,
                    relation,
                    moves,
                    children,
                } => self.reorder(*parent, *relation, moves, children)?,
                Mutation::Destroy { object } => {
                    if self.owners.contains_key(object) {
                        return Err(WinUiError::StillOwned(*object));
                    }
                    self.handles
                        .remove(object)
                        .ok_or(WinUiError::MissingObject(*object))?;
                    self.event_queue
                        .feedback
                        .borrow_mut()
                        .remove_object(*object);
                    self.event_queue
                        .selection_items
                        .borrow_mut()
                        .retain(|item| item.object != *object && item.owner != *object);
                }
            }
        }
        Ok(())
    }
}

fn relation_contract(
    kind: ObjectType,
    relation: RelationId,
) -> Result<&'static RelationContract, WinUiError> {
    relation_contracts(kind)
        .iter()
        .find(|contract| contract.id == relation)
        .ok_or(WinUiError::MissingContract(kind, relation))
}

fn index32(index: usize) -> Result<u32, WinUiError> {
    index
        .try_into()
        .map_err(|_| WinUiError::IndexOverflow(index))
}

fn native_selection_index(value: Option<usize>) -> Result<i32, WinUiError> {
    value
        .map(|value| i32::try_from(value).map_err(|_| WinUiError::IndexOverflow(value)))
        .transpose()
        .map(|value| value.unwrap_or(-1))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn policy_preserves_requested_window_state() {
        let policy = WindowPolicy::new()
            .title("Solitaire")
            .theme(WindowTheme::Dark)
            .client_size(800.0, 600.0)
            .minimum_client_size(800.0, 600.0);

        assert_eq!(policy.title.as_deref(), Some("Solitaire"));
        assert_eq!(policy.theme, WindowTheme::Dark);
        assert_eq!(policy.client_size, Some((800.0, 600.0)));
        assert_eq!(policy.minimum_client_size, Some((800.0, 600.0)));
    }

    #[test]
    fn policy_rejects_invalid_window_sizes() {
        for (width, height) in [
            (0.0, 1.0),
            (1.0, -1.0),
            (f64::NAN, 1.0),
            (1.0, f64::INFINITY),
        ] {
            assert!(
                std::panic::catch_unwind(|| WindowPolicy::new().client_size(width, height))
                    .is_err()
            );
            assert!(
                std::panic::catch_unwind(|| {
                    WindowPolicy::new().minimum_client_size(width, height)
                })
                .is_err()
            );
        }
    }

    #[test]
    fn pointer_event_queue_preserves_payload_and_revision() {
        let mut runtime = crate::Runtime::new(crate::RecordingAdapter::default());
        runtime.update(crate::Border::new()).unwrap();
        let object = runtime.graph().root().unwrap();
        let event = Rc::new(RefCell::new(NativePointerEventInfoEvent {
            revision: 7,
            callback: Some(crate::Callback::new(|_| {})),
        }));
        let event_queue = Rc::new(NativeEventQueue::default());
        let payload = crate::PointerEventInfo {
            pointer_id: 42,
            is_captured: true,
            is_right_button_pressed: true,
            ..Default::default()
        };

        assert!(WinUiAdapter::queue_pointer_event_info(
            &event,
            &event_queue,
            object,
            EventId::PointerReleased,
            payload,
        ));
        let queued = event_queue.events.borrow();
        assert_eq!(queued.len(), 1);
        assert_eq!(queued[0].object, object);
        assert_eq!(queued[0].event, EventId::PointerReleased);
        assert_eq!(queued[0].revision, 7);
        assert_eq!(queued[0].payload, EventPayload::PointerEventInfo(payload));
    }

    #[test]
    fn optional_numeric_adapters_use_distinct_sentinels() {
        assert!(native_number_box_value(None).is_nan());
        assert_eq!(number_box_value(f64::NAN), None);
        assert_eq!(number_box_value(-1.0), Some(-1.0));

        assert_eq!(native_rating_value(None), -1.0);
        assert_eq!(rating_value(-1.0), None);
        assert!(rating_value(f64::NAN).unwrap().is_nan());
    }

    #[test]
    fn selection_index_conversion_reports_overflow() {
        assert_eq!(native_selection_index(None), Ok(-1));
        assert_eq!(
            native_selection_index(Some(i32::MAX as usize)),
            Ok(i32::MAX)
        );
        let overflow = i32::MAX as usize + 1;
        assert_eq!(
            native_selection_index(Some(overflow)),
            Err(WinUiError::IndexOverflow(overflow))
        );
    }

    #[test]
    fn generated_feedback_contracts_distinguish_exact_normalized_and_clear() {
        let exact = GeneratedHandle::feedback_expectation(
            ObjectType::ToggleSwitch,
            PropertyId::IsOn,
            Some(&PropertyValue::Bool(true)),
        );
        assert_eq!(
            exact,
            Some((
                EventId::Toggled,
                FeedbackExpectation::Exact(Property {
                    id: PropertyId::IsOn,
                    value: PropertyValue::Bool(true),
                })
            ))
        );
        assert_eq!(
            GeneratedHandle::feedback_expectation(ObjectType::ToggleSwitch, PropertyId::IsOn, None,),
            Some((
                EventId::Toggled,
                FeedbackExpectation::Exact(Property {
                    id: PropertyId::IsOn,
                    value: PropertyValue::Bool(false),
                })
            ))
        );
        assert_eq!(
            GeneratedHandle::feedback_expectation(
                ObjectType::Slider,
                PropertyId::Value,
                Some(&PropertyValue::F64(0.5)),
            ),
            Some((
                EventId::ValueChanged,
                FeedbackExpectation::Normalized { observation: None }
            ))
        );
        assert_eq!(
            GeneratedHandle::feedback_expectation(
                ObjectType::ProgressBar,
                PropertyId::Value,
                Some(&PropertyValue::F64(0.5)),
            ),
            None
        );
    }
}
