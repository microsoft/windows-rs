use super::*;

pub(super) struct NativeMenu {
    pub(super) menu: Menu,
    pub(super) revision: u64,
    pub(super) flyout: Option<native::MenuFlyout>,
    pub(super) _revokers: Vec<windows_core::EventRevoker>,
}

pub(super) struct NativeCommandBarFlyout {
    pub(super) flyout: CommandBarFlyout,
    pub(super) revision: u64,
    native: native::CommandBarFlyout,
    _revokers: Vec<windows_core::EventRevoker>,
    images: Vec<Rc<NativeImageSource>>,
}

impl WinUiAdapter {
    pub(super) fn set_tooltip(
        &mut self,
        target: ObjectId,
        tooltip: Option<ObjectId>,
        placement: TooltipPlacement,
    ) -> Result<(), WinUiError> {
        if self.tooltip_owners.contains_key(&target) {
            return Err(WinUiError::StillOwned(target));
        }
        let target_element = self
            .ui_element(target)?
            .cast::<native::DependencyObject>()?;
        let tooltip_element = tooltip
            .map(|tooltip| {
                if tooltip == target || self.kind(tooltip)? != ObjectType::ToolTip {
                    return Err(WinUiError::InvalidObject(tooltip));
                }
                if self.owners.contains_key(&tooltip)
                    || self
                        .tooltip_owners
                        .get(&tooltip)
                        .is_some_and(|owner| *owner != target)
                {
                    return Err(WinUiError::StillOwned(tooltip));
                }
                self.ui_element(tooltip)
            })
            .transpose()?;
        if let Some(tooltip) = &tooltip_element {
            native::ToolTipService::SetToolTip(&target_element, tooltip)?;
        } else {
            native::ToolTipService::SetToolTip(&target_element, None::<&IInspectable>)?;
        }
        native::ToolTipService::SetPlacement(
            &target_element,
            match placement {
                TooltipPlacement::Top => native::PlacementMode::Top,
                TooltipPlacement::Bottom => native::PlacementMode::Bottom,
                TooltipPlacement::Left => native::PlacementMode::Left,
                TooltipPlacement::Right => native::PlacementMode::Right,
                TooltipPlacement::Mouse => native::PlacementMode::Mouse,
            },
        )?;
        if let Some((previous, _)) = self.tooltips.remove(&target) {
            self.tooltip_owners.remove(&previous);
        }
        if let Some(tooltip) = tooltip {
            self.tooltips.insert(target, (tooltip, placement));
            self.tooltip_owners.insert(tooltip, target);
        }
        Ok(())
    }

    pub(super) fn set_flyout(
        &mut self,
        target: ObjectId,
        content: Option<ObjectId>,
        placement: FlyoutPlacement,
    ) -> Result<(), WinUiError> {
        if self.flyout_owners.contains_key(&target) {
            return Err(WinUiError::StillOwned(target));
        }
        let content_element = content
            .map(|content| {
                if content == target
                    || self.owners.contains_key(&content)
                    || self.tooltip_owners.contains_key(&content)
                    || self
                        .flyout_owners
                        .get(&content)
                        .is_some_and(|owner| *owner != target)
                    || self.content_dialog_owners.contains_key(&content)
                {
                    return Err(WinUiError::StillOwned(content));
                }
                self.ui_element(content)
            })
            .transpose()?;
        let handle = self
            .handles
            .get(&target)
            .ok_or(WinUiError::InvalidObject(target))?;
        if let Some(content) = content {
            let flyout = if let Some((flyout, previous, _)) = self.flyouts.get(&target) {
                if *previous != content {
                    flyout.SetContent(None::<&native::UIElement>)?;
                }
                flyout.clone()
            } else {
                let flyout = native::Flyout::new()?;
                match handle {
                    Handle::Generated(GeneratedHandle::Button(control)) => {
                        control.value.SetFlyout(&flyout)?;
                    }
                    Handle::Generated(GeneratedHandle::SplitButton(control)) => {
                        control.value.SetFlyout(&flyout)?;
                    }
                    _ => return Err(WinUiError::InvalidObject(target)),
                }
                flyout
            };
            flyout.SetContent(content_element.as_ref().unwrap())?;
            flyout
                .cast::<native::IFlyoutBase>()?
                .SetPlacement(match placement {
                    FlyoutPlacement::Top => native::FlyoutPlacementMode::Top,
                    FlyoutPlacement::Bottom => native::FlyoutPlacementMode::Bottom,
                    FlyoutPlacement::Left => native::FlyoutPlacementMode::Left,
                    FlyoutPlacement::Right => native::FlyoutPlacementMode::Right,
                    FlyoutPlacement::Full => native::FlyoutPlacementMode::Full,
                    FlyoutPlacement::TopEdgeAlignedLeft => {
                        native::FlyoutPlacementMode::TopEdgeAlignedLeft
                    }
                    FlyoutPlacement::TopEdgeAlignedRight => {
                        native::FlyoutPlacementMode::TopEdgeAlignedRight
                    }
                    FlyoutPlacement::BottomEdgeAlignedLeft => {
                        native::FlyoutPlacementMode::BottomEdgeAlignedLeft
                    }
                    FlyoutPlacement::BottomEdgeAlignedRight => {
                        native::FlyoutPlacementMode::BottomEdgeAlignedRight
                    }
                    FlyoutPlacement::LeftEdgeAlignedTop => {
                        native::FlyoutPlacementMode::LeftEdgeAlignedTop
                    }
                    FlyoutPlacement::LeftEdgeAlignedBottom => {
                        native::FlyoutPlacementMode::LeftEdgeAlignedBottom
                    }
                    FlyoutPlacement::RightEdgeAlignedTop => {
                        native::FlyoutPlacementMode::RightEdgeAlignedTop
                    }
                    FlyoutPlacement::RightEdgeAlignedBottom => {
                        native::FlyoutPlacementMode::RightEdgeAlignedBottom
                    }
                    FlyoutPlacement::Auto => native::FlyoutPlacementMode::Auto,
                })?;
            if let Some((_, previous, _)) =
                self.flyouts.insert(target, (flyout, content, placement))
            {
                self.flyout_owners.remove(&previous);
            }
            self.flyout_owners.insert(content, target);
        } else {
            if let Some((flyout, previous, _)) = self.flyouts.remove(&target) {
                flyout.SetContent(None::<&native::UIElement>)?;
                self.flyout_owners.remove(&previous);
            }
            match handle {
                Handle::Generated(GeneratedHandle::Button(control)) => {
                    control.value.SetFlyout(None::<&native::FlyoutBase>)?;
                }
                Handle::Generated(GeneratedHandle::SplitButton(control)) => {
                    control.value.SetFlyout(None::<&native::FlyoutBase>)?;
                }
                _ => return Err(WinUiError::InvalidObject(target)),
            }
        }
        Ok(())
    }

    pub(super) fn set_menu(
        &mut self,
        target: ObjectId,
        menu: Option<&Menu>,
        revision: u64,
    ) -> Result<(), WinUiError> {
        let kind = self.kind(target)?;
        if let Some(menu) = menu
            && let Some(current) = self.menus.get_mut(&target)
            && current.revision == revision
        {
            current.menu.clone_from(menu);
            return Ok(());
        }
        if let Some(previous) = self.menus.remove(&target) {
            if let Some(flyout) = &previous.flyout {
                flyout.cast::<native::IFlyoutBase>()?.Hide()?;
            }
            match kind {
                ObjectType::Button | ObjectType::DropDownButton => self
                    .ui_element(target)?
                    .cast::<native::IButton>()?
                    .SetFlyout(None::<&native::FlyoutBase>)?,
                ObjectType::MenuBarItem => {
                    let Some(Handle::Generated(GeneratedHandle::MenuBarItem(item))) =
                        self.handles.get(&target)
                    else {
                        return Err(WinUiError::InvalidObject(target));
                    };
                    let peer = native::FrameworkElementAutomationPeer::CreatePeerForElement(item)?;
                    let provider = peer
                        .GetPattern(native::PatternInterface::ExpandCollapse)?
                        .cast::<native::IExpandCollapseProvider>()?;
                    if provider.ExpandCollapseState()? == native::ExpandCollapseState::Expanded {
                        provider.Collapse()?;
                    }
                    // WinUI's MenuBarItem mirrors item removals, but ignores vector resets.
                    let items = item.Items()?;
                    while items.Size()? != 0 {
                        items.RemoveAtEnd()?;
                    }
                }
                _ => return Err(WinUiError::InvalidObject(target)),
            }
            drop(previous);
        }
        let Some(menu) = menu else {
            return Ok(());
        };
        let mut revokers = Vec::new();
        let flyout = match kind {
            ObjectType::Button | ObjectType::DropDownButton => {
                let flyout = native::MenuFlyout::new()?;
                build_menu_items(
                    &menu.items,
                    &flyout.Items()?,
                    &mut revokers,
                    &self.event_queue,
                    target,
                    revision,
                )?;
                self.ui_element(target)?
                    .cast::<native::IButton>()?
                    .SetFlyout(&flyout)?;
                Some(flyout)
            }
            ObjectType::MenuBarItem => {
                let Some(Handle::Generated(GeneratedHandle::MenuBarItem(item))) =
                    self.handles.get(&target)
                else {
                    return Err(WinUiError::InvalidObject(target));
                };
                build_menu_items(
                    &menu.items,
                    &item.Items()?,
                    &mut revokers,
                    &self.event_queue,
                    target,
                    revision,
                )?;
                None
            }
            _ => return Err(WinUiError::InvalidObject(target)),
        };
        self.menus.insert(
            target,
            NativeMenu {
                menu: menu.clone(),
                revision,
                flyout,
                _revokers: revokers,
            },
        );
        Ok(())
    }

    pub(super) fn set_command_bar_flyout(
        &mut self,
        target: ObjectId,
        flyout: Option<&CommandBarFlyout>,
        revision: u64,
    ) -> Result<(), WinUiError> {
        if self.kind(target)? != ObjectType::Button {
            return Err(WinUiError::InvalidObject(target));
        }
        if let Some(flyout) = flyout
            && let Some(current) = self.command_bar_flyouts.get_mut(&target)
            && current.revision == revision
        {
            current.flyout.clone_from(flyout);
            return Ok(());
        }
        if let Some(previous) = self.command_bar_flyouts.remove(&target) {
            previous.native.cast::<native::IFlyoutBase>()?.Hide()?;
            self.ui_element(target)?
                .cast::<native::IButton>()?
                .SetFlyout(None::<&native::FlyoutBase>)?;
            self.released_images.extend(previous.images);
        }
        let Some(flyout) = flyout else {
            return Ok(());
        };
        let native = native::CommandBarFlyout::new()?;
        let primary = native.PrimaryCommands()?;
        let secondary = native.SecondaryCommands()?;
        let mut revokers = Vec::new();
        let mut images = Vec::new();
        for command in flyout.primary.iter() {
            primary.Append(&self.build_command_bar_element(
                command,
                &mut revokers,
                &mut images,
                target,
                revision,
            )?)?;
        }
        for command in flyout.secondary.iter() {
            secondary.Append(&self.build_command_bar_element(
                command,
                &mut revokers,
                &mut images,
                target,
                revision,
            )?)?;
        }
        self.ui_element(target)?
            .cast::<native::IButton>()?
            .SetFlyout(&native)?;
        self.command_bar_flyouts.insert(
            target,
            NativeCommandBarFlyout {
                flyout: flyout.clone(),
                revision,
                native,
                _revokers: revokers,
                images,
            },
        );
        Ok(())
    }

    fn build_command_bar_element(
        &mut self,
        command: &CommandBarCommand,
        revokers: &mut Vec<windows_core::EventRevoker>,
        images: &mut Vec<Rc<NativeImageSource>>,
        target: ObjectId,
        revision: u64,
    ) -> Result<native::ICommandBarElement, WinUiError> {
        match command {
            CommandBarCommand::Button {
                key,
                label,
                icon,
                enabled,
            } => {
                let button = native::AppBarButton::new()?;
                button.SetLabel(label)?;
                button.cast::<native::IControl>()?.SetIsEnabled(*enabled)?;
                if let Some(icon) = icon {
                    let image = self.shared_icon_image(icon)?;
                    button.SetIcon(&icon_element(
                        icon,
                        image.as_ref().map(|image| &image.value),
                    )?)?;
                    images.extend(image);
                }
                let clicked_key = key.clone();
                let clicked_queue = Rc::clone(&self.event_queue);
                revokers.push(button.cast::<native::IButtonBase>()?.Click(move |_, _| {
                    clicked_queue.queue(
                        None,
                        Some(QueuedEvent {
                            object: target,
                            event: EventId::CommandInvoked,
                            handler: QueuedHandler::Presentation(revision),
                            payload: EventPayload::Key(clicked_key.clone()),
                        }),
                    );
                    Self::schedule_event_wake(&clicked_queue);
                })?);
                Ok(button.cast()?)
            }
            CommandBarCommand::Separator { .. } => Ok(native::AppBarSeparator::new()?.cast()?),
        }
    }
}

fn build_menu_items(
    items: &[MenuItem],
    output: &IVector<native::MenuFlyoutItemBase>,
    revokers: &mut Vec<windows_core::EventRevoker>,
    event_queue: &Rc<NativeEventQueue>,
    target: ObjectId,
    revision: u64,
) -> Result<(), WinUiError> {
    for entry in items {
        let item: native::MenuFlyoutItemBase = match entry {
            MenuItem::Item {
                key,
                label,
                enabled,
            } => {
                let item = native::MenuFlyoutItem::new()?;
                item.SetText(label)?;
                item.cast::<native::IControl>()?.SetIsEnabled(*enabled)?;
                let clicked_key = key.clone();
                let clicked_queue = Rc::clone(event_queue);
                revokers.push(item.Click(move |_, _| {
                    clicked_queue.queue(
                        None,
                        Some(QueuedEvent {
                            object: target,
                            event: EventId::MenuItemInvoked,
                            handler: QueuedHandler::Presentation(revision),
                            payload: EventPayload::Key(clicked_key.clone()),
                        }),
                    );
                    WinUiAdapter::schedule_event_wake(&clicked_queue);
                })?);
                item.cast()?
            }
            MenuItem::Separator { .. } => native::MenuFlyoutSeparator::new()?.cast()?,
            MenuItem::Submenu { label, items, .. } => {
                let item = native::MenuFlyoutSubItem::new()?;
                item.SetText(label)?;
                build_menu_items(
                    items,
                    &item.Items()?,
                    revokers,
                    event_queue,
                    target,
                    revision,
                )?;
                item.cast()?
            }
        };
        output.Append(&item)?;
    }
    Ok(())
}
