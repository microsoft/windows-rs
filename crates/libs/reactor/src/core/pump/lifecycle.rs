use std::cmp::Reverse;

use super::*;

impl<R: NativeRuntime> Pump<R> {
    pub(super) fn prepare_component_effects(&self, changes: &ComponentChanges) {
        let cleanup = changes
            .retired
            .iter()
            .chain(changes.composed.iter())
            .copied()
            .filter(|token| !changes.reserved.contains(token))
            .collect::<HashSet<_>>();
        let retired = changes.retired.iter().copied().collect::<HashSet<_>>();
        let mut ordered = cleanup
            .into_iter()
            .map(|token| {
                let node = self.tree.component_node(token.scope()).unwrap();
                (Reverse(self.tree.depth(node)), node, token)
            })
            .collect::<Vec<_>>();
        ordered.sort_unstable_by(|left, right| (&left.0, &left.1).cmp(&(&right.0, &right.1)));
        for (_, _, token) in ordered {
            if retired.contains(&token) {
                self.components.cleanup_effects(token);
            } else {
                self.components.prepare_effects(token);
            }
        }
    }

    pub(super) fn commit_component_effects(&self, changes: &ComponentChanges) {
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
                let node = self.tree.component_node(token.scope()).unwrap();
                (self.tree.depth(node), node, token)
            })
            .collect::<Vec<_>>();
        ordered.sort_unstable_by(|left, right| (&left.0, &left.1).cmp(&(&right.0, &right.1)));
        for (_, _, token) in ordered {
            self.components.commit_effects(token);
        }
    }

    pub(super) fn finalize_component_changes(&mut self, changes: &ComponentChanges) {
        let retired = changes.retired.iter().copied().collect::<HashSet<_>>();
        for token in changes.reserved.iter().copied() {
            if !retired.contains(&token) {
                self.components.publish(token);
            }
        }
        for (token, dependencies) in &changes.context_reads {
            if !retired.contains(token) {
                self.components
                    .set_context_dependencies(*token, dependencies.clone());
            }
        }
        for token in changes.retired.iter().copied() {
            self.components.remove(token);
        }
    }

    pub(super) fn remove_reservations(
        components: &mut ComponentStore,
        reserved: &[ComponentToken],
    ) {
        for token in reserved.iter().rev().copied() {
            components.remove(token);
        }
    }
}
