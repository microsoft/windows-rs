use std::panic::AssertUnwindSafe;
use std::rc::Rc;

use super::*;
use rustc_hash::{FxHashMap, FxHashSet};

impl<B: Backend + 'static> Reconciler<B> {
    pub fn mount_component(&mut self, ce: &ComponentElement) -> Option<ControlId> {
        let mut cx = RenderCx::new(Rc::clone(&self.request_rerender));
        cx.set_context_stack(self.context_stack_handle());
        cx.set_marshaller(self.marshaller.clone());
        cx.set_inner_size_cell(Rc::clone(&self.inner_size));
        cx.set_dpi_cell(Rc::clone(&self.dpi));
        cx.begin_render();
        let rendered = ce.obj.render(&mut cx);
        cx.flush_effects();
        let read_contexts = cx.take_read_contexts();

        let rendered_id = self.mount(&rendered)?;
        self.register_component_instance(
            rendered_id,
            ComponentInstance {
                render_cx: cx,
                last_rendered: rendered,
                last_obj: Rc::clone(&ce.obj),
                read_contexts,
            },
        );
        Some(rendered_id)
    }

    pub fn update_component(
        &mut self,
        old: &ComponentElement,
        new: &ComponentElement,
        id: ControlId,
    ) -> Option<ControlId> {
        if old.obj.component_type_id() != new.obj.component_type_id() {
            self.unmount(id);
            return self.mount_component(new);
        }

        let forced_by_context = self.forced_components.contains(&id);
        let state_dirty = self
            .component_instances
            .get(&id)
            .is_some_and(|inst| inst.render_cx.take_state_dirty());
        let needs_update = if forced_by_context || state_dirty {
            true
        } else if new.memoised {
            !old.obj.is_equivalent(&*new.obj)
        } else {
            old.obj.should_update(&*new.obj)
        };

        if !needs_update {
            if let Some(inst) = self.component_instances.get_mut(&id) {
                inst.last_obj = Rc::clone(&new.obj);
            }
            return Some(id);
        }

        let Some(mut inst) = self.take_component_instance(id) else {
            return self.mount_component(new);
        };

        inst.render_cx
            .set_context_stack(self.context_stack_handle());
        inst.render_cx
            .set_inner_size_cell(Rc::clone(&self.inner_size));
        inst.render_cx.set_dpi_cell(Rc::clone(&self.dpi));
        inst.render_cx.begin_render();
        let rendered = new.obj.render(&mut inst.render_cx);
        inst.render_cx.flush_effects();
        inst.read_contexts = inst.render_cx.take_read_contexts();

        let new_id = self.update(&inst.last_rendered, &rendered, id);

        inst.last_rendered = rendered;
        inst.last_obj = Rc::clone(&new.obj);

        if let Some(nid) = new_id {
            self.register_component_instance(nid, inst);
            Some(nid)
        } else {
            inst.render_cx.run_cleanups();
            None
        }
    }

    pub fn mount_error_boundary(&mut self, eb: &ErrorBoundaryElement) -> Option<ControlId> {
        let result = std::panic::catch_unwind(AssertUnwindSafe(|| self.mount(&eb.child)));
        match result {
            Ok(id) => id,
            Err(payload) => {
                let msg = panic_message(payload);
                let fallback = eb.fallback.invoke(&msg);
                let id = self.mount(&fallback);
                if let Some(id) = id {
                    self.error_boundary_fallbacks.insert(id);
                }
                id
            }
        }
    }

    pub fn update_error_boundary(
        &mut self,
        old: &ErrorBoundaryElement,
        new: &ErrorBoundaryElement,
        id: ControlId,
    ) -> Option<ControlId> {
        if self.error_boundary_fallbacks.remove(&id) {
            self.unmount(id);
            let result = std::panic::catch_unwind(AssertUnwindSafe(|| self.mount(&new.child)));
            return match result {
                Ok(nid) => nid,
                Err(payload) => {
                    let msg = panic_message(payload);
                    let fallback = new.fallback.invoke(&msg);
                    let nid = self.mount(&fallback);
                    if let Some(nid) = nid {
                        self.error_boundary_fallbacks.insert(nid);
                    }
                    nid
                }
            };
        }

        let result =
            std::panic::catch_unwind(AssertUnwindSafe(|| self.update(&old.child, &new.child, id)));
        match result {
            Ok(nid) => nid,
            Err(payload) => {
                let msg = panic_message(payload);
                let fallback = new.fallback.invoke(&msg);
                self.unmount(id);
                let nid = self.mount(&fallback);
                if let Some(nid) = nid {
                    self.error_boundary_fallbacks.insert(nid);
                }
                nid
            }
        }
    }

    pub fn mount_provider(&mut self, p: &ProviderElement) -> Option<ControlId> {
        let pushed = self.push_provisions(&p.provisions);
        let result = std::panic::catch_unwind(AssertUnwindSafe(|| self.mount(&p.child)));
        self.pop_provisions(pushed);
        match result {
            Ok(id) => id,
            Err(payload) => std::panic::resume_unwind(payload),
        }
    }

    pub fn update_provider(
        &mut self,
        old: &ProviderElement,
        new: &ProviderElement,
        id: ControlId,
    ) -> Option<ControlId> {
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
                Some(old_p) => {
                    if old_p != new_p {
                        changed_ids.insert(new_p.context_id);
                    }
                }
            }
        }

        for old_id in old_by_id.into_keys() {
            changed_ids.insert(old_id);
        }

        let prev_flag = self.force_component_rerender;
        let saved_forced = self.forced_components.clone();
        if !changed_ids.is_empty() {
            let affected = self.collect_affected_components(id, &changed_ids);
            if !affected.is_empty() {
                self.force_component_rerender = true;
                self.forced_components.extend(affected);
            }
        }

        let pushed = self.push_provisions(&new.provisions);
        let result =
            std::panic::catch_unwind(AssertUnwindSafe(|| self.update(&old.child, &new.child, id)));
        self.pop_provisions(pushed);

        self.forced_components = saved_forced;
        self.force_component_rerender = prev_flag;

        match result {
            Ok(nid) => nid,
            Err(payload) => std::panic::resume_unwind(payload),
        }
    }

    fn push_provisions(&self, provisions: &[ContextProvision]) -> usize {
        for p in provisions {
            self.context_stack
                .push_raw_retain(p.context_id, p.value_type_id, Rc::clone(&p.value));
        }
        provisions.len()
    }

    fn pop_provisions(&self, count: usize) {
        for _ in 0..count {
            self.context_stack.pop_raw();
        }
    }

    pub fn mount_custom(&mut self, c: &CustomElementHandle) -> ControlId {
        self.debug_ui_elements_created += 1;
        let id = c.0.mount(&mut self.backend);
        self.custom_handles.insert(id, c.0.clone_dyn());
        id
    }

    pub fn update_custom(
        &mut self,
        old: &CustomElementHandle,
        new: &CustomElementHandle,
        id: ControlId,
    ) -> ControlId {
        debug_assert_eq!(
            CustomElement::type_id(&*old.0),
            CustomElement::type_id(&*new.0),
            "update_custom invoked across different CustomElement types — \
             reconciler should have unmount+remount via kind_matches",
        );
        if !old.0.eq_dyn(&*new.0) {
            new.0.update(&*old.0, id, &mut self.backend);
            self.custom_handles.insert(id, new.0.clone_dyn());
        }
        id
    }
}
