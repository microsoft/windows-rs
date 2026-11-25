use super::*;

implement_decl! {
    impl NativeElementFactory as NativeElementFactory_Impl: [IElementFactory]
}

struct NativeElementFactory(local::Local<NativeElementFactoryState>);

struct NativeElementFactoryState {
    collection: ObjectId,
    queue: Rc<NativeEventQueue>,
    source: ComObject<NativeVirtualSource>,
    source_revision: Rc<Cell<u64>>,
    shells: Rc<RealizedShells>,
}

pub(super) struct NativeVirtualItems {
    _factory: IElementFactory,
    source: ComObject<NativeVirtualSource>,
    source_revision: Rc<Cell<u64>>,
    pub(super) shells: Rc<RealizedShells>,
}

impl NativeVirtualItems {
    pub(super) fn new(
        repeater: &native::ItemsRepeater,
        collection: ObjectId,
        keys: &Arc<Vec<u64>>,
        source_revision: u64,
        queue: Rc<NativeEventQueue>,
    ) -> Result<Self, windows_core::Error> {
        let shells = Rc::new(RealizedShells::default());
        let source_revision = Rc::new(Cell::new(source_revision));
        let source = ComObject::new(NativeVirtualSource::new(Arc::clone(keys))?);
        let factory: IElementFactory = ComObject::new(NativeElementFactory(local::Local::new(
            NativeElementFactoryState {
                collection,
                queue,
                source: source.clone(),
                source_revision: Rc::clone(&source_revision),
                shells: Rc::clone(&shells),
            },
        )))
        .into_interface();
        let source_interface: IObservableVector<IInspectable> = source.to_interface();
        repeater.SetItemTemplate(&factory)?;
        repeater.SetItemsSource(&source_interface)?;
        Ok(Self {
            _factory: factory,
            source,
            source_revision,
            shells,
        })
    }

    pub(super) fn reset(
        &self,
        keys: &Arc<Vec<u64>>,
        source_revision: u64,
    ) -> windows_core::Result<()> {
        self.source_revision.set(source_revision);
        self.source.set_keys(Arc::clone(keys))
    }
}

pub(super) const E_BOUNDS: HRESULT = HRESULT(0x8000000B_u32 as i32);

pub(super) const E_ILLEGAL_METHOD_CALL: HRESULT = HRESULT(0x8000000E_u32 as i32);

type NativeObservableVector = IObservableVector<IInspectable>;

type NativeVector = IVector<IInspectable>;

type NativeVectorView = IVectorView<IInspectable>;

type NativeIterable = IIterable<IInspectable>;

type NativeIterator = IIterator<IInspectable>;

type NativeVectorChangedEventArgs = IVectorChangedEventArgs;

type NativeKeyIndexMapping = native::IKeyIndexMapping;

implement_decl! {
    impl NativeVirtualSource as pub(super) NativeVirtualSource_Impl: [
        NativeObservableVector,
        NativeVector,
        NativeVectorView,
        NativeIterable,
        NativeKeyIndexMapping,
    ]
}

pub(super) struct NativeVirtualSource {
    keys: Mutex<Keys>,
    handlers: WinEvent<VectorChangedEventHandler<IInspectable>>,
}

/// Keys with their positions, so that resolving a realized item does not scan the list.
#[derive(Default)]
struct KeyList {
    keys: Arc<Vec<u64>>,
    positions: HashMap<u64, usize>,
}

impl KeyList {
    fn new(keys: Arc<Vec<u64>>) -> Arc<Self> {
        let positions: HashMap<u64, usize> = keys
            .iter()
            .enumerate()
            .map(|(index, &key)| (key, index))
            .collect();
        debug_assert_eq!(positions.len(), keys.len());
        Arc::new(Self { keys, positions })
    }

    fn contains(&self, key: u64) -> bool {
        self.positions.contains_key(&key)
    }
}

/// The keys WinUI observes: `head[..head_len]` followed by `tail[tail_start..]`. Between granular
/// notifications this describes each intermediate state without shifting either vector.
struct Keys {
    head: Arc<KeyList>,
    head_len: usize,
    tail: Arc<KeyList>,
    tail_start: usize,
}

impl Keys {
    fn new(keys: Arc<KeyList>) -> Self {
        Self {
            head_len: keys.keys.len(),
            head: keys,
            tail: Arc::default(),
            tail_start: 0,
        }
    }

    fn iter(&self) -> impl Iterator<Item = u64> + '_ {
        self.head.keys[..self.head_len]
            .iter()
            .chain(&self.tail.keys[self.tail_start..])
            .copied()
    }

    fn len(&self) -> usize {
        self.head_len + self.tail.keys.len() - self.tail_start
    }

    fn get(&self, index: usize) -> Option<u64> {
        if index < self.head_len {
            Some(self.head.keys[index])
        } else {
            self.tail
                .keys
                .get(self.tail_start + index - self.head_len)
                .copied()
        }
    }

    fn position(&self, key: u64) -> Option<usize> {
        match self.head.positions.get(&key) {
            Some(&index) if index < self.head_len => Some(index),
            _ => self
                .tail
                .positions
                .get(&key)
                .filter(|&&index| index >= self.tail_start)
                .map(|index| self.head_len + index - self.tail_start),
        }
    }

    fn current(&self) -> Arc<KeyList> {
        if self.head_len == self.head.keys.len() && self.tail_start == self.tail.keys.len() {
            Arc::clone(&self.head)
        } else {
            KeyList::new(Arc::new(self.iter().collect()))
        }
    }
}

impl NativeVirtualSource {
    pub(super) fn new(keys: Arc<Vec<u64>>) -> windows_core::Result<Self> {
        virtual_item_count(keys.len())?;
        Ok(Self {
            keys: Mutex::new(Keys::new(KeyList::new(keys))),
            handlers: WinEvent::new(),
        })
    }
}

impl NativeVirtualSource_Impl {
    fn count(&self) -> u32 {
        self.keys.lock().unwrap().len().try_into().unwrap()
    }

    pub(super) fn position(&self, key: u64) -> Option<usize> {
        self.keys.lock().unwrap().position(key)
    }

    fn get_at(&self, index: u32) -> windows_core::Result<IInspectable> {
        let key = self.keys.lock().unwrap().get(index as usize);
        key.map(virtual_item_value).ok_or_else(|| E_BOUNDS.into())
    }

    fn index_of(&self, value: Ref<IInspectable>, result: &mut u32) -> windows_core::Result<bool> {
        let key = value
            .ok()?
            .cast::<windows_reference::IReference<u64>>()?
            .Value()?;
        let index = self.position(key);
        *result = index.map_or(0, |index| index.try_into().unwrap());
        Ok(index.is_some())
    }

    fn get_many(
        &self,
        start_index: u32,
        items: &mut [Option<IInspectable>],
    ) -> windows_core::Result<u32> {
        let keys = self.keys.lock().unwrap();
        let mut count = 0;
        for (item, key) in items.iter_mut().zip(keys.iter().skip(start_index as usize)) {
            *item = Some(virtual_item_value(key));
            count += 1;
        }
        Ok(count)
    }

    fn read_only_error() -> windows_core::Error {
        windows_core::Error::new(E_ILLEGAL_METHOD_CALL, "virtual item source is read-only")
    }

    /// Replaces the keys, reporting removals and insertions individually when the surviving keys
    /// keep their order so that WinUI can preserve realized elements and scroll anchoring. A
    /// reorder, or a change that keeps no keys, is reported as a reset.
    pub(super) fn set_keys(&self, keys: Arc<Vec<u64>>) -> windows_core::Result<()> {
        virtual_item_count(keys.len())?;
        let previous = self.keys.lock().unwrap().current();
        let keys = KeyList::new(keys);
        let survivors = KeyList::new(Arc::new(
            previous
                .keys
                .iter()
                .copied()
                .filter(|&key| keys.contains(key))
                .collect(),
        ));
        let ordered = keys
            .keys
            .iter()
            .filter(|&&key| survivors.contains(key))
            .eq(survivors.keys.iter());
        if survivors.keys.is_empty() || !ordered {
            *self.keys.lock().unwrap() = Keys::new(keys);
            return self.notify(CollectionChange::Reset, 0);
        }
        // Handlers read the source, so it reflects each change before it is reported.
        let removed: Vec<usize> = previous
            .keys
            .iter()
            .enumerate()
            .filter(|&(_, &key)| !survivors.contains(key))
            .map(|(index, _)| index)
            .collect();
        for (count, &index) in removed.iter().enumerate().rev() {
            *self.keys.lock().unwrap() = Keys {
                head: Arc::clone(&previous),
                head_len: index,
                tail: Arc::clone(&survivors),
                tail_start: index - count,
            };
            self.notify(CollectionChange::ItemRemoved, index)?;
        }
        let mut survivor = 0;
        for (index, key) in keys.keys.iter().enumerate() {
            if survivors.keys.get(survivor) == Some(key) {
                survivor += 1;
                continue;
            }
            *self.keys.lock().unwrap() = Keys {
                head: Arc::clone(&keys),
                head_len: index + 1,
                tail: Arc::clone(&survivors),
                tail_start: survivor,
            };
            self.notify(CollectionChange::ItemInserted, index)?;
        }
        *self.keys.lock().unwrap() = Keys::new(keys);
        Ok(())
    }
    fn notify(&self, change: CollectionChange, index: usize) -> windows_core::Result<()> {
        let source: IObservableVector<IInspectable> = self.to_object().into_interface();
        let args: IVectorChangedEventArgs = ComObject::new(NativeVirtualChangedEventArgs {
            change,
            index: index.try_into().unwrap(),
        })
        .into_interface();
        let mut result = Ok(());
        self.handlers
            .call(|handler: &VectorChangedEventHandler<IInspectable>| {
                let invoked = handler.Invoke(&source, &args);
                if let (Ok(()), Err(error)) = (&result, &invoked) {
                    result = Err(error.clone());
                }
                invoked
            });
        result
    }
}
impl native::IKeyIndexMapping_Impl for NativeVirtualSource_Impl {
    fn KeyFromIndex(&self, index: i32) -> windows_core::Result<HSTRING> {
        let index = usize::try_from(index).map_err(|_| windows_core::Error::from(E_BOUNDS))?;
        self.keys
            .lock()
            .unwrap()
            .get(index)
            .map(|token| HSTRING::from(token.to_string()))
            .ok_or_else(|| E_BOUNDS.into())
    }

    fn IndexFromKey(&self, key: &HSTRING) -> windows_core::Result<i32> {
        let text = key.to_string_lossy();
        let Ok(token) = text.parse::<u64>() else {
            return Ok(-1);
        };
        if text != token.to_string() {
            return Ok(-1);
        }
        Ok(self
            .position(token)
            .map_or(-1, |index| index.try_into().unwrap()))
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
    fn First(&self) -> windows_core::Result<IIterator<IInspectable>> {
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

    fn GetView(&self) -> windows_core::Result<IVectorView<IInspectable>> {
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

struct NativeVirtualChangedEventArgs {
    change: CollectionChange,
    index: u32,
}

impl IVectorChangedEventArgs_Impl for NativeVirtualChangedEventArgs_Impl {
    fn CollectionChange(&self) -> windows_core::Result<CollectionChange> {
        Ok(self.change)
    }

    fn Index(&self) -> windows_core::Result<u32> {
        Ok(self.index)
    }
}

#[derive(Default)]
pub(super) struct RealizedShells {
    pub(super) pool: RefCell<ShellPool<RealizedShell>>,
}

/// A pooled container and its COM identity, which `recycle` uses to find the shell that WinUI
/// returns.
#[derive(Clone)]
pub(super) struct RealizedShell {
    control: native::ContentControl,
    identity: usize,
}

impl RealizedShell {
    fn new() -> Result<Self, windows_core::Error> {
        let control = native::ContentControl::new()?;
        let alignment: IInspectable =
            windows_reference::IReference::from(native::HorizontalAlignment::Stretch).into();
        control.cast::<native::IDependencyObject>()?.SetValue(
            &native::Control::HorizontalContentAlignmentProperty()?,
            &alignment,
        )?;
        let identity = com_identity(&control)?;
        Ok(Self { control, identity })
    }
}

pub(super) struct ShellPool<T> {
    available: Vec<T>,
    next: u64,
    pub(super) retired: HashSet<RealizedContainer>,
    pub(super) shells: HashMap<RealizedContainer, T>,
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
    pub(super) fn take(
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

    pub(super) fn retire(&mut self, container: RealizedContainer) -> bool {
        let Some(shell) = self.shells.remove(&container) else {
            return false;
        };
        self.available.push(shell);
        self.retired.insert(container);
        true
    }

    pub(super) fn acknowledge_recycle(&mut self, container: RealizedContainer) {
        self.retired.remove(&container);
    }
}

impl ShellPool<RealizedShell> {
    fn recycle(
        &mut self,
        element: &native::UIElement,
    ) -> Result<Option<RealizedContainer>, windows_core::Error> {
        let identity = com_identity(element)?;
        let Some((container, shell)) = self.shells.iter().find_map(|(container, shell)| {
            (shell.identity == identity).then(|| (*container, shell.control.clone()))
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
        let (container, shell) = self.pool.borrow_mut().take(RealizedShell::new)?;
        Ok((container, shell.control.cast()?))
    }

    fn recycle(
        &self,
        element: &native::UIElement,
    ) -> Result<Option<RealizedContainer>, windows_core::Error> {
        self.pool.borrow_mut().recycle(element)
    }

    pub(super) fn set_content(
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
            shell.control.SetContent(content)
        } else {
            shell.control.SetContent(None::<&IInspectable>)
        }
    }

    pub(super) fn acknowledge_recycle(
        &self,
        container: RealizedContainer,
    ) -> Result<(), windows_core::Error> {
        self.pool.borrow_mut().acknowledge_recycle(container);
        Ok(())
    }
}

impl native::IElementFactory_Impl for NativeElementFactory_Impl {
    fn GetElement(
        &self,
        args: Ref<native::ElementFactoryGetArgs>,
    ) -> windows_core::Result<native::UIElement> {
        self.0.try_with(|state| state.get_element(args))?
    }

    fn RecycleElement(
        &self,
        args: Ref<native::ElementFactoryRecycleArgs>,
    ) -> windows_core::Result<()> {
        self.0.try_with(|state| state.recycle_element(args))?
    }
}

impl NativeElementFactoryState {
    fn get_element(
        &self,
        args: Ref<native::ElementFactoryGetArgs>,
    ) -> windows_core::Result<native::UIElement> {
        let token = args
            .ok()?
            .Data()?
            .cast::<windows_reference::IReference<u64>>()?
            .Value()?;
        let index = self.source.position(token).ok_or_else(|| {
            windows_core::Error::new(native::E_FAIL, "element factory received an unknown item")
        })?;
        let (container, element) = self.shells.take()?;
        self.queue
            .queue_realization(RealizationRequest::RealizeKey {
                collection: self.collection,
                container,
                index,
                token,
            });
        self.queue.realize_now();
        Ok(element)
    }

    fn recycle_element(
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

pub(super) fn virtual_item_count(item_count: usize) -> windows_core::Result<u32> {
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

pub(super) fn virtual_item_value(key: u64) -> IInspectable {
    windows_reference::IReference::<u64>::from(key).into()
}
