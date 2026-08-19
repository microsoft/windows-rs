use super::*;

impl WinUiRuntime {
    pub(super) fn attach(
        &mut self,
        parent: NodeId,
        child: NodeId,
        attachment: Attachment,
    ) -> WindowsResult<()> {
        assert!(
            self.node(child)?.parent.is_none(),
            "native node already has a parent"
        );
        let position = match attachment {
            Attachment::Child { index } => {
                let child_ui = self.node(child)?.handle.ui_element()?;
                let children = self
                    .node(parent)?
                    .handle
                    .panel()
                    .and_then(|panel| panel.Children())?;
                children.InsertAt(index as u32, &child_ui)?;
                index
            }
            Attachment::Command { section, index } => {
                let element = self
                    .node(child)?
                    .handle
                    .command_bar_element()
                    .cloned()
                    .unwrap_or_else(|| panic!("command attachment child is invalid"));
                let (Handle::CommandBar { state, .. } | Handle::CommandBarFlyout { state, .. }) =
                    &self.node(parent)?.handle
                else {
                    panic!("command attachment parent is invalid");
                };
                let commands = match section {
                    CommandSection::Primary => &state.primary,
                    CommandSection::Secondary => &state.secondary,
                };
                commands.InsertAt(index as u32, &element)?;
                match section {
                    CommandSection::Primary => index,
                    CommandSection::Secondary => {
                        self.node(parent)?
                            .children
                            .iter()
                            .filter(|sibling| {
                                matches!(
                                    self.nodes.get(sibling).and_then(|node| node.attachment),
                                    Some(Attachment::Command {
                                        section: CommandSection::Primary,
                                        ..
                                    })
                                )
                            })
                            .count()
                            + index
                    }
                }
            }
            Attachment::Content => {
                let child_ui = self.node(child)?.handle.ui_element()?;
                let handle = &self.node(parent)?.handle;
                if let Some(content) = handle.content_control() {
                    content.and_then(|content| content.SetContent(&child_ui))?;
                } else {
                    match handle {
                        Handle::Border(value) => value.SetChild(&child_ui)?,
                        Handle::Flyout { value, .. } => value.SetContent(&child_ui)?,
                        Handle::Viewbox(value) => value.SetChild(&child_ui)?,
                        Handle::ScrollView { value, .. } => value.SetContent(&child_ui)?,
                        Handle::SplitView { value, .. } => value.SetContent(&child_ui)?,
                        Handle::TitleBar { value, .. } => value.SetContent(&child_ui)?,
                        _ => panic!("content attachment target is invalid"),
                    }
                }
                self.node(parent)?.children.len()
            }
            Attachment::Pane => {
                let child_ui = self.node(child)?.handle.ui_element()?;
                match &self.node(parent)?.handle {
                    Handle::SplitView { value, .. } => value.SetPane(&child_ui)?,
                    Handle::TitleBar { value, .. } => value.SetRightHeader(&child_ui)?,
                    _ => panic!("pane attachment target is invalid"),
                }
                self.node(parent)?.children.len()
            }
            Attachment::PaneFooter => {
                let child_ui = self.node(child)?.handle.ui_element()?;
                let Handle::NavigationView(state) = &self.node(parent)?.handle else {
                    panic!("pane-footer attachment target is invalid");
                };
                state.value.SetPaneFooter(&child_ui)?;
                self.node(parent)?.children.len()
            }
            Attachment::Header => {
                let child_ui = self.node(child)?.handle.ui_element()?;
                match &self.node(parent)?.handle {
                    Handle::ContentDialog { value, .. } => value.SetTitle(&child_ui)?,
                    Handle::Expander(state) => state.value.SetHeader(&child_ui)?,
                    Handle::Collection { value, .. } => value.SetHeader(&child_ui)?,
                    _ => panic!("header attachment target is invalid"),
                }
                0
            }
            Attachment::Item { index } => {
                if let (Handle::NavigationView(state), Handle::NavigationViewItem(item)) =
                    (&self.node(parent)?.handle, &self.node(child)?.handle)
                {
                    let key = item.key.get().unwrap();
                    let items = state.value.MenuItems()?;
                    state
                        .items
                        .borrow_mut()
                        .insert(index, (key, item.value.clone()));
                    state.callback.suppressing.set(true);
                    let result = item
                        .value
                        .cast::<windows_core::IInspectable>()
                        .and_then(|item| items.InsertAt(index as u32, &item));
                    state.callback.suppressing.set(false);
                    if result.is_err() {
                        state.items.borrow_mut().remove(index);
                    }
                    result?;
                    index
                } else if let (
                    Handle::SelectorBar { value, state, .. },
                    Handle::SelectorBarItem(item),
                ) = (&self.node(parent)?.handle, &self.node(child)?.handle)
                {
                    let key = item.key.get().unwrap();
                    let items = value.Items()?;
                    state.items.borrow_mut().push((key, item.value.clone()));
                    state.callback.suppressing.set(true);
                    let result = items.InsertAt(index as u32, &item.value);
                    state.callback.suppressing.set(false);
                    if result.is_err() {
                        state
                            .items
                            .borrow_mut()
                            .retain(|(candidate, _)| *candidate != key);
                    }
                    result?;
                    index
                } else {
                    let child_ui = self.node(child)?.handle.ui_element()?;
                    let Some(items) = self.node(parent)?.handle.item_collection() else {
                        panic!("item attachment parent is invalid");
                    };
                    let items = items?;
                    let registration = match (&self.node(parent)?.handle, &self.node(child)?.handle)
                    {
                        (Handle::TabView { state, .. }, Handle::TabViewItem(item)) => {
                            let key = item.key.get().unwrap();
                            let inspectable: windows_core::IInspectable = item.value.cast()?;
                            let identity = Self::identity(&inspectable)?;
                            state.item_keys.borrow_mut().push((identity, key));
                            state.suppressing_items.set(true);
                            Some((state, identity))
                        }
                        (Handle::TabView { .. }, _) => panic!("TabView item child is invalid"),
                        _ => None,
                    };
                    let result = items.InsertAt(index as u32, &child_ui);
                    if let Some((state, identity)) = registration {
                        state.suppressing_items.set(false);
                        if result.is_err() {
                            state
                                .item_keys
                                .borrow_mut()
                                .retain(|(candidate, _)| *candidate != identity);
                        }
                    }
                    result?;
                    index
                }
            }
            Attachment::VirtualItem { index, lease } => {
                let child_ui = self.node(child)?.handle.ui_element()?;
                let list = self.collection(parent)?;
                if let Some(slot) = list.slots.borrow().get(&(index, lease)).cloned() {
                    slot.content.SetContent(&child_ui)?;
                }
                let siblings = &self.node(parent)?.children;
                siblings
                    .iter()
                    .position(|sibling| {
                        matches!(
                            self.nodes.get(sibling).and_then(|node| node.attachment),
                            Some(Attachment::VirtualItem {
                                index: sibling_index,
                                ..
                            }) if sibling_index > index
                        )
                    })
                    .unwrap_or(siblings.len())
            }
        };
        self.node_mut(parent)?.children.insert(position, child);
        let child = self.node_mut(child)?;
        child.parent = Some(parent);
        child.attachment = Some(attachment);
        Ok(())
    }

    pub(super) fn detach(&mut self, parent: NodeId, child: NodeId) -> WindowsResult<()> {
        let child_node = self.node(child)?;
        assert!(
            child_node.parent == Some(parent),
            "detach does not match native parent"
        );
        let attachment = child_node.attachment.unwrap();
        match attachment {
            Attachment::Child { .. } => {
                let position = self
                    .node(parent)?
                    .children
                    .iter()
                    .position(|candidate| *candidate == child)
                    .unwrap();
                self.node(parent)?
                    .handle
                    .panel()
                    .and_then(|panel| panel.Children())
                    .and_then(|children| children.RemoveAt(position as u32))?;
            }
            Attachment::Command { section, .. } => {
                let position = self
                    .node(parent)?
                    .children
                    .iter()
                    .take_while(|candidate| **candidate != child)
                    .filter(|sibling| {
                        matches!(
                            self.nodes.get(sibling).and_then(|node| node.attachment),
                            Some(Attachment::Command {
                                section: sibling_section,
                                ..
                            }) if sibling_section == section
                        )
                    })
                    .count();
                let (Handle::CommandBar { state, .. } | Handle::CommandBarFlyout { state, .. }) =
                    &self.node(parent)?.handle
                else {
                    panic!("command parent is invalid");
                };
                match section {
                    CommandSection::Primary => &state.primary,
                    CommandSection::Secondary => &state.secondary,
                }
                .RemoveAt(position as u32)?;
            }
            Attachment::Content => {
                let handle = &self.node(parent)?.handle;
                if let Some(content) = handle.content_control() {
                    content.and_then(|content| {
                        content.SetContent(None::<&windows_core::IInspectable>)
                    })?;
                } else {
                    match handle {
                        Handle::Border(value) => {
                            value.SetChild(None::<&bindings::UIElement>)?;
                        }
                        Handle::Flyout { value, .. } => {
                            value.SetContent(None::<&bindings::UIElement>)?;
                        }
                        Handle::Viewbox(value) => {
                            value.SetChild(None::<&bindings::UIElement>)?;
                        }
                        Handle::ScrollView { value, .. } => {
                            value.SetContent(None::<&bindings::UIElement>)?;
                        }
                        Handle::SplitView { value, .. } => {
                            value.SetContent(None::<&bindings::UIElement>)?;
                        }
                        Handle::TitleBar { value, .. } => {
                            value.SetContent(None::<&bindings::UIElement>)?;
                        }
                        _ => panic!("content parent is invalid"),
                    }
                }
            }
            Attachment::Pane => match &self.node(parent)?.handle {
                Handle::SplitView { value, .. } => {
                    value.SetPane(None::<&bindings::UIElement>)?;
                }
                Handle::TitleBar { value, .. } => {
                    value.SetRightHeader(None::<&bindings::UIElement>)?;
                }
                _ => panic!("pane parent is invalid"),
            },
            Attachment::PaneFooter => match &self.node(parent)?.handle {
                Handle::NavigationView(state) => {
                    state.value.SetPaneFooter(None::<&bindings::UIElement>)?;
                }
                _ => panic!("pane-footer parent is invalid"),
            },
            Attachment::Header => match &self.node(parent)?.handle {
                Handle::ContentDialog { value, .. } => {
                    value.SetTitle(None::<&windows_core::IInspectable>)?;
                }
                Handle::Expander(state) => {
                    state.value.SetHeader(None::<&windows_core::IInspectable>)?;
                }
                Handle::Collection { value, .. } => {
                    value.SetHeader(None::<&windows_core::IInspectable>)?;
                }
                _ => panic!("header parent is invalid"),
            },
            Attachment::Item { .. } => {
                let position = self
                    .node(parent)?
                    .children
                    .iter()
                    .position(|candidate| *candidate == child)
                    .unwrap();
                if let (Handle::NavigationView(state), Handle::NavigationViewItem(item)) =
                    (&self.node(parent)?.handle, &self.node(child)?.handle)
                {
                    let key = item.key.get().unwrap();
                    let items = state.value.MenuItems()?;
                    state.callback.suppressing.set(true);
                    let result = items.RemoveAt(position as u32);
                    state.callback.suppressing.set(false);
                    if result.is_ok() {
                        state
                            .items
                            .borrow_mut()
                            .retain(|(candidate, _)| *candidate != key);
                    }
                    result?;
                } else if let (
                    Handle::SelectorBar { value, state, .. },
                    Handle::SelectorBarItem(item),
                ) = (&self.node(parent)?.handle, &self.node(child)?.handle)
                {
                    let key = item.key.get().unwrap();
                    let items = value.Items()?;
                    state.callback.suppressing.set(true);
                    let result = items.RemoveAt(position as u32);
                    state.callback.suppressing.set(false);
                    if result.is_ok() {
                        state
                            .items
                            .borrow_mut()
                            .retain(|(candidate, _)| *candidate != key);
                    }
                    result?;
                } else {
                    let Some(items) = self.node(parent)?.handle.item_collection() else {
                        panic!("item parent is invalid");
                    };
                    let items = items?;
                    let registration = match (&self.node(parent)?.handle, &self.node(child)?.handle)
                    {
                        (Handle::TabView { state, .. }, Handle::TabViewItem(item)) => {
                            let inspectable: windows_core::IInspectable = item.value.cast()?;
                            let identity = Self::identity(&inspectable)?;
                            state.suppressing_items.set(true);
                            Some((state, identity))
                        }
                        (Handle::TabView { .. }, _) => panic!("TabView item child is invalid"),
                        _ => None,
                    };
                    let result = items.RemoveAt(position as u32);
                    if let Some((state, identity)) = registration {
                        state.suppressing_items.set(false);
                        if result.is_ok() {
                            state
                                .item_keys
                                .borrow_mut()
                                .retain(|(candidate, _)| *candidate != identity);
                        }
                    }
                    result?;
                }
            }
            Attachment::VirtualItem { index, lease } => {
                if let Some(slot) = self
                    .collection(parent)?
                    .slots
                    .borrow()
                    .get(&(index, lease))
                    .cloned()
                {
                    slot.content
                        .SetContent(None::<&windows_core::IInspectable>)?;
                }
            }
        }
        self.node_mut(parent)?
            .children
            .retain(|candidate| *candidate != child);
        let child = self.node_mut(child)?;
        child.parent = None;
        child.attachment = None;
        Ok(())
    }

    pub(super) fn bind_owner(
        &self,
        owner: NodeId,
        accessory: NodeId,
        relation: OwnerRelation,
    ) -> WindowsResult<()> {
        match relation {
            OwnerRelation::ButtonFlyout => {
                let flyout: bindings::FlyoutBase = match &self.node(accessory)?.handle {
                    Handle::Flyout { value, .. } => value.cast()?,
                    Handle::MenuFlyout { value, .. } => value.cast()?,
                    Handle::CommandBarFlyout { value, .. } => value.cast()?,
                    _ => panic!("flyout relation accessory is invalid"),
                };
                match &self.node(owner)?.handle {
                    Handle::SplitButton { value: owner, .. } => owner.SetFlyout(&flyout),
                    handle => {
                        let owner: bindings::IButton =
                            handle.control().and_then(|control| control.cast())?;
                        owner.SetFlyout(&flyout)
                    }
                }
            }
            OwnerRelation::TeachingTipTarget => {
                let owner = self.node(owner)?.handle.framework_element()?;
                let Handle::TeachingTip { value, state } = &self.node(accessory)?.handle else {
                    panic!("teaching-tip relation accessory is invalid");
                };
                if state.open.native.get() {
                    let revision = state.deferred_open.revision.get().wrapping_add(1);
                    state.deferred_open.revision.set(revision);
                    value.SetTarget(&owner)?;
                    state.current_target.borrow_mut().replace(owner);
                    return Ok(());
                }
                value.SetTarget(&owner)?;
                state.current_target.borrow_mut().replace(owner);
                if state.open.expected.get() {
                    self.defer_teaching_tip_open(accessory)?;
                }
                Ok(())
            }
            OwnerRelation::ToolTip => {
                let owner = self.node(owner)?.handle.ui_element()?;
                let tooltip = self
                    .node(accessory)?
                    .handle
                    .ui_element()
                    .and_then(|value| value.cast::<windows_core::IInspectable>())?;
                bindings::ToolTipService::SetToolTip(&owner, &tooltip)
            }
        }
    }

    pub(super) fn unbind_owner(
        &self,
        owner: NodeId,
        accessory: NodeId,
        relation: OwnerRelation,
    ) -> WindowsResult<()> {
        match relation {
            OwnerRelation::ButtonFlyout => {
                let flyout: bindings::IFlyoutBase = match &self.node(accessory)?.handle {
                    Handle::Flyout { value, .. } => value.cast()?,
                    Handle::MenuFlyout { value, .. } => value.cast()?,
                    Handle::CommandBarFlyout { value, .. } => value.cast()?,
                    _ => panic!("flyout relation accessory is invalid"),
                };
                flyout.Hide()?;
                match &self.node(owner)?.handle {
                    Handle::SplitButton { value: owner, .. } => {
                        owner.SetFlyout(None::<&bindings::FlyoutBase>)
                    }
                    handle => {
                        let owner: bindings::IButton =
                            handle.control().and_then(|control| control.cast())?;
                        owner.SetFlyout(None::<&bindings::FlyoutBase>)
                    }
                }
            }
            OwnerRelation::TeachingTipTarget => {
                let Handle::TeachingTip { .. } = &self.node(accessory)?.handle else {
                    panic!("teaching-tip relation accessory is invalid");
                };
                Ok(())
            }
            OwnerRelation::ToolTip => {
                let owner = self.node(owner)?.handle.ui_element()?;
                bindings::ToolTipService::SetToolTip(&owner, None::<&windows_core::IInspectable>)
            }
        }
    }

    pub(super) fn move_child(
        &mut self,
        parent: NodeId,
        child: NodeId,
        index: usize,
    ) -> WindowsResult<()> {
        let child_node = self.node(child)?;
        assert!(
            child_node.parent == Some(parent),
            "move does not match native parent"
        );
        let attachment = child_node.attachment.unwrap();
        let parent_node = self.node(parent)?;
        let siblings = &parent_node.children;
        assert!(index < siblings.len(), "move index is out of bounds");
        let current = siblings
            .iter()
            .position(|candidate| *candidate == child)
            .unwrap_or_else(|| panic!("native child is missing"));
        match attachment {
            Attachment::Child { .. } => {
                let child_ui = child_node.handle.ui_element()?;
                let children = parent_node
                    .handle
                    .panel()
                    .and_then(|panel| panel.Children())?;
                children.RemoveAt(current as u32)?;
                children.InsertAt(index as u32, &child_ui)?;
            }
            Attachment::Command { section, .. } => {
                let element = child_node.handle.command_bar_element().cloned().unwrap();
                let primary_count = siblings
                    .iter()
                    .filter(|sibling| {
                        matches!(
                            self.nodes.get(sibling).and_then(|node| node.attachment),
                            Some(Attachment::Command {
                                section: CommandSection::Primary,
                                ..
                            })
                        )
                    })
                    .count();
                let current_section = siblings[..current]
                    .iter()
                    .filter(|sibling| {
                        matches!(
                            self.nodes.get(sibling).and_then(|node| node.attachment),
                            Some(Attachment::Command {
                                section: sibling_section,
                                ..
                            }) if sibling_section == section
                        )
                    })
                    .count();
                let target_section = match section {
                    CommandSection::Primary => index,
                    CommandSection::Secondary => index - primary_count,
                };
                let (Handle::CommandBar { state, .. } | Handle::CommandBarFlyout { state, .. }) =
                    &parent_node.handle
                else {
                    panic!("command move parent is invalid");
                };
                let commands = match section {
                    CommandSection::Primary => &state.primary,
                    CommandSection::Secondary => &state.secondary,
                };
                commands.RemoveAt(current_section as u32)?;
                commands.InsertAt(target_section as u32, &element)?;
            }
            Attachment::Item { .. } => {
                if let (Handle::NavigationView(state), Handle::NavigationViewItem(item)) =
                    (&parent_node.handle, &child_node.handle)
                {
                    let native_items = state.value.MenuItems()?;
                    let item_value: windows_core::IInspectable = item.value.cast()?;
                    let selected = state
                        .callback
                        .expected
                        .get()
                        .and_then(|selected| {
                            state
                                .items
                                .borrow()
                                .iter()
                                .find(|(key, _)| *key == selected)
                                .map(|(_, item)| item.clone())
                        })
                        .map(|item| item.cast::<windows_core::IInspectable>())
                        .transpose()?;
                    state.callback.suppressing.set(true);
                    let result = native_items
                        .RemoveAt(current as u32)
                        .and_then(|()| native_items.InsertAt(index as u32, &item_value))
                        .and_then(|()| state.value.SetSelectedItem(selected.as_ref()));
                    state.callback.suppressing.set(false);
                    result?;
                    let mut items = state.items.borrow_mut();
                    let item = items.remove(current);
                    items.insert(index, item);
                } else if let (
                    Handle::SelectorBar { value, state, .. },
                    Handle::SelectorBarItem(item),
                ) = (&parent_node.handle, &child_node.handle)
                {
                    let items = value.Items()?;
                    state.callback.suppressing.set(true);
                    let result = items
                        .RemoveAt(current as u32)
                        .and_then(|()| items.InsertAt(index as u32, &item.value));
                    state.callback.suppressing.set(false);
                    result?;
                } else {
                    let child = child_node.handle.ui_element()?;
                    let Some(items) = parent_node.handle.item_collection() else {
                        panic!("item move parent is invalid");
                    };
                    let items = items?;
                    let tab_state = match &parent_node.handle {
                        Handle::TabView { state, .. } => {
                            state.suppressing_items.set(true);
                            Some(state)
                        }
                        _ => None,
                    };
                    let result = items
                        .RemoveAt(current as u32)
                        .and_then(|()| items.InsertAt(index as u32, &child));
                    if let Some(state) = tab_state {
                        state.suppressing_items.set(false);
                    }
                    result?;
                }
            }
            _ => panic!("native attachment cannot be moved"),
        }
        let siblings = &mut self.node_mut(parent)?.children;
        let child = siblings.remove(current);
        siblings.insert(index, child);
        Ok(())
    }
}
