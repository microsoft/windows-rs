use super::*;

impl WinUiRuntime {
    pub(super) fn create_info_bar(&self, id: NodeId) -> WindowsResult<Handle> {
        let value = bindings::InfoBar::new()?;
        let expected_open = Rc::new(Cell::new(false));
        let closing_expected_open = Rc::clone(&expected_open);
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let revoker = value.Closing(move |_sender, args| {
            if !closing_expected_open.get() {
                return;
            }
            args.as_ref().unwrap().SetCancel(true).unwrap();
            events
                .borrow_mut()
                .push_back(NativeEvent::InfoBarCloseRequested { target: id });
            if let Some(wake) = waker.borrow().as_ref() {
                wake();
            }
        })?;
        Ok(Handle::InfoBar {
            _revoker: revoker,
            expected_open,
            value,
        })
    }

    pub(super) fn apply_info_badge_value(
        &self,
        id: NodeId,
        value: Option<i32>,
    ) -> WindowsResult<()> {
        let Handle::InfoBadge(badge) = &self.node(id)?.handle else {
            panic!("InfoBadge value target is not an InfoBadge");
        };
        badge.SetValue(value.unwrap_or(-1))
    }

    pub(super) fn apply_info_bar_update(
        &self,
        id: NodeId,
        update: &InfoBarUpdate,
    ) -> WindowsResult<()> {
        let Handle::InfoBar {
            expected_open,
            value,
            ..
        } = &self.node(id)?.handle
        else {
            panic!("InfoBar update target is not an InfoBar");
        };
        value.SetTitle(&update.title)?;
        value.SetMessage(&update.message)?;
        value.SetSeverity(match update.severity {
            InfoBarSeverity::Informational => bindings::InfoBarSeverity::Informational,
            InfoBarSeverity::Success => bindings::InfoBarSeverity::Success,
            InfoBarSeverity::Warning => bindings::InfoBarSeverity::Warning,
            InfoBarSeverity::Error => bindings::InfoBarSeverity::Error,
        })?;
        value.SetIsClosable(update.closable)?;
        expected_open.set(update.open);
        value.SetIsOpen(update.open)
    }

    pub(super) fn apply_person_picture_update(
        &self,
        id: NodeId,
        update: &PersonPictureUpdate,
    ) -> WindowsResult<()> {
        let Handle::PersonPicture(value) = &self.node(id)?.handle else {
            panic!("PersonPicture update target is not a PersonPicture");
        };
        value.SetDisplayName(update.display_name.as_deref().unwrap_or_default())?;
        value.SetInitials(update.initials.as_deref().unwrap_or_default())
    }
}
