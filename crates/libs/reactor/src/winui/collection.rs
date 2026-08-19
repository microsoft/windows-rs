use super::*;

const CONTENT_TEMPLATE_XAML: &str = "<DataTemplate xmlns='http://schemas.microsoft.com/winfx/2006/xaml/presentation'><ContentControl HorizontalContentAlignment='Stretch' VerticalContentAlignment='Stretch'/></DataTemplate>";

#[derive(Clone)]
struct NativeSelectorItem {
    key: u64,
    text: String,
    value: windows_core::IInspectable,
}

pub(super) struct SelectorState {
    items: Rc<RefCell<Vec<NativeSelectorItem>>>,
    callback: Rc<SelectorCallbackState>,
}

pub(super) struct SelectorCallbackState {
    selection_mode: Cell<SelectionMode>,
    expected_selection: RefCell<CollectionSelection>,
    native_selection: RefCell<CollectionSelection>,
    suppressing_selection: Cell<bool>,
}

pub(super) struct SingleSelectorState {
    items: Rc<RefCell<Vec<NativeSelectorItem>>>,
    callback: Rc<SingleSelectorCallbackState>,
    deferred_selection: Option<deferred::DeferredUpdate>,
    pending_selection: Rc<RefCell<Option<DeferredSelection>>>,
}

pub(super) struct SingleSelectorCallbackState {
    expected_selection: Cell<Option<u64>>,
    native_selection: Cell<Option<u64>>,
    suppressing_selection: Cell<bool>,
}

impl std::ops::Deref for SelectorState {
    type Target = SelectorCallbackState;

    fn deref(&self) -> &Self::Target {
        &self.callback
    }
}

impl std::ops::Deref for SingleSelectorState {
    type Target = SingleSelectorCallbackState;

    fn deref(&self) -> &Self::Target {
        &self.callback
    }
}

struct DeferredSelection {
    value: Option<windows_core::IInspectable>,
    key: Option<u64>,
}

#[derive(Clone)]
pub(super) struct RealizationSlot {
    pub(super) content: bindings::IContentControl,
}

pub(super) struct VirtualCollection {
    pub(super) slots: Rc<RefCell<BTreeMap<(usize, u64), RealizationSlot>>>,
    source: Rc<RefCell<Option<windows_collections::IObservableVector<windows_core::IInspectable>>>>,
    callback: Rc<VirtualCollectionCallbackState>,
    keyed: bool,
    selection_mode: SelectionMode,
    selection_dirty: Cell<bool>,
    _revokers: Box<[windows_core::EventRevoker; 4]>,
}

pub(super) struct VirtualCollectionCallbackState {
    expected_order: RefCell<Vec<u64>>,
    expected_selection: RefCell<CollectionSelection>,
    native_selection: RefCell<CollectionSelection>,
    suppressing_selection: Cell<bool>,
}

impl std::ops::Deref for VirtualCollection {
    type Target = VirtualCollectionCallbackState;

    fn deref(&self) -> &Self::Target {
        &self.callback
    }
}

fn publish_collection_selection(
    id: NodeId,
    selection: CollectionSelection,
    expected_selection: &RefCell<CollectionSelection>,
    native_selection: &RefCell<CollectionSelection>,
    events: &RefCell<VecDeque<NativeEvent>>,
    waker: &RefCell<Option<Rc<dyn Fn()>>>,
) {
    if *expected_selection.borrow() == selection {
        *native_selection.borrow_mut() = selection;
        return;
    }
    *native_selection.borrow_mut() = selection.clone();
    *expected_selection.borrow_mut() = selection.clone();
    queue_latest_event(
        events,
        NativeEvent::SelectionChanged {
            target: id,
            selection,
        },
    );
    if let Some(wake) = waker.borrow().as_ref() {
        wake();
    }
}

fn publish_single_selection(
    id: NodeId,
    selection: Option<u64>,
    expected_selection: &Cell<Option<u64>>,
    native_selection: &Cell<Option<u64>>,
    events: &RefCell<VecDeque<NativeEvent>>,
    waker: &RefCell<Option<Rc<dyn Fn()>>>,
) {
    if expected_selection.get() == selection {
        native_selection.set(selection);
        return;
    }
    native_selection.set(selection);
    expected_selection.set(selection);
    queue_latest_event(
        events,
        NativeEvent::SelectedKeyChanged {
            target: id,
            key: selection,
        },
    );
    if let Some(wake) = waker.borrow().as_ref() {
        wake();
    }
}

fn selector_state(
    selector: &bindings::ISelector,
    id: NodeId,
    events: Rc<RefCell<VecDeque<NativeEvent>>>,
    waker: Rc<RefCell<Option<Rc<dyn Fn()>>>>,
) -> WindowsResult<(windows_core::EventRevoker, Box<SelectorState>)> {
    let items = Rc::new(RefCell::new(Vec::<NativeSelectorItem>::new()));
    let callback = Rc::new(SelectorCallbackState {
        selection_mode: Cell::new(SelectionMode::Single),
        expected_selection: RefCell::new(CollectionSelection::default()),
        native_selection: RefCell::new(CollectionSelection::default()),
        suppressing_selection: Cell::new(false),
    });
    let selection_items = Rc::clone(&items);
    let selection_state = Rc::clone(&callback);
    let callback_events = Rc::clone(&events);
    let callback_waker = Rc::clone(&waker);
    let revoker = selector.SelectionChanged(move |_sender, args| {
        if selection_state.suppressing_selection.get() {
            return;
        }
        let args = args.as_ref().unwrap();
        let selection = apply_list_box_selection_delta(
            &selection_state.native_selection.borrow(),
            args,
            &selection_items.borrow(),
        )
        .unwrap();
        publish_collection_selection(
            id,
            selection,
            &selection_state.expected_selection,
            &selection_state.native_selection,
            &callback_events,
            &callback_waker,
        );
    })?;
    Ok((revoker, Box::new(SelectorState { items, callback })))
}

fn combo_box_state(
    value: &bindings::ISelector,
    id: NodeId,
    events: Rc<RefCell<VecDeque<NativeEvent>>>,
    waker: Rc<RefCell<Option<Rc<dyn Fn()>>>>,
) -> WindowsResult<(windows_core::EventRevoker, Box<SingleSelectorState>)> {
    let items = Rc::new(RefCell::new(Vec::<NativeSelectorItem>::new()));
    let callback = Rc::new(SingleSelectorCallbackState {
        expected_selection: Cell::new(None),
        native_selection: Cell::new(None),
        suppressing_selection: Cell::new(false),
    });
    let selection_items = Rc::clone(&items);
    let selection_state = Rc::clone(&callback);
    let callback_events = Rc::clone(&events);
    let callback_waker = Rc::clone(&waker);
    let revoker = value.SelectionChanged(move |_sender, args| {
        if selection_state.suppressing_selection.get() {
            return;
        }
        let args = args.as_ref().unwrap();
        let selection = apply_single_selection_delta(
            selection_state.native_selection.get(),
            args,
            &selection_items.borrow(),
            "ComboBox",
        )
        .unwrap();
        publish_single_selection(
            id,
            selection,
            &selection_state.expected_selection,
            &selection_state.native_selection,
            &callback_events,
            &callback_waker,
        );
    })?;
    Ok((
        revoker,
        Box::new(SingleSelectorState {
            items,
            callback,
            deferred_selection: None,
            pending_selection: Rc::new(RefCell::new(None)),
        }),
    ))
}

fn radio_buttons_state(
    value: &bindings::IRadioButtons,
    id: NodeId,
    events: Rc<RefCell<VecDeque<NativeEvent>>>,
    waker: Rc<RefCell<Option<Rc<dyn Fn()>>>>,
) -> WindowsResult<(windows_core::EventRevoker, Box<SingleSelectorState>)> {
    let items = Rc::new(RefCell::new(Vec::<NativeSelectorItem>::new()));
    let callback = Rc::new(SingleSelectorCallbackState {
        expected_selection: Cell::new(None),
        native_selection: Cell::new(None),
        suppressing_selection: Cell::new(false),
    });
    let selection_control = value.clone();
    let selection_items = Rc::clone(&items);
    let selection_state = Rc::clone(&callback);
    let callback_events = Rc::clone(&events);
    let callback_waker = Rc::clone(&waker);
    let revoker = value.SelectionChanged(move |_sender, _args| {
        if selection_state.suppressing_selection.get() {
            return;
        }
        let index = selection_control.SelectedIndex().unwrap();
        let key = usize::try_from(index)
            .ok()
            .and_then(|index| selection_items.borrow().get(index).map(|item| item.key));
        publish_single_selection(
            id,
            key,
            &selection_state.expected_selection,
            &selection_state.native_selection,
            &callback_events,
            &callback_waker,
        );
    })?;
    Ok((
        revoker,
        Box::new(SingleSelectorState {
            items,
            callback,
            deferred_selection: Some(deferred::DeferredUpdate::new()),
            pending_selection: Rc::new(RefCell::new(None)),
        }),
    ))
}

fn set_selector_items(
    native: &windows_collections::IVector<windows_core::IInspectable>,
    current_items: &RefCell<Vec<NativeSelectorItem>>,
    suppressing_selection: &Cell<bool>,
    items: &[SelectorItem],
    create: impl Fn(&str) -> WindowsResult<windows_core::IInspectable>,
    replace_all: bool,
) -> WindowsResult<()> {
    let mut previous = current_items
        .borrow()
        .iter()
        .cloned()
        .map(|item| (item.key, item))
        .collect::<BTreeMap<_, _>>();
    let next = items
        .iter()
        .map(|item| -> WindowsResult<NativeSelectorItem> {
            if let Some(previous) = previous.remove(&item.key())
                && previous.text == item.text()
            {
                return Ok(previous);
            }
            Ok(NativeSelectorItem {
                key: item.key(),
                text: item.text().to_string(),
                value: create(item.text())?,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    suppressing_selection.set(true);
    let result: WindowsResult<()> = (|| {
        if replace_all {
            native.Clear()?;
            for item in &next {
                native.Append(&item.value)?;
            }
        } else {
            let mut working = current_items.borrow().clone();
            for (index, item) in next.iter().enumerate() {
                if working
                    .get(index)
                    .is_some_and(|current| current.value == item.value)
                {
                    continue;
                }
                if let Some(source) = working
                    .get(index..)
                    .unwrap_or_default()
                    .iter()
                    .position(|current| current.value == item.value)
                    .map(|source| source + index)
                {
                    let current = working.remove(source);
                    native.RemoveAt(u32::try_from(source).unwrap())?;
                    native.InsertAt(u32::try_from(index).unwrap(), &current.value)?;
                    working.insert(index, current);
                } else {
                    native.InsertAt(u32::try_from(index).unwrap(), &item.value)?;
                    working.insert(index, item.clone());
                }
            }
            while working.len() > next.len() {
                native.RemoveAt(u32::try_from(next.len()).unwrap())?;
                working.remove(next.len());
            }
        }
        Ok(())
    })();
    suppressing_selection.set(false);
    result?;
    *current_items.borrow_mut() = next;
    Ok(())
}

impl WinUiRuntime {
    pub(super) fn create_list_box(&self, id: NodeId) -> WindowsResult<Handle> {
        let value = bindings::ListBox::new()?;
        let list: bindings::IListBox = value.cast()?;
        let selector: bindings::ISelector = value.cast()?;
        let (revoker, state) = selector_state(
            &selector,
            id,
            Rc::clone(&self.events),
            Rc::clone(&self.waker),
        )?;
        Ok(Handle::ListBox {
            _revoker: revoker,
            value: list,
            state,
        })
    }

    pub(super) fn create_combo_box(&self, id: NodeId) -> WindowsResult<Handle> {
        let value = bindings::ComboBox::new()?;
        let selector: bindings::ISelector = value.cast()?;
        let (revoker, state) = combo_box_state(
            &selector,
            id,
            Rc::clone(&self.events),
            Rc::clone(&self.waker),
        )?;
        Ok(Handle::ComboBox {
            _revoker: revoker,
            value: selector,
            state,
        })
    }

    pub(super) fn create_radio_buttons(&self, id: NodeId) -> WindowsResult<Handle> {
        let value = bindings::RadioButtons::new()?;
        let radio: bindings::IRadioButtons = value.cast()?;
        let (revoker, state) =
            radio_buttons_state(&radio, id, Rc::clone(&self.events), Rc::clone(&self.waker))?;
        Ok(Handle::RadioButtons {
            _revoker: revoker,
            value: radio,
            state,
        })
    }

    pub(super) fn create_virtual_collection(
        &self,
        id: NodeId,
        kind: NativeKind,
    ) -> WindowsResult<Handle> {
        let list: bindings::IListViewBase = match kind {
            NativeKind::ListView => bindings::ListView::new().and_then(|value| value.cast())?,
            NativeKind::GridView => bindings::GridView::new().and_then(|value| value.cast())?,
            _ => {
                panic!("virtual collection kind is not ListView or GridView");
            }
        };
        let slots = Rc::new(RefCell::new(
            BTreeMap::<(usize, u64), RealizationSlot>::new(),
        ));
        let source = Rc::new(RefCell::new(
            None::<windows_collections::IObservableVector<windows_core::IInspectable>>,
        ));
        let callback = Rc::new(VirtualCollectionCallbackState {
            expected_order: RefCell::new(Vec::new()),
            expected_selection: RefCell::new(CollectionSelection::default()),
            native_selection: RefCell::new(CollectionSelection::default()),
            suppressing_selection: Cell::new(false),
        });
        let slots_callback = Rc::clone(&slots);
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let next_lease = Rc::clone(&self.next_lease);
        let realization_revoker = list.ContainerContentChanging(move |_sender, args| {
            let args = args.as_ref().unwrap();
            let pending = (|| -> WindowsResult<Vec<NativeEvent>> {
                let container = args.ItemContainer()?;
                let root = container.cast::<bindings::IContentControl>()?;
                let content = root
                    .ContentTemplateRoot()
                    .and_then(|root| root.cast::<bindings::IContentControl>())?;
                let mut pending = Vec::new();
                if args.InRecycleQueue()? {
                    content.SetContent(None::<&windows_core::IInspectable>)?;
                    let key = slots_callback
                        .borrow()
                        .iter()
                        .find(|(_, slot)| slot.content == content)
                        .map(|(key, _)| *key);
                    if let Some((index, lease)) = key {
                        slots_callback.borrow_mut().remove(&(index, lease));
                        pending.push(NativeEvent::Recycle {
                            host: id,
                            index,
                            lease,
                        });
                    }
                } else {
                    let index = usize::try_from(args.ItemIndex()?)
                        .unwrap_or_else(|_| panic!("realized item index is negative"));
                    args.SetHandled(true)?;
                    let lease = next_lease.get();
                    next_lease.set(
                        lease
                            .checked_add(1)
                            .unwrap_or_else(|| panic!("realization lease exhausted")),
                    );
                    let stale = slots_callback
                        .borrow()
                        .iter()
                        .find(|(_, slot)| slot.content == content)
                        .map(|(key, _)| *key);
                    if let Some((stale_index, stale_lease)) = stale {
                        slots_callback
                            .borrow_mut()
                            .remove(&(stale_index, stale_lease));
                        pending.push(NativeEvent::Recycle {
                            host: id,
                            index: stale_index,
                            lease: stale_lease,
                        });
                    }
                    slots_callback
                        .borrow_mut()
                        .insert((index, lease), RealizationSlot { content });
                    pending.push(NativeEvent::Realize {
                        host: id,
                        index,
                        lease,
                    });
                }
                Ok(pending)
            })()
            .unwrap();
            if !pending.is_empty() {
                events.borrow_mut().extend(pending);
                if let Some(wake) = waker.borrow().as_ref() {
                    wake();
                }
            }
        })?;

        let item_events = Rc::clone(&self.events);
        let item_waker = Rc::clone(&self.waker);
        let item_click_revoker = list.ItemClick(move |_sender, args| {
            let args = args.as_ref().unwrap();
            let item = args.ClickedItem().unwrap();
            let key = unbox_key(&item).unwrap();
            item_events
                .borrow_mut()
                .push_back(NativeEvent::ItemInvoked { target: id, key });
            if let Some(wake) = item_waker.borrow().as_ref() {
                wake();
            }
        })?;

        let selector: bindings::ISelector = list.cast()?;
        let selection_state = Rc::clone(&callback);
        let selection_events = Rc::clone(&self.events);
        let selection_waker = Rc::clone(&self.waker);
        let selection_revoker = selector.SelectionChanged(move |_sender, args| {
            if selection_state.suppressing_selection.get() {
                return;
            }
            let args = args.as_ref().unwrap();
            let selection =
                apply_selection_delta(&selection_state.native_selection.borrow(), args).unwrap();
            publish_collection_selection(
                id,
                selection,
                &selection_state.expected_selection,
                &selection_state.native_selection,
                &selection_events,
                &selection_waker,
            );
        })?;

        let reorder_source = Rc::clone(&source);
        let reorder_state = Rc::clone(&callback);
        let reorder_events = Rc::clone(&self.events);
        let reorder_waker = Rc::clone(&self.waker);
        let reorder_revoker = list.DragItemsCompleted(move |_sender, _args| {
            let source = reorder_source.borrow();
            let source = source
                .as_ref()
                .unwrap_or_else(|| panic!("reordered ListView has no item source"));
            let keys = (0..source.Size().unwrap())
                .map(|index| unbox_key(&source.GetAt(index).unwrap()).unwrap())
                .collect::<Vec<_>>();
            if *reorder_state.expected_order.borrow() == keys {
                return;
            }
            reorder_state.expected_order.borrow_mut().clone_from(&keys);
            queue_latest_event(
                &reorder_events,
                NativeEvent::ItemsReordered { target: id, keys },
            );
            if let Some(wake) = reorder_waker.borrow().as_ref() {
                wake();
            }
        })?;

        Ok(Handle::Collection {
            value: list,
            state: VirtualCollection {
                slots,
                source,
                callback,
                keyed: false,
                selection_mode: SelectionMode::Single,
                selection_dirty: Cell::new(false),
                _revokers: Box::new([
                    realization_revoker,
                    item_click_revoker,
                    selection_revoker,
                    reorder_revoker,
                ]),
            },
        })
    }

    pub(super) fn apply_list_box_items(
        &self,
        id: NodeId,
        items: &[SelectorItem],
    ) -> WindowsResult<()> {
        let Handle::ListBox {
            value: list, state, ..
        } = &self.node(id)?.handle
        else {
            panic!("items target is not a ListBox");
        };
        let items_control: bindings::IItemsControl = list.cast()?;
        let native = items_control.Items().and_then(|items| items.cast())?;
        set_selector_items(
            &native,
            &state.items,
            &state.suppressing_selection,
            items,
            |value| {
                let text = bindings::TextBlock::new()?;
                text.cast::<bindings::ITextBlock>()
                    .and_then(|text| text.SetText(value))?;
                Ok(text.into())
            },
            true,
        )?;
        *state.native_selection.borrow_mut() = CollectionSelection::default();
        Ok(())
    }

    pub(super) fn apply_combo_box_update(
        &self,
        id: NodeId,
        update: &ComboBoxUpdate,
    ) -> WindowsResult<()> {
        match update {
            ComboBoxUpdate::Items(value) => self.apply_combo_box_items(id, value),
            ComboBoxUpdate::Selection(value) => self.apply_combo_box_selection(id, *value),
            ComboBoxUpdate::Header(value) => self.apply_combo_box_header(id, value),
            ComboBoxUpdate::Placeholder(value) => self.apply_combo_box_placeholder(id, value),
            ComboBoxUpdate::Editable(value) => self.apply_combo_box_editable(id, *value),
        }
    }

    pub(super) fn apply_combo_box_items(
        &self,
        id: NodeId,
        items: &[SelectorItem],
    ) -> WindowsResult<()> {
        let Handle::ComboBox {
            value: selector,
            state,
            ..
        } = &self.node(id)?.handle
        else {
            panic!("items target is not a ComboBox");
        };
        let items_control: bindings::IItemsControl = selector.cast()?;
        let native = items_control.Items().and_then(|items| items.cast())?;
        set_selector_items(
            &native,
            &state.items,
            &state.suppressing_selection,
            items,
            |value| {
                let text = bindings::TextBlock::new()?;
                text.cast::<bindings::ITextBlock>()
                    .and_then(|text| text.SetText(value))?;
                Ok(text.into())
            },
            true,
        )?;
        state.native_selection.set(None);
        Ok(())
    }

    pub(super) fn apply_radio_buttons_items(
        &self,
        id: NodeId,
        items: &[SelectorItem],
    ) -> WindowsResult<()> {
        let Handle::RadioButtons { value, state, .. } = &self.node(id)?.handle else {
            panic!("items target is not RadioButtons");
        };
        set_selector_items(
            &value.Items()?,
            &state.items,
            &state.suppressing_selection,
            items,
            |value| Ok(windows_reference::IReference::from(value).into()),
            false,
        )?;
        state.native_selection.set(None);
        Ok(())
    }

    pub(super) fn apply_list_box_selection_mode(
        &self,
        id: NodeId,
        value: SelectionMode,
    ) -> WindowsResult<()> {
        let Handle::ListBox {
            value: list, state, ..
        } = &self.node(id)?.handle
        else {
            panic!("selection-mode target is not a ListBox");
        };
        let value = match value {
            SelectionMode::None => {
                panic!("ListBox does not support SelectionMode::None");
            }
            SelectionMode::Single => bindings::SelectionMode::Single,
            SelectionMode::Multiple => bindings::SelectionMode::Multiple,
            SelectionMode::Extended => bindings::SelectionMode::Extended,
        };
        list.SetSelectionMode(value)?;
        state.selection_mode.set(match value {
            bindings::SelectionMode::Single => SelectionMode::Single,
            bindings::SelectionMode::Multiple => SelectionMode::Multiple,
            bindings::SelectionMode::Extended => SelectionMode::Extended,
            _ => unreachable!(),
        });
        Ok(())
    }

    pub(super) fn apply_list_box_selection(
        &self,
        id: NodeId,
        selection: &CollectionSelection,
    ) -> WindowsResult<()> {
        let Handle::ListBox {
            value: list, state, ..
        } = &self.node(id)?.handle
        else {
            panic!("selection target is not a ListBox");
        };
        let items = state.items.borrow();
        let selected = items
            .iter()
            .filter(|item| selection.as_slice().binary_search(&item.key).is_ok())
            .collect::<Vec<_>>();
        state.suppressing_selection.set(true);
        let result: WindowsResult<()> = (|| {
            if state.selection_mode.get() == SelectionMode::Single {
                let selector: bindings::ISelector = list.cast()?;
                selector.SetSelectedItem(selected.first().map(|item| &item.value))?;
            } else {
                let native = list.SelectedItems()?;
                native.Clear()?;
                for item in &selected {
                    native.Append(&item.value)?;
                }
            }
            Ok(())
        })();
        state.suppressing_selection.set(false);
        result?;
        *state.expected_selection.borrow_mut() = selection.clone();
        *state.native_selection.borrow_mut() =
            CollectionSelection::new(selected.into_iter().map(|item| item.key));
        Ok(())
    }

    pub(super) fn apply_combo_box_selection(
        &self,
        id: NodeId,
        selection: Option<u64>,
    ) -> WindowsResult<()> {
        let Handle::ComboBox {
            value: selector,
            state,
            ..
        } = &self.node(id)?.handle
        else {
            panic!("selection target is not a ComboBox");
        };
        let items = state.items.borrow();
        let selected = selection.and_then(|key| items.iter().find(|item| item.key == key));
        state.suppressing_selection.set(true);
        let result = selector.SetSelectedItem(selected.map(|item| &item.value));
        state.suppressing_selection.set(false);
        result?;
        state.expected_selection.set(selection);
        state
            .native_selection
            .set(selected.map(|selected| selected.key));
        Ok(())
    }

    pub(super) fn apply_combo_box_header(
        &self,
        id: NodeId,
        value: &Option<String>,
    ) -> WindowsResult<()> {
        let Handle::ComboBox {
            value: selector, ..
        } = &self.node(id)?.handle
        else {
            panic!("header target is not a ComboBox");
        };
        let header = value.as_deref().map(controlled::inspectable_text);
        selector
            .cast::<bindings::IComboBox>()?
            .SetHeader(header.as_ref())
    }

    pub(super) fn apply_combo_box_placeholder(
        &self,
        id: NodeId,
        value: &Option<String>,
    ) -> WindowsResult<()> {
        let Handle::ComboBox {
            value: selector, ..
        } = &self.node(id)?.handle
        else {
            panic!("placeholder target is not a ComboBox");
        };
        selector
            .cast::<bindings::IComboBox>()?
            .SetPlaceholderText(value.as_deref().unwrap_or_default())
    }

    pub(super) fn apply_combo_box_editable(&self, id: NodeId, value: bool) -> WindowsResult<()> {
        let Handle::ComboBox {
            value: selector, ..
        } = &self.node(id)?.handle
        else {
            panic!("editable target is not a ComboBox");
        };
        selector.cast::<bindings::IComboBox>()?.SetIsEditable(value)
    }

    pub(super) fn apply_radio_buttons_update(
        &self,
        id: NodeId,
        update: &RadioButtonsUpdate,
    ) -> WindowsResult<()> {
        match update {
            RadioButtonsUpdate::Items(value) => self.apply_radio_buttons_items(id, value),
            RadioButtonsUpdate::Selection(value) => self.apply_radio_buttons_selection(id, *value),
            RadioButtonsUpdate::Header(value) => self.apply_radio_buttons_header(id, value),
            RadioButtonsUpdate::MaxColumns(value) => {
                self.apply_radio_buttons_max_columns(id, *value)
            }
        }
    }

    pub(super) fn apply_radio_buttons_selection(
        &self,
        id: NodeId,
        selection: Option<u64>,
    ) -> WindowsResult<()> {
        let Handle::RadioButtons { state, .. } = &self.node(id)?.handle else {
            panic!("selection target is not RadioButtons");
        };
        let items = state.items.borrow();
        let selected = selection.and_then(|key| items.iter().find(|item| item.key == key));
        state.expected_selection.set(selection);
        let selected_value = selected.map(|item| item.value.clone());
        let selected_key = selected.map(|item| item.key);
        state
            .pending_selection
            .borrow_mut()
            .replace(DeferredSelection {
                value: selected_value,
                key: selected_key,
            });
        let deferred = state.deferred_selection.as_ref().unwrap();
        let revision = deferred.revision.get().wrapping_add(1);
        deferred.revision.set(revision);
        let active = Rc::clone(&deferred.active);
        let selection_revision = Rc::clone(&deferred.revision);
        self.enqueue_deferred_ready(
            id,
            revision,
            DeferredAction::RadioButtonsSelection,
            active,
            selection_revision,
            "dispatcher rejected RadioButtons selection update",
        )
    }

    pub(super) fn apply_radio_buttons_max_columns(
        &self,
        id: NodeId,
        value: i32,
    ) -> WindowsResult<()> {
        let Handle::RadioButtons { value: radio, .. } = &self.node(id)?.handle else {
            panic!("max-columns target is not RadioButtons");
        };
        radio.SetMaxColumns(value)
    }

    pub(super) fn apply_radio_buttons_header(
        &self,
        id: NodeId,
        value: &Option<String>,
    ) -> WindowsResult<()> {
        let Handle::RadioButtons { value: control, .. } = &self.node(id)?.handle else {
            panic!("header target is not RadioButtons");
        };
        let header = value.as_deref().map(|value| {
            windows_reference::IReference::<windows_core::HSTRING>::from(
                windows_core::HSTRING::from(value),
            )
            .into()
        });
        control.SetHeader(header.as_ref())
    }

    pub(super) fn apply_collection_item_count(
        &mut self,
        id: NodeId,
        count: usize,
    ) -> WindowsResult<()> {
        let source = self.collection(id)?.source.borrow().clone();
        if let Some(source) = source {
            let current = source.Size()? as usize;
            let callback = Rc::clone(&self.collection(id)?.callback);
            let list = match &self.node(id)?.handle {
                Handle::Collection { value, .. } => value.clone(),
                _ => unreachable!(),
            };
            callback.suppressing_selection.set(true);
            let result: WindowsResult<()> = (|| {
                if self.collection(id)?.keyed {
                    for index in 0..current.min(count) {
                        source.SetAt(index as u32, &box_key(index as u64))?;
                    }
                }
                if count > current {
                    for index in current..count {
                        source.Append(&box_key(index as u64))?;
                    }
                } else {
                    for _ in count..current {
                        source.RemoveAtEnd()?;
                    }
                }
                Ok(())
            })();
            callback.suppressing_selection.set(false);
            result?;
            *callback.native_selection.borrow_mut() = selected_item_keys(&list)?;
            let state = self.collection_mut(id)?;
            *state.expected_order.borrow_mut() = (0..count).map(|index| index as u64).collect();
            state.keyed = false;
            state.selection_dirty.set(true);
        } else {
            let keys = (0..count).map(|index| index as u64);
            self.create_collection_source(id, keys, false)?;
        }
        Ok(())
    }

    pub(super) fn apply_collection_item_keys(
        &mut self,
        id: NodeId,
        keys: &[u64],
    ) -> WindowsResult<()> {
        let source = self.collection(id)?.source.borrow().clone();
        if let Some(source) = source {
            let current = source.Size()? as usize;
            let callback = Rc::clone(&self.collection(id)?.callback);
            let list = match &self.node(id)?.handle {
                Handle::Collection { value, .. } => value.clone(),
                _ => unreachable!(),
            };
            callback.suppressing_selection.set(true);
            let result: WindowsResult<()> = (|| {
                for (index, key) in keys.iter().copied().enumerate().take(current) {
                    let item = source.GetAt(index as u32)?;
                    if unbox_key(&item)? != key {
                        source.SetAt(index as u32, &box_key(key))?;
                    }
                }
                if keys.len() > current {
                    for key in &keys[current..] {
                        source.Append(&box_key(*key))?;
                    }
                } else {
                    for _ in keys.len()..current {
                        source.RemoveAtEnd()?;
                    }
                }
                Ok(())
            })();
            callback.suppressing_selection.set(false);
            result?;
            *callback.native_selection.borrow_mut() = selected_item_keys(&list)?;
            let state = self.collection_mut(id)?;
            *state.expected_order.borrow_mut() = keys.to_vec();
            state.keyed = true;
            state.selection_dirty.set(true);
        } else {
            self.create_collection_source(id, keys.iter().copied(), true)?;
        }
        Ok(())
    }

    fn create_collection_source(
        &mut self,
        id: NodeId,
        keys: impl IntoIterator<Item = u64>,
        keyed: bool,
    ) -> WindowsResult<()> {
        let template = self.collection_template()?;
        let keys = keys.into_iter().collect::<Vec<_>>();
        let values: Vec<Option<windows_core::IInspectable>> =
            keys.iter().map(|key| Some(box_key(*key))).collect();
        let source: windows_collections::IObservableVector<windows_core::IInspectable> =
            values.into();
        let Handle::Collection { value, .. } = &self.node(id)?.handle else {
            panic!("items target is not a virtual collection");
        };
        let items: bindings::IItemsControl = value.cast()?;
        let callback = Rc::clone(&self.collection(id)?.callback);
        callback.suppressing_selection.set(true);
        let result = items
            .SetItemTemplate(&template)
            .and_then(|_| items.SetItemsSource(&source));
        callback.suppressing_selection.set(false);
        result?;
        let state = self.collection_mut(id)?;
        *state.source.borrow_mut() = Some(source);
        *state.expected_order.borrow_mut() = keys;
        state.keyed = keyed;
        *state.native_selection.borrow_mut() = CollectionSelection::default();
        state.selection_dirty.set(true);
        Ok(())
    }

    pub(super) fn apply_collection_item_click_enabled(
        &self,
        id: NodeId,
        value: bool,
    ) -> WindowsResult<()> {
        let Handle::Collection { value: list, .. } = &self.node(id)?.handle else {
            panic!("item-click target is not a virtual collection");
        };
        list.SetIsItemClickEnabled(value)
    }

    pub(super) fn apply_collection_selection_display_only(
        &self,
        id: NodeId,
        value: bool,
    ) -> WindowsResult<()> {
        let Handle::Collection { value: list, .. } = &self.node(id)?.handle else {
            panic!("display-only target is not a virtual collection");
        };
        list.cast::<bindings::IControl>()?.SetIsEnabled(!value)
    }

    pub(super) fn apply_collection_can_reorder_items(
        &self,
        id: NodeId,
        value: bool,
    ) -> WindowsResult<()> {
        let Handle::Collection { value: list, .. } = &self.node(id)?.handle else {
            panic!("reorder target is not a virtual collection");
        };
        list.SetCanDragItems(value)?;
        list.SetCanReorderItems(value)?;
        list.cast::<bindings::IUIElement>()?.SetAllowDrop(value)
    }

    pub(super) fn apply_collection_selection_mode(
        &mut self,
        id: NodeId,
        value: SelectionMode,
    ) -> WindowsResult<()> {
        let Handle::Collection {
            value: list, state, ..
        } = &self.node(id)?.handle
        else {
            panic!("selection-mode target is not a virtual collection");
        };
        let callback = Rc::clone(&state.callback);
        let list = list.clone();
        callback.suppressing_selection.set(true);
        let result = list.SetSelectionMode(match value {
            SelectionMode::None => bindings::ListViewSelectionMode::None,
            SelectionMode::Single => bindings::ListViewSelectionMode::Single,
            SelectionMode::Multiple => bindings::ListViewSelectionMode::Multiple,
            SelectionMode::Extended => bindings::ListViewSelectionMode::Extended,
        });
        callback.suppressing_selection.set(false);
        result?;
        let state = self.collection_mut(id)?;
        state.selection_mode = value;
        *state.native_selection.borrow_mut() = selected_item_keys(&list)?;
        state.selection_dirty.set(true);
        Ok(())
    }

    pub(super) fn apply_collection_selection(
        &self,
        id: NodeId,
        selection: &CollectionSelection,
    ) -> WindowsResult<()> {
        let Handle::Collection {
            value: list, state, ..
        } = &self.node(id)?.handle
        else {
            panic!("selection target is not a virtual collection");
        };
        assert!(
            !(state.selection_mode == SelectionMode::None && !selection.is_empty()),
            "ListView selection must be empty in None mode"
        );
        assert!(
            !(state.selection_mode == SelectionMode::Single && selection.len() > 1),
            "ListView selection has more than one key in Single mode"
        );
        if !state.selection_dirty.get() && *state.expected_selection.borrow() == *selection {
            return Ok(());
        }
        let source = state.source.borrow();
        let source = source
            .as_ref()
            .unwrap_or_else(|| panic!("ListView selection has no item source"));
        let previous = state.expected_selection.replace(selection.clone());
        state.suppressing_selection.set(true);
        let mut actual = Vec::new();
        let result = (|| {
            if matches!(
                state.selection_mode,
                SelectionMode::None | SelectionMode::Single
            ) {
                let selector: bindings::ISelector = list.cast()?;
                selector.SetSelectedItem(None::<&windows_core::IInspectable>)?;
                if let Some(key) = selection.as_slice().first().copied() {
                    let count = source.Size()?;
                    for index in 0..count {
                        let item = source.GetAt(index)?;
                        if unbox_key(&item)? == key {
                            selector.SetSelectedItem(&item)?;
                            actual.push(key);
                            break;
                        }
                    }
                }
            } else {
                let selected = list.SelectedItems()?;
                selected.Clear()?;
                let count = source.Size()?;
                for index in 0..count {
                    let item = source.GetAt(index)?;
                    let key = unbox_key(&item)?;
                    if selection.as_slice().binary_search(&key).is_ok() {
                        selected.Append(&item)?;
                        actual.push(key);
                    }
                }
            }
            Ok(())
        })();
        state.suppressing_selection.set(false);
        if result.is_err() {
            state.expected_selection.replace(previous);
        } else {
            *state.native_selection.borrow_mut() = CollectionSelection::new(actual);
            state.selection_dirty.set(false);
        }
        result
    }

    fn collection_template(&mut self) -> WindowsResult<bindings::DataTemplate> {
        if let Some(template) = &self.template {
            return Ok(template.clone());
        }
        let template = bindings::XamlReader::Load(CONTENT_TEMPLATE_XAML)
            .and_then(|value| value.cast::<bindings::DataTemplate>())?;
        self.template = Some(template.clone());
        Ok(template)
    }
}

impl WinUiRuntime {
    pub(super) fn run_radio_buttons_deferred(
        &self,
        target: NodeId,
        revision: u64,
    ) -> WindowsResult<()> {
        let Handle::RadioButtons { value, state, .. } = &self.node(target)?.handle else {
            panic!("deferred RadioButtons target is invalid");
        };
        let deferred = state.deferred_selection.as_ref().unwrap();
        if !deferred.active.get() || deferred.revision.get() != revision {
            return Ok(());
        }
        let selection = state
            .pending_selection
            .borrow_mut()
            .take()
            .unwrap_or_else(|| panic!("deferred selection is unavailable"));
        state.suppressing_selection.set(true);
        let result = value.SetSelectedItem(selection.value.as_ref());
        state.suppressing_selection.set(false);
        result?;
        state.native_selection.set(selection.key);
        Ok(())
    }
}

fn box_key(key: u64) -> windows_core::IInspectable {
    windows_reference::IReference::<u64>::from(key).into()
}

fn selected_item_keys(list: &bindings::IListViewBase) -> WindowsResult<CollectionSelection> {
    let selected = list.SelectedItems()?;
    let count = selected.Size()?;
    let mut keys = Vec::with_capacity(count as usize);
    for index in 0..count {
        keys.push(unbox_key(&selected.GetAt(index)?)?);
    }
    Ok(CollectionSelection::new(keys))
}

fn unbox_key(value: &windows_core::IInspectable) -> WindowsResult<u64> {
    value
        .cast::<windows_reference::IReference<u64>>()
        .and_then(|value| value.Value())
}

fn apply_list_box_selection_delta(
    current: &CollectionSelection,
    args: &bindings::SelectionChangedEventArgs,
    items: &[NativeSelectorItem],
) -> WindowsResult<CollectionSelection> {
    let mut keys = current.as_slice().iter().copied().collect::<BTreeSet<_>>();
    let removed = args.RemovedItems()?;
    for index in 0..removed.Size()? {
        let value = removed.GetAt(index)?;
        keys.remove(&list_box_item_key(&value, items)?);
    }
    let added = args.AddedItems()?;
    for index in 0..added.Size()? {
        let value = added.GetAt(index)?;
        keys.insert(list_box_item_key(&value, items)?);
    }
    Ok(CollectionSelection::new(keys))
}

fn list_box_item_key(
    value: &windows_core::IInspectable,
    items: &[NativeSelectorItem],
) -> WindowsResult<u64> {
    items
        .iter()
        .find(|item| item.value == *value)
        .map(|item| item.key)
        .ok_or_else(|| panic!("ListBox selection references an unknown item"))
}

fn apply_selection_delta(
    current: &CollectionSelection,
    args: &bindings::SelectionChangedEventArgs,
) -> WindowsResult<CollectionSelection> {
    let mut keys = current.as_slice().iter().copied().collect::<BTreeSet<_>>();
    let removed = args.RemovedItems()?;
    for index in 0..removed.Size()? {
        keys.remove(&unbox_key(&removed.GetAt(index)?)?);
    }
    let added = args.AddedItems()?;
    for index in 0..added.Size()? {
        keys.insert(unbox_key(&added.GetAt(index)?)?);
    }
    Ok(CollectionSelection::new(keys))
}

fn apply_single_selection_delta(
    current: Option<u64>,
    args: &bindings::SelectionChangedEventArgs,
    items: &[NativeSelectorItem],
    name: &str,
) -> WindowsResult<Option<u64>> {
    let mut keys = current.into_iter().collect::<BTreeSet<_>>();
    let removed = args.RemovedItems()?;
    for index in 0..removed.Size()? {
        let value = removed.GetAt(index)?;
        keys.remove(&list_box_item_key(&value, items)?);
    }
    let added = args.AddedItems()?;
    for index in 0..added.Size()? {
        let value = added.GetAt(index)?;
        keys.insert(list_box_item_key(&value, items)?);
    }
    assert!(
        keys.len() <= 1,
        "{name} reported more than one selected item"
    );
    Ok(keys.into_iter().next())
}

#[cfg(test)]
mod callback_state_tests {
    use super::*;

    #[test]
    fn selection_state_suppresses_reentry_before_borrowing_items() {
        let state = Rc::new(SingleSelectorCallbackState {
            expected_selection: Cell::new(None),
            native_selection: Cell::new(None),
            suppressing_selection: Cell::new(false),
        });
        let items = RefCell::new(vec![10_u64, 20]);
        let callback_state = Rc::clone(&state);
        let published = Cell::new(0);
        let callback = |selection| {
            if callback_state.suppressing_selection.get() {
                return;
            }
            assert!(items.borrow().contains(&selection));
            callback_state.native_selection.set(Some(selection));
            callback_state.expected_selection.set(Some(selection));
            published.set(published.get() + 1);
        };

        let borrowed_items = items.borrow();
        state.expected_selection.set(Some(20));
        state.suppressing_selection.set(true);
        callback(20);
        state.suppressing_selection.set(false);
        drop(borrowed_items);
        assert_eq!(published.get(), 0);

        callback(10);
        assert_eq!(state.native_selection.get(), Some(10));
        assert_eq!(state.expected_selection.get(), Some(10));
        assert_eq!(published.get(), 1);
    }
}

#[cfg(test)]
#[path = "../../testing/private/winui/collection_access.rs"]
pub(super) mod tests;
