use super::*;

/// The templates every virtual list shares.
pub(super) struct ListTemplates {
    item: native::DataTemplate,
    container: native::Style,
    panel: native::ItemsPanelTemplate,
}

pub(super) struct NativeVirtualItems {
    _changing: windows_core::EventRevoker,
    _layout: windows_core::EventRevoker,
    source: ComObject<NativeVirtualSource>,
    source_revision: Rc<Cell<u64>>,
    pub(super) shells: Rc<RealizedShells>,
}

impl NativeVirtualItems {
    pub(super) fn new(
        list: &native::ListView,
        templates: &ListTemplates,
        collection: ObjectId,
        keys: &Arc<Vec<u64>>,
        source_revision: u64,
        queue: Rc<NativeEventQueue>,
    ) -> Result<Self, windows_core::Error> {
        let shells = Rc::new(RealizedShells::default());
        let source_revision = Rc::new(Cell::new(source_revision));
        let source = ComObject::new(NativeVirtualSource::new(Arc::clone(keys))?);
        let binder = Rc::new(ContainerBinder {
            collection,
            queue,
            source_revision: Rc::clone(&source_revision),
            shells: Rc::clone(&shells),
            bound: RefCell::default(),
            orphans: RefCell::default(),
        });
        let base = list.cast::<native::IListViewBase>()?;
        // Rows are realized while their containers are prepared, so placeholders only flash.
        base.SetShowsScrollingPlaceholders(false)?;
        let changing = base.ContainerContentChanging(local::handler::<
            _,
            native::ContainerContentChangingEventArgs,
            _,
        >({
            let binder = Rc::clone(&binder);
            move |_, args| binder.changing(args.unwrap()).unwrap()
        }))?;
        let layout = list
            .cast::<native::IFrameworkElement>()?
            .LayoutUpdated(local::handler(move |_, _| binder.flush().unwrap()))?;
        let items = list.cast::<native::IItemsControl>()?;
        items.SetItemTemplate(&templates.item)?;
        items.SetItemContainerStyle(&templates.container)?;
        items.SetItemsPanel(&templates.panel)?;
        // The default container transitions restart on every source change, so rows that update
        // frequently would stay invisible.
        items.SetItemContainerTransitions(&native::TransitionCollection::new()?)?;
        let source_interface: IObservableVector<IInspectable> = source.to_interface();
        items.SetItemsSource(&source_interface)?;
        Ok(Self {
            _changing: changing,
            _layout: layout,
            source,
            source_revision,
            shells,
        })
    }

    pub(super) fn templates() -> windows_core::Result<ListTemplates> {
        // A list item takes its UIA name from its row host, which forwards the row's own name.
        // Otherwise UIA would read the item's token.
        let item = native::XamlReader::Load(
            "<DataTemplate xmlns='http://schemas.microsoft.com/winfx/2006/xaml/presentation'>\
             <ContentControl IsTabStop='False' HorizontalContentAlignment='Stretch' \
             AutomationProperties.Name='{Binding Content.(AutomationProperties.Name), \
             RelativeSource={RelativeSource Self}}' \
             AutomationProperties.HelpText='{Binding Content.(AutomationProperties.HelpText), \
             RelativeSource={RelativeSource Self}}'/>\
             </DataTemplate>",
        )?
        .cast()?;
        // Rows size their items. The default 40 pixel minimum is too tall for dense data.
        let container = native::XamlReader::Load(
            "<Style xmlns='http://schemas.microsoft.com/winfx/2006/xaml/presentation' \
             TargetType='ListViewItem'>\
             <Setter Property='HorizontalContentAlignment' Value='Stretch'/>\
             <Setter Property='MinHeight' Value='0'/>\
             </Style>",
        )?
        .cast()?;
        // The default panel adjusts the offset to keep the first visible item in view as items are
        // inserted and removed, as when ListView moves a dragged item.
        let panel = native::XamlReader::Load(
            "<ItemsPanelTemplate xmlns='http://schemas.microsoft.com/winfx/2006/xaml/presentation'>\
             <ItemsStackPanel ItemsUpdatingScrollMode='KeepScrollOffset' CacheLength='1'/>\
             </ItemsPanelTemplate>",
        )?
        .cast()?;
        Ok(ListTemplates {
            item,
            container,
            panel,
        })
    }

    pub(super) fn tokens(&self) -> Arc<Vec<u64>> {
        self.source.tokens()
    }

    pub(super) fn reset(
        &self,
        list: &native::ListView,
        keys: &Arc<Vec<u64>>,
        source_revision: u64,
        preserve_selection: bool,
    ) -> windows_core::Result<()> {
        let mut selected = if preserve_selection {
            selected_indices(list, keys)?
        } else {
            Vec::new()
        };
        let viewport = list_scroll_viewer(&list.cast()?)?
            .map(|scroll| -> windows_core::Result<_> {
                let horizontal = scroll.HorizontalOffset()?;
                let vertical = scroll.VerticalOffset()?;
                Ok((scroll, horizontal, vertical))
            })
            .transpose()?;
        self.source_revision.set(source_revision);
        self.source.update(Arc::clone(keys))?;
        if let Some((first, rest)) = selected.split_first_mut() {
            list.cast::<native::ISelector>()?
                .SetSelectedItem(&self.source.get_at(*first)?)?;
            if !rest.is_empty() {
                let list = list.cast::<native::IListViewBase>()?;
                rest.sort_unstable();
                for range in rest.chunk_by(|left, right| *left + 1 == *right) {
                    list.SelectRange(&native::ItemIndexRange::CreateInstance(
                        range[0].try_into().unwrap(),
                        range.len().try_into().unwrap(),
                    )?)?;
                }
            }
        }
        if let Some((scroll, horizontal, vertical)) = viewport {
            // Reset requests offset zero when its old item identities disappear. Replace that
            // request in the same update, before layout, without following a moved key.
            scroll.ChangeViewWithOptionalAnimation(Some(horizontal), Some(vertical), None, true)?;
        }
        Ok(())
    }
}

fn selected_indices(list: &native::ListView, keys: &[u64]) -> windows_core::Result<Vec<u32>> {
    let items = list.cast::<native::IListViewBase>()?.SelectedItems()?;
    let count = items.Size()?;
    if count == 0 {
        return Ok(Vec::new());
    }
    let mut selected = HashMap::with_capacity(count as usize);
    for index in 0..count {
        selected.insert(virtual_token(&items.GetAt(index)?)?, index as usize);
    }
    // Keep the primary selected item first without scanning the source once per selection.
    let mut indices = vec![None; count as usize];
    for (index, token) in keys.iter().enumerate() {
        if let Some(order) = selected.get(token) {
            indices[*order] = Some(index.try_into().unwrap());
        }
    }
    Ok(indices.into_iter().flatten().collect())
}

fn list_scroll_viewer(
    object: &native::DependencyObject,
) -> windows_core::Result<Option<native::ScrollViewer>> {
    for index in 0..native::VisualTreeHelper::GetChildrenCount(object)? {
        let child = native::VisualTreeHelper::GetChild(object, index)?;
        if let Ok(scroll) = child.cast::<native::ScrollViewer>() {
            return Ok(Some(scroll));
        }
        if let Some(scroll) = list_scroll_viewer(&child)? {
            return Ok(Some(scroll));
        }
    }
    Ok(None)
}

/// A row bound to a ListView container.
struct Binding {
    container: RealizedContainer,
    token: u64,
    /// ListView recycled the container, but may hand it the same item again in this layout pass.
    parked: bool,
}

/// Binds Reactor rows into the containers ListView realizes, keyed by container COM identity.
///
/// ListView recycles every container on a reset and then reuses them for arbitrary items. A
/// recycled row stays alive until the layout pass ends, so an item that lands in another container
/// moves its row there and keeps its state. Rows that no container reclaims are then recycled.
struct ContainerBinder {
    collection: ObjectId,
    queue: Rc<NativeEventQueue>,
    source_revision: Rc<Cell<u64>>,
    shells: Rc<RealizedShells>,
    bound: RefCell<HashMap<usize, Binding>>,
    /// Retired containers whose rows Reactor recycles when the layout pass ends.
    orphans: RefCell<Vec<RealizedContainer>>,
}

impl ContainerBinder {
    fn changing(
        &self,
        args: &native::ContainerContentChangingEventArgs,
    ) -> windows_core::Result<()> {
        args.SetHandled(true)?;
        if args.Phase()? != 0 {
            return Ok(());
        }
        let item = args.ItemContainer()?;
        let identity = com_identity(&item)?;
        if args.InRecycleQueue()? {
            if let Some(binding) = self.bound.borrow_mut().get_mut(&identity) {
                binding.parked = true;
            }
            return Ok(());
        }
        let token = virtual_token(&args.Item()?)?;
        {
            let mut bound = self.bound.borrow_mut();
            if let Some(binding) = bound.get_mut(&identity)
                && binding.token == token
            {
                // The container still holds this item's row, which updates by key.
                binding.parked = false;
                return Ok(());
            }
            // The container's previous row, and the item's row in any other container, wait for
            // the end of the layout pass. Realizing the item here moves its row.
            let other = bound
                .iter()
                .find_map(|(other, binding)| (binding.token == token).then_some(*other));
            for identity in [Some(identity), other].into_iter().flatten() {
                if let Some(binding) = bound.remove(&identity) {
                    self.orphan(binding.container)?;
                }
            }
        }
        let index = usize::try_from(args.ItemIndex()?)
            .map_err(|_| windows_core::Error::new(native::E_FAIL, "negative item index"))?;
        let host = item
            .cast::<native::IContentControl>()?
            .ContentTemplateRoot()?
            .cast::<native::ContentControl>()?;
        let container = self.shells.take(host)?;
        self.bound.borrow_mut().insert(
            identity,
            Binding {
                container,
                token,
                parked: false,
            },
        );
        self.queue
            .queue_realization(RealizationRequest::RealizeKey {
                collection: self.collection,
                container,
                index,
                token,
            });
        self.queue.realize_now();
        Ok(())
    }

    /// Ends a container's lease. A row Reactor has not yet realized there is canceled now; a
    /// realized row waits for the end of the layout pass, when another container may hold it.
    fn orphan(&self, container: RealizedContainer) -> windows_core::Result<()> {
        self.shells.retire(container)?;
        if self.queue.realization_pending(self.collection, container) {
            self.recycle(container);
        } else {
            self.orphans.borrow_mut().push(container);
        }
        Ok(())
    }

    fn recycle(&self, container: RealizedContainer) {
        self.queue.queue_realization(RealizationRequest::Recycle {
            collection: self.collection,
            container,
            source_revision: self.source_revision.get(),
        });
    }

    /// Recycles the rows no container reclaimed during the layout pass.
    fn flush(&self) -> windows_core::Result<()> {
        let mut orphans = self.orphans.take();
        self.bound.borrow_mut().retain(|_, binding| {
            if binding.parked {
                orphans.push(binding.container);
            }
            !binding.parked
        });
        if orphans.is_empty() {
            return Ok(());
        }
        for container in orphans {
            self.shells.retire(container)?;
            self.recycle(container);
        }
        self.queue.realize_now();
        Ok(())
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

implement_decl! {
    impl NativeVirtualSource as pub(super) NativeVirtualSource_Impl: [
        NativeObservableVector,
        NativeVector,
        NativeVectorView,
        NativeIterable,
    ]
}

/// The ListView item source. Each item is its key's boxed token. A reset boxes new items, so
/// ListView does not anchor to a moved key. The owner restores the viewport after the reset.
pub(super) struct NativeVirtualSource {
    keys: Mutex<Arc<Vec<u64>>>,
    values: Mutex<HashMap<u64, IInspectable>>,
    handlers: WinEvent<VectorChangedEventHandler<IInspectable>>,
}

#[cfg(feature = "test")]
pub fn native_test_list_source(keys: Vec<u64>) -> windows_core::Result<IInspectable> {
    Ok(ComObject::new(NativeVirtualSource::new(Arc::new(keys))?).into_interface())
}

impl NativeVirtualSource {
    pub(super) fn new(keys: Arc<Vec<u64>>) -> windows_core::Result<Self> {
        virtual_item_count(keys.len())?;
        Ok(Self {
            keys: Mutex::new(keys),
            values: Mutex::default(),
            handlers: WinEvent::new(),
        })
    }
}

impl NativeVirtualSource_Impl {
    fn count(&self) -> u32 {
        self.keys.lock().unwrap().len().try_into().unwrap()
    }

    fn get_at(&self, index: u32) -> windows_core::Result<IInspectable> {
        let token = *self
            .keys
            .lock()
            .unwrap()
            .get(index as usize)
            .ok_or(E_BOUNDS)?;
        Ok(self
            .values
            .lock()
            .unwrap()
            .entry(token)
            .or_insert_with(|| windows_reference::IReference::from(token).into())
            .clone())
    }

    fn index_of(&self, value: Ref<IInspectable>, result: &mut u32) -> windows_core::Result<bool> {
        *result = 0;
        let value = value.ok()?;
        let Ok(token) = virtual_token(value) else {
            return Ok(false);
        };
        let current = self.values.lock().unwrap().get(&token).cloned();
        if current.map(|current| com_identity(&current)).transpose()? != Some(com_identity(value)?)
        {
            return Ok(false);
        }
        let index = self
            .keys
            .lock()
            .unwrap()
            .iter()
            .position(|key| *key == token);
        if let Some(index) = index {
            *result = index.try_into().unwrap();
        }
        Ok(index.is_some())
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
            *item = Some(self.get_at(start_index + offset)?);
        }
        u32::try_from(actual)
            .map_err(|_| windows_core::Error::new(E_BOUNDS, "item buffer is too large"))
    }

    fn read_only_error() -> windows_core::Error {
        windows_core::Error::new(E_ILLEGAL_METHOD_CALL, "virtual item source is read-only")
    }

    /// ListView moves a dragged item by removing it and inserting it again, so the source accepts
    /// removal and the reinsertion of a removed item.
    fn remove_at(&self, index: u32) -> windows_core::Result<()> {
        {
            let mut keys = self.keys.lock().unwrap();
            if index as usize >= keys.len() {
                return Err(E_BOUNDS.into());
            }
            Arc::make_mut(&mut keys).remove(index as usize);
        }
        self.notify(CollectionChange::ItemRemoved, index)
    }

    fn insert_at(&self, index: u32, value: &IInspectable) -> windows_core::Result<()> {
        let token = virtual_token(value)?;
        {
            let mut keys = self.keys.lock().unwrap();
            if index as usize > keys.len() {
                return Err(E_BOUNDS.into());
            }
            if keys.contains(&token) || !self.values.lock().unwrap().contains_key(&token) {
                return Err(Self::read_only_error());
            }
            Arc::make_mut(&mut keys).insert(index as usize, token);
        }
        self.notify(CollectionChange::ItemInserted, index)
    }

    /// The source's tokens in native order.
    pub(super) fn tokens(&self) -> Arc<Vec<u64>> {
        Arc::clone(&self.keys.lock().unwrap())
    }

    /// Replaces the source's keys with a reset.
    pub(super) fn update(&self, keys: Arc<Vec<u64>>) -> windows_core::Result<()> {
        virtual_item_count(keys.len())?;
        self.values.lock().unwrap().clear();
        *self.keys.lock().unwrap() = keys;
        self.notify(CollectionChange::Reset, 0)
    }

    fn notify(&self, change: CollectionChange, index: u32) -> windows_core::Result<()> {
        let source: IObservableVector<IInspectable> = self.to_object().into_interface();
        let args: IVectorChangedEventArgs =
            ComObject::new(NativeVirtualChangedEventArgs { change, index }).into_interface();
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

    fn InsertAt(&self, index: u32, value: Ref<IInspectable>) -> windows_core::Result<()> {
        self.insert_at(index, value.ok()?)
    }

    fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        self.remove_at(index)
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
    pub(super) pool: RefCell<ShellPool<native::ContentControl>>,
}

/// Leases containers to Reactor rows. Each lease gets a new identity, so a retired container's
/// pending recycle cannot clear the row of the container's next lease.
pub(super) struct ShellPool<T> {
    next: u64,
    pub(super) shells: HashMap<RealizedContainer, T>,
}

impl<T> Default for ShellPool<T> {
    fn default() -> Self {
        Self {
            next: 0,
            shells: HashMap::new(),
        }
    }
}

impl<T> ShellPool<T> {
    pub(super) fn take(&mut self, shell: T) -> Result<RealizedContainer, windows_core::Error> {
        let container = RealizedContainer(self.next);
        self.next = self.next.checked_add(1).ok_or_else(|| {
            windows_core::Error::new(native::E_FAIL, "container identity exhausted")
        })?;
        self.shells.insert(container, shell);
        Ok(container)
    }

    pub(super) fn retire(&mut self, container: RealizedContainer) -> Option<T> {
        self.shells.remove(&container)
    }

    pub(super) fn issued(&self, container: RealizedContainer) -> bool {
        container.0 < self.next
    }
}

impl RealizedShells {
    fn take(&self, host: native::ContentControl) -> Result<RealizedContainer, windows_core::Error> {
        self.pool.borrow_mut().take(host)
    }

    /// Ends a lease, detaching its row. Retiring an ended lease does nothing.
    fn retire(&self, container: RealizedContainer) -> Result<(), windows_core::Error> {
        let shell = self.pool.borrow_mut().retire(container);
        shell.map_or(Ok(()), |shell| shell.SetContent(None::<&IInspectable>))
    }

    pub(super) fn leased(&self, container: RealizedContainer) -> bool {
        self.pool.borrow().shells.contains_key(&container)
    }

    pub(super) fn set_content(
        &self,
        container: RealizedContainer,
        content: Option<&native::UIElement>,
    ) -> Result<(), windows_core::Error> {
        let pool = self.pool.borrow();
        let Some(shell) = pool.shells.get(&container) else {
            return if content.is_none() && pool.issued(container) {
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

pub(super) fn virtual_token(value: &IInspectable) -> windows_core::Result<u64> {
    value.cast::<windows_reference::IReference<u64>>()?.Value()
}
