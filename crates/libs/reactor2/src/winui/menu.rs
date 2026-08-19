use super::*;
use crate::element::props::{MenuBarItemSpec, MenuItemSpec};

pub(super) struct MenuState {
    revokers: Vec<windows_core::EventRevoker>,
}

impl WinUiRuntime {
    pub(super) fn create_menu_bar(&self, _id: NodeId) -> WindowsResult<Handle> {
        Ok(Handle::MenuBar {
            value: bindings::MenuBar::new()?,
            state: Box::new(MenuState {
                revokers: Vec::new(),
            }),
        })
    }

    pub(super) fn create_menu_flyout(&self, id: NodeId) -> WindowsResult<Handle> {
        let value = bindings::MenuFlyout::new()?;
        let flyout: bindings::IFlyoutBase = value.cast()?;
        let revokers = overlay::subscribe_flyout(
            &flyout,
            id,
            Rc::clone(&self.events),
            Rc::clone(&self.waker),
        )?;
        Ok(Handle::MenuFlyout {
            _revokers: revokers,
            value,
            state: Box::new(MenuState {
                revokers: Vec::new(),
            }),
        })
    }

    pub(super) fn apply_menu_bar_update(
        &mut self,
        id: NodeId,
        items: &[MenuBarItemSpec],
    ) -> WindowsResult<()> {
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let Handle::MenuBar { value, state } = &mut self.node_mut(id)?.handle else {
            panic!("MenuBar update target is not a MenuBar");
        };
        state.revokers.clear();
        let native_items = value.Items()?;
        native_items.Clear()?;
        for item in items {
            let native = bindings::MenuBarItem::new()?;
            native.SetTitle(&item.title)?;
            let children = native.Items()?;
            append_items(
                &children,
                &item.items,
                id,
                &events,
                &waker,
                &mut state.revokers,
            )?;
            native_items.Append(&native)?;
        }
        Ok(())
    }

    pub(super) fn apply_menu_flyout_update(
        &mut self,
        id: NodeId,
        items: &[MenuItemSpec],
    ) -> WindowsResult<()> {
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let Handle::MenuFlyout { value, state, .. } = &mut self.node_mut(id)?.handle else {
            panic!("MenuFlyout update target is not a MenuFlyout");
        };
        state.revokers.clear();
        let native_items = value.Items()?;
        native_items.Clear()?;
        append_items(
            &native_items,
            items,
            id,
            &events,
            &waker,
            &mut state.revokers,
        )
    }
}

fn append_items(
    target: &windows_collections::IVector<bindings::MenuFlyoutItemBase>,
    items: &[MenuItemSpec],
    target_id: NodeId,
    events: &Rc<RefCell<VecDeque<NativeEvent>>>,
    waker: &Rc<RefCell<Option<Rc<dyn Fn()>>>>,
    revokers: &mut Vec<windows_core::EventRevoker>,
) -> WindowsResult<()> {
    for item in items {
        match item {
            MenuItemSpec::Item { key, text, enabled } => {
                let native = bindings::MenuFlyoutItem::new()?;
                native.SetText(text)?;
                native
                    .cast::<bindings::IControl>()?
                    .SetIsEnabled(*enabled)?;
                let event_key = *key;
                let event_queue = Rc::clone(events);
                let event_waker = Rc::clone(waker);
                revokers.push(native.Click(move |_sender, _args| {
                    event_queue
                        .borrow_mut()
                        .push_back(NativeEvent::MenuItemClick {
                            target: target_id,
                            key: event_key,
                        });
                    if let Some(wake) = event_waker.borrow().as_ref() {
                        wake();
                    }
                })?);
                let base: bindings::MenuFlyoutItemBase = native.cast()?;
                target.Append(&base)?;
            }
            MenuItemSpec::Separator { .. } => {
                let base: bindings::MenuFlyoutItemBase =
                    bindings::MenuFlyoutSeparator::new()?.cast()?;
                target.Append(&base)?;
            }
            MenuItemSpec::Submenu { text, items, .. } => {
                let native = bindings::MenuFlyoutSubItem::new()?;
                native.SetText(text)?;
                append_items(&native.Items()?, items, target_id, events, waker, revokers)?;
                let base: bindings::MenuFlyoutItemBase = native.cast()?;
                target.Append(&base)?;
            }
        }
    }
    Ok(())
}
