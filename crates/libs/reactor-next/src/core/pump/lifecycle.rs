use std::cmp::Reverse;

use super::*;

impl<R: NativeRuntime> Pump<R> {
    pub(super) fn prepare_component_effects(
        &self,
        changes: &ComponentChanges,
    ) -> Result<(), PumpError> {
        for token in changes.retired.iter().copied() {
            self.components.cleanup_effects(token)?;
        }
        let retired = changes.retired.iter().copied().collect::<HashSet<_>>();
        let mut composed = changes
            .composed
            .iter()
            .copied()
            .filter(|token| !retired.contains(token))
            .map(|token| {
                let node = self
                    .tree
                    .component_node(token.scope())?
                    .ok_or(PumpError::StructureUnsupported)?;
                Ok((self.tree.depth(node)?, token))
            })
            .collect::<Result<Vec<_>, PumpError>>()?;
        composed.sort_unstable_by_key(|(depth, _)| Reverse(*depth));
        for (_, token) in composed {
            self.components.prepare_effects(token)?;
        }
        Ok(())
    }

    pub(super) fn commit_component_effects(
        &self,
        changes: &ComponentChanges,
    ) -> Result<(), PumpError> {
        let retired = changes.retired.iter().copied().collect::<HashSet<_>>();
        let mut tokens = changes
            .reserved
            .iter()
            .chain(changes.composed.iter())
            .copied()
            .filter(|token| !retired.contains(token))
            .collect::<HashSet<_>>()
            .into_iter()
            .map(|token| {
                let node = self
                    .tree
                    .component_node(token.scope())?
                    .ok_or(PumpError::StructureUnsupported)?;
                Ok((self.tree.depth(node)?, token))
            })
            .collect::<Result<Vec<_>, PumpError>>()?;
        tokens.sort_unstable_by_key(|(depth, _)| *depth);
        for (_, token) in tokens {
            self.components.commit_effects(token)?;
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
