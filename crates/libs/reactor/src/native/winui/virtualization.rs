use super::*;

implement_decl! {
    impl NativeElementFactory as NativeElementFactory_Impl: [IElementFactory]
}

struct NativeElementFactory {
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
        let factory: IElementFactory = ComObject::new(NativeElementFactory {
            collection,
            queue,
            source: source.clone(),
            source_revision: Rc::clone(&source_revision),
            shells: Rc::clone(&shells),
        })
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
        self.source.reset(Arc::clone(keys))
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
    keys: Mutex<Arc<Vec<u64>>>,
    handlers: WinEvent<VectorChangedEventHandler<IInspectable>>,
}

impl NativeVirtualSource {
    pub(super) fn new(keys: Arc<Vec<u64>>) -> windows_core::Result<Self> {
        virtual_item_count(keys.len())?;
        Ok(Self {
            keys: Mutex::new(keys),
            handlers: WinEvent::new(),
        })
    }
}

impl NativeVirtualSource_Impl {
    fn count(&self) -> u32 {
        self.keys.lock().unwrap().len().try_into().unwrap()
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

    pub(super) fn reset(&self, keys: Arc<Vec<u64>>) -> windows_core::Result<()> {
        virtual_item_count(keys.len())?;
        *self.keys.lock().unwrap() = keys;
        let source: IObservableVector<IInspectable> = self.to_object().into_interface();
        let args: IVectorChangedEventArgs =
            ComObject::new(NativeVirtualChangedEventArgs).into_interface();
        self.handlers
            .call(|handler: &VectorChangedEventHandler<IInspectable>| {
                handler.Invoke(&source, &args)
            });
        Ok(())
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
            .keys
            .lock()
            .unwrap()
            .iter()
            .position(|candidate| *candidate == token)
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
        let data = args.ok()?.Data()?;
        let value = data.cast::<windows_reference::IReference<i32>>()?.Value()?;
        let index = usize::try_from(value)
            .map_err(|_| windows_core::Error::new(native::E_FAIL, "negative item index"))?;
        let token = self.source.keys.lock().unwrap()[index];
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

pub(super) fn virtual_item_value(index: u32) -> windows_core::Result<IInspectable> {
    let index = i32::try_from(index)
        .map_err(|_| windows_core::Error::new(E_BOUNDS, "item index exceeds i32"))?;
    Ok(windows_reference::IReference::<i32>::from(index).into())
}
