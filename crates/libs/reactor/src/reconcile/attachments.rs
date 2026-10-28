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

    pub(super) fn mount_tooltip(&mut self, target: ObjectId, tooltip: &DeclaredTooltip) {
        let object = self.mount(&tooltip.declaration());
        self.retained_attachments(target).tooltip = Some(RetainedTooltip {
            object,
            placement: tooltip.placement,
        });
        self.mutations.push(Mutation::SetTooltip {
            target,
            tooltip: Some(object),
            placement: tooltip.placement,
        });
    }

    pub(super) fn clear_tooltip(&mut self, target: ObjectId) {
        let tooltip = self
            .retained
            .get_mut(target)
            .attachments
            .as_mut()
            .and_then(|attachments| attachments.tooltip.take());
        if let Some(tooltip) = tooltip {
            self.mutations.push(Mutation::SetTooltip {
                target,
                tooltip: None,
                placement: tooltip.placement,
            });
            self.retire(tooltip.object);
        }
    }

    pub(super) fn reconcile_tooltip(
        &mut self,
        target: ObjectId,
        previous: Option<RetainedTooltip>,
        desired: Option<&DeclaredTooltip>,
        target_replaced: bool,
    ) {
        match (previous, desired) {
            (None, None) => {}
            (None, Some(desired)) => self.mount_tooltip(target, desired),
            (Some(previous), None) => {
                if !target_replaced {
                    self.mutations.push(Mutation::SetTooltip {
                        target,
                        tooltip: None,
                        placement: previous.placement,
                    });
                }
                self.retire(previous.object);
            }
            (Some(previous), Some(desired)) => {
                let object = self.reconcile_object(previous.object, &desired.declaration());
                if target_replaced
                    || object != previous.object
                    || desired.placement != previous.placement
                {
                    self.mutations.push(Mutation::SetTooltip {
                        target,
                        tooltip: Some(object),
                        placement: desired.placement,
                    });
                }
                self.retained_attachments(target).tooltip = Some(RetainedTooltip {
                    object,
                    placement: desired.placement,
                });
            }
        }
    }

    pub(super) fn mount_flyout(&mut self, target: ObjectId, flyout: &DeclaredFlyout) {
        let object = self.mount(&flyout.declaration());
        self.retained_attachments(target).flyout = Some(RetainedFlyout {
            object,
            placement: flyout.placement,
        });
        self.mutations.push(Mutation::SetFlyout {
            target,
            content: Some(object),
            placement: flyout.placement,
        });
    }

    pub(super) fn clear_flyout(&mut self, target: ObjectId) {
        let flyout = self
            .retained
            .get_mut(target)
            .attachments
            .as_mut()
            .and_then(|attachments| attachments.flyout.take());
        if let Some(flyout) = flyout {
            self.mutations.push(Mutation::SetFlyout {
                target,
                content: None,
                placement: flyout.placement,
            });
            self.retire(flyout.object);
        }
    }

    pub(super) fn reconcile_flyout(
        &mut self,
        target: ObjectId,
        previous: Option<RetainedFlyout>,
        desired: Option<&DeclaredFlyout>,
        target_replaced: bool,
    ) {
        match (previous, desired) {
            (None, None) => {}
            (None, Some(desired)) => self.mount_flyout(target, desired),
            (Some(previous), None) => {
                if !target_replaced {
                    self.mutations.push(Mutation::SetFlyout {
                        target,
                        content: None,
                        placement: previous.placement,
                    });
                }
                self.retire(previous.object);
            }
            (Some(previous), Some(desired)) => {
                let object = self.reconcile_object(previous.object, &desired.declaration());
                if target_replaced
                    || object != previous.object
                    || desired.placement != previous.placement
                {
                    self.mutations.push(Mutation::SetFlyout {
                        target,
                        content: Some(object),
                        placement: desired.placement,
                    });
                }
                self.retained_attachments(target).flyout = Some(RetainedFlyout {
                    object,
                    placement: desired.placement,
                });
            }
        }
    }

    pub(super) fn mount_menu(&mut self, target: ObjectId, menu: &DeclaredMenu) {
        let revision = 1;
        self.retained_attachments(target).menu = Some(RetainedMenu {
            menu: menu.menu.clone(),
            revision,
        });
        self.mutations.push(Mutation::SetMenu {
            target,
            menu: Some(menu.menu.clone()),
            revision,
        });
    }

    pub(super) fn clear_menu(&mut self, target: ObjectId) {
        let menu = self
            .retained
            .get_mut(target)
            .attachments
            .as_mut()
            .and_then(|attachments| attachments.menu.take());
        if let Some(menu) = menu {
            self.mutations.push(Mutation::SetMenu {
                target,
                menu: None,
                revision: menu.revision.wrapping_add(1),
            });
        }
    }

    pub(super) fn reconcile_menu(
        &mut self,
        target: ObjectId,
        previous: Option<RetainedMenu>,
        desired: Option<&DeclaredMenu>,
        target_replaced: bool,
    ) {
        match (previous, desired) {
            (None, None) => {}
            (None, Some(desired)) => self.mount_menu(target, desired),
            (Some(previous), None) => {
                if !target_replaced {
                    self.mutations.push(Mutation::SetMenu {
                        target,
                        menu: None,
                        revision: previous.revision.wrapping_add(1),
                    });
                }
            }
            (Some(previous), Some(desired))
                if !target_replaced && previous.menu == desired.menu =>
            {
                self.retained_attachments(target).menu = Some(previous);
            }
            (Some(previous), Some(desired)) => {
                let revision = if !target_replaced && previous.menu.same_content(&desired.menu) {
                    previous.revision
                } else {
                    previous.revision.wrapping_add(1)
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
        }
    }

    pub(super) fn mount_command_bar_flyout(
        &mut self,
        target: ObjectId,
        flyout: &DeclaredCommandBarFlyout,
    ) {
        let revision = 1;
        self.retained_attachments(target).command_bar_flyout = Some(RetainedCommandBarFlyout {
            flyout: flyout.flyout.clone(),
            revision,
        });
        self.mutations.push(Mutation::SetCommandBarFlyout {
            target,
            flyout: Some(flyout.flyout.clone()),
            revision,
        });
    }

    pub(super) fn clear_command_bar_flyout(&mut self, target: ObjectId) {
        let flyout = self
            .retained
            .get_mut(target)
            .attachments
            .as_mut()
            .and_then(|attachments| attachments.command_bar_flyout.take());
        if let Some(flyout) = flyout {
            self.mutations.push(Mutation::SetCommandBarFlyout {
                target,
                flyout: None,
                revision: flyout.revision.wrapping_add(1),
            });
        }
    }

    pub(super) fn reconcile_command_bar_flyout(
        &mut self,
        target: ObjectId,
        previous: Option<RetainedCommandBarFlyout>,
        desired: Option<&DeclaredCommandBarFlyout>,
        target_replaced: bool,
    ) {
        match (previous, desired) {
            (None, None) => {}
            (None, Some(desired)) => self.mount_command_bar_flyout(target, desired),
            (Some(previous), None) => {
                if !target_replaced {
                    self.mutations.push(Mutation::SetCommandBarFlyout {
                        target,
                        flyout: None,
                        revision: previous.revision.wrapping_add(1),
                    });
                }
            }
            (Some(previous), Some(desired))
                if !target_replaced && previous.flyout == desired.flyout =>
            {
                self.retained_attachments(target).command_bar_flyout = Some(previous);
            }
            (Some(previous), Some(desired)) => {
                let revision = if !target_replaced && previous.flyout.same_content(&desired.flyout)
                {
                    previous.revision
                } else {
                    previous.revision.wrapping_add(1)
                };
                self.retained_attachments(target).command_bar_flyout =
                    Some(RetainedCommandBarFlyout {
                        flyout: desired.flyout.clone(),
                        revision,
                    });
                self.mutations.push(Mutation::SetCommandBarFlyout {
                    target,
                    flyout: Some(desired.flyout.clone()),
                    revision,
                });
            }
        }
    }

    pub(super) fn mount_content_dialog(&mut self, owner: ObjectId, dialog: &DeclaredContentDialog) {
        let object = self.mount(&dialog.declaration);
        self.retained_attachments(owner).content_dialog = Some(RetainedContentDialog {
            object,
            open: dialog.open,
        });
        self.mutations.push(Mutation::SetContentDialog {
            owner,
            dialog: Some(object),
        });
        if dialog.open {
            self.mutations.push(Mutation::SetContentDialogOpen {
                dialog: object,
                open: true,
            });
        }
    }

    pub(super) fn clear_content_dialog(&mut self, owner: ObjectId) {
        let dialog = self
            .retained
            .get_mut(owner)
            .attachments
            .as_mut()
            .and_then(|attachments| attachments.content_dialog.take());
        if let Some(dialog) = dialog {
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
            self.retire(dialog.object);
        }
    }

    pub(super) fn reconcile_content_dialog(
        &mut self,
        owner: ObjectId,
        previous: Option<RetainedContentDialog>,
        desired: Option<&DeclaredContentDialog>,
        owner_replaced: bool,
    ) {
        match (previous, desired) {
            (None, None) => {}
            (None, Some(desired)) => self.mount_content_dialog(owner, desired),
            (Some(previous), None) => {
                if !owner_replaced {
                    if previous.open {
                        self.mutations.push(Mutation::SetContentDialogOpen {
                            dialog: previous.object,
                            open: false,
                        });
                    }
                    self.mutations.push(Mutation::SetContentDialog {
                        owner,
                        dialog: None,
                    });
                }
                self.retire(previous.object);
            }
            (Some(previous), Some(desired)) => {
                let object = self.reconcile_object(previous.object, &desired.declaration);
                if owner_replaced || object != previous.object {
                    self.mutations.push(Mutation::SetContentDialog {
                        owner,
                        dialog: Some(object),
                    });
                }
                if owner_replaced && desired.open || previous.open != desired.open {
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
    }
}
