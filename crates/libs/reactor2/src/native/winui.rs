use super::bindings as native;
use crate::reconcile::{FeedbackExpectation, FeedbackState};
use crate::{
    Adapter, Callback, ComponentHost, CompositionHostEvent, Event, EventDispatch, EventId,
    EventPayload, EventValue, GridLength, GridLengthSize, ImperativeRequest, IntegrationError,
    Mutation, NativeEvent, ObjectId, ObjectType, Observation, Property, PropertyId, PropertyValue,
    Realization, RealizationRequest, RealizedContainer, RelationContract, RelationId,
    RetirementCompletion, Runtime, SelectionContract, SwapChainPanelEvent, relation_contracts,
    selection_for_item_property, selection_for_relation,
};
use native::IElementFactory;
use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;
use std::sync::atomic::{AtomicU32, Ordering};
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

#[derive(Default)]
struct NativeEventQueue {
    events: RefCell<VecDeque<QueuedNativeEvent>>,
    errors: RefCell<VecDeque<WinUiError>>,
    feedback: RefCell<FeedbackState>,
    selection_items: RefCell<Vec<NativeSelectionItem>>,
    waker: RefCell<Option<Rc<dyn Fn()>>>,
    wake_pending: Cell<bool>,
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
    StateMismatch(ObjectId),
    MissingContract(ObjectType, RelationId),
    IndexOverflow(usize),
    ChildNotFound(ObjectId),
    StillOwned(ObjectId),
    InvalidReplacement(ObjectId),
    InvalidEventArgs,
    InvalidDuration,
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
    data_text_key: HSTRING,
    event_queue: Rc<NativeEventQueue>,
    retirements: HashMap<ObjectId, NativeRetirement>,
    virtual_items: HashMap<ObjectId, NativeVirtualItems>,
    observations: HashMap<(ObjectId, u64), ObservationSubscription>,
    webview_initializations: Rc<RefCell<HashMap<ObjectId, WebViewInitialization>>>,
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
            data_text_key: HSTRING::from("Text"),
            event_queue: Rc::new(NativeEventQueue::default()),
            retirements: HashMap::new(),
            virtual_items: HashMap::new(),
            observations: HashMap::new(),
            webview_initializations: Rc::new(RefCell::new(HashMap::new())),
        }
    }
}

impl Drop for WinUiAdapter {
    fn drop(&mut self) {
        for retirement in self.retirements.values() {
            _ = retirement.timer.Stop();
        }
        self.retirements.clear();
        self.event_queue.events.borrow_mut().clear();
        self.observations.clear();
        for initialization in
            std::mem::take(&mut *self.webview_initializations.borrow_mut()).into_values()
        {
            for completion in initialization.completions {
                completion.call(Err(IntegrationError::Unavailable));
            }
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
                        if contract.realization != Realization::Container {
                            self.validate_native_order(object, contract, children)?;
                        }
                    }
                }
            }
        }
        let retired = self
            .retirements
            .values()
            .flat_map(|retirement| retirement.nodes.iter().copied())
            .collect::<HashSet<_>>();
        if visited.len() + retired.len() != self.handles.len() {
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

    pub fn simulate_rich_edit_input(&self, object: ObjectId, text: &str) -> Result<(), WinUiError> {
        let Some(Handle::Generated(GeneratedHandle::RichEditBox(value))) =
            self.handles.get(&object)
        else {
            return Err(WinUiError::InvalidObject(object));
        };
        value
            .value
            .Document()?
            .SetText(native::TextSetOptions::None, text)?;
        let text = read_rich_edit_text(&value.value).map_err(WinUiError::from)?;
        Self::dispatch_string(
            &value.text_changed,
            &self.event_queue,
            object,
            EventId::TextChanged,
            Some(Observation::SetProperty {
                object,
                property: Property {
                    id: PropertyId::Document,
                    value: PropertyValue::String(Rc::clone(&text)),
                },
            }),
            text,
        );
        Ok(())
    }

    pub fn rich_edit_state(&self, object: ObjectId) -> Result<(String, bool), WinUiError> {
        let Some(Handle::Generated(GeneratedHandle::RichEditBox(value))) =
            self.handles.get(&object)
        else {
            return Err(WinUiError::InvalidObject(object));
        };
        Ok((
            read_rich_edit_text(&value.value)
                .map_err(WinUiError::from)?
                .to_string(),
            value.value.IsReadOnly()?,
        ))
    }

    pub fn grid_definition_counts(&self, object: ObjectId) -> Result<(u32, u32), WinUiError> {
        let Some(Handle::Generated(GeneratedHandle::Grid(value))) = self.handles.get(&object)
        else {
            return Err(WinUiError::InvalidObject(object));
        };
        Ok((
            value.RowDefinitions()?.Size()?,
            value.ColumnDefinitions()?.Size()?,
        ))
    }

    pub fn realize_virtual_item(&self, object: ObjectId, index: usize) -> Result<(), WinUiError> {
        let index = i32::try_from(index).map_err(|_| WinUiError::IndexOverflow(index))?;
        match self.handles.get(&object) {
            Some(Handle::Generated(GeneratedHandle::ItemsRepeater(repeater))) => {
                repeater.GetOrCreateElement(index)?;
                Ok(())
            }
            Some(_) => Err(WinUiError::InvalidObject(object)),
            None => Err(WinUiError::MissingObject(object)),
        }
    }

    pub fn virtual_shell_count(&self, object: ObjectId) -> Result<usize, WinUiError> {
        let items = self
            .virtual_items
            .get(&object)
            .ok_or(WinUiError::InvalidObject(object))?;
        Ok(items.shells.pool.borrow().shells.len())
    }

    #[cfg(any(test, feature = "test"))]
    pub fn queued_event_count(&self) -> usize {
        self.event_queue.events.borrow().len()
    }

    #[cfg(any(test, feature = "test"))]
    pub fn total_virtual_shell_count(&self) -> usize {
        self.virtual_items
            .values()
            .map(|items| items.shells.pool.borrow().shells.len())
            .sum()
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

    pub fn retirement_count(&self) -> usize {
        self.retirements.len()
    }

    pub fn owned_physical_children(
        &self,
        parent: ObjectId,
        relation: RelationId,
    ) -> Result<Vec<ObjectId>, WinUiError> {
        let identities = self
            .owners
            .iter()
            .filter(|(_, owner)| **owner == (parent, relation))
            .map(|(object, _)| {
                self.ui_element(*object)
                    .and_then(|value| com_identity(&value))
                    .map(|identity| (identity, *object))
            })
            .collect::<Result<HashMap<_, _>, _>>()?;
        let values = self.owned_collection(parent, relation)?;
        (0..values.size()?)
            .map(|index| {
                let value = values.get_at(index)?;
                identities
                    .get(&com_identity(&value)?)
                    .copied()
                    .ok_or(WinUiError::StateMismatch(parent))
            })
            .collect()
    }

    pub fn contains_object(&self, object: ObjectId) -> bool {
        self.handles.contains_key(&object)
    }

    pub fn opacity(&self, object: ObjectId) -> Result<f64, WinUiError> {
        self.ui_element(object)?.Opacity().map_err(Into::into)
    }

    pub fn simulate_click(&self, object: ObjectId) -> Result<(), WinUiError> {
        let Some(Handle::Generated(handle)) = self.handles.get(&object) else {
            return Err(WinUiError::InvalidObject(object));
        };
        let Some(event) = handle.unit_event(EventId::Click) else {
            return Err(WinUiError::InvalidObject(object));
        };
        Self::dispatch_unit(event, &self.event_queue, object, EventId::Click, None);
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
            None,
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
        let observation = Observation::SetProperty {
            object,
            property: Property {
                id: PropertyId::Password,
                value: PropertyValue::String(Rc::clone(&value)),
            },
        };
        let event = password_box.password_changed.borrow();
        self.event_queue.queue(
            Some(observation),
            event.callback.is_some().then(|| QueuedEvent {
                object,
                event: EventId::PasswordChanged,
                revision: event.revision,
                payload: EventPayload::String(value),
            }),
        );
        Self::schedule_event_wake(&self.event_queue);
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
        let observation = Observation::SetProperty {
            object,
            property: Property {
                id: PropertyId::RatingControlValue,
                value: PropertyValue::OptionalF64(value),
            },
        };
        let event = rating.value_changed.borrow();
        self.event_queue.queue(
            Some(observation),
            event.callback.is_some().then(|| QueuedEvent {
                object,
                event: EventId::ValueChanged,
                revision: event.revision,
                payload: EventPayload::OptionalF64(value),
            }),
        );
        Self::schedule_event_wake(&self.event_queue);
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

    pub fn capability_state(
        &self,
        object: ObjectId,
    ) -> Result<(f64, f64, i32, bool, String, bool), WinUiError> {
        let element = self
            .ui_element(object)?
            .cast::<native::FrameworkElement>()?;
        Ok((
            element.MinWidth()?,
            element.MaxHeight()?,
            native::Grid::GetRow(&element)?,
            native::RelativePanel::GetAlignLeftWithPanel(&element)?,
            native::AutomationProperties::GetName(&element)?,
            element.cast::<native::IControl>()?.IsEnabled()?,
        ))
    }

    pub fn text_block_font_weight(&self, object: ObjectId) -> Result<u16, WinUiError> {
        let Some(Handle::Generated(GeneratedHandle::TextBlock(value))) = self.handles.get(&object)
        else {
            return Err(WinUiError::InvalidObject(object));
        };
        Ok(value.cast::<native::ITextBlock>()?.FontWeight()?.weight)
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
                let retired = self
                    .retirements
                    .iter()
                    .filter_map(|(root, retirement)| {
                        (retirement.parent == parent && retirement.relation == contract.id)
                            .then_some(*root)
                    })
                    .map(|root| self.ui_element(root).and_then(|value| com_identity(&value)))
                    .collect::<Result<HashSet<_>, _>>()?;
                let mut active = Vec::new();
                for index in 0..values.size()? {
                    let value = values.get_at(index)?;
                    let identity = com_identity(&value)?;
                    if !retired.contains(&identity) {
                        active.push(identity);
                    }
                }
                let desired = children
                    .iter()
                    .map(|child| {
                        self.ui_element(*child)
                            .and_then(|value| com_identity(&value))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                active == desired
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
                self.event_queue.queue(Some(observation), None);
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
                self.event_queue.queue(Some(observation), None);
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
        self.event_queue.events.borrow_mut().retain(|event| {
            event
                .event
                .as_ref()
                .is_none_or(|event| event.object != object)
                && event.observation.as_ref().is_none_or(|observation| {
                    !matches!(
                        observation,
                        Observation::SetProperty {
                            object: observed,
                            ..
                        } | Observation::SetSelection {
                            object: observed,
                            ..
                        } if *observed == object
                    )
                })
        });
        self.event_queue.feedback.borrow_mut().remove_object(object);
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
        let event = event.borrow();
        event_queue.queue(
            Some(observation),
            event.callback.is_some().then(|| QueuedEvent {
                object,
                event: EventId::TextChanged,
                revision: event.revision,
                payload: EventPayload::String(text),
            }),
        );
        Self::schedule_event_wake(event_queue);
    }

    fn dispatch_unit(
        event: &Rc<RefCell<NativeUnitEvent>>,
        event_queue: &Rc<NativeEventQueue>,
        object: ObjectId,
        event_id: EventId,
        observation: Option<Observation>,
    ) {
        let event = event.borrow();
        let queued = event.callback.is_some().then(|| QueuedEvent {
            object,
            event: event_id,
            revision: event.revision,
            payload: EventPayload::Unit,
        });
        if observation.is_some() || queued.is_some() {
            event_queue.queue(observation, queued);
            Self::schedule_event_wake(event_queue);
        }
    }

    fn dispatch_bool(
        event: &Rc<RefCell<NativeBoolEvent>>,
        event_queue: &Rc<NativeEventQueue>,
        object: ObjectId,
        event_id: EventId,
        observation: Option<Observation>,
        value: bool,
    ) {
        let event = event.borrow();
        let queued = event.callback.is_some().then(|| QueuedEvent {
            object,
            event: event_id,
            revision: event.revision,
            payload: EventPayload::Bool(value),
        });
        if observation.is_some() || queued.is_some() {
            event_queue.queue(observation, queued);
            Self::schedule_event_wake(event_queue);
        }
    }

    fn dispatch_f64(
        event: &Rc<RefCell<NativeF64Event>>,
        event_queue: &Rc<NativeEventQueue>,
        object: ObjectId,
        event_id: EventId,
        observation: Option<Observation>,
        value: f64,
    ) {
        let event = event.borrow();
        let queued = event.callback.is_some().then(|| QueuedEvent {
            object,
            event: event_id,
            revision: event.revision,
            payload: EventPayload::F64(value),
        });
        if observation.is_some() || queued.is_some() {
            event_queue.queue(observation, queued);
            Self::schedule_event_wake(event_queue);
        }
    }

    fn dispatch_optional_bool(
        event: &Rc<RefCell<NativeOptionalBoolEvent>>,
        event_queue: &Rc<NativeEventQueue>,
        object: ObjectId,
        event_id: EventId,
        observation: Option<Observation>,
        value: Option<bool>,
    ) {
        let event = event.borrow();
        let queued = event.callback.is_some().then(|| QueuedEvent {
            object,
            event: event_id,
            revision: event.revision,
            payload: EventPayload::OptionalBool(value),
        });
        if observation.is_some() || queued.is_some() {
            event_queue.queue(observation, queued);
            Self::schedule_event_wake(event_queue);
        }
    }

    fn dispatch_optional_f64(
        event: &Rc<RefCell<NativeOptionalF64Event>>,
        event_queue: &Rc<NativeEventQueue>,
        object: ObjectId,
        event_id: EventId,
        observation: Option<Observation>,
        value: Option<f64>,
    ) {
        let event = event.borrow();
        let queued = event.callback.is_some().then(|| QueuedEvent {
            object,
            event: event_id,
            revision: event.revision,
            payload: EventPayload::OptionalF64(value),
        });
        if observation.is_some() || queued.is_some() {
            event_queue.queue(observation, queued);
            Self::schedule_event_wake(event_queue);
        }
    }

    fn dispatch_selection_index(
        event: &Rc<RefCell<NativeSelectionIndexEvent>>,
        event_queue: &Rc<NativeEventQueue>,
        object: ObjectId,
        event_id: EventId,
        observation: Option<Observation>,
        value: Option<usize>,
    ) {
        let event = event.borrow();
        let queued = event.callback.is_some().then(|| QueuedEvent {
            object,
            event: event_id,
            revision: event.revision,
            payload: EventPayload::SelectionIndex(value),
        });
        if observation.is_some() || queued.is_some() {
            event_queue.queue(observation, queued);
            Self::schedule_event_wake(event_queue);
        }
    }

    fn dispatch_string(
        event: &Rc<RefCell<NativeTextEvent>>,
        event_queue: &Rc<NativeEventQueue>,
        object: ObjectId,
        event_id: EventId,
        observation: Option<Observation>,
        value: Rc<str>,
    ) {
        let event = event.borrow();
        let queued = event.callback.is_some().then(|| QueuedEvent {
            object,
            event: event_id,
            revision: event.revision,
            payload: EventPayload::String(value),
        });
        if observation.is_some() || queued.is_some() {
            event_queue.queue(observation, queued);
            Self::schedule_event_wake(event_queue);
        }
    }

    fn dispatch_pointer_event_info(
        event: &Rc<RefCell<NativePointerEventInfoEvent>>,
        event_queue: &Rc<NativeEventQueue>,
        object: ObjectId,
        event_id: EventId,
        observation: Option<Observation>,
        value: crate::PointerEventInfo,
    ) {
        if Self::queue_pointer_event_info(event, event_queue, object, event_id, observation, value)
        {
            Self::schedule_event_wake(event_queue);
        }
    }

    fn queue_pointer_event_info(
        event: &Rc<RefCell<NativePointerEventInfoEvent>>,
        event_queue: &Rc<NativeEventQueue>,
        object: ObjectId,
        event_id: EventId,
        observation: Option<Observation>,
        value: crate::PointerEventInfo,
    ) -> bool {
        let event = event.borrow();
        let queued = event.callback.is_some().then(|| QueuedEvent {
            object,
            event: event_id,
            revision: event.revision,
            payload: EventPayload::PointerEventInfo(value),
        });
        let dispatch = observation.is_some() || queued.is_some();
        if dispatch {
            event_queue.queue(observation, queued);
        }
        dispatch
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
        let event = event.borrow();
        event_queue.queue(
            Some(observation),
            event.callback.is_some().then(|| QueuedEvent {
                object,
                event: event_id,
                revision: event.revision,
                payload: EventPayload::Selection(crate::SelectionChange {
                    item: selected_object,
                    value,
                }),
            }),
        );
        Self::schedule_event_wake(event_queue);
    }

    fn pointer_event_info(
        element: &native::UIElement,
        args: Ref<native::PointerRoutedEventArgs>,
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

    fn active_native_index(
        &self,
        parent: ObjectId,
        relation: RelationId,
        active_index: usize,
    ) -> Result<usize, WinUiError> {
        let retired = self
            .retirements
            .iter()
            .filter_map(|(root, retirement)| {
                (retirement.parent == parent && retirement.relation == relation).then_some(*root)
            })
            .collect::<Vec<_>>();
        if retired.is_empty() {
            return Ok(active_index);
        }
        let values = self.owned_collection(parent, relation)?;
        let retired = retired
            .into_iter()
            .map(|root| {
                self.ui_element(root)
                    .and_then(|value| value.cast().map_err(Into::into))
            })
            .collect::<Result<Vec<IInspectable>, _>>()?;
        let mut active = 0;
        for index in 0..values.size()? {
            if retired.contains(&values.get_at(index)?) {
                continue;
            }
            if active == active_index {
                return Ok(index as usize);
            }
            active += 1;
        }
        if active == active_index {
            Ok(values.size()? as usize)
        } else {
            Err(WinUiError::InvalidMutation(relation))
        }
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
        let index = self.active_native_index(parent, relation, index)?;
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
        let index = self.active_native_index(parent, relation, index)?;
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
                    let mut current = (0..values.size()?)
                        .map(|index| {
                            let value = values.get_at(index)?;
                            Ok((com_identity(&value)?, value))
                        })
                        .collect::<Result<Vec<_>, WinUiError>>()?;
                    let desired = children
                        .iter()
                        .map(|child| {
                            self.ui_element(*child)
                                .and_then(|value| com_identity(&value))
                        })
                        .collect::<Result<Vec<_>, _>>()?;
                    let active = desired.iter().copied().collect::<HashSet<_>>();
                    let swaps = simulate_active_slot_reorder(&mut current, &active, &desired)
                        .ok_or(WinUiError::InvalidMutation(relation))?;
                    for (left, right, left_value, right_value) in swaps {
                        values.remove_at(index32(right)?)?;
                        values.remove_at(index32(left)?)?;
                        values.insert_at(index32(left)?, &left_value)?;
                        values.insert_at(index32(right)?, &right_value)?;
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
        let active = children.iter().copied().collect::<HashSet<_>>();
        debug_assert_eq!(
            children.len(),
            self.owners
                .iter()
                .filter(|(object, owner)| **owner == (parent, relation) && active.contains(object))
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

    fn start_retirement(
        &mut self,
        root: ObjectId,
        nodes: Vec<ObjectId>,
        parent: ObjectId,
        relation: RelationId,
        duration: std::time::Duration,
    ) -> Result<(), WinUiError> {
        if self.retirements.contains_key(&root)
            || self.owners.get(&root) != Some(&(parent, relation))
            || nodes.iter().any(|node| !self.handles.contains_key(node))
        {
            return Err(WinUiError::MissingObject(root));
        }
        let duration =
            windows_time::TimeSpan::try_from(duration).map_err(|_| WinUiError::InvalidDuration)?;
        let root_element = self.ui_element(root)?;
        let transition = native::ScalarTransition::new()?;
        transition.SetDuration(duration)?;
        root_element.SetOpacityTransition(&transition)?;
        let timer = native::DispatcherQueue::GetForCurrentThread()?.CreateTimer()?;
        timer.SetInterval(duration)?;
        timer.SetIsRepeating(false)?;
        let event_queue = Rc::clone(&self.event_queue);
        let tick = timer.Tick(move |_, _| {
            event_queue
                .events
                .borrow_mut()
                .push_back(QueuedNativeEvent {
                    observation: None,
                    event: None,
                    retirement: Some(RetirementCompletion { root }),
                    realization: None,
                });
            Self::schedule_event_wake(&event_queue);
        })?;
        self.retirements.insert(
            root,
            NativeRetirement {
                nodes,
                parent,
                relation,
                timer: timer.clone(),
                _tick: tick,
            },
        );
        if let Err(error) = root_element.SetOpacity(0.0) {
            self.retirements.remove(&root);
            return Err(error.into());
        }
        if let Err(error) = timer.Start() {
            self.retirements.remove(&root);
            return Err(error.into());
        }
        Ok(())
    }

    fn complete_retirement(
        &mut self,
        root: ObjectId,
        nodes: &[ObjectId],
    ) -> Result<(), WinUiError> {
        let retirement = self
            .retirements
            .get(&root)
            .ok_or(WinUiError::MissingObject(root))?;
        if retirement.nodes != nodes {
            return Err(WinUiError::InvalidObject(root));
        }
        retirement.timer.Stop()?;
        let parent = retirement.parent;
        let relation = retirement.relation;
        let values = self.owned_collection(parent, relation)?;
        let root_value: IInspectable = self.ui_element(root)?.cast()?;
        let mut index = 0;
        while index < values.size()? && values.get_at(index)? != root_value {
            index += 1;
        }
        if index == values.size()? {
            return Err(WinUiError::ChildNotFound(root));
        }
        values.remove_at(index)?;
        self.retirements.remove(&root);
        for object in nodes {
            self.owners.remove(object);
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
        Ok(())
    }

    fn native_event(&self, queued: &QueuedNativeEvent) -> Option<NativeEvent> {
        if let Some(completion) = queued.retirement {
            return Some(NativeEvent::retirement(completion));
        }
        if let Some(request) = queued.realization {
            return Some(NativeEvent::realization(request));
        }
        let event = queued.event.as_ref().and_then(|queued| {
            let callback = match (self.handles.get(&queued.object), queued.event) {
                (Some(Handle::TextBox(text_box)), EventId::TextChanged) => {
                    let event = text_box.event.borrow();
                    (event.revision == queued.revision)
                        .then(|| event.callback.clone().map(EventValue::String))
                        .flatten()
                }
                (Some(Handle::Generated(handle)), event) => {
                    handle.event_callback(event, queued.revision)
                }
                _ => None,
            }?;
            Some(EventDispatch::new(
                queued.object,
                queued.event,
                callback,
                queued.payload.clone(),
            ))
        });
        Some(NativeEvent::new(queued.observation.clone(), event))
    }
}

impl Runtime<WinUiAdapter> {
    pub fn set_native_event_waker(&mut self, waker: impl Fn() + 'static) {
        let waker = Rc::new(waker);
        let native = Rc::clone(&waker);
        self.adapter_mut().set_event_waker(move || native());
        self.set_imperative_waker(move || waker());
    }
}

impl ComponentHost<WinUiAdapter> {
    pub fn set_native_event_waker(&mut self, waker: impl Fn() + 'static) {
        self.runtime_mut_internal().set_native_event_waker(waker);
    }
}

impl Adapter for WinUiAdapter {
    type Error = WinUiError;

    fn preview_native_events(&self, events: &mut Vec<NativeEvent>) {
        events.extend(
            self.event_queue
                .events
                .borrow()
                .iter()
                .filter_map(|event| self.native_event(event)),
        );
    }

    fn pop_native_event(&mut self) -> Option<NativeEvent> {
        loop {
            let queued = self.event_queue.events.borrow_mut().pop_front()?;
            if let Some(event) = self.native_event(&queued) {
                return Some(event);
            }
        }
    }

    fn take_error(&mut self) -> Option<Self::Error> {
        self.event_queue.errors.borrow_mut().pop_front()
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
                Mutation::SetVirtualSource {
                    object,
                    item_count,
                    source_revision,
                } => {
                    if let Some(items) = self.virtual_items.get(object) {
                        items.reset(*item_count, *source_revision)?;
                    } else {
                        let repeater = match self.handles.get(object) {
                            Some(Handle::Generated(GeneratedHandle::ItemsRepeater(repeater))) => {
                                repeater.clone()
                            }
                            Some(_) => return Err(WinUiError::InvalidObject(*object)),
                            None => return Err(WinUiError::MissingObject(*object)),
                        };
                        let items = NativeVirtualItems::new(
                            &repeater,
                            *object,
                            *item_count,
                            *source_revision,
                            Rc::clone(&self.event_queue),
                        )?;
                        self.virtual_items.insert(*object, items);
                    }
                }
                Mutation::Realize {
                    parent,
                    relation,
                    container,
                    child,
                    ..
                } => {
                    let content = self.ui_element(*child)?;
                    let items = self
                        .virtual_items
                        .get(parent)
                        .ok_or(WinUiError::InvalidObject(*parent))?;
                    items.shells.set_content(*container, Some(&content))?;
                    self.owners.insert(*child, (*parent, *relation));
                }
                Mutation::Recycle {
                    parent,
                    container,
                    child,
                    ..
                } => {
                    let items = self
                        .virtual_items
                        .get(parent)
                        .ok_or(WinUiError::InvalidObject(*parent))?;
                    items.shells.set_content(*container, None)?;
                    items.shells.acknowledge_recycle(*container)?;
                    if let Some(child) = child
                        && self
                            .owners
                            .get(child)
                            .is_some_and(|(owner, _)| owner == parent)
                    {
                        self.owners.remove(child);
                    }
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
                Mutation::Retire {
                    root,
                    nodes,
                    parent,
                    relation,
                    duration,
                } => {
                    self.start_retirement(*root, nodes.clone(), *parent, *relation, *duration)?;
                }
                Mutation::CompleteRetirement { root, nodes } => {
                    self.complete_retirement(*root, nodes)?;
                }
                Mutation::Destroy { object } => {
                    if self.owners.contains_key(object) {
                        return Err(WinUiError::StillOwned(*object));
                    }
                    self.handles
                        .remove(object)
                        .ok_or(WinUiError::MissingObject(*object))?;
                    self.virtual_items.remove(object);
                    self.observations
                        .retain(|(observed, _), _| *observed != *object);
                    if let Some(initialization) =
                        self.webview_initializations.borrow_mut().remove(object)
                    {
                        for completion in initialization.completions {
                            completion.call(Err(IntegrationError::Unavailable));
                        }
                    }
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

    fn focus(&mut self, object: ObjectId) -> Result<bool, Self::Error> {
        self.ui_element(object)?
            .cast::<native::IUIElement>()?
            .Focus(native::FocusState::Programmatic)
            .map_err(Into::into)
    }

    fn imperative(&mut self, request: ImperativeRequest) -> Result<(), Self::Error> {
        match request {
            ImperativeRequest::InitializeWebView2 { object, completion } => {
                let Some(Handle::Generated(GeneratedHandle::WebView2(control))) =
                    self.handles.get(&object)
                else {
                    completion.call(Err(IntegrationError::Unavailable));
                    return Ok(());
                };
                if let Ok(core) = control.CoreWebView2() {
                    completion.call(Ok(core.into()));
                    return Ok(());
                }
                if let Some(initialization) =
                    self.webview_initializations.borrow_mut().get_mut(&object)
                {
                    initialization.completions.push(completion);
                    return Ok(());
                }
                let setup = (|| {
                    let initializations = Rc::clone(&self.webview_initializations);
                    let initialized = control.CoreWebView2Initialized(move |sender, _| {
                        let result = sender
                            .as_ref()
                            .ok_or_else(windows_core::Error::empty)
                            .and_then(|control| control.CoreWebView2())
                            .map(Into::into)
                            .map_err(integration_error);
                        complete_webview_initialization(&initializations, object, result);
                    })?;
                    let framework = control.cast::<native::IFrameworkElement>()?;
                    let is_loaded = framework.IsLoaded()?;
                    Ok::<_, windows_core::Error>((initialized, framework, is_loaded))
                })();
                let (initialized, framework, is_loaded) = match setup {
                    Ok(setup) => setup,
                    Err(error) => {
                        completion.call(Err(integration_error(error)));
                        return Ok(());
                    }
                };
                let loaded = if is_loaded {
                    None
                } else {
                    let control = control.clone();
                    let initializations = Rc::clone(&self.webview_initializations);
                    let loaded = framework.Loaded(move |_, _| {
                        let result = control.EnsureCoreWebView2Async();
                        match result {
                            Ok(action) => {
                                if let Some(initialization) =
                                    initializations.borrow_mut().get_mut(&object)
                                {
                                    initialization._action = Some(action);
                                }
                            }
                            Err(error) => complete_webview_initialization(
                                &initializations,
                                object,
                                Err(integration_error(error)),
                            ),
                        }
                    });
                    match loaded {
                        Ok(loaded) => Some(loaded),
                        Err(error) => {
                            completion.call(Err(integration_error(error)));
                            return Ok(());
                        }
                    }
                };
                self.webview_initializations.borrow_mut().insert(
                    object,
                    WebViewInitialization {
                        _action: None,
                        _loaded: loaded,
                        _initialized: initialized,
                        completions: vec![completion],
                    },
                );
                if is_loaded {
                    match control.EnsureCoreWebView2Async() {
                        Ok(action) => {
                            if let Some(initialization) =
                                self.webview_initializations.borrow_mut().get_mut(&object)
                            {
                                initialization._action = Some(action);
                            }
                        }
                        Err(error) => complete_webview_initialization(
                            &self.webview_initializations,
                            object,
                            Err(integration_error(error)),
                        ),
                    }
                }
            }
            ImperativeRequest::ObserveSwapChainPanel {
                object,
                observation,
                binding,
                callback,
            } => {
                let Some(Handle::Generated(GeneratedHandle::SwapChainPanel(control))) =
                    self.handles.get(&object)
                else {
                    return Ok(());
                };
                let element = control.cast::<native::IFrameworkElement>()?;
                callback.call(SwapChainPanelEvent::Metrics {
                    binding,
                    width: element.ActualWidth().unwrap_or(0.0),
                    height: element.ActualHeight().unwrap_or(0.0),
                    scale_x: control.CompositionScaleX().unwrap_or(1.0),
                    scale_y: control.CompositionScaleY().unwrap_or(1.0),
                });
                let size_callback = callback.clone();
                let size_control = control.clone();
                let size = element.SizeChanged(move |_, args| {
                    if let Some(args) = args.as_ref()
                        && let Ok(size) = args.NewSize()
                    {
                        size_callback.call(SwapChainPanelEvent::Metrics {
                            binding,
                            width: f64::from(size.width),
                            height: f64::from(size.height),
                            scale_x: size_control.CompositionScaleX().unwrap_or(1.0),
                            scale_y: size_control.CompositionScaleY().unwrap_or(1.0),
                        });
                    }
                })?;
                let scale_callback = callback.clone();
                let scale_element = element;
                let scale = control.CompositionScaleChanged(move |sender, _| {
                    if let Some(sender) = sender.as_ref() {
                        scale_callback.call(SwapChainPanelEvent::Metrics {
                            binding,
                            width: scale_element.ActualWidth().unwrap_or(0.0),
                            height: scale_element.ActualHeight().unwrap_or(0.0),
                            scale_x: sender.CompositionScaleX().unwrap_or(1.0),
                            scale_y: sender.CompositionScaleY().unwrap_or(1.0),
                        });
                    }
                })?;
                let rendering_callback = callback;
                let rendering = native::CompositionTarget::Rendering(move |_, _| {
                    rendering_callback.call(SwapChainPanelEvent::Rendering);
                })?;
                self.observations.insert(
                    (object, observation),
                    ObservationSubscription::SwapChainPanel {
                        _rendering: rendering,
                        _scale: scale,
                        _size: size,
                    },
                );
            }
            ImperativeRequest::RequestSwapChainPanelFrame { object, completion } => {
                if !matches!(
                    self.handles.get(&object),
                    Some(Handle::Generated(GeneratedHandle::SwapChainPanel(_)))
                ) {
                    completion.call(Err(IntegrationError::Unavailable));
                    return Ok(());
                }
                let queued_completion = completion.clone();
                let queued = native::DispatcherQueue::GetForCurrentThread().and_then(|queue| {
                    queue.TryEnqueueWithPriority(
                        native::DispatcherQueuePriority::Normal,
                        &native::DispatcherQueueHandler::new(move || {
                            queued_completion.call(Ok(()));
                        }),
                    )
                });
                match queued {
                    Ok(true) => {}
                    Ok(false) => completion.call(Err(IntegrationError::Unavailable)),
                    Err(error) => completion.call(Err(integration_error(error))),
                }
            }
            ImperativeRequest::SetSwapChain {
                object,
                swap_chain,
                completion,
            } => {
                let result = match self.handles.get(&object) {
                    Some(Handle::Generated(GeneratedHandle::SwapChainPanel(control))) => control
                        .cast::<native::ISwapChainPanelNative>()
                        .and_then(|panel| unsafe {
                            panel
                                .SetSwapChain(
                                    swap_chain
                                        .as_ref()
                                        .map_or(std::ptr::null_mut(), Interface::as_raw),
                                )
                                .ok()
                        }),
                    _ => Err(windows_core::Error::new(
                        HRESULT(0x8000000E_u32 as i32),
                        "swap-chain panel unavailable",
                    )),
                }
                .map_err(integration_error);
                completion.call(result);
            }
            ImperativeRequest::SetNativeImageSource {
                object,
                source,
                completion,
            } => {
                let result = match self.handles.get(&object) {
                    Some(Handle::Generated(GeneratedHandle::Image(control))) => source
                        .as_ref()
                        .map(|source| source.cast::<native::ImageSource>())
                        .transpose()
                        .and_then(|source| control.value.SetSource(source.as_ref())),
                    _ => Err(windows_core::Error::new(
                        HRESULT(0x8000000E_u32 as i32),
                        "image unavailable",
                    )),
                }
                .map_err(integration_error);
                completion.call(result);
            }
            ImperativeRequest::ObserveImageScale {
                object,
                observation,
                callback,
            } => {
                let Some(Handle::Generated(GeneratedHandle::Image(control))) =
                    self.handles.get(&object)
                else {
                    return Ok(());
                };
                let element = control.value.cast::<native::UIElement>()?;
                let framework = control.value.cast::<native::IFrameworkElement>()?;
                let changed = Rc::new(RefCell::new(None));
                observe_xaml_scale(&element, &changed, &self.event_queue, callback.clone())?;
                let loaded_element = element;
                let loaded_changed = Rc::clone(&changed);
                let loaded_errors = Rc::clone(&self.event_queue);
                let loaded = framework.Loaded(move |_, _| {
                    if let Err(error) = observe_xaml_scale(
                        &loaded_element,
                        &loaded_changed,
                        &loaded_errors,
                        callback.clone(),
                    ) {
                        loaded_errors.errors.borrow_mut().push_back(error.into());
                    }
                })?;
                self.observations.insert(
                    (object, observation),
                    ObservationSubscription::ImageScale {
                        _changed: changed,
                        _loaded: loaded,
                    },
                );
            }
            ImperativeRequest::ObserveCompositionHost {
                object,
                observation,
                callback,
            } => {
                let Some(Handle::Generated(GeneratedHandle::Grid(control))) =
                    self.handles.get(&object)
                else {
                    return Ok(());
                };
                let element = control.cast::<native::UIElement>()?;
                let framework = control.cast::<native::IFrameworkElement>()?;
                let visual = native::ElementCompositionPreview::GetElementVisual(&element)?;
                let compositor = visual.cast::<native::ICompositionObject>()?.Compositor()?;
                callback.call(CompositionHostEvent::Ready {
                    compositor: compositor.into(),
                    width: framework.ActualWidth().unwrap_or(0.0),
                    height: framework.ActualHeight().unwrap_or(0.0),
                    scale: xaml_scale(&element)?,
                });
                let size_callback = callback.clone();
                let size_element = element.clone();
                let size_errors = Rc::clone(&self.event_queue);
                let size = framework.SizeChanged(move |_, args| {
                    if let Some(args) = args.as_ref()
                        && let Ok(value) = args.NewSize()
                    {
                        match xaml_scale(&size_element) {
                            Ok(scale) => size_callback.call(CompositionHostEvent::Metrics {
                                width: f64::from(value.width),
                                height: f64::from(value.height),
                                scale,
                            }),
                            Err(error) => {
                                size_errors.errors.borrow_mut().push_back(error.into());
                            }
                        }
                    }
                })?;
                let changed = Rc::new(RefCell::new(None));
                let scale_callback = callback.clone();
                let scale_framework = framework.clone();
                observe_xaml_scale(
                    &element,
                    &changed,
                    &self.event_queue,
                    Callback::new(move |scale| {
                        scale_callback.call(CompositionHostEvent::Metrics {
                            width: scale_framework.ActualWidth().unwrap_or(0.0),
                            height: scale_framework.ActualHeight().unwrap_or(0.0),
                            scale,
                        });
                    }),
                )?;
                let loaded_element = element;
                let loaded_changed = Rc::clone(&changed);
                let loaded_errors = Rc::clone(&self.event_queue);
                let loaded_callback = callback;
                let loaded_framework = framework;
                let loaded = loaded_framework.clone().Loaded(move |_, _| {
                    let framework = loaded_framework.clone();
                    if let Err(error) = observe_xaml_scale(
                        &loaded_element,
                        &loaded_changed,
                        &loaded_errors,
                        Callback::new({
                            let callback = loaded_callback.clone();
                            move |scale| {
                                callback.call(CompositionHostEvent::Metrics {
                                    width: framework.ActualWidth().unwrap_or(0.0),
                                    height: framework.ActualHeight().unwrap_or(0.0),
                                    scale,
                                });
                            }
                        }),
                    ) {
                        loaded_errors.errors.borrow_mut().push_back(error.into());
                    }
                })?;
                self.observations.insert(
                    (object, observation),
                    ObservationSubscription::CompositionHost {
                        _changed: changed,
                        _loaded: loaded,
                        _size: size,
                    },
                );
            }
            ImperativeRequest::RevokeObservation {
                object,
                observation,
            } => {
                self.observations.remove(&(object, observation));
            }
            ImperativeRequest::SetCompositionChildVisual {
                object,
                visual,
                completion,
            } => {
                let result = match self.handles.get(&object) {
                    Some(Handle::Generated(GeneratedHandle::Grid(control))) => {
                        control.cast::<native::UIElement>().and_then(|element| {
                            visual
                                .as_ref()
                                .map(|visual| visual.cast::<native::Visual>())
                                .transpose()
                                .and_then(|visual| {
                                    native::ElementCompositionPreview::SetElementChildVisual(
                                        &element,
                                        visual.as_ref(),
                                    )
                                })
                        })
                    }
                    _ => Err(windows_core::Error::new(
                        HRESULT(0x8000000E_u32 as i32),
                        "composition host unavailable",
                    )),
                }
                .map_err(integration_error);
                completion.call(result);
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
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn active_slot_reorder_preserves_interleaved_retiring_indices() {
        let mut current = vec![(1, 1), (99, 99), (2, 2), (3, 3), (4, 4)];
        let active = HashSet::from([1, 2, 3, 4]);

        let swaps = simulate_active_slot_reorder(&mut current, &active, &[4, 3, 2, 1]).unwrap();

        assert_eq!(current, [(4, 4), (99, 99), (3, 3), (2, 2), (1, 1)]);
        assert_eq!(swaps.len(), 2);
        assert!(swaps.iter().all(|(left, right, _, _)| left < right));
    }

    #[test]
    fn active_slot_reorder_rejects_missing_or_duplicate_active_identity() {
        let mut current = vec![(1, ()), (99, ()), (2, ())];
        let active = HashSet::from([1, 2]);

        assert!(simulate_active_slot_reorder(&mut current, &active, &[2, 3]).is_none());
        assert!(simulate_active_slot_reorder(&mut current, &active, &[2, 2]).is_none());
    }
    use std::sync::atomic::{AtomicUsize, Ordering};

    fn virtual_index(value: IInspectable) -> i32 {
        value
            .cast::<windows_reference::IReference<i32>>()
            .unwrap()
            .Value()
            .unwrap()
    }

    #[test]
    fn recycled_physical_shell_keeps_old_token_until_acknowledged() {
        let mut shells = ShellPool::<usize>::default();
        let (old, physical) = shells.take(|| Ok::<_, windows_core::Error>(42)).unwrap();

        assert!(shells.retire(old));
        assert!(shells.retired.contains(&old));
        let (new, reused) = shells
            .take(|| panic!("retired physical shell was not reused"))
            .unwrap();

        assert_ne!(new, old);
        assert_eq!(reused, physical);
        assert!(shells.retired.contains(&old));
        assert!(shells.shells.contains_key(&new));

        shells.acknowledge_recycle(old);
        assert!(!shells.retired.contains(&old));
        assert!(shells.shells.contains_key(&new));
    }

    #[test]
    fn virtual_source_generates_indices_on_demand() {
        let source = ComObject::new(NativeVirtualSource::new(3).unwrap());
        let vector: windows_collections::IObservableVector<IInspectable> = source.to_interface();

        assert_eq!(vector.Size().unwrap(), 3);
        assert_eq!(virtual_index(vector.GetAt(0).unwrap()), 0);
        assert_eq!(virtual_index(vector.GetAt(2).unwrap()), 2);
        assert_eq!(vector.GetAt(3).unwrap_err().code(), E_BOUNDS);

        let mut index = u32::MAX;
        assert!(
            vector
                .IndexOf(&virtual_item_value(2).unwrap(), &mut index)
                .unwrap()
        );
        assert_eq!(index, 2);
        let missing: IInspectable = windows_reference::IReference::<i32>::from(9).into();
        assert!(!vector.IndexOf(&missing, &mut index).unwrap());
        assert_eq!(index, 0);
        let negative: IInspectable = windows_reference::IReference::<i32>::from(-1).into();
        assert!(!vector.IndexOf(&negative, &mut index).unwrap());
        let invalid: IInspectable =
            windows_reference::IReference::<HSTRING>::from(HSTRING::from("invalid")).into();
        assert!(vector.IndexOf(&invalid, &mut index).is_err());

        let mut values = vec![None; 4];
        assert_eq!(vector.GetMany(1, &mut values).unwrap(), 2);
        assert_eq!(virtual_index(values[0].take().unwrap()), 1);
        assert_eq!(virtual_index(values[1].take().unwrap()), 2);
        assert!(values[2].is_none());

        let view = vector.GetView().unwrap();
        assert_eq!(view.Size().unwrap(), 3);
        assert_eq!(virtual_index(view.GetAt(1).unwrap()), 1);

        let iterator = vector.First().unwrap();
        assert!(iterator.HasCurrent().unwrap());
        assert_eq!(virtual_index(iterator.Current().unwrap()), 0);
        assert!(iterator.MoveNext().unwrap());
        assert_eq!(virtual_index(iterator.Current().unwrap()), 1);
        let mut tail = vec![None; 3];
        assert_eq!(iterator.GetMany(&mut tail).unwrap(), 2);
        assert_eq!(virtual_index(tail[0].take().unwrap()), 1);
        assert_eq!(virtual_index(tail[1].take().unwrap()), 2);
        assert!(!iterator.HasCurrent().unwrap());
        assert_eq!(iterator.Current().unwrap_err().code(), E_BOUNDS);
    }

    #[test]
    fn virtual_source_is_read_only_and_validates_index_limits() {
        let max_count = i32::MAX as usize + 1;
        let source = ComObject::new(NativeVirtualSource::new(max_count).unwrap());
        let vector: windows_collections::IObservableVector<IInspectable> = source.to_interface();

        assert_eq!(vector.Size().unwrap(), max_count as u32);
        assert_eq!(
            virtual_index(vector.GetAt(i32::MAX as u32).unwrap()),
            i32::MAX
        );
        assert_eq!(
            vector.GetAt(i32::MAX as u32 + 1).unwrap_err().code(),
            E_BOUNDS
        );
        assert!(NativeVirtualSource::new(max_count + 1).is_err());

        let value = virtual_item_value(0).unwrap();
        assert_eq!(
            vector.SetAt(0, &value).unwrap_err().code(),
            E_ILLEGAL_METHOD_CALL
        );
        assert_eq!(
            vector.Append(&value).unwrap_err().code(),
            E_ILLEGAL_METHOD_CALL
        );
        assert_eq!(vector.Clear().unwrap_err().code(), E_ILLEGAL_METHOD_CALL);
        assert_eq!(
            vector.ReplaceAll(&[Some(value)]).unwrap_err().code(),
            E_ILLEGAL_METHOD_CALL
        );
    }

    #[test]
    fn virtual_source_reset_notifies_once_and_honors_event_removal() {
        let source = ComObject::new(NativeVirtualSource::new(0).unwrap());
        let vector: windows_collections::IObservableVector<IInspectable> = source.to_interface();
        let first = Arc::new(AtomicUsize::new(0));
        let second = Arc::new(AtomicUsize::new(0));
        let first_observed = Arc::clone(&first);
        let first_revoker = vector
            .VectorChanged(move |_, args| {
                let args = args.ok().unwrap();
                assert_eq!(args.CollectionChange().unwrap(), CollectionChange::Reset);
                assert_eq!(args.Index().unwrap(), 0);
                first_observed.fetch_add(1, Ordering::Relaxed);
            })
            .unwrap();
        let second_observed = Arc::clone(&second);
        let second_revoker = vector
            .VectorChanged(move |_, _| {
                second_observed.fetch_add(1, Ordering::Relaxed);
            })
            .unwrap();

        source.reset(10_000);
        assert_eq!(vector.Size().unwrap(), 10_000);
        assert_eq!(first.load(Ordering::Relaxed), 1);
        assert_eq!(second.load(Ordering::Relaxed), 1);

        drop(first_revoker);
        source.reset(10_000);
        assert_eq!(first.load(Ordering::Relaxed), 1);
        assert_eq!(second.load(Ordering::Relaxed), 2);

        drop(second_revoker);
        source.reset(0);
        assert_eq!(vector.Size().unwrap(), 0);
        assert_eq!(first.load(Ordering::Relaxed), 1);
        assert_eq!(second.load(Ordering::Relaxed), 2);
    }

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
        let mut runtime = Runtime::new(crate::RecordingAdapter::default());
        runtime.update(crate::Border::new()).unwrap();
        let object = runtime.graph().root().unwrap();
        let event = Rc::new(RefCell::new(NativePointerEventInfoEvent {
            revision: 7,
            callback: Some(Callback::new(|_| {})),
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
            None,
            payload,
        ));
        let queued = event_queue.events.borrow();
        assert_eq!(queued.len(), 1);
        assert!(queued[0].observation.is_none());
        let event = queued[0].event.as_ref().unwrap();
        assert_eq!(event.object, object);
        assert_eq!(event.event, EventId::PointerReleased);
        assert_eq!(event.revision, 7);
        assert_eq!(event.payload, EventPayload::PointerEventInfo(payload));
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
        assert_eq!(
            GeneratedHandle::feedback_expectation(
                ObjectType::RichEditBox,
                PropertyId::Document,
                None,
            ),
            Some((
                EventId::TextChanged,
                FeedbackExpectation::DeferredExact(Property {
                    id: PropertyId::Document,
                    value: PropertyValue::String(Rc::from("")),
                })
            ))
        );
    }
}
