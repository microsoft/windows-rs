use super::*;

impl RetainedGraph {
    pub(crate) fn tooltip(&self, target: ObjectId) -> Option<ObjectId> {
        self.get(target)?
            .attachments
            .as_ref()
            .and_then(|attachments| attachments.tooltip.as_ref())
            .map(|tooltip| tooltip.object)
    }

    pub fn flyout(&self, target: ObjectId) -> Option<(ObjectId, FlyoutPlacement)> {
        self.get(target)?
            .attachments
            .as_ref()
            .and_then(|attachments| attachments.flyout.as_ref())
            .map(|flyout| (flyout.object, flyout.placement))
    }

    pub fn content_dialog(&self, owner: ObjectId) -> Option<(ObjectId, bool)> {
        self.get(owner)?
            .attachments
            .as_ref()
            .and_then(|attachments| attachments.content_dialog.as_ref())
            .map(|dialog| (dialog.object, dialog.open))
    }
}

impl Planner<'_> {
    fn retained_attachments(&mut self, owner: ObjectId) -> &mut RetainedAttachments {
        self.retained
            .get_mut(owner)
            .attachments
            .get_or_insert_with(Default::default)
    }

    pub(super) fn take_attachments(&mut self, owner: ObjectId) -> RetainedAttachments {
        self.retained
            .get_mut(owner)
            .attachments
            .as_deref_mut()
            .map(std::mem::take)
            .unwrap_or_default()
    }

    /// Detaches every attachment from a target whose native object is about to be replaced. The
    /// attachment objects stay alive for `reconcile_attachments` to reuse or retire.
    pub(super) fn detach_attachments(&mut self, target: ObjectId, previous: &RetainedAttachments) {
        if let Some(tooltip) = &previous.tooltip {
            self.detach_tooltip(target, tooltip);
        }
        if let Some(flyout) = &previous.flyout {
            self.detach_flyout(target, flyout);
        }
        if let Some(menu) = &previous.menu {
            self.detach_menu(target, menu);
        }
        if let Some(flyout) = &previous.command_bar_flyout {
            self.detach_command_bar_flyout(target, flyout);
        }
        if let Some(dialog) = &previous.content_dialog {
            self.detach_content_dialog(target, dialog);
        }
    }

    pub(super) fn clear_attachments(&mut self, target: ObjectId) {
        let previous = self.take_attachments(target);
        if let Some(dialog) = previous.content_dialog {
            self.detach_content_dialog(target, &dialog);
            self.retire(dialog.object);
        }
        if let Some(flyout) = previous.command_bar_flyout {
            self.detach_command_bar_flyout(target, &flyout);
        }
        if let Some(menu) = previous.menu {
            self.detach_menu(target, &menu);
        }
        if let Some(flyout) = previous.flyout {
            self.detach_flyout(target, &flyout);
            self.retire(flyout.object);
        }
        if let Some(tooltip) = previous.tooltip {
            self.detach_tooltip(target, &tooltip);
            self.retire(tooltip.object);
        }
    }

    /// Reconciles `previous`, taken from the target, with the declared attachments. A replaced
    /// target has already had `previous` detached and needs every desired attachment set again.
    pub(super) fn reconcile_attachments(
        &mut self,
        target: ObjectId,
        previous: RetainedAttachments,
        desired: Option<&DeclaredAttachments>,
        replaced: bool,
    ) {
        let desired = desired.unwrap_or(&DeclaredAttachments::EMPTY);
        self.reconcile_tooltip(target, previous.tooltip, desired.tooltip.as_ref(), replaced);
        self.reconcile_flyout(target, previous.flyout, desired.flyout.as_ref(), replaced);
        self.reconcile_menu(target, previous.menu, desired.menu.as_ref(), replaced);
        self.reconcile_command_bar_flyout(
            target,
            previous.command_bar_flyout,
            desired.command_bar_flyout.as_ref(),
            replaced,
        );
        self.reconcile_content_dialog(
            target,
            previous.content_dialog,
            desired.content_dialog.as_ref(),
            replaced,
        );
    }

    fn detach_tooltip(&mut self, target: ObjectId, tooltip: &RetainedTooltip) {
        self.mutations.push(Mutation::SetTooltip {
            target,
            tooltip: None,
            placement: tooltip.placement,
        });
    }

    fn reconcile_tooltip(
        &mut self,
        target: ObjectId,
        previous: Option<RetainedTooltip>,
        desired: Option<&DeclaredTooltip>,
        replaced: bool,
    ) {
        let Some(desired) = desired else {
            if let Some(previous) = previous {
                if !replaced {
                    self.detach_tooltip(target, &previous);
                }
                self.retire(previous.object);
            }
            return;
        };
        let declaration = desired.declaration();
        let object = match &previous {
            Some(previous) => self.reconcile_object(previous.object, &declaration),
            None => self.mount(&declaration),
        };
        let tooltip = RetainedTooltip {
            object,
            placement: desired.placement,
        };
        if replaced || previous.as_ref() != Some(&tooltip) {
            self.mutations.push(Mutation::SetTooltip {
                target,
                tooltip: Some(object),
                placement: desired.placement,
            });
        }
        self.retained_attachments(target).tooltip = Some(tooltip);
    }

    fn detach_flyout(&mut self, target: ObjectId, flyout: &RetainedFlyout) {
        self.mutations.push(Mutation::SetFlyout {
            target,
            content: None,
            placement: flyout.placement,
        });
    }

    fn reconcile_flyout(
        &mut self,
        target: ObjectId,
        previous: Option<RetainedFlyout>,
        desired: Option<&DeclaredFlyout>,
        replaced: bool,
    ) {
        let Some(desired) = desired else {
            if let Some(previous) = previous {
                if !replaced {
                    self.detach_flyout(target, &previous);
                }
                self.retire(previous.object);
            }
            return;
        };
        let declaration = desired.declaration();
        let object = match &previous {
            Some(previous) => self.reconcile_object(previous.object, &declaration),
            None => self.mount(&declaration),
        };
        let flyout = RetainedFlyout {
            object,
            placement: desired.placement,
        };
        if replaced || previous.as_ref() != Some(&flyout) {
            self.mutations.push(Mutation::SetFlyout {
                target,
                content: Some(object),
                placement: desired.placement,
            });
        }
        self.retained_attachments(target).flyout = Some(flyout);
    }

    fn detach_menu(&mut self, target: ObjectId, menu: &RetainedMenu) {
        self.mutations.push(Mutation::SetMenu {
            target,
            menu: None,
            revision: menu.revision.wrapping_add(1),
        });
    }

    fn reconcile_menu(
        &mut self,
        target: ObjectId,
        previous: Option<RetainedMenu>,
        desired: Option<&DeclaredMenu>,
        replaced: bool,
    ) {
        let Some(desired) = desired else {
            if let Some(previous) = previous.filter(|_| !replaced) {
                self.detach_menu(target, &previous);
            }
            return;
        };
        let revision = match previous {
            Some(previous) if !replaced && previous.menu == desired.menu => {
                self.retained_attachments(target).menu = Some(previous);
                return;
            }
            Some(previous) if !replaced && previous.menu.same_content(&desired.menu) => {
                previous.revision
            }
            Some(previous) => previous.revision.wrapping_add(1),
            None => 1,
        };
        self.retained_attachments(target).menu = Some(RetainedMenu {
            menu: desired.menu.clone(),
            revision,
        });
        self.mutations.push(Mutation::SetMenu {
            target,
            menu: Some(desired.menu.clone()),
            revision,
        });
    }

    fn detach_command_bar_flyout(&mut self, target: ObjectId, flyout: &RetainedCommandBarFlyout) {
        self.mutations.push(Mutation::SetCommandBarFlyout {
            target,
            flyout: None,
            revision: flyout.revision.wrapping_add(1),
        });
    }

    fn reconcile_command_bar_flyout(
        &mut self,
        target: ObjectId,
        previous: Option<RetainedCommandBarFlyout>,
        desired: Option<&DeclaredCommandBarFlyout>,
        replaced: bool,
    ) {
        let Some(desired) = desired else {
            if let Some(previous) = previous.filter(|_| !replaced) {
                self.detach_command_bar_flyout(target, &previous);
            }
            return;
        };
        let revision = match previous {
            Some(previous) if !replaced && previous.flyout == desired.flyout => {
                self.retained_attachments(target).command_bar_flyout = Some(previous);
                return;
            }
            Some(previous) if !replaced && previous.flyout.same_content(&desired.flyout) => {
                previous.revision
            }
            Some(previous) => previous.revision.wrapping_add(1),
            None => 1,
        };
        self.retained_attachments(target).command_bar_flyout = Some(RetainedCommandBarFlyout {
            flyout: desired.flyout.clone(),
            revision,
        });
        self.mutations.push(Mutation::SetCommandBarFlyout {
            target,
            flyout: Some(desired.flyout.clone()),
            revision,
        });
    }

    fn detach_content_dialog(&mut self, owner: ObjectId, dialog: &RetainedContentDialog) {
        if dialog.open {
            self.mutations.push(Mutation::SetContentDialogOpen {
                dialog: dialog.object,
                open: false,
            });
        }
        self.mutations.push(Mutation::SetContentDialog {
            owner,
            dialog: None,
        });
    }

    fn reconcile_content_dialog(
        &mut self,
        owner: ObjectId,
        previous: Option<RetainedContentDialog>,
        desired: Option<&DeclaredContentDialog>,
        replaced: bool,
    ) {
        let Some(desired) = desired else {
            if let Some(previous) = previous {
                if !replaced {
                    self.detach_content_dialog(owner, &previous);
                }
                self.retire(previous.object);
            }
            return;
        };
        let object = match &previous {
            Some(previous) => self.reconcile_object(previous.object, &desired.declaration),
            None => self.mount(&desired.declaration),
        };
        if replaced || previous.as_ref().map(|previous| previous.object) != Some(object) {
            self.mutations.push(Mutation::SetContentDialog {
                owner,
                dialog: Some(object),
            });
        }
        let was_open = previous.is_some_and(|previous| previous.open);
        if replaced && desired.open || was_open != desired.open {
            self.mutations.push(Mutation::SetContentDialogOpen {
                dialog: object,
                open: desired.open,
            });
        }
        self.retained_attachments(owner).content_dialog = Some(RetainedContentDialog {
            object,
            open: desired.open,
        });
    }
}
