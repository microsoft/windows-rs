use super::*;

pub(super) struct AutoSuggestBoxState {
    _revokers: [windows_core::EventRevoker; 3],
    pub(super) value: bindings::IAutoSuggestBox,
    expected_text: Rc<RefCell<String>>,
    items: Rc<RefCell<Vec<(u64, windows_core::IInspectable)>>>,
}

impl WinUiRuntime {
    pub(super) fn create_auto_suggest_box(&self, id: NodeId) -> WindowsResult<Handle> {
        let value = bindings::AutoSuggestBox::new()?;
        let control: bindings::IAutoSuggestBox = value.cast()?;
        let expected_text = Rc::new(RefCell::new(String::new()));
        let items = Rc::new(RefCell::new(Vec::new()));
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);

        let text_control = control.clone();
        let text_expected = Rc::clone(&expected_text);
        let text_events = Rc::clone(&events);
        let text_waker = Rc::clone(&waker);
        let text_revoker = control.TextChanged(move |_sender, args| {
            if args.as_ref().unwrap().Reason().unwrap()
                != bindings::AutoSuggestionBoxTextChangeReason::UserInput
            {
                return;
            }
            let value = text_control.Text().unwrap();
            text_expected.borrow_mut().clone_from(&value);
            queue_latest_event(&text_events, NativeEvent::TextChanged { target: id, value });
            if let Some(wake) = text_waker.borrow().as_ref() {
                wake();
            }
        })?;

        let query_events = Rc::clone(&events);
        let query_waker = Rc::clone(&waker);
        let query_revoker = control.QuerySubmitted(move |_sender, args| {
            let value = args.as_ref().unwrap().QueryText().unwrap();
            query_events
                .borrow_mut()
                .push_back(NativeEvent::QuerySubmitted { target: id, value });
            if let Some(wake) = query_waker.borrow().as_ref() {
                wake();
            }
        })?;

        let chosen_control = control.clone();
        let chosen_expected = Rc::clone(&expected_text);
        let chosen_items = Rc::clone(&items);
        let chosen_events = Rc::clone(&events);
        let chosen_waker = Rc::clone(&waker);
        let chosen_revoker = control.SuggestionChosen(move |_sender, args| {
            let selected = args.as_ref().unwrap().SelectedItem().unwrap();
            let key = chosen_items
                .borrow()
                .iter()
                .find_map(|(key, item)| (*item == selected).then_some(*key))
                .unwrap();
            chosen_events
                .borrow_mut()
                .push_back(NativeEvent::ItemInvoked { target: id, key });
            let expected = chosen_expected.borrow().clone();
            chosen_control.SetText(&expected).unwrap();
            if let Some(wake) = chosen_waker.borrow().as_ref() {
                wake();
            }
        })?;

        Ok(Handle::AutoSuggestBox(Box::new(AutoSuggestBoxState {
            _revokers: [text_revoker, query_revoker, chosen_revoker],
            value: control,
            expected_text,
            items,
        })))
    }

    pub(super) fn apply_auto_suggest_box_update(
        &self,
        id: NodeId,
        update: &AutoSuggestUpdate,
    ) -> WindowsResult<()> {
        let Handle::AutoSuggestBox(state) = &self.node(id)?.handle else {
            panic!("AutoSuggestBox update target is not an AutoSuggestBox");
        };
        let value = &state.value;
        match update {
            AutoSuggestUpdate::Text(text) => {
                remove_queued_event(&self.events, id, LatestEventSlot::TextChanged);
                state.expected_text.borrow_mut().clone_from(text);
                if value.Text().ok().as_deref() == Some(text.as_str()) {
                    Ok(())
                } else {
                    value.SetText(text)
                }
            }
            AutoSuggestUpdate::Items(items) => {
                let native = items
                    .iter()
                    .map(|item| {
                        let value: windows_core::IInspectable =
                            windows_reference::IReference::from(item.text()).into();
                        (item.key(), value)
                    })
                    .collect::<Vec<_>>();
                let source: windows_collections::IObservableVector<windows_core::IInspectable> =
                    native
                        .iter()
                        .map(|(_, value)| Some(value.clone()))
                        .collect::<Vec<_>>()
                        .into();
                value
                    .cast::<bindings::IItemsControl>()?
                    .SetItemsSource(&source)?;
                *state.items.borrow_mut() = native;
                Ok(())
            }
            AutoSuggestUpdate::Header(header) => {
                let header: Option<windows_core::IInspectable> = header
                    .as_ref()
                    .map(|value| windows_reference::IReference::from(value.as_str()).into());
                value.SetHeader(header.as_ref())
            }
            AutoSuggestUpdate::Placeholder(placeholder) => value.SetPlaceholderText(placeholder),
        }
    }
}
