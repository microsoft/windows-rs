use super::*;

pub(super) struct SelectorBarCallbackState {
    expected: Cell<Option<u64>>,
    pub(super) suppressing: Cell<bool>,
}

pub(super) struct SelectorBarState {
    pub(super) callback: Rc<SelectorBarCallbackState>,
    pub(super) items: Rc<RefCell<Vec<(u64, bindings::SelectorBarItem)>>>,
}

pub(super) struct SelectorBarItemState {
    pub(super) value: bindings::SelectorBarItem,
    pub(super) key: Cell<Option<u64>>,
}

impl WinUiRuntime {
    pub(super) fn create_selector_bar(&self, id: NodeId) -> WindowsResult<Handle> {
        let value = bindings::SelectorBar::new()?;
        let selector: bindings::ISelectorBar = value.cast()?;
        let callback = Rc::new(SelectorBarCallbackState {
            expected: Cell::new(None),
            suppressing: Cell::new(false),
        });
        let items = Rc::new(RefCell::new(Vec::new()));
        let callback_selector = selector.clone();
        let callback_state = Rc::clone(&callback);
        let callback_items = Rc::clone(&items);
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let revoker = selector.SelectionChanged(move |_sender, _args| {
            if callback_state.suppressing.get() {
                return;
            }
            let selected = callback_selector.SelectedItem().ok().and_then(|selected| {
                callback_items
                    .borrow()
                    .iter()
                    .find_map(|(key, item)| (*item == selected).then_some(*key))
            });
            if callback_state.expected.replace(selected) == selected {
                return;
            }
            queue_latest_event(
                &events,
                NativeEvent::SelectedKeyChanged {
                    target: id,
                    key: selected,
                },
            );
            if let Some(wake) = waker.borrow().as_ref() {
                wake();
            }
        })?;
        Ok(Handle::SelectorBar {
            _revoker: revoker,
            value: selector,
            state: Box::new(SelectorBarState { callback, items }),
        })
    }

    pub(super) fn create_selector_bar_item(&self) -> WindowsResult<Handle> {
        Ok(Handle::SelectorBarItem(Box::new(SelectorBarItemState {
            value: bindings::SelectorBarItem::new()?,
            key: Cell::new(None),
        })))
    }

    pub(super) fn apply_selector_bar_selection(
        &self,
        id: NodeId,
        key: Option<u64>,
    ) -> WindowsResult<()> {
        let Handle::SelectorBar { value, state, .. } = &self.node(id)?.handle else {
            panic!("SelectorBar update target is not a SelectorBar");
        };
        remove_queued_event(&self.events, id, LatestEventSlot::SelectedKeyChanged);
        let selected = key.map(|key| {
            state
                .items
                .borrow()
                .iter()
                .find_map(|(candidate, item)| (*candidate == key).then(|| item.clone()))
                .unwrap()
        });
        state.callback.expected.set(key);
        state.callback.suppressing.set(true);
        let result = value.SetSelectedItem(selected.as_ref());
        state.callback.suppressing.set(false);
        result
    }

    pub(super) fn apply_selector_bar_item_update(
        &self,
        id: NodeId,
        update: &SelectorBarItemUpdate,
    ) -> WindowsResult<()> {
        let Handle::SelectorBarItem(state) = &self.node(id)?.handle else {
            panic!("SelectorBarItem update target is not a SelectorBarItem");
        };
        match update {
            SelectorBarItemUpdate::Key(value) => {
                state.key.set(Some(*value));
                Ok(())
            }
            SelectorBarItemUpdate::Text(value) => state.value.SetText(value),
            SelectorBarItemUpdate::Icon(value) => {
                let icon = media::create_icon(value.as_deref())?;
                state.value.SetIcon(icon.as_ref())
            }
        }
    }
}
