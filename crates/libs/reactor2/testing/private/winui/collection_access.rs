use super::*;

pub(in crate::winui) fn collections(runtime: &WinUiRuntime, grid: bool) -> Vec<NodeId> {
    runtime
        .nodes
        .iter()
        .filter_map(|(id, node)| {
            let Handle::Collection { value, .. } = &node.handle else {
                return None;
            };
            (value.cast::<bindings::GridView>().is_ok() == grid).then_some(*id)
        })
        .collect()
}

pub(in crate::winui) fn item_count(runtime: &WinUiRuntime, id: NodeId) -> WindowsResult<usize> {
    let Handle::Collection { state, .. } = &runtime.node(id)?.handle else {
        panic!("native node is not a virtual collection");
    };
    let source = state.source.borrow();
    Ok(source
        .as_ref()
        .map(|source| source.Size().map(|value| value as usize))
        .transpose()?
        .unwrap_or(0))
}

pub(in crate::winui) fn item_keys(runtime: &WinUiRuntime, id: NodeId) -> WindowsResult<Vec<u64>> {
    let Handle::Collection { state, .. } = &runtime.node(id)?.handle else {
        panic!("native node is not a virtual collection");
    };
    let source = state.source.borrow();
    let Some(source) = source.as_ref() else {
        return Ok(Vec::new());
    };
    (0..source.Size()?)
        .map(|index| {
            let item = source.GetAt(index)?;
            unbox_key(&item)
        })
        .collect()
}

pub(in crate::winui) fn selection_mode(
    runtime: &WinUiRuntime,
    id: NodeId,
) -> WindowsResult<SelectionMode> {
    let Handle::Collection { value, .. } = &runtime.node(id)?.handle else {
        panic!("native node is not a virtual collection");
    };
    match value.SelectionMode()? {
        bindings::ListViewSelectionMode::None => Ok(SelectionMode::None),
        bindings::ListViewSelectionMode::Single => Ok(SelectionMode::Single),
        bindings::ListViewSelectionMode::Multiple => Ok(SelectionMode::Multiple),
        bindings::ListViewSelectionMode::Extended => Ok(SelectionMode::Extended),
        _ => panic!("ListView returned an unknown selection mode"),
    }
}

pub(in crate::winui) fn selection(
    runtime: &WinUiRuntime,
    id: NodeId,
) -> WindowsResult<CollectionSelection> {
    let Handle::Collection { value, .. } = &runtime.node(id)?.handle else {
        panic!("native node is not a virtual collection");
    };
    selected_item_keys(value)
}

pub(in crate::winui) fn item_click_enabled(
    runtime: &WinUiRuntime,
    id: NodeId,
) -> WindowsResult<bool> {
    let Handle::Collection { value, .. } = &runtime.node(id)?.handle else {
        panic!("native node is not a virtual collection");
    };
    value.IsItemClickEnabled()
}

pub(in crate::winui) fn can_reorder_items(
    runtime: &WinUiRuntime,
    id: NodeId,
) -> WindowsResult<bool> {
    let Handle::Collection { value, .. } = &runtime.node(id)?.handle else {
        panic!("native node is not a virtual collection");
    };
    Ok(value.CanDragItems()?
        && value.CanReorderItems()?
        && value.cast::<bindings::IUIElement>()?.AllowDrop()?)
}

pub(in crate::winui) fn set_selection(
    runtime: &WinUiRuntime,
    id: NodeId,
    selection: &CollectionSelection,
) -> WindowsResult<()> {
    let Handle::Collection { value, state, .. } = &runtime.node(id)?.handle else {
        panic!("native node is not a virtual collection");
    };
    let source = state.source.borrow();
    let source = source
        .as_ref()
        .unwrap_or_else(|| panic!("native ListView has no item source"));
    if matches!(
        state.selection_mode,
        SelectionMode::None | SelectionMode::Single
    ) {
        let selector: bindings::ISelector = value.cast()?;
        selector.SetSelectedItem(None::<&windows_core::IInspectable>)?;
        if let Some(key) = selection.as_slice().first().copied() {
            for index in 0..source.Size()? {
                let item = source.GetAt(index)?;
                if unbox_key(&item)? == key {
                    selector.SetSelectedItem(&item)?;
                    break;
                }
            }
        }
    } else {
        let selected = value.SelectedItems()?;
        selected.Clear()?;
        for index in 0..source.Size()? {
            let item = source.GetAt(index)?;
            if selection
                .as_slice()
                .binary_search(&unbox_key(&item)?)
                .is_ok()
            {
                selected.Append(&item)?;
            }
        }
    }
    Ok(())
}

#[derive(Clone)]
pub(in crate::winui) struct SingleSelectorProbe {
    kind: SingleSelectorProbeKind,
}

#[derive(Clone)]
enum SingleSelectorProbeKind {
    ComboBox {
        value: bindings::ISelector,
        items: Rc<RefCell<Vec<NativeSelectorItem>>>,
    },
    RadioButtons {
        value: bindings::IRadioButtons,
        items: Rc<RefCell<Vec<NativeSelectorItem>>>,
    },
}

impl SingleSelectorProbe {
    pub(in crate::winui) fn new(runtime: &WinUiRuntime, kind: NativeKind) -> Self {
        let handle = runtime
            .nodes
            .values()
            .find_map(|node| {
                let matches = matches!(
                    (&node.handle, kind),
                    (Handle::ComboBox { .. }, NativeKind::ComboBox)
                        | (Handle::RadioButtons { .. }, NativeKind::RadioButtons)
                );
                matches.then_some(&node.handle)
            })
            .unwrap_or_else(|| panic!("single-selector native node is missing"));
        let kind = match handle {
            Handle::ComboBox { value, state, .. } => SingleSelectorProbeKind::ComboBox {
                value: value.clone(),
                items: Rc::clone(&state.items),
            },
            Handle::RadioButtons { value, state, .. } => SingleSelectorProbeKind::RadioButtons {
                value: value.clone(),
                items: Rc::clone(&state.items),
            },
            _ => unreachable!(),
        };
        Self { kind }
    }

    pub(in crate::winui) fn selection(&self) -> WindowsResult<Option<u64>> {
        match &self.kind {
            SingleSelectorProbeKind::ComboBox { value, items } => {
                let selected = value.SelectedItem().ok();
                Ok(selected.and_then(|selected| {
                    items
                        .borrow()
                        .iter()
                        .find_map(|item| (item.value == selected).then_some(item.key))
                }))
            }
            SingleSelectorProbeKind::RadioButtons { value, items } => {
                Ok(usize::try_from(value.SelectedIndex()?)
                    .ok()
                    .and_then(|index| items.borrow().get(index).map(|item| item.key)))
            }
        }
    }

    pub(in crate::winui) fn set_selection(&self, key: Option<u64>) -> WindowsResult<()> {
        match &self.kind {
            SingleSelectorProbeKind::ComboBox { value, items } => {
                let items = items.borrow();
                let selected = key.and_then(|key| items.iter().find(|item| item.key == key));
                value.SetSelectedItem(selected.map(|item| &item.value))
            }
            SingleSelectorProbeKind::RadioButtons { value, items } => {
                let items = items.borrow();
                let selected = key.and_then(|key| items.iter().find(|item| item.key == key));
                value.SetSelectedItem(selected.map(|item| &item.value))
            }
        }
    }
}
