use super::*;

pub(super) struct CommandBarState {
    pub(super) primary: windows_collections::IObservableVector<bindings::ICommandBarElement>,
    pub(super) secondary: windows_collections::IObservableVector<bindings::ICommandBarElement>,
}

pub(super) struct AppBarButtonState {
    _revoker: windows_core::EventRevoker,
    value: bindings::AppBarButton,
    pub(super) element: bindings::ICommandBarElement,
    pub(super) control: bindings::IControl,
    icon: Option<bindings::IconElement>,
    icon_value: Option<Icon>,
}

pub(super) struct AppBarToggleButtonState {
    _revokers: [windows_core::EventRevoker; 2],
    value: bindings::AppBarToggleButton,
    pub(super) element: bindings::ICommandBarElement,
    pub(super) control: bindings::IControl,
    toggle: bindings::IToggleButton,
    expected: Rc<Cell<bool>>,
    icon: Option<bindings::IconElement>,
    icon_value: Option<Icon>,
}

impl WinUiRuntime {
    pub(super) fn create_command_bar(&self) -> WindowsResult<Handle> {
        let value = bindings::CommandBar::new()?;
        let primary = value.PrimaryCommands()?;
        let secondary = value.SecondaryCommands()?;
        Ok(Handle::CommandBar {
            value,
            state: Box::new(CommandBarState { primary, secondary }),
        })
    }

    pub(super) fn create_command_bar_flyout(&self, id: NodeId) -> WindowsResult<Handle> {
        let value = bindings::CommandBarFlyout::new()?;
        let primary = value.PrimaryCommands()?;
        let secondary = value.SecondaryCommands()?;
        let flyout: bindings::IFlyoutBase = value.cast()?;
        let revokers = overlay::subscribe_flyout(
            &flyout,
            id,
            Rc::clone(&self.events),
            Rc::clone(&self.waker),
        )?;
        Ok(Handle::CommandBarFlyout {
            _revokers: revokers,
            value,
            state: Box::new(CommandBarState { primary, secondary }),
        })
    }

    pub(super) fn create_app_bar_button(&self, id: NodeId) -> WindowsResult<Handle> {
        let value = bindings::AppBarButton::new()?;
        let element = value.cast()?;
        let control = value.cast()?;
        let button: bindings::IButtonBase = value.cast()?;
        let revoker =
            subscribe_click(&button, id, Rc::clone(&self.events), Rc::clone(&self.waker))?;
        Ok(Handle::AppBarButton(Box::new(AppBarButtonState {
            _revoker: revoker,
            value,
            element,
            control,
            icon: None,
            icon_value: None,
        })))
    }

    pub(super) fn create_app_bar_toggle_button(&self, id: NodeId) -> WindowsResult<Handle> {
        let value = bindings::AppBarToggleButton::new()?;
        let element = value.cast()?;
        let control = value.cast()?;
        let toggle = value.cast()?;
        let expected = Rc::new(Cell::new(false));
        let revokers = subscribe_toggle(
            &toggle,
            id,
            &expected,
            Rc::clone(&self.events),
            Rc::clone(&self.waker),
        )?;
        Ok(Handle::AppBarToggleButton(Box::new(
            AppBarToggleButtonState {
                _revokers: revokers,
                value,
                element,
                control,
                toggle,
                expected,
                icon: None,
                icon_value: None,
            },
        )))
    }

    pub(super) fn create_app_bar_separator(&self) -> WindowsResult<Handle> {
        let value = bindings::AppBarSeparator::new()?;
        let element = value.cast()?;
        Ok(Handle::AppBarSeparator { element })
    }

    pub(super) fn apply_app_bar_button_update(
        &mut self,
        id: NodeId,
        update: &AppBarButtonUpdate,
    ) -> WindowsResult<()> {
        let icon = match &self.node(id)?.handle {
            Handle::AppBarButton(state) if state.icon_value != update.icon => {
                Some(media::create_icon(update.icon.as_ref())?)
            }
            Handle::AppBarButton(_) => None,
            _ => {
                panic!("AppBarButton update target is not an AppBarButton");
            }
        };
        let Handle::AppBarButton(state) = &mut self.node_mut(id)?.handle else {
            unreachable!()
        };
        state.value.SetLabel(&update.label)?;
        if let Some(icon) = icon {
            state.value.SetIcon(icon.as_ref())?;
            state.icon = icon;
            state.icon_value.clone_from(&update.icon);
        }
        state.control.SetIsEnabled(update.enabled)
    }

    pub(super) fn apply_app_bar_toggle_button_update(
        &mut self,
        id: NodeId,
        update: &AppBarToggleButtonUpdate,
    ) -> WindowsResult<()> {
        let icon = match &self.node(id)?.handle {
            Handle::AppBarToggleButton(state) if state.icon_value != update.icon => {
                Some(media::create_icon(update.icon.as_ref())?)
            }
            Handle::AppBarToggleButton(_) => None,
            _ => {
                panic!("AppBarToggleButton update target is not an AppBarToggleButton");
            }
        };
        let Handle::AppBarToggleButton(state) = &mut self.node_mut(id)?.handle else {
            unreachable!()
        };
        state.value.SetLabel(&update.label)?;
        if let Some(icon) = icon {
            state.value.SetIcon(icon.as_ref())?;
            state.icon = icon;
            state.icon_value.clone_from(&update.icon);
        }
        state.control.SetIsEnabled(update.enabled)?;
        if state.expected.replace(update.checked) != update.checked {
            state.toggle.SetIsChecked(Some(update.checked))?;
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "../../testing/private/winui/command_access.rs"]
pub(super) mod tests;
