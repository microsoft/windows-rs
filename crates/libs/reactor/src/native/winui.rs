use super::bindings as native;
use crate::reconcile::{FeedbackExpectation, FeedbackState};
use crate::{
    AcceleratorKey, AcceleratorModifiers, Adapter, Brush, ButtonStyle, Callback, ColorScheme,
    CommandBarCommand, CommandBarFlyout, ComponentHost, CompositionHostEvent, ContentDialogResult,
    DragDropPolicy, ElementFocusState, EncodedImage, Event, EventDispatch, EventId, EventPayload,
    EventValue, FlyoutPlacement, GridLength, GridLengthSize, ImageSourceValue, ImperativeRequest,
    IntegrationError, KeyAccelerators, Menu, MenuItem, Mutation, NativeEvent, ObjectId, ObjectType,
    Observation, Property, PropertyId, PropertyValue, Realization, RealizationRequest,
    RealizedContainer, RelationContract, RelationId, ResourceOverrides, ResourceValue,
    RetirementCompletion, Runtime, SelectionContract, SwapChainPanelEvent, ThemeBrush,
    TooltipPlacement, WindowBackdrop, WindowPolicy, WindowSize, WindowTheme, WindowVisuals,
    relation_contracts, selection_for_item_property, selection_for_relation,
};
use native::IElementFactory;
use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::rc::{Rc, Weak};
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use windows_collections::{
    CollectionChange, IIterable_Impl, IIterator_Impl, IObservableVector_Impl, IVector_Impl,
    IVectorChangedEventArgs_Impl, IVectorView_Impl, VectorChangedEventHandler,
};
use windows_core::{
    ComObject, Event as WinEvent, HRESULT, HSTRING, IInspectable, IUnknownImpl, Interface, Ref,
    implement_decl,
};

enum Handle {
    Generated(GeneratedHandle),
    TextBox(NativeTextBox),
    TreeView(NativeTreeView),
    TreeNode(NativeTreeNode),
    ListView(NativeListView),
    Data(windows_collections::IObservableMap<HSTRING, IInspectable>),
}

struct NativeListView {
    value: native::ListView,
    selection_changed: Rc<RefCell<NativeSelectionIndexEvent>>,
    drag_items_completed: Rc<RefCell<NativeStringListEvent>>,
    _selection_changed: windows_core::EventRevoker,
    _drag_items_completed: windows_core::EventRevoker,
}

struct NativeTreeView {
    value: native::TreeView,
    item_invoked: Rc<RefCell<NativeStringEvent>>,
    _item_invoked: windows_core::EventRevoker,
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
    callback: Option<Callback<T>>,
}

impl<T> Default for NativeValueEvent<T> {
    fn default() -> Self {
        Self {
            revision: 0,
            callback: None,
        }
    }
}

struct NativeRoutedValueEvent<T> {
    revision: u64,
    callback: Option<crate::RoutedCallback<T>>,
}

impl<T> Default for NativeRoutedValueEvent<T> {
    fn default() -> Self {
        Self {
            revision: 0,
            callback: None,
        }
    }
}

type NativeBoolEvent = NativeValueEvent<bool>;
type NativeColorEvent = NativeValueEvent<crate::Color>;
type NativeCharacterEventInfoEvent = NativeRoutedValueEvent<crate::CharacterEventInfo>;
type NativeContentDialogResultEvent = NativeValueEvent<ContentDialogResult>;
type NativeDragKindEvent = NativeValueEvent<crate::DragKind>;
type NativeDroppedDataEvent = NativeValueEvent<crate::DroppedData>;
type NativeF64Event = NativeValueEvent<f64>;
type NativeFocusEventInfoEvent = NativeValueEvent<crate::FocusEventInfo>;
type NativeOptionalBoolEvent = NativeValueEvent<Option<bool>>;
type NativeOptionalDateTimeEvent = NativeValueEvent<Option<windows_time::DateTime>>;
type NativeOptionalF64Event = NativeValueEvent<Option<f64>>;
type NativeOptionalTimeSpanEvent = NativeValueEvent<Option<windows_time::TimeSpan>>;
type NativeNavigationViewDisplayModeEvent = NativeValueEvent<crate::NavigationViewDisplayMode>;
type NativePointerEventInfoEvent = NativeValueEvent<crate::PointerEventInfo>;
type NativeKeyEventInfoEvent = NativeRoutedValueEvent<crate::KeyEventInfo>;
type NativeSelectionEvent = NativeValueEvent<Option<Rc<str>>>;
type NativeSelectionIndexEvent = NativeValueEvent<Option<usize>>;
type NativeTextEvent = NativeValueEvent<Rc<str>>;
type NativeStringEvent = NativeTextEvent;
type NativeStringListEvent = NativeValueEvent<Vec<String>>;
type NativeUnitEvent = NativeValueEvent<()>;

static NEXT_IMAGE_DECODE_COMPLETION: AtomicU64 = AtomicU64::new(1);

thread_local! {
    static IMAGE_DECODE_COMPLETIONS: RefCell<HashMap<u64, Box<dyn FnOnce(bool)>>> =
        RefCell::new(HashMap::new());
}

enum GeneratedRevoker {
    Event(windows_core::EventRevoker),
    Property(PropertyChangedRevoker),
}

impl Drop for GeneratedRevoker {
    fn drop(&mut self) {
        match self {
            Self::Event(value) => _ = value,
            Self::Property(value) => _ = value,
        }
    }
}

struct PropertyChangedRevoker {
    object: native::DependencyObject,
    property: native::DependencyProperty,
    token: i64,
}

impl Drop for PropertyChangedRevoker {
    fn drop(&mut self) {
        let _ = self
            .object
            .UnregisterPropertyChangedCallback(&self.property, self.token);
    }
}

include!("generated.rs");

struct QueuedEvent {
    object: ObjectId,
    event: EventId,
    revision: u64,
    payload: EventPayload,
}

struct QueuedNativeEvent {
    observation: Option<Observation>,
    event: Option<QueuedEvent>,
    retirement: Option<RetirementCompletion>,
    realization: Option<RealizationRequest>,
}

struct NativeSelectionItem {
    owner: ObjectId,
    object: ObjectId,
    value: IInspectable,
}

struct NativeContentDialogState {
    value: native::ContentDialog,
    owner: Option<ObjectId>,
    desired_open: bool,
    root: Option<native::XamlRoot>,
    loaded: Option<windows_core::EventRevoker>,
    pending_generation: Option<u64>,
    suppress_generation: Option<u64>,
    queued: bool,
    retired: bool,
    retained_handle: Option<GeneratedContentDialog>,
}

struct NativeMenu {
    menu: Menu,
    revision: u64,
    flyout: Option<native::MenuFlyout>,
    _revokers: Vec<windows_core::EventRevoker>,
}

struct NativeCommandBarFlyout {
    flyout: CommandBarFlyout,
    revision: u64,
    native: native::CommandBarFlyout,
    _revokers: Vec<windows_core::EventRevoker>,
}

struct NativeContentDialogRoot {
    root: native::XamlRoot,
    schedule: ContentDialogSchedule,
}

#[derive(Default)]
struct ContentDialogSchedule {
    active: Option<(ObjectId, u64)>,
    queue: VecDeque<ObjectId>,
}

impl ContentDialogSchedule {
    fn enqueue(&mut self, dialog: ObjectId) {
        if !self.queue.contains(&dialog) {
            self.queue.push_back(dialog);
        }
    }

    fn cancel(&mut self, dialog: ObjectId) {
        self.queue.retain(|current| *current != dialog);
    }

    fn next(&mut self) -> Option<ObjectId> {
        self.queue.pop_front()
    }

    fn begin(&mut self, dialog: ObjectId, generation: u64) {
        self.active = Some((dialog, generation));
    }

    fn complete(&mut self, dialog: ObjectId, generation: u64) -> bool {
        if self.active != Some((dialog, generation)) {
            return false;
        }
        self.active = None;
        true
    }
}

#[derive(Default)]
struct NativeContentDialogScheduler {
    dialogs: HashMap<ObjectId, NativeContentDialogState>,
    roots: Vec<NativeContentDialogRoot>,
    next_generation: u64,
}

#[derive(Default)]
struct NativeEventQueue {
    events: RefCell<VecDeque<QueuedNativeEvent>>,
    errors: RefCell<VecDeque<WinUiError>>,
    feedback: RefCell<FeedbackState>,
    selection_items: RefCell<Vec<NativeSelectionItem>>,
    waker: RefCell<Option<Rc<dyn Fn()>>>,
    wake_pending: Cell<bool>,
    content_dialogs: RefCell<NativeContentDialogScheduler>,
}

enum ObservationSubscription {
    SwapChainPanel {
        _rendering: windows_core::EventRevoker,
        _scale: windows_core::EventRevoker,
        _size: windows_core::EventRevoker,
    },
    ImageScale {
        _changed: Rc<RefCell<Option<windows_core::EventRevoker>>>,
        _loaded: windows_core::EventRevoker,
    },
    CompositionHost {
        _changed: Rc<RefCell<Option<windows_core::EventRevoker>>>,
        _loaded: windows_core::EventRevoker,
        _size: windows_core::EventRevoker,
    },
}

struct WebViewInitialization {
    _action: Option<windows_future::IAsyncAction>,
    _loaded: Option<windows_core::EventRevoker>,
    _initialized: windows_core::EventRevoker,
    completions: Vec<Callback<Result<windows_core::IUnknown, IntegrationError>>>,
}

impl NativeEventQueue {
    fn observe(&self, object: ObjectId, event: EventId, observation: Observation) -> bool {
        self.feedback
            .borrow_mut()
            .observe(object, event, observation)
    }

    fn queue(&self, observation: Option<Observation>, event: Option<QueuedEvent>) {
        self.events.borrow_mut().push_back(QueuedNativeEvent {
            observation,
            event,
            retirement: None,
            realization: None,
        });
    }

    fn queue_realization(self: &Rc<Self>, request: RealizationRequest) {
        let mut events = self.events.borrow_mut();
        let request = coalesce_queued_realization(&mut events, request);
        events.push_back(QueuedNativeEvent {
            observation: None,
            event: None,
            retirement: None,
            realization: Some(request),
        });
        drop(events);
        WinUiAdapter::schedule_event_wake(self);
    }
}

fn coalesce_queued_realization(
    events: &mut VecDeque<QueuedNativeEvent>,
    request: RealizationRequest,
) -> RealizationRequest {
    let RealizationRequest::Recycle {
        collection,
        container,
        source_revision,
    } = request
    else {
        return request;
    };
    let pending = events.iter().rposition(|event| {
        matches!(
            event.realization,
            Some(RealizationRequest::Realize {
                collection: pending_collection,
                container: pending_container,
                ..
            }) if pending_collection == collection && pending_container == container
        )
    });
    if let Some(pending) = pending {
        events.remove(pending);
        RealizationRequest::Cancel {
            collection,
            container,
            source_revision,
        }
    } else {
        request
    }
}

impl NativeContentDialogScheduler {
    fn create(&mut self, object: ObjectId, value: native::ContentDialog) {
        self.dialogs.insert(
            object,
            NativeContentDialogState {
                value,
                owner: None,
                desired_open: false,
                root: None,
                loaded: None,
                pending_generation: None,
                suppress_generation: None,
                queued: false,
                retired: false,
                retained_handle: None,
            },
        );
    }

    fn root_index(&self, root: &native::XamlRoot) -> Option<usize> {
        self.roots.iter().position(|state| state.root == *root)
    }

    fn ensure_root(&mut self, root: native::XamlRoot) -> usize {
        if let Some(index) = self.root_index(&root) {
            index
        } else {
            self.roots.push(NativeContentDialogRoot {
                root,
                schedule: ContentDialogSchedule::default(),
            });
            self.roots.len() - 1
        }
    }

    fn remove_from_queues(&mut self, dialog: ObjectId) {
        for root in &mut self.roots {
            root.schedule.cancel(dialog);
        }
        if let Some(state) = self.dialogs.get_mut(&dialog) {
            state.queued = false;
        }
    }

    fn set_root(&mut self, dialog: ObjectId, root: native::XamlRoot) -> Result<(), WinUiError> {
        self.remove_from_queues(dialog);
        let state = self
            .dialogs
            .get_mut(&dialog)
            .ok_or(WinUiError::MissingObject(dialog))?;
        state
            .value
            .cast::<native::IUIElement>()?
            .SetXamlRoot(&root)?;
        state.root = Some(root);
        state.loaded = None;
        if state.desired_open && state.pending_generation.is_none() {
            self.enqueue(dialog)?;
        }
        Ok(())
    }

    fn enqueue(&mut self, dialog: ObjectId) -> Result<(), WinUiError> {
        let root = self
            .dialogs
            .get(&dialog)
            .ok_or(WinUiError::MissingObject(dialog))?
            .root
            .clone();
        let Some(root) = root else {
            return Ok(());
        };
        let index = self.ensure_root(root);
        self.roots[index].schedule.enqueue(dialog);
        self.dialogs.get_mut(&dialog).unwrap().queued = true;
        Ok(())
    }

    fn set_open(
        &mut self,
        dialog: ObjectId,
        open: bool,
    ) -> Result<Option<native::ContentDialog>, WinUiError> {
        let state = self
            .dialogs
            .get_mut(&dialog)
            .ok_or(WinUiError::MissingObject(dialog))?;
        if state.desired_open == open {
            return Ok(None);
        }
        state.desired_open = open;
        if !open {
            let pending = state.pending_generation;
            let value = state.value.clone();
            self.remove_from_queues(dialog);
            if let Some(generation) = pending {
                self.dialogs.get_mut(&dialog).unwrap().suppress_generation = Some(generation);
                return Ok(Some(value));
            }
            return Ok(None);
        }
        if state.pending_generation.is_some() {
            self.enqueue(dialog)?;
            return Ok(None);
        }
        self.enqueue(dialog)?;
        self.start_next_for_dialog(dialog)?;
        Ok(None)
    }

    fn start_next_for_dialog(&mut self, dialog: ObjectId) -> Result<(), WinUiError> {
        let Some(root) = self
            .dialogs
            .get(&dialog)
            .and_then(|state| state.root.clone())
        else {
            return Ok(());
        };
        self.start_next(&root)
    }

    fn start_next(&mut self, root: &native::XamlRoot) -> Result<(), WinUiError> {
        let Some(index) = self.root_index(root) else {
            return Ok(());
        };
        if self.roots[index].schedule.active.is_some() {
            return Ok(());
        }
        while let Some(dialog) = self.roots[index].schedule.next() {
            let eligible = self.dialogs.get(&dialog).is_some_and(|state| {
                state.desired_open
                    && !state.retired
                    && state.pending_generation.is_none()
                    && state.root.as_ref().is_some_and(|current| current == root)
            });
            if !eligible {
                if let Some(state) = self.dialogs.get_mut(&dialog) {
                    state.queued = false;
                }
                continue;
            }
            self.next_generation = self.next_generation.wrapping_add(1);
            let generation = self.next_generation;
            let value = {
                let state = self.dialogs.get_mut(&dialog).unwrap();
                state.queued = false;
                state.pending_generation = Some(generation);
                state.suppress_generation = None;
                state
                    .value
                    .cast::<native::IUIElement>()?
                    .SetXamlRoot(root)?;
                state.value.clone()
            };
            self.roots[index].schedule.begin(dialog, generation);
            if let Err(error) = show_content_dialog(&value) {
                self.roots[index].schedule.active = None;
                self.dialogs.get_mut(&dialog).unwrap().pending_generation = None;
                return Err(error);
            }
            break;
        }
        Ok(())
    }

    fn closed(
        &mut self,
        dialog: ObjectId,
        generation: u64,
    ) -> Result<(bool, Option<native::XamlRoot>), WinUiError> {
        let state = self
            .dialogs
            .get_mut(&dialog)
            .ok_or(WinUiError::MissingObject(dialog))?;
        if state.pending_generation != Some(generation) {
            return Ok((false, None));
        }
        let root = state.root.clone();
        let dispatch = state.suppress_generation != Some(generation);
        state.pending_generation = None;
        state.suppress_generation = None;
        if state.desired_open && !state.retired {
            self.enqueue(dialog)?;
        }
        if let Some(root) = &root
            && let Some(index) = self.root_index(root)
        {
            self.roots[index].schedule.complete(dialog, generation);
        }
        Ok((dispatch, root))
    }

    fn cleanup_retired(&mut self) {
        let retired = self
            .dialogs
            .iter()
            .filter_map(|(object, state)| {
                (state.retired && state.pending_generation.is_none()).then_some(*object)
            })
            .collect::<Vec<_>>();
        for object in retired {
            self.remove_from_queues(object);
            self.dialogs.remove(&object);
        }
        self.roots
            .retain(|root| root.schedule.active.is_some() || !root.schedule.queue.is_empty());
    }

    fn retire(
        &mut self,
        dialog: ObjectId,
        handle: GeneratedContentDialog,
    ) -> Result<bool, WinUiError> {
        let state = self
            .dialogs
            .get_mut(&dialog)
            .ok_or(WinUiError::MissingObject(dialog))?;
        state.retired = true;
        state.owner = None;
        state.loaded = None;
        state.desired_open = false;
        let pending = state.pending_generation.is_some();
        let _ = state;
        self.remove_from_queues(dialog);
        if pending {
            self.dialogs.get_mut(&dialog).unwrap().retained_handle = Some(handle);
            Ok(true)
        } else {
            self.dialogs.remove(&dialog);
            Ok(false)
        }
    }

    fn reset(&mut self) {
        for state in self.dialogs.values_mut() {
            state.desired_open = false;
            state.loaded = None;
            if let Some(generation) = state.pending_generation {
                state.suppress_generation = Some(generation);
                _ = state.value.Hide();
            }
        }
        for root in &mut self.roots {
            root.schedule.queue.clear();
        }
    }
}

fn content_dialog_result(value: native::ContentDialogResult) -> ContentDialogResult {
    match value {
        native::ContentDialogResult::Primary => ContentDialogResult::Primary,
        native::ContentDialogResult::Secondary => ContentDialogResult::Secondary,
        _ => ContentDialogResult::None,
    }
}

fn show_content_dialog(value: &native::ContentDialog) -> Result<(), WinUiError> {
    match value.ShowAsync() {
        Ok(operation) => {
            drop(operation);
            Ok(())
        }
        Err(error) if error.code().is_ok() => Ok(()),
        Err(error) => Err(error.into()),
    }
}

implement_decl! {
    impl NativeElementFactory as NativeElementFactory_Impl: [IElementFactory]
}

struct NativeElementFactory {
    collection: ObjectId,
    queue: Rc<NativeEventQueue>,
    source_revision: Rc<Cell<u64>>,
    shells: Rc<RealizedShells>,
}

struct NativeVirtualItems {
    _factory: IElementFactory,
    source: ComObject<NativeVirtualSource>,
    source_revision: Rc<Cell<u64>>,
    shells: Rc<RealizedShells>,
}

impl NativeVirtualItems {
    fn new(
        repeater: &native::ItemsRepeater,
        collection: ObjectId,
        item_count: usize,
        source_revision: u64,
        queue: Rc<NativeEventQueue>,
    ) -> Result<Self, windows_core::Error> {
        let shells = Rc::new(RealizedShells::default());
        let source_revision = Rc::new(Cell::new(source_revision));
        let factory: IElementFactory = ComObject::new(NativeElementFactory {
            collection,
            queue,
            source_revision: Rc::clone(&source_revision),
            shells: Rc::clone(&shells),
        })
        .into_interface();
        let source = ComObject::new(NativeVirtualSource::new(item_count)?);
        let source_interface: windows_collections::IObservableVector<IInspectable> =
            source.to_interface();
        repeater.SetItemTemplate(&factory)?;
        repeater.SetItemsSource(&source_interface)?;
        Ok(Self {
            _factory: factory,
            source,
            source_revision,
            shells,
        })
    }

    fn reset(&self, item_count: usize, source_revision: u64) -> windows_core::Result<()> {
        let item_count = virtual_item_count(item_count)?;
        self.source_revision.set(source_revision);
        self.source.reset(item_count);
        Ok(())
    }
}

const E_BOUNDS: HRESULT = HRESULT(0x8000000B_u32 as i32);
const E_ILLEGAL_METHOD_CALL: HRESULT = HRESULT(0x8000000E_u32 as i32);

type NativeObservableVector = windows_collections::IObservableVector<IInspectable>;
type NativeVector = windows_collections::IVector<IInspectable>;
type NativeVectorView = windows_collections::IVectorView<IInspectable>;
type NativeIterable = windows_collections::IIterable<IInspectable>;
type NativeIterator = windows_collections::IIterator<IInspectable>;
type NativeVectorChangedEventArgs = windows_collections::IVectorChangedEventArgs;

implement_decl! {
    impl NativeVirtualSource as NativeVirtualSource_Impl: [
        NativeObservableVector,
        NativeVector,
        NativeVectorView,
        NativeIterable,
    ]
}

struct NativeVirtualSource {
    item_count: AtomicU32,
    handlers: WinEvent<VectorChangedEventHandler<IInspectable>>,
}

impl NativeVirtualSource {
    fn new(count: usize) -> windows_core::Result<Self> {
        Ok(Self {
            item_count: AtomicU32::new(virtual_item_count(count)?),
            handlers: WinEvent::new(),
        })
    }
}

impl NativeVirtualSource_Impl {
    fn count(&self) -> u32 {
        self.item_count.load(Ordering::Acquire)
    }

    fn get_at(&self, index: u32) -> windows_core::Result<IInspectable> {
        if index >= self.count() {
            return Err(E_BOUNDS.into());
        }
        virtual_item_value(index)
    }

    fn index_of(&self, value: Ref<IInspectable>, result: &mut u32) -> windows_core::Result<bool> {
        let value = value
            .ok()?
            .cast::<windows_reference::IReference<i32>>()?
            .Value()?;
        let Ok(value) = u32::try_from(value) else {
            *result = 0;
            return Ok(false);
        };
        if value < self.count() {
            *result = value;
            Ok(true)
        } else {
            *result = 0;
            Ok(false)
        }
    }

    fn get_many(
        &self,
        start_index: u32,
        items: &mut [Option<IInspectable>],
    ) -> windows_core::Result<u32> {
        let count = self.count();
        if start_index >= count {
            return Ok(0);
        }
        let available = usize::try_from(count - start_index)
            .map_err(|_| windows_core::Error::new(E_BOUNDS, "item count exceeds usize"))?;
        let actual = available.min(items.len());
        for (offset, item) in items[..actual].iter_mut().enumerate() {
            let offset = u32::try_from(offset)
                .map_err(|_| windows_core::Error::new(E_BOUNDS, "item buffer is too large"))?;
            *item = Some(virtual_item_value(start_index + offset)?);
        }
        u32::try_from(actual)
            .map_err(|_| windows_core::Error::new(E_BOUNDS, "item buffer is too large"))
    }

    fn read_only_error() -> windows_core::Error {
        windows_core::Error::new(E_ILLEGAL_METHOD_CALL, "virtual item source is read-only")
    }

    fn reset(&self, count: u32) {
        self.item_count.store(count, Ordering::Release);
        let source: windows_collections::IObservableVector<IInspectable> =
            self.to_object().into_interface();
        let args: windows_collections::IVectorChangedEventArgs =
            ComObject::new(NativeVirtualChangedEventArgs).into_interface();
        self.handlers
            .call(|handler: &VectorChangedEventHandler<IInspectable>| {
                handler.Invoke(&source, &args)
            });
    }
}

impl IObservableVector_Impl<IInspectable> for NativeVirtualSource_Impl {
    fn VectorChanged(
        &self,
        handler: Ref<VectorChangedEventHandler<IInspectable>>,
    ) -> windows_core::Result<i64> {
        self.handlers.add(handler.ok()?)
    }

    fn RemoveVectorChanged(&self, token: i64) -> windows_core::Result<()> {
        self.handlers.remove(token);
        Ok(())
    }
}

impl IIterable_Impl<IInspectable> for NativeVirtualSource_Impl {
    fn First(&self) -> windows_core::Result<windows_collections::IIterator<IInspectable>> {
        Ok(ComObject::new(NativeVirtualSourceIterator {
            source: self.to_object(),
            current: AtomicU32::new(0),
        })
        .into_interface())
    }
}

impl IVector_Impl<IInspectable> for NativeVirtualSource_Impl {
    fn GetAt(&self, index: u32) -> windows_core::Result<IInspectable> {
        self.get_at(index)
    }

    fn Size(&self) -> windows_core::Result<u32> {
        Ok(self.count())
    }

    fn GetView(&self) -> windows_core::Result<windows_collections::IVectorView<IInspectable>> {
        Ok(self.to_object().into_interface())
    }

    fn IndexOf(&self, value: Ref<IInspectable>, result: &mut u32) -> windows_core::Result<bool> {
        self.index_of(value, result)
    }

    fn SetAt(&self, _index: u32, _value: Ref<IInspectable>) -> windows_core::Result<()> {
        Err(Self::read_only_error())
    }

    fn InsertAt(&self, _index: u32, _value: Ref<IInspectable>) -> windows_core::Result<()> {
        Err(Self::read_only_error())
    }

    fn RemoveAt(&self, _index: u32) -> windows_core::Result<()> {
        Err(Self::read_only_error())
    }

    fn Append(&self, _value: Ref<IInspectable>) -> windows_core::Result<()> {
        Err(Self::read_only_error())
    }

    fn RemoveAtEnd(&self) -> windows_core::Result<()> {
        Err(Self::read_only_error())
    }

    fn Clear(&self) -> windows_core::Result<()> {
        Err(Self::read_only_error())
    }

    fn GetMany(
        &self,
        start_index: u32,
        items: &mut [Option<IInspectable>],
    ) -> windows_core::Result<u32> {
        self.get_many(start_index, items)
    }

    fn ReplaceAll(&self, _items: &[Option<IInspectable>]) -> windows_core::Result<()> {
        Err(Self::read_only_error())
    }
}

impl IVectorView_Impl<IInspectable> for NativeVirtualSource_Impl {
    fn GetAt(&self, index: u32) -> windows_core::Result<IInspectable> {
        self.get_at(index)
    }

    fn Size(&self) -> windows_core::Result<u32> {
        Ok(self.count())
    }

    fn IndexOf(&self, value: Ref<IInspectable>, result: &mut u32) -> windows_core::Result<bool> {
        self.index_of(value, result)
    }

    fn GetMany(
        &self,
        start_index: u32,
        items: &mut [Option<IInspectable>],
    ) -> windows_core::Result<u32> {
        self.get_many(start_index, items)
    }
}

implement_decl! {
    impl NativeVirtualSourceIterator as NativeVirtualSourceIterator_Impl: [
        NativeIterator,
    ]
}

struct NativeVirtualSourceIterator {
    source: ComObject<NativeVirtualSource>,
    current: AtomicU32,
}

impl IIterator_Impl<IInspectable> for NativeVirtualSourceIterator_Impl {
    fn Current(&self) -> windows_core::Result<IInspectable> {
        self.source.get_at(self.current.load(Ordering::Acquire))
    }

    fn HasCurrent(&self) -> windows_core::Result<bool> {
        Ok(self.current.load(Ordering::Acquire) < self.source.count())
    }

    fn MoveNext(&self) -> windows_core::Result<bool> {
        let count = self.source.count();
        let current = self.current.load(Ordering::Acquire);
        if current < count {
            self.current.store(current + 1, Ordering::Release);
        }
        Ok(current.saturating_add(1) < count)
    }

    fn GetMany(&self, items: &mut [Option<IInspectable>]) -> windows_core::Result<u32> {
        let current = self.current.load(Ordering::Acquire);
        let actual = self.source.get_many(current, items)?;
        self.current
            .store(current.saturating_add(actual), Ordering::Release);
        Ok(actual)
    }
}

implement_decl! {
    impl NativeVirtualChangedEventArgs as NativeVirtualChangedEventArgs_Impl: [
        NativeVectorChangedEventArgs,
    ]
}

struct NativeVirtualChangedEventArgs;

impl IVectorChangedEventArgs_Impl for NativeVirtualChangedEventArgs_Impl {
    fn CollectionChange(&self) -> windows_core::Result<CollectionChange> {
        Ok(CollectionChange::Reset)
    }

    fn Index(&self) -> windows_core::Result<u32> {
        Ok(0)
    }
}

#[derive(Default)]
struct RealizedShells {
    pool: RefCell<ShellPool<native::ContentControl>>,
}

struct ShellPool<T> {
    available: Vec<T>,
    next: u64,
    retired: HashSet<RealizedContainer>,
    shells: HashMap<RealizedContainer, T>,
}

impl<T> Default for ShellPool<T> {
    fn default() -> Self {
        Self {
            available: Vec::new(),
            next: 0,
            retired: HashSet::new(),
            shells: HashMap::new(),
        }
    }
}

impl<T: Clone> ShellPool<T> {
    fn take(
        &mut self,
        create: impl FnOnce() -> Result<T, windows_core::Error>,
    ) -> Result<(RealizedContainer, T), windows_core::Error> {
        let container = RealizedContainer(self.next);
        self.next = self.next.checked_add(1).ok_or_else(|| {
            windows_core::Error::new(native::E_FAIL, "container identity exhausted")
        })?;
        let shell = if let Some(shell) = self.available.pop() {
            shell
        } else {
            create()?
        };
        self.shells.insert(container, shell.clone());
        Ok((container, shell))
    }

    fn retire(&mut self, container: RealizedContainer) -> bool {
        let Some(shell) = self.shells.remove(&container) else {
            return false;
        };
        self.available.push(shell);
        self.retired.insert(container);
        true
    }

    fn acknowledge_recycle(&mut self, container: RealizedContainer) {
        self.retired.remove(&container);
    }
}

impl ShellPool<native::ContentControl> {
    fn recycle(
        &mut self,
        element: &native::UIElement,
    ) -> Result<Option<RealizedContainer>, windows_core::Error> {
        let Some((container, shell)) = self.shells.iter().find_map(|(container, shell)| {
            (shell.cast::<native::UIElement>().as_ref() == Ok(element))
                .then(|| (*container, shell.clone()))
        }) else {
            return Ok(None);
        };
        shell.SetContent(None::<&IInspectable>)?;
        if !self.retire(container) {
            return Err(windows_core::Error::new(
                native::E_FAIL,
                "realized container retired twice",
            ));
        }
        Ok(Some(container))
    }
}

impl RealizedShells {
    fn take(&self) -> Result<(RealizedContainer, native::UIElement), windows_core::Error> {
        let (container, shell) = self.pool.borrow_mut().take(native::ContentControl::new)?;
        Ok((container, shell.cast()?))
    }

    fn recycle(
        &self,
        element: &native::UIElement,
    ) -> Result<Option<RealizedContainer>, windows_core::Error> {
        self.pool.borrow_mut().recycle(element)
    }

    fn set_content(
        &self,
        container: RealizedContainer,
        content: Option<&native::UIElement>,
    ) -> Result<(), windows_core::Error> {
        let pool = self.pool.borrow();
        let Some(shell) = pool.shells.get(&container) else {
            return if content.is_none() && pool.retired.contains(&container) {
                Ok(())
            } else {
                Err(windows_core::Error::new(
                    native::E_FAIL,
                    "missing realized container",
                ))
            };
        };
        if let Some(content) = content {
            shell.SetContent(content)
        } else {
            shell.SetContent(None::<&IInspectable>)
        }
    }

    fn acknowledge_recycle(&self, container: RealizedContainer) -> Result<(), windows_core::Error> {
        self.pool.borrow_mut().acknowledge_recycle(container);
        Ok(())
    }
}

impl native::IElementFactory_Impl for NativeElementFactory_Impl {
    fn GetElement(
        &self,
        args: Ref<native::ElementFactoryGetArgs>,
    ) -> windows_core::Result<native::UIElement> {
        let data = args.ok()?.Data()?;
        let value = data.cast::<windows_reference::IReference<i32>>()?.Value()?;
        let index = usize::try_from(value)
            .map_err(|_| windows_core::Error::new(native::E_FAIL, "negative item index"))?;
        let (container, element) = self.shells.take()?;
        self.queue.queue_realization(RealizationRequest::Realize {
            collection: self.collection,
            container,
            index,
            source_revision: self.source_revision.get(),
        });
        Ok(element)
    }

    fn RecycleElement(
        &self,
        args: Ref<native::ElementFactoryRecycleArgs>,
    ) -> windows_core::Result<()> {
        let element = args.ok()?.Element()?;
        let container = self.shells.recycle(&element)?.ok_or_else(|| {
            windows_core::Error::new(native::E_FAIL, "element factory received an unknown shell")
        })?;
        self.queue.queue_realization(RealizationRequest::Recycle {
            collection: self.collection,
            container,
            source_revision: self.source_revision.get(),
        });
        Ok(())
    }
}

fn virtual_item_count(item_count: usize) -> windows_core::Result<u32> {
    let max_count = i32::MAX as usize + 1;
    if item_count > max_count {
        return Err(windows_core::Error::new(
            native::E_FAIL,
            "item count exceeds i32 index range",
        ));
    }
    item_count
        .try_into()
        .map_err(|_| windows_core::Error::new(native::E_FAIL, "item count exceeds u32"))
}

fn virtual_item_value(index: u32) -> windows_core::Result<IInspectable> {
    let index = i32::try_from(index)
        .map_err(|_| windows_core::Error::new(E_BOUNDS, "item index exceeds i32"))?;
    Ok(windows_reference::IReference::<i32>::from(index).into())
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WinUiError {
    DuplicateObject(ObjectId),
    MissingObject(ObjectId),
    InvalidObject(ObjectId),
    InvalidRelation(ObjectId, RelationId),
    InvalidMutation(RelationId),
    InvalidEvent(ObjectId, EventId),
    StateMismatch(ObjectId),
    MissingContract(ObjectType, RelationId),
    IndexOverflow(usize),
    ChildNotFound(ObjectId),
    StillOwned(ObjectId),
    InvalidReplacement(ObjectId),
    InvalidEventArgs,
    InvalidDuration,
    DuplicateWindowRoot(ObjectId),
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
            error => Self::new(HRESULT(0x80004005_u32 as i32), format!("{error:?}")),
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

fn to_native_color(value: crate::Color) -> native::Color {
    native::Color {
        a: value.a,
        r: value.r,
        g: value.g,
        b: value.b,
    }
}

fn from_native_color(value: native::Color) -> crate::Color {
    crate::Color::argb(value.a, value.r, value.g, value.b)
}

fn navigation_view_display_mode(
    value: native::NavigationViewDisplayMode,
) -> crate::NavigationViewDisplayMode {
    match value {
        native::NavigationViewDisplayMode::Minimal => crate::NavigationViewDisplayMode::Minimal,
        native::NavigationViewDisplayMode::Compact => crate::NavigationViewDisplayMode::Compact,
        native::NavigationViewDisplayMode::Expanded => crate::NavigationViewDisplayMode::Expanded,
        _ => unreachable!(),
    }
}

pub(crate) fn validate_native_uri(value: &str) -> windows_core::Result<()> {
    native::Uri::CreateUri(value).map(drop)
}

fn uri_image(value: &str) -> Result<native::ImageSource, WinUiError> {
    let uri = native::Uri::CreateUri(value)?;
    let path = value.split(['?', '#']).next().unwrap_or(value);
    if path.to_ascii_lowercase().ends_with(".svg") {
        let image = native::SvgImageSource::new()?;
        image.SetUriSource(&uri)?;
        image.cast().map_err(Into::into)
    } else {
        let image = native::BitmapImage::new()?;
        image.SetUriSource(&uri)?;
        image.cast().map_err(Into::into)
    }
}

fn queue_image_decode_completion(
    dispatch: Option<(native::DispatcherQueue, u64)>,
    decode_failed: bool,
) {
    let Some((queue, token)) = dispatch else {
        return;
    };
    let handler = native::DispatcherQueueHandler::new(move || {
        IMAGE_DECODE_COMPLETIONS.with(|completions| {
            if let Some(completion) = completions.borrow_mut().remove(&token) {
                completion(decode_failed);
            }
        });
    });
    match queue.TryEnqueueWithPriority(native::DispatcherQueuePriority::Normal, &handler) {
        Ok(true) => {}
        Ok(false) => super::app::report_error(windows_core::Error::new(
            native::E_FAIL,
            "dispatcher rejected encoded image completion",
        )),
        Err(error) => super::app::report_error(error),
    }
}

fn encoded_bitmap_image(
    value: &EncodedImage,
    failed: Option<Rc<dyn Fn(&native::BitmapImage)>>,
) -> Result<native::BitmapImage, WinUiError> {
    let stream = native::InMemoryRandomAccessStream::new()?;
    let output = stream.GetOutputStreamAt(0)?;
    let writer = native::DataWriter::CreateDataWriter(&output)?;
    writer.WriteBytes(value.as_bytes())?;
    let store = writer.StoreAsync()?;
    let image = native::BitmapImage::new()?;
    let failed_dispatch = failed
        .map(|failed| {
            let failed_image = image.clone();
            let queue = native::DispatcherQueue::GetForCurrentThread()?;
            let token = NEXT_IMAGE_DECODE_COMPLETION.fetch_add(1, Ordering::Relaxed);
            IMAGE_DECODE_COMPLETIONS.with(|completions| {
                completions.borrow_mut().insert(
                    token,
                    Box::new(move |decode_failed| {
                        if decode_failed {
                            failed(&failed_image);
                        }
                    }),
                );
            });
            Ok::<_, windows_core::Error>((queue, token))
        })
        .transpose()?;
    let decode_image = image.clone();
    if let Err(error) = store.when(move |result| {
        let mut failed_dispatch = failed_dispatch;
        if let Err(error) = result {
            queue_image_decode_completion(failed_dispatch.take(), false);
            super::app::report_error(error);
            return;
        }
        if let Err(error) = writer.DetachStream() {
            queue_image_decode_completion(failed_dispatch.take(), false);
            super::app::report_error(error);
            return;
        }
        if let Err(error) = stream.Seek(0) {
            queue_image_decode_completion(failed_dispatch.take(), false);
            super::app::report_error(error);
            return;
        }
        let source = match decode_image.cast::<native::IBitmapSource>() {
            Ok(source) => source,
            Err(error) => {
                queue_image_decode_completion(failed_dispatch.take(), false);
                super::app::report_error(error);
                return;
            }
        };
        let operation = match source.SetSourceAsync(&stream) {
            Ok(operation) => operation,
            Err(error) => {
                queue_image_decode_completion(failed_dispatch.take(), false);
                super::app::report_error(error);
                return;
            }
        };
        let completion_dispatch = failed_dispatch.clone();
        if let Err(error) = operation.when(move |result| {
            drop(stream);
            queue_image_decode_completion(completion_dispatch, result.is_err());
        }) {
            queue_image_decode_completion(failed_dispatch, false);
            super::app::report_error(error);
        }
    }) {
        return Err(error.into());
    }
    Ok(image)
}

fn build_menu_items(
    items: &[MenuItem],
    output: &windows_collections::IVector<native::MenuFlyoutItemBase>,
    revokers: &mut Vec<windows_core::EventRevoker>,
    event_queue: &Rc<NativeEventQueue>,
    target: ObjectId,
    revision: u64,
) -> Result<(), WinUiError> {
    for entry in items {
        let item: native::MenuFlyoutItemBase = match entry {
            MenuItem::Item {
                key,
                label,
                enabled,
            } => {
                let item = native::MenuFlyoutItem::new()?;
                item.SetText(label)?;
                item.cast::<native::IControl>()?.SetIsEnabled(*enabled)?;
                let clicked_key = key.clone();
                let clicked_queue = Rc::clone(event_queue);
                revokers.push(item.Click(move |_, _| {
                    clicked_queue.queue(
                        None,
                        Some(QueuedEvent {
                            object: target,
                            event: EventId::MenuItemInvoked,
                            revision,
                            payload: EventPayload::Key(clicked_key.clone()),
                        }),
                    );
                    WinUiAdapter::schedule_event_wake(&clicked_queue);
                })?);
                item.cast()?
            }
            MenuItem::Separator { .. } => native::MenuFlyoutSeparator::new()?.cast()?,
            MenuItem::Submenu { label, items, .. } => {
                let item = native::MenuFlyoutSubItem::new()?;
                item.SetText(label)?;
                build_menu_items(
                    items,
                    &item.Items()?,
                    revokers,
                    event_queue,
                    target,
                    revision,
                )?;
                item.cast()?
            }
        };
        output.Append(&item)?;
    }
    Ok(())
}

fn build_command_bar_element(
    command: &CommandBarCommand,
    revokers: &mut Vec<windows_core::EventRevoker>,
    event_queue: &Rc<NativeEventQueue>,
    target: ObjectId,
    revision: u64,
) -> Result<native::ICommandBarElement, WinUiError> {
    match command {
        CommandBarCommand::Button {
            key,
            label,
            icon,
            enabled,
        } => {
            let button = native::AppBarButton::new()?;
            button.SetLabel(label)?;
            button.cast::<native::IControl>()?.SetIsEnabled(*enabled)?;
            if let Some(symbol) = icon {
                let icon = native::SymbolIcon::new()?;
                GeneratedHandle::SymbolIcon(icon.clone())
                    .set_property(PropertyId::Symbol, Some(&symbol.property_value()))
                    .unwrap()?;
                button.SetIcon(&icon)?;
            }
            let clicked_key = key.clone();
            let clicked_queue = Rc::clone(event_queue);
            revokers.push(button.cast::<native::IButtonBase>()?.Click(move |_, _| {
                clicked_queue.queue(
                    None,
                    Some(QueuedEvent {
                        object: target,
                        event: EventId::CommandInvoked,
                        revision,
                        payload: EventPayload::Key(clicked_key.clone()),
                    }),
                );
                WinUiAdapter::schedule_event_wake(&clicked_queue);
            })?);
            Ok(button.cast()?)
        }
        CommandBarCommand::Separator { .. } => Ok(native::AppBarSeparator::new()?.cast()?),
    }
}

fn set_rich_text_blocks(
    target: &native::RichTextBlock,
    value: Option<&crate::RichText>,
) -> Result<(), WinUiError> {
    let control = target.cast::<native::IRichTextBlock>()?;
    let blocks: windows_collections::IVector<native::Block> = control.Blocks()?.cast()?;
    blocks.Clear()?;
    let Some(value) = value else {
        return Ok(());
    };
    let append_run = |inlines: &windows_collections::IVector<native::Inline>,
                      value: &crate::RichTextRun|
     -> Result<(), WinUiError> {
        let run = native::Run::new()?;
        run.SetText(&value.text)?;
        if value.is_bold {
            run.cast::<native::ITextElement>()?
                .SetFontWeight(native::FontWeight { weight: 700 })?;
        }
        if value.is_italic {
            run.cast::<native::ITextElement>()?
                .SetFontStyle(native::FontStyle::Italic)?;
        }
        let run: native::Inline = run.cast()?;
        inlines.Append(&run)?;
        Ok(())
    };
    for paragraph in value.paragraphs.iter() {
        let native_paragraph = native::Paragraph::new()?;
        let inlines: windows_collections::IVector<native::Inline> =
            native_paragraph.Inlines()?.cast()?;
        for inline in &paragraph.inlines {
            match inline {
                crate::RichTextInline::Run(value) => append_run(&inlines, value)?,
                crate::RichTextInline::Hyperlink(value) => {
                    let hyperlink = native::Hyperlink::new()?;
                    hyperlink.SetNavigateUri(&native::Uri::CreateUri(&value.uri)?)?;
                    let hyperlink_inlines: windows_collections::IVector<native::Inline> =
                        hyperlink.cast::<native::ISpan>()?.Inlines()?.cast()?;
                    append_run(&hyperlink_inlines, &crate::RichTextRun::plain(&value.text))?;
                    let hyperlink: native::Inline = hyperlink.cast()?;
                    inlines.Append(&hyperlink)?;
                }
                crate::RichTextInline::LineBreak => {
                    let line_break: native::Inline = native::LineBreak::new()?.cast()?;
                    inlines.Append(&line_break)?;
                }
            }
        }
        let native_paragraph: native::Block = native_paragraph.cast()?;
        blocks.Append(&native_paragraph)?;
    }
    Ok(())
}

fn parse_path_data(value: &str) -> Result<native::Geometry, WinUiError> {
    let type_name = native::TypeName {
        name: "Microsoft.UI.Xaml.Media.Geometry".into(),
        kind: native::TypeKind::Metadata,
    };
    let value = windows_reference::IReference::<HSTRING>::from(value);
    Ok(native::XamlBindingHelper::ConvertValue(&type_name, &value)?.cast()?)
}

fn set_opacity_transition(
    element: &native::UIElement,
    duration: Option<std::time::Duration>,
) -> Result<(), WinUiError> {
    let Some(duration) = duration else {
        return element
            .SetOpacityTransition(None::<&native::ScalarTransition>)
            .map_err(Into::into);
    };
    let duration =
        windows_time::TimeSpan::try_from(duration).map_err(|_| WinUiError::InvalidDuration)?;
    let transition = native::ScalarTransition::new()?;
    transition.SetDuration(duration)?;
    element.SetOpacityTransition(&transition)?;
    Ok(())
}

fn set_implicit_scale(element: &native::UIElement, value: f64) -> Result<(), WinUiError> {
    let framework = element.cast::<native::IFrameworkElement>()?;
    element.SetCenterPoint(native::Vector3 {
        x: framework.ActualWidth()? as f32 / 2.0,
        y: framework.ActualHeight()? as f32 / 2.0,
        z: 0.0,
    })?;
    let value = value as f32;
    element.SetScale(native::Vector3 {
        x: value,
        y: value,
        z: 1.0,
    })?;
    Ok(())
}

fn set_scale_transition(
    element: &native::UIElement,
    duration: Option<std::time::Duration>,
) -> Result<(), WinUiError> {
    let Some(duration) = duration else {
        return element
            .SetScaleTransition(None::<&native::Vector3Transition>)
            .map_err(Into::into);
    };
    let duration =
        windows_time::TimeSpan::try_from(duration).map_err(|_| WinUiError::InvalidDuration)?;
    let transition = native::Vector3Transition::new()?;
    transition.SetDuration(duration)?;
    element.SetScaleTransition(&transition)?;
    Ok(())
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct NativeStyleState {
    base: Option<ButtonStyle>,
    brushes: BTreeMap<PropertyId, ThemeBrush>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct NativeStyleKey {
    kind: ObjectType,
    base: Option<ButtonStyle>,
    brushes: Vec<(PropertyId, ThemeBrush)>,
}

fn button_style_resource(style: ButtonStyle) -> &'static str {
    match style {
        ButtonStyle::Default => unreachable!("default button style has no resource"),
        ButtonStyle::Accent => "AccentButtonStyle",
        ButtonStyle::Subtle => "SubtleButtonStyle",
        ButtonStyle::TextLink => "TextBlockButtonStyle",
    }
}

fn integration_error(error: windows_core::Error) -> IntegrationError {
    IntegrationError::Native(error.code().0)
}

fn complete_webview_initialization(
    initializations: &Rc<RefCell<HashMap<ObjectId, WebViewInitialization>>>,
    object: ObjectId,
    result: Result<windows_core::IUnknown, IntegrationError>,
) {
    if let Some(initialization) = initializations.borrow_mut().remove(&object) {
        for completion in initialization.completions {
            completion.call(result.clone());
        }
    }
}

fn xaml_scale(element: &native::UIElement) -> windows_core::Result<f64> {
    match element.XamlRoot() {
        Ok(root) => root.RasterizationScale(),
        Err(error) if error.code().is_ok() => Ok(1.0),
        Err(error) => Err(error),
    }
}

fn observe_xaml_scale(
    element: &native::UIElement,
    changed: &Rc<RefCell<Option<windows_core::EventRevoker>>>,
    errors: &Rc<NativeEventQueue>,
    callback: Callback<f64>,
) -> windows_core::Result<()> {
    let root = match element.XamlRoot() {
        Ok(root) => root,
        Err(error) if error.code().is_ok() => return Ok(()),
        Err(error) => return Err(error),
    };
    callback.call(root.RasterizationScale()?);
    let callback_root = callback;
    let callback_errors = Rc::clone(errors);
    changed.replace(Some(root.Changed(move |sender, _| {
        let result = sender
            .as_ref()
            .ok_or_else(windows_core::Error::empty)
            .and_then(|sender| sender.RasterizationScale());
        match result {
            Ok(scale) => callback_root.call(scale),
            Err(error) => callback_errors.errors.borrow_mut().push_back(error.into()),
        }
    })?));
    Ok(())
}

pub struct WinUiAdapter {
    handles: HashMap<ObjectId, Handle>,
    owners: HashMap<ObjectId, (ObjectId, RelationId)>,
    tree_template: Option<native::DataTemplate>,
    list_template: Option<native::DataTemplate>,
    data_content_key: HSTRING,
    data_text_key: HSTRING,
    event_queue: Rc<NativeEventQueue>,
    retirements: HashMap<ObjectId, NativeRetirement>,
    virtual_items: HashMap<ObjectId, NativeVirtualItems>,
    observations: HashMap<(ObjectId, u64), ObservationSubscription>,
    webview_initializations: Rc<RefCell<HashMap<ObjectId, WebViewInitialization>>>,
    windows: Rc<RefCell<Vec<Weak<NativeWindowState>>>>,
    window_title_bar: Option<(ObjectId, WindowTitleBarHeight)>,
    tooltips: HashMap<ObjectId, (ObjectId, TooltipPlacement)>,
    tooltip_owners: HashMap<ObjectId, ObjectId>,
    flyouts: HashMap<ObjectId, (native::Flyout, ObjectId, FlyoutPlacement)>,
    flyout_owners: HashMap<ObjectId, ObjectId>,
    menus: HashMap<ObjectId, NativeMenu>,
    command_bar_flyouts: HashMap<ObjectId, NativeCommandBarFlyout>,
    content_dialogs: HashMap<ObjectId, ObjectId>,
    content_dialog_owners: HashMap<ObjectId, ObjectId>,
    resource_override_keys: HashMap<ObjectId, HashSet<String>>,
    style_states: HashMap<ObjectId, NativeStyleState>,
    style_cache: HashMap<NativeStyleKey, native::Style>,
    pending_focus_states: Rc<RefCell<HashMap<ObjectId, ElementFocusState>>>,
    tree_node_texts: Rc<RefCell<HashMap<usize, Rc<str>>>>,
}

struct NativeRetirement {
    nodes: Vec<ObjectId>,
    parent: ObjectId,
    relation: RelationId,
    timer: native::DispatcherQueueTimer,
    _tick: windows_core::EventRevoker,
}

impl Default for WinUiAdapter {
    fn default() -> Self {
        Self {
            handles: HashMap::new(),
            owners: HashMap::new(),
            tree_template: None,
            list_template: None,
            data_content_key: HSTRING::from("Content"),
            data_text_key: HSTRING::from("Text"),
            event_queue: Rc::new(NativeEventQueue::default()),
            retirements: HashMap::new(),
            virtual_items: HashMap::new(),
            observations: HashMap::new(),
            webview_initializations: Rc::new(RefCell::new(HashMap::new())),
            windows: Rc::new(RefCell::new(Vec::new())),
            window_title_bar: None,
            tooltips: HashMap::new(),
            tooltip_owners: HashMap::new(),
            flyouts: HashMap::new(),
            flyout_owners: HashMap::new(),
            menus: HashMap::new(),
            command_bar_flyouts: HashMap::new(),
            content_dialogs: HashMap::new(),
            content_dialog_owners: HashMap::new(),
            resource_override_keys: HashMap::new(),
            style_states: HashMap::new(),
            style_cache: HashMap::new(),
            pending_focus_states: Rc::new(RefCell::new(HashMap::new())),
            tree_node_texts: Rc::new(RefCell::new(HashMap::new())),
        }
    }
}

impl Drop for WinUiAdapter {
    fn drop(&mut self) {
        for retirement in self.retirements.values() {
            _ = retirement.timer.Stop();
        }
        self.retirements.clear();
        self.event_queue.content_dialogs.borrow_mut().reset();
        self.event_queue.events.borrow_mut().clear();
        self.observations.clear();
        for target in self.menus.keys().copied().collect::<Vec<_>>() {
            _ = self.set_menu(target, None, 0);
        }
        for target in self.command_bar_flyouts.keys().copied().collect::<Vec<_>>() {
            _ = self.set_command_bar_flyout(target, None, 0);
        }
        for (target, (flyout, _, _)) in self.flyouts.drain() {
            _ = flyout.SetContent(None::<&native::UIElement>);
            match self.handles.get(&target) {
                Some(Handle::Generated(GeneratedHandle::Button(control))) => {
                    _ = control.value.SetFlyout(None::<&native::FlyoutBase>);
                }
                Some(Handle::Generated(GeneratedHandle::SplitButton(control))) => {
                    _ = control.value.SetFlyout(None::<&native::FlyoutBase>);
                }
                _ => {}
            }
        }
        self.flyout_owners.clear();
        for initialization in
            std::mem::take(&mut *self.webview_initializations.borrow_mut()).into_values()
        {
            for completion in initialization.completions {
                completion.call(Err(IntegrationError::Unavailable));
            }
        }
    }
}

/// Advanced native window handle.
///
/// Clones share the native window but not the `Closed` registration. The registration belongs to
/// the handle passed to [`NativeWindow::set_closed`] and is revoked when that handle is dropped or
/// registers a replacement.
pub struct NativeWindow {
    state: Rc<NativeWindowState>,
    actual_theme_changed: Option<windows_core::EventRevoker>,
    closed: Option<windows_core::EventRevoker>,
    published_title: bool,
    published_visuals: bool,
    size_changed: Option<windows_core::EventRevoker>,
    visuals: WindowVisuals,
}

struct NativeWindowState {
    window: native::Window,
    root: ObjectId,
    root_element: native::FrameworkElement,
    title_bar: Cell<Option<(ObjectId, WindowTitleBarHeight)>>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum WindowTitleBarHeight {
    #[default]
    Standard,
    Tall,
}

impl Clone for NativeWindow {
    fn clone(&self) -> Self {
        Self {
            state: Rc::clone(&self.state),
            actual_theme_changed: None,
            closed: None,
            published_title: self.published_title,
            published_visuals: self.published_visuals,
            size_changed: None,
            visuals: self.visuals.clone(),
        }
    }
}

impl NativeWindow {
    pub fn raw_handle(&self) -> Result<*mut core::ffi::c_void, WinUiError> {
        let mut handle = std::ptr::null_mut();
        unsafe {
            self.state
                .window
                .cast::<native::IWindowNative>()?
                .WindowHandle(&mut handle)
                .ok()?;
        }
        Ok(handle)
    }

    pub fn set_title(&self, title: &str) -> Result<(), WinUiError> {
        self.state.window.SetTitle(title).map_err(Into::into)
    }

    pub(crate) fn apply_publication(
        &mut self,
        title: Option<&str>,
        visuals: Option<&WindowVisuals>,
        color_scheme_observer: Option<Rc<dyn Fn(ColorScheme)>>,
        size_observer: Option<Rc<dyn Fn(WindowSize)>>,
    ) -> Result<(), WinUiError> {
        self.actual_theme_changed = None;
        self.size_changed = size_observer
            .map(|observer| {
                self.state.root_element.SizeChanged(move |_, args| {
                    if let Some(args) = args.as_ref()
                        && let Ok(size) = args.NewSize()
                    {
                        observer(WindowSize {
                            width: f64::from(size.width),
                            height: f64::from(size.height),
                        });
                    }
                })
            })
            .transpose()?;

        if let Some(title) = title {
            self.set_title(title)?;
            self.published_title = true;
        } else if self.published_title {
            self.set_title("")?;
            self.published_title = false;
        }

        if let Some(visuals) = visuals {
            self.apply_visuals(visuals)?;
            self.published_visuals = true;
        } else if self.published_visuals {
            self.apply_visuals(&WindowVisuals::default())?;
            self.published_visuals = false;
        }

        self.actual_theme_changed = color_scheme_observer
            .map(|observer| {
                let initial = match self.state.root_element.ActualTheme()? {
                    native::ElementTheme::Dark => ColorScheme::Dark,
                    _ => ColorScheme::Light,
                };
                observer(initial);
                self.state
                    .root_element
                    .ActualThemeChanged(move |sender, _| {
                        if let Some(sender) = sender.as_ref()
                            && let Ok(theme) = sender.ActualTheme()
                        {
                            observer(match theme {
                                native::ElementTheme::Dark => ColorScheme::Dark,
                                _ => ColorScheme::Light,
                            });
                        }
                    })
            })
            .transpose()?;

        Ok(())
    }

    fn apply_visuals(&mut self, visuals: &WindowVisuals) -> Result<(), WinUiError> {
        let changes = window_visual_changes(&self.visuals, visuals);
        let window_2 = self.state.window.cast::<native::IWindow2>()?;
        let app_window = window_2.AppWindow()?;
        let mut hwnd = None;
        let mut window_handle = || -> Result<*mut core::ffi::c_void, WinUiError> {
            if let Some(hwnd) = hwnd {
                return Ok(hwnd);
            }
            let value = self.raw_handle()?;
            hwnd = Some(value);
            Ok(value)
        };

        if changes.backdrop {
            match visuals.backdrop {
                WindowBackdrop::None => {
                    window_2.SetSystemBackdrop(None::<&native::SystemBackdrop>)?;
                }
                WindowBackdrop::Mica | WindowBackdrop::MicaAlt => {
                    let mica = native::MicaBackdrop::new()?;
                    mica.SetKind(match visuals.backdrop {
                        WindowBackdrop::Mica => native::MicaKind::Base,
                        WindowBackdrop::MicaAlt => native::MicaKind::BaseAlt,
                        _ => unreachable!(),
                    })?;
                    let backdrop: native::SystemBackdrop = mica.cast()?;
                    window_2.SetSystemBackdrop(&backdrop)?;
                }
                WindowBackdrop::Acrylic => {
                    let backdrop: native::SystemBackdrop =
                        native::DesktopAcrylicBackdrop::new()?.cast()?;
                    window_2.SetSystemBackdrop(&backdrop)?;
                }
            }
        }

        if changes.theme {
            app_window
                .TitleBar()?
                .cast::<native::IAppWindowTitleBar3>()?
                .SetPreferredTheme(match visuals.theme {
                    WindowTheme::System => native::TitleBarTheme::UseDefaultAppMode,
                    WindowTheme::Light => native::TitleBarTheme::Light,
                    WindowTheme::Dark => native::TitleBarTheme::Dark,
                })?;
            self.state
                .root_element
                .SetRequestedTheme(match visuals.theme {
                    WindowTheme::System => native::ElementTheme::Default,
                    WindowTheme::Light => native::ElementTheme::Light,
                    WindowTheme::Dark => native::ElementTheme::Dark,
                })?;
        }

        if changes.icon {
            if let Some(path) = &visuals.icon {
                app_window.SetIcon(path)?;
            } else {
                clear_window_icon(window_handle()?);
            }
        }

        if changes.constraints {
            let presenter = app_window
                .Presenter()?
                .cast::<native::IOverlappedPresenter3>()?;
            let (min_width, min_height, max_width, max_height) =
                if let Some(constraints) = visuals.constraints {
                    let dpi = unsafe { native::GetDpiForWindow(window_handle()?.cast()) }.max(96);
                    let pixels = |dips: f64| (dips * f64::from(dpi) / 96.0).round() as i32;
                    let client_window = app_window.cast::<native::IAppWindow2>()?;
                    let outer = app_window.Size()?;
                    let inner = client_window.ClientSize()?;
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
            presenter.SetPreferredMinimumWidth(min_width)?;
            presenter.SetPreferredMinimumHeight(min_height)?;
            presenter.SetPreferredMaximumWidth(max_width)?;
            presenter.SetPreferredMaximumHeight(max_height)?;
        }

        if changes.client_size
            && let Some((width, height)) = visuals.client_size
        {
            let dpi = unsafe { native::GetDpiForWindow(window_handle()?.cast()) }.max(96);
            let pixels = |dips: f64| (dips * f64::from(dpi) / 96.0).round() as i32;
            app_window
                .cast::<native::IAppWindow2>()?
                .ResizeClient(native::SizeInt32 {
                    width: pixels(width),
                    height: pixels(height),
                })?;
        }

        self.visuals = visuals.clone();
        Ok(())
    }

    pub fn activate(&self) -> Result<(), WinUiError> {
        self.state.window.Activate().map_err(Into::into)
    }

    pub fn close(&self) -> Result<(), WinUiError> {
        self.state.window.Close().map_err(Into::into)
    }

    pub fn set_closed(
        &mut self,
        callback: impl Fn() -> windows_core::Result<()> + 'static,
    ) -> Result<(), WinUiError> {
        self.closed = Some(self.state.window.Closed(move |_, _| {
            if let Err(error) = callback() {
                super::app::report_error(error);
            }
        })?);
        Ok(())
    }
}

#[derive(Clone, Copy)]
struct WindowVisualChanges {
    backdrop: bool,
    client_size: bool,
    constraints: bool,
    icon: bool,
    theme: bool,
}

fn window_visual_changes(previous: &WindowVisuals, next: &WindowVisuals) -> WindowVisualChanges {
    WindowVisualChanges {
        backdrop: previous.backdrop != next.backdrop,
        client_size: previous.client_size != next.client_size,
        constraints: previous.constraints != next.constraints,
        icon: previous.icon != next.icon,
        theme: previous.theme != next.theme,
    }
}

fn clear_window_icon(hwnd: *mut core::ffi::c_void) {
    unsafe {
        _ = native::SendMessageW(
            hwnd.cast(),
            native::WM_SETICON as u32,
            native::ICON_BIG as usize,
            0,
        );
        _ = native::SendMessageW(
            hwnd.cast(),
            native::WM_SETICON as u32,
            native::ICON_SMALL as usize,
            0,
        );
    }
}

include!("winui/hosting.rs");

#[cfg(any(test, feature = "test"))]
include!("winui/diagnostics.rs");

include!("winui/objects.rs");
include!("winui/events.rs");
include!("winui/relations.rs");

include!("winui/adapter.rs");

fn relation_contract(
    kind: ObjectType,
    relation: RelationId,
) -> Result<&'static RelationContract, WinUiError> {
    relation_contracts(kind)
        .iter()
        .find(|contract| contract.id == relation)
        .ok_or(WinUiError::MissingContract(kind, relation))
}

fn com_identity(value: &impl Interface) -> Result<usize, WinUiError> {
    Ok(value.cast::<windows_core::IUnknown>()?.as_raw() as usize)
}

fn simulate_active_slot_reorder<T: Clone>(
    current: &mut [(usize, T)],
    active: &HashSet<usize>,
    desired: &[usize],
) -> Option<Vec<(usize, usize, T, T)>> {
    let slots = current
        .iter()
        .enumerate()
        .filter_map(|(index, (identity, _))| active.contains(identity).then_some(index))
        .collect::<Vec<_>>();
    if slots.len() != desired.len() || active.len() != desired.len() {
        return None;
    }
    let mut swaps = Vec::new();
    for (desired_index, left) in slots.iter().copied().enumerate() {
        if current[left].0 == desired[desired_index] {
            continue;
        }
        let right = slots[desired_index + 1..]
            .iter()
            .copied()
            .find(|right| current[*right].0 == desired[desired_index])?;
        let left_value = current[right].1.clone();
        let right_value = current[left].1.clone();
        current.swap(left, right);
        swaps.push((left, right, left_value, right_value));
    }
    current
        .iter()
        .filter_map(|(identity, _)| active.contains(identity).then_some(*identity))
        .eq(desired.iter().copied())
        .then_some(swaps)
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

fn selection_index(value: i32) -> Result<Option<usize>, windows_core::Error> {
    match value {
        -1 => Ok(None),
        0.. => Ok(Some(value as usize)),
        _ => Err(windows_core::Error::new(
            HRESULT(0x80070057_u32 as i32),
            "invalid negative selection index",
        )),
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

fn read_rich_edit_text(value: &native::RichEditBox) -> windows_core::Result<Rc<str>> {
    let document = value.Document()?;
    let mut text = HSTRING::new();
    document.GetText(native::TextGetOptions::UseLf, &mut text)?;
    Ok(Rc::from(text.to_string_lossy()))
}

fn set_rich_edit_text(value: &native::RichEditBox, text: &str) -> Result<(), WinUiError> {
    if read_rich_edit_text(value)
        .map_err(WinUiError::from)?
        .as_ref()
        == text
    {
        return Ok(());
    }
    let document = value.Document()?;
    let read_only = value.IsReadOnly()?;
    if read_only {
        value.SetIsReadOnly(false)?;
    }
    let write = document
        .SetText(native::TextSetOptions::None, text)
        .map_err(WinUiError::from);
    let restore = if read_only {
        value.SetIsReadOnly(true).map_err(WinUiError::from)
    } else {
        Ok(())
    };
    write.and(restore)
}

fn native_grid_length(length: GridLength) -> native::GridLength {
    let (value, grid_unit_type) = match length.size {
        GridLengthSize::Auto => (0.0, native::GridUnitType::Auto),
        GridLengthSize::Pixel(value) => (value, native::GridUnitType::Pixel),
        GridLengthSize::Star(value) => (value, native::GridUnitType::Star),
    };
    native::GridLength {
        value,
        grid_unit_type,
    }
}

fn set_grid_definitions(
    grid: &native::Grid,
    values: &[GridLength],
    rows: bool,
) -> Result<(), WinUiError> {
    if rows {
        let definitions = grid.RowDefinitions()?;
        definitions.Clear()?;
        for length in values {
            let definition = native::RowDefinition::new()?;
            definition.SetHeight(native_grid_length(*length))?;
            if let Some(value) = length.min {
                definition.SetMinHeight(value)?;
            }
            if let Some(value) = length.max {
                definition.SetMaxHeight(value)?;
            }
            definitions.Append(&definition)?;
        }
    } else {
        let definitions = grid.ColumnDefinitions()?;
        definitions.Clear()?;
        for length in values {
            let definition = native::ColumnDefinition::new()?;
            definition.SetWidth(native_grid_length(*length))?;
            if let Some(value) = length.min {
                definition.SetMinWidth(value)?;
            }
            if let Some(value) = length.max {
                definition.SetMaxWidth(value)?;
            }
            definitions.Append(&definition)?;
        }
    }
    Ok(())
}

#[cfg(test)]
include!("winui/tests.rs");
