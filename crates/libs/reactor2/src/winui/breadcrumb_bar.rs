use super::*;

pub(super) struct BreadcrumbBarState {
    keys: Rc<RefCell<Rc<[u64]>>>,
}

impl WinUiRuntime {
    pub(super) fn create_breadcrumb_bar(&self, id: NodeId) -> WindowsResult<Handle> {
        let value = bindings::BreadcrumbBar::new()?;
        let bar: bindings::IBreadcrumbBar = value.cast()?;
        let keys = Rc::new(RefCell::new(Rc::<[u64]>::from([])));
        let callback_keys = Rc::clone(&keys);
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let revoker = bar.ItemClicked(move |_sender, args| {
            let index = usize::try_from(args.as_ref().unwrap().Index().unwrap()).unwrap();
            let key = callback_keys.borrow()[index];
            events
                .borrow_mut()
                .push_back(NativeEvent::ItemInvoked { target: id, key });
            if let Some(wake) = waker.borrow().as_ref() {
                wake();
            }
        })?;
        Ok(Handle::BreadcrumbBar {
            _revoker: revoker,
            value: bar,
            state: Box::new(BreadcrumbBarState { keys }),
        })
    }

    pub(super) fn apply_breadcrumb_bar_items(
        &self,
        id: NodeId,
        items: &[SelectorItem],
    ) -> WindowsResult<()> {
        let Handle::BreadcrumbBar { value, state, .. } = &self.node(id)?.handle else {
            panic!("items target is not a BreadcrumbBar");
        };
        let values: Vec<Option<windows_core::IInspectable>> = items
            .iter()
            .map(|item| Some(windows_reference::IReference::from(item.text()).into()))
            .collect();
        let source: windows_collections::IObservableVector<windows_core::IInspectable> =
            values.into();
        value.SetItemsSource(&source)?;
        *state.keys.borrow_mut() = items
            .iter()
            .map(SelectorItem::key)
            .collect::<Vec<_>>()
            .into();
        Ok(())
    }
}
