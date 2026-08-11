use std::panic::AssertUnwindSafe;
use std::rc::Rc;

use super::*;
use rustc_hash::{FxHashMap, FxHashSet};

impl<B: Backend + 'static> Reconciler<B> {
    pub(crate) fn mount_component_output(
        &mut self,
        ce: &ComponentElement,
        slot: LogicalSlotId,
    ) -> MountedOutput {
        let node_id = self.allocate_logical_node_id();
        let parent = self.tree.logical.active_parent();
        let mut cx = RenderCx::new(Rc::clone(&self.host.request_rerender));
        cx.set_context_stack(self.context_stack_handle());
        cx.set_marshaller(self.host.marshaller.clone());
        cx.set_host_id(self.host.host_id);
        cx.set_inner_size_cell(Rc::clone(&self.host.inner_size));
        cx.set_dpi_cell(Rc::clone(&self.host.dpi));
        cx.begin_render();
        let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
            let rendered = ce.obj.render(&mut cx);
            let read_contexts = cx.take_read_contexts();
            let rendered_output = {
                let _parent = self.enter_logical_parent(node_id);
                self.mount_output(&rendered)
            };
            (rendered, read_contexts, rendered_output)
        }));
        let (rendered, read_contexts, rendered_output) = match result {
            Ok(mounted) => mounted,
            Err(payload) => {
                cx.run_cleanups();
                std::panic::resume_unwind(payload);
            }
        };
        self.tree.logical.register_component(ComponentInstance {
            node_id,
            parent,
            native_root: rendered_output.native,
            rendered_output,
            render_cx: cx,
            last_rendered: rendered,
            last_obj: Rc::clone(&ce.obj),
            read_contexts,
        });
        self.queue_pending_effects(node_id);
        MountedOutput {
            slot,
            native: rendered_output.native,
            logical: Some(node_id),
        }
    }

    pub(crate) fn update_component_output(
        &mut self,
        old: &ComponentElement,
        new: &ComponentElement,
        old_output: MountedOutput,
    ) -> MountedOutput {
        if old.obj.component_type_id() != new.obj.component_type_id() {
            self.unmount_output(old_output);
            return self.mount_output(&Element::Component(new.clone()));
        }

        let parent = self.tree.logical.active_parent();
        let Some(node_id) = old_output.logical else {
            self.unmount_output(old_output);
            return self.mount_output(&Element::Component(new.clone()));
        };
        let forced = self.pass.forced_nodes.contains(&node_id);
        let state_dirty = self
            .tree
            .logical
            .instance(node_id)
            .is_some_and(|inst| inst.render_cx.take_state_dirty());
        let needs_update = if forced || state_dirty {
            true
        } else if new.memoised {
            !old.obj.is_equivalent(&*new.obj)
        } else {
            old.obj.should_update(&*new.obj)
        };

        if !needs_update {
            self.tree.logical.refresh_instance(
                node_id,
                Rc::clone(&new.obj),
                parent,
                old_output.native,
            );
            return old_output;
        }

        let Some(mut inst) = self.take_component_instance(node_id) else {
            self.unmount_output(old_output);
            return self.mount_output(&Element::Component(new.clone()));
        };

        let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
            inst.render_cx
                .set_context_stack(self.context_stack_handle());
            inst.render_cx
                .set_inner_size_cell(Rc::clone(&self.host.inner_size));
            inst.render_cx.set_dpi_cell(Rc::clone(&self.host.dpi));
            inst.render_cx.begin_render();
            let rendered = new.obj.render(&mut inst.render_cx);
            let read_contexts = inst.render_cx.take_read_contexts();
            let rendered_output = {
                let _parent = self.enter_logical_parent(node_id);
                self.update_output(&inst.last_rendered, &rendered, inst.rendered_output)
            };
            (rendered, read_contexts, rendered_output)
        }));
        let (rendered, read_contexts, rendered_output) = match result {
            Ok(updated) => updated,
            Err(payload) => {
                self.tree.logical.register_component(inst);
                std::panic::resume_unwind(payload);
            }
        };

        inst.last_rendered = rendered;
        inst.last_obj = Rc::clone(&new.obj);
        inst.native_root = rendered_output.native;
        inst.rendered_output = rendered_output;
        inst.read_contexts = read_contexts;
        inst.parent = parent;

        self.tree.logical.register_component(inst);
        self.queue_pending_effects(node_id);
        MountedOutput {
            slot: old_output.slot,
            native: rendered_output.native,
            logical: Some(node_id),
        }
    }

    pub(crate) fn mount_provider_output(
        &mut self,
        provider: &ProviderElement,
        slot: LogicalSlotId,
    ) -> MountedOutput {
        let node_id = self.allocate_logical_node_id();
        let parent = self.tree.logical.active_parent();
        let pushed = self.push_provisions(&provider.provisions);
        let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
            let _parent = self.enter_logical_parent(node_id);
            self.mount_output(&provider.child)
        }));
        self.pop_provisions(pushed);
        match result {
            Ok(output) => {
                self.tree.logical.register_wrapper(LogicalWrapperNode {
                    node_id,
                    parent,
                    native_root: output.native,
                    child_output: output,
                });
                MountedOutput {
                    slot,
                    native: output.native,
                    logical: Some(node_id),
                }
            }
            Err(payload) => std::panic::resume_unwind(payload),
        }
    }

    pub(crate) fn update_provider_output(
        &mut self,
        old: &ProviderElement,
        new: &ProviderElement,
        old_output: MountedOutput,
    ) -> MountedOutput {
        let mut changed_ids: FxHashSet<ContextId> = FxHashSet::default();

        let mut old_by_id: FxHashMap<ContextId, &ContextProvision> = FxHashMap::default();
        for p in &old.provisions {
            old_by_id.insert(p.context_id, p);
        }
        for new_p in &new.provisions {
            match old_by_id.remove(&new_p.context_id) {
                None => {
                    changed_ids.insert(new_p.context_id);
                }
                Some(old_p) if old_p != new_p => {
                    changed_ids.insert(new_p.context_id);
                }
                Some(_) => {}
            }
        }
        changed_ids.extend(old_by_id.into_keys());

        let Some(node_id) = old_output.logical else {
            self.unmount_output(old_output);
            return self.mount_provider_output(new, old_output.slot);
        };
        let saved_nodes = self.pass.forced_nodes.clone();
        let saved_controls = self.pass.forced_controls.clone();
        if !changed_ids.is_empty() {
            let affected = self.collect_affected_components_for_node(node_id, &changed_ids);
            self.add_forced_node_paths(affected);
        }

        let Some(mut provider) = self.tree.logical.take_provider(node_id) else {
            self.unmount_output(old_output);
            return self.mount_provider_output(new, old_output.slot);
        };
        provider.parent = self.tree.logical.active_parent();

        let pushed = self.push_provisions(&new.provisions);
        let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
            let _parent = self.enter_logical_parent(node_id);
            self.update_output(&old.child, &new.child, provider.child_output)
        }));
        self.pop_provisions(pushed);
        self.pass.forced_nodes = saved_nodes;
        self.pass.forced_controls = saved_controls;

        match result {
            Ok(output) => {
                provider.child_output = output;
                provider.native_root = output.native;
                self.tree.logical.register_wrapper(provider);
                MountedOutput {
                    slot: old_output.slot,
                    native: output.native,
                    logical: Some(node_id),
                }
            }
            Err(payload) => {
                self.tree.logical.register_wrapper(provider);
                std::panic::resume_unwind(payload);
            }
        }
    }

    fn push_provisions(&self, provisions: &[ContextProvision]) -> usize {
        for p in provisions {
            self.host.context_stack.push_raw_retain(
                p.context_id,
                p.value_type_id,
                Rc::clone(&p.value),
            );
        }
        provisions.len()
    }

    pub(crate) fn pop_provisions(&self, count: usize) {
        for _ in 0..count {
            self.host.context_stack.pop_raw();
        }
    }
}
