use std::cmp::Reverse;

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
        let retired = changes.retired.iter().copied().collect::<HashSet<_>>();
        let mut ordered = cleanup
            .into_iter()
            .map(|token| {
                let node = self
                    .tree
                    .component_node(token.scope())?
                    .ok_or(PumpError::StructureUnsupported)?;
                Ok((Reverse(self.tree.depth(node)?), node, token))
            })
            .collect::<Result<Vec<_>, PumpError>>()?;
        ordered.sort_unstable_by(|left, right| (&left.0, &left.1).cmp(&(&right.0, &right.1)));
        for (_, _, token) in ordered {
            if retired.contains(&token) {
                self.components.cleanup_effects(token)?;
            } else {
                self.components.prepare_effects(token)?;
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
        let mut ordered = setup
            .into_iter()
            .map(|token| {
                let node = self
                    .tree
                    .component_node(token.scope())?
                    .ok_or(PumpError::StructureUnsupported)?;
                Ok((self.tree.depth(node)?, node, token))
            })
            .collect::<Result<Vec<_>, PumpError>>()?;
        ordered.sort_unstable_by(|left, right| (&left.0, &left.1).cmp(&(&right.0, &right.1)));
        for (_, _, token) in ordered {
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
        let retired = changes.retired.iter().copied().collect::<HashSet<_>>();
        for (token, dependencies) in &changes.context_reads {
            if !retired.contains(token)
                && let Err(error) = self
                    .components
                    .set_context_dependencies(*token, dependencies.clone())
            {
                self.poisoned = true;
                return Err(error.into());
            }
        }
        for token in changes.retired.iter().copied() {
            if let Err(error) = self.components.remove(token) {
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
