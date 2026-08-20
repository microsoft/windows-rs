use super::*;

impl<R: NativeRuntime> Pump<R> {
    pub(super) fn prepare_component_effects(
        &self,
        changes: &ComponentChanges,
    ) -> Result<(), PumpError> {
        let cleanup = changes
            .retired
            .iter()
            .chain(changes.composed.iter())
            .copied()
            .collect::<HashSet<_>>();
        let root = self.root.ok_or(PumpError::NotMounted)?;
        for node in self.tree.subtree_postorder(root)? {
            if self.tree.kind(node)? != NodeKind::Component {
                continue;
            }
            let token = self.components.token(self.tree.component_scope(node)?)?;
            if cleanup.contains(&token) {
                self.components.cleanup_effects(token)?;
            }
        }
        Ok(())
    }

    pub(super) fn commit_component_effects(
        &self,
        changes: &ComponentChanges,
    ) -> Result<(), PumpError> {
        let retired = changes.retired.iter().copied().collect::<HashSet<_>>();
        let setup = changes
            .reserved
            .iter()
            .chain(changes.composed.iter())
            .copied()
            .filter(|token| !retired.contains(token))
            .collect::<HashSet<_>>();
        let root = self.root.ok_or(PumpError::NotMounted)?;
        let mut pending = vec![root];
        while let Some(node) = pending.pop() {
            pending.extend(self.tree.children(node)?.iter().rev().copied());
            if self.tree.kind(node)? != NodeKind::Component {
                continue;
            }
            let token = self.components.token(self.tree.component_scope(node)?)?;
            if setup.contains(&token) {
                self.components.commit_effects(token)?;
            }
        }
        Ok(())
    }

    pub(super) fn finalize_component_changes(
        &mut self,
        changes: &ComponentChanges,
    ) -> Result<(), PumpError> {
        for token in changes.reserved.iter().copied() {
            if let Err(error) = self.components.publish(token) {
                self.poisoned = true;
                return Err(error.into());
            }
        }
        for token in changes.retired.iter().copied() {
            if let Err(error) = self
                .components
                .retire(token)
                .and_then(|()| self.components.remove(token))
            {
                self.poisoned = true;
                return Err(error.into());
            }
        }
        Ok(())
    }

    pub(super) fn remove_reservations(
        components: &mut ComponentStore,
        reserved: &[ComponentToken],
    ) {
        for token in reserved.iter().rev().copied() {
            _ = components.remove(token);
        }
    }
}
