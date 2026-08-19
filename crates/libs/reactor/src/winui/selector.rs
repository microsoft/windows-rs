use super::*;

fn optional_index(index: i32) -> Option<usize> {
    if index == -1 {
        None
    } else {
        Some(usize::try_from(index).unwrap())
    }
}

pub(super) struct IndexSelectorState {
    pub(super) expected: Cell<i32>,
    pub(super) suppressing: Cell<bool>,
}

pub(super) struct TabViewState {
    pub(super) index: IndexSelectorState,
    pub(super) item_keys: RefCell<Vec<(usize, u64)>>,
    pub(super) suppressing_items: Cell<bool>,
}

pub(super) struct TabViewItemState {
    pub(super) value: bindings::TabViewItem,
    pub(super) key: Cell<Option<u64>>,
}

impl WinUiRuntime {
    pub(super) fn create_pivot(&self, id: NodeId) -> WindowsResult<Handle> {
        let value = bindings::Pivot::new()?;
        let pivot: bindings::IPivot = value.cast()?;
        let state = Rc::new(IndexSelectorState {
            expected: Cell::new(0),
            suppressing: Cell::new(false),
        });
        let callback_pivot = pivot.clone();
        let callback_state = Rc::clone(&state);
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let revoker = pivot.SelectionChanged(move |_sender, _args| {
            if callback_state.suppressing.get() {
                return;
            }
            let index = callback_pivot.SelectedIndex().unwrap();
            if callback_state.expected.replace(index) == index {
                return;
            }
            queue_latest_event(
                &events,
                NativeEvent::IndexChanged {
                    target: id,
                    index: optional_index(index),
                },
            );
            if let Some(wake) = waker.borrow().as_ref() {
                wake();
            }
        })?;
        Ok(Handle::Pivot {
            _revoker: revoker,
            value: pivot,
            state,
        })
    }

    pub(super) fn create_flip_view(&self, id: NodeId) -> WindowsResult<Handle> {
        let value = bindings::FlipView::new()?;
        let selector: bindings::ISelector = value.cast()?;
        let state = Rc::new(IndexSelectorState {
            expected: Cell::new(0),
            suppressing: Cell::new(false),
        });
        let callback_selector = selector.clone();
        let callback_state = Rc::clone(&state);
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let revoker = selector.SelectionChanged(move |_sender, _args| {
            if callback_state.suppressing.get() {
                return;
            }
            let index = callback_selector.SelectedIndex().unwrap();
            if callback_state.expected.replace(index) == index {
                return;
            }
            queue_latest_event(
                &events,
                NativeEvent::IndexChanged {
                    target: id,
                    index: optional_index(index),
                },
            );
            if let Some(wake) = waker.borrow().as_ref() {
                wake();
            }
        })?;
        Ok(Handle::FlipView {
            _revoker: revoker,
            value: selector,
            state,
        })
    }

    pub(super) fn create_tab_view(&self, id: NodeId) -> WindowsResult<Handle> {
        let value = bindings::TabView::new()?;
        let tab_view: bindings::ITabView = value.cast()?;
        let state = Rc::new(TabViewState {
            index: IndexSelectorState {
                expected: Cell::new(0),
                suppressing: Cell::new(false),
            },
            item_keys: RefCell::new(Vec::new()),
            suppressing_items: Cell::new(false),
        });

        let callback_tab_view = tab_view.clone();
        let callback_state = Rc::clone(&state);
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let selection_revoker = tab_view.SelectionChanged(move |_sender, _args| {
            if callback_state.index.suppressing.get() {
                return;
            }
            let index = callback_tab_view.SelectedIndex().unwrap();
            if callback_state.index.expected.replace(index) == index {
                return;
            }
            queue_latest_event(
                &events,
                NativeEvent::IndexChanged {
                    target: id,
                    index: optional_index(index),
                },
            );
            if let Some(wake) = waker.borrow().as_ref() {
                wake();
            }
        })?;

        let callback_state = Rc::clone(&state);
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let close_revoker = tab_view.TabCloseRequested(move |_sender, args| {
            let tab = args
                .as_ref()
                .unwrap()
                .Tab()
                .unwrap()
                .cast::<windows_core::IInspectable>()
                .unwrap();
            let key = Self::item_key(&callback_state.item_keys, &tab).unwrap();
            Self::queue_event(
                &events,
                &waker,
                NativeEvent::TabCloseRequested { target: id, key },
            );
        })?;

        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let add_revoker = tab_view.AddTabButtonClick(move |_sender, _args| {
            Self::queue_event(
                &events,
                &waker,
                NativeEvent::AddTabButtonClick { target: id },
            );
        })?;

        let callback_tab_view = tab_view.clone();
        let callback_state = Rc::clone(&state);
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let items_revoker = tab_view.TabItemsChanged(move |_sender, _args| {
            if callback_state.suppressing_items.get() {
                return;
            }
            let Some(keys) = Self::tab_keys(&callback_tab_view, &callback_state.item_keys) else {
                return;
            };
            queue_latest_event(&events, NativeEvent::TabsReordered { target: id, keys });
            if let Some(wake) = waker.borrow().as_ref() {
                wake();
            }
        })?;

        Ok(Handle::TabView {
            _revokers: Box::new([selection_revoker, close_revoker, add_revoker, items_revoker]),
            value: tab_view,
            state,
        })
    }

    pub(super) fn create_tab_view_item(&self) -> WindowsResult<Handle> {
        Ok(Handle::TabViewItem(Box::new(TabViewItemState {
            value: bindings::TabViewItem::new()?,
            key: Cell::new(None),
        })))
    }

    pub(super) fn apply_pivot_update(&self, id: NodeId, update: &PivotUpdate) -> WindowsResult<()> {
        let Handle::Pivot { value, .. } = &self.node(id)?.handle else {
            panic!("Pivot update target is not a Pivot");
        };
        match update {
            PivotUpdate::Title(title) => {
                let title = title.as_deref().map(controlled::inspectable_text);
                value.SetTitle(title.as_ref())
            }
        }
    }

    pub(super) fn apply_index_selector(&self, id: NodeId, index: i32) -> WindowsResult<()> {
        match &self.node(id)?.handle {
            Handle::FlipView { value, state, .. } => {
                self.apply_index_selection(id, index, state, || value.SetSelectedIndex(index))
            }
            Handle::Pivot { value, state, .. } => {
                self.apply_index_selection(id, index, state, || value.SetSelectedIndex(index))
            }
            Handle::TabView { value, state, .. } => {
                self.apply_index_selection(id, index, &state.index, || {
                    value.SetSelectedIndex(index)
                })
            }
            _ => panic!("selector update target is not an index selector"),
        }
    }

    pub(super) fn apply_tab_view_update(
        &self,
        id: NodeId,
        update: &TabViewUpdate,
    ) -> WindowsResult<()> {
        let Handle::TabView { value, .. } = &self.node(id)?.handle else {
            panic!("TabView update target is not a TabView");
        };
        match update {
            TabViewUpdate::CanReorderTabs(enabled) => value.SetCanReorderTabs(*enabled),
            TabViewUpdate::IsAddTabButtonVisible(visible) => {
                value.SetIsAddTabButtonVisible(*visible)
            }
        }
    }

    pub(super) fn apply_tab_view_item_update(
        &self,
        id: NodeId,
        update: &TabViewItemUpdate,
    ) -> WindowsResult<()> {
        let Handle::TabViewItem(state) = &self.node(id)?.handle else {
            panic!("TabViewItem update target is not a TabViewItem");
        };
        match update {
            TabViewItemUpdate::Key(value) => {
                state.key.set(Some(*value));
                Ok(())
            }
            TabViewItemUpdate::Header(value) => {
                let item: bindings::ITabViewItem = state.value.cast()?;
                let header = controlled::inspectable_text(value);
                item.SetHeader(&header)
            }
            TabViewItemUpdate::Closable(value) => {
                let item: bindings::ITabViewItem = state.value.cast()?;
                item.SetIsClosable(*value)
            }
        }
    }

    fn apply_index_selection(
        &self,
        id: NodeId,
        index: i32,
        state: &IndexSelectorState,
        apply: impl FnOnce() -> WindowsResult<()>,
    ) -> WindowsResult<()> {
        remove_queued_event(&self.events, id, LatestEventSlot::IndexChanged);
        state.expected.set(index);
        state.suppressing.set(true);
        let result = apply();
        state.suppressing.set(false);
        result
    }

    pub(super) fn identity(value: &windows_core::IInspectable) -> WindowsResult<usize> {
        let identity: windows_core::IUnknown = value.cast()?;
        Ok(Interface::as_raw(&identity) as usize)
    }

    fn item_key(
        item_keys: &RefCell<Vec<(usize, u64)>>,
        item: &windows_core::IInspectable,
    ) -> Option<u64> {
        let identity = Self::identity(item).ok()?;
        item_keys
            .borrow()
            .iter()
            .find_map(|(candidate, key)| (*candidate == identity).then_some(*key))
    }

    fn tab_keys(
        tab_view: &bindings::ITabView,
        item_keys: &RefCell<Vec<(usize, u64)>>,
    ) -> Option<Vec<u64>> {
        let tabs = tab_view.TabItems().ok()?;
        let count = usize::try_from(tabs.Size().ok()?).ok()?;
        if count != item_keys.borrow().len() {
            return None;
        }
        Some(
            (0..count)
                .map(|index| {
                    let item = tabs.GetAt(index as u32).unwrap();
                    Self::item_key(item_keys, &item).unwrap()
                })
                .collect(),
        )
    }

    fn queue_event(
        events: &RefCell<VecDeque<NativeEvent>>,
        waker: &RefCell<Option<Rc<dyn Fn()>>>,
        event: NativeEvent,
    ) {
        events.borrow_mut().push_back(event);
        if let Some(wake) = waker.borrow().as_ref() {
            wake();
        }
    }
}

#[cfg(test)]
mod callback_state_tests {
    use super::*;

    struct TrackedState {
        index: IndexSelectorState,
        drops: Rc<Cell<usize>>,
    }

    impl Drop for TrackedState {
        fn drop(&mut self) {
            self.drops.set(self.drops.get() + 1);
        }
    }

    fn callback(state: Rc<TrackedState>) -> Box<dyn Fn(i32)> {
        Box::new(move |index| state.index.expected.set(index))
    }

    #[test]
    fn callback_replacement_removal_drops_state_without_stale_sharing() {
        let drops = Rc::new(Cell::new(0));
        let first = Rc::new(TrackedState {
            index: IndexSelectorState {
                expected: Cell::new(1),
                suppressing: Cell::new(false),
            },
            drops: Rc::clone(&drops),
        });
        let first_weak = Rc::downgrade(&first);
        let mut slot = Some(callback(Rc::clone(&first)));
        slot.as_ref().unwrap()(3);
        assert_eq!(first.index.expected.get(), 3);
        drop(first);

        let second = Rc::new(TrackedState {
            index: IndexSelectorState {
                expected: Cell::new(2),
                suppressing: Cell::new(false),
            },
            drops: Rc::clone(&drops),
        });
        let second_weak = Rc::downgrade(&second);
        slot = Some(callback(Rc::clone(&second)));
        assert!(first_weak.upgrade().is_none());
        assert_eq!(drops.get(), 1);

        slot.as_ref().unwrap()(7);
        assert_eq!(second.index.expected.get(), 7);
        slot = None;
        drop(second);
        assert!(second_weak.upgrade().is_none());
        assert_eq!(drops.get(), 2);
        assert!(slot.is_none());
    }
}
