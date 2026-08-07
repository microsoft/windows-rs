use std::cell::RefCell;
use std::rc::Rc;

use super::*;

pub struct TemplatedListState {
    pub element: TemplatedListElement,
    pub rows: Vec<Option<RealizedRow>>,
}

pub struct RealizedRow {
    pub rendered: Element,
    pub content_id: ControlId,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RealizationRequest {
    Realize { list_id: ControlId, row_idx: usize },
    Recycle { list_id: ControlId, row_idx: usize },
}

pub type RealizationQueue = Rc<RefCell<Vec<RealizationRequest>>>;

pub fn new_realization_queue() -> RealizationQueue {
    Rc::new(RefCell::new(Vec::new()))
}

impl<B: Backend + 'static> Reconciler<B> {
    pub fn mount_templated_list(&mut self, tl: &TemplatedListElement) -> ControlId {
        let kind = match tl.kind {
            TemplatedKind::ListView => ControlKind::ListView,
            TemplatedKind::GridView => ControlKind::GridView,
            TemplatedKind::FlipView => ControlKind::FlipView,
        };
        let id = self.acquire_control(kind);
        self.apply_modifiers(id, &tl.modifiers);

        let count = tl.item_count();
        let mut rows: Vec<Option<RealizedRow>> = Vec::with_capacity(count);
        rows.resize_with(count, || None);
        self.templated_lists.insert(
            id,
            TemplatedListState {
                element: tl.clone(),
                rows,
            },
        );

        if tl.has_selection_handler() {
            let trampoline = self.selection_callbacks.entry(id).or_default().clone();
            *trampoline.borrow_mut() = tl.raw_selection_callback();
            let trampoline_c = Rc::clone(&trampoline);
            let cb = Callback::new(move |idx: i32| {
                if let Some(inner) = trampoline_c.borrow().as_ref() {
                    inner.invoke(idx);
                }
            });
            self.backend.attach_templated_selection_changed(id, cb);
        }

        if tl.has_reorder_handler() {
            let trampoline = self.reorder_callbacks.entry(id).or_default().clone();
            *trampoline.borrow_mut() = tl.raw_reorder_callback();
            let trampoline_c = Rc::clone(&trampoline);
            let cb = Callback::new(move |order: Vec<usize>| {
                if let Some(inner) = trampoline_c.borrow().as_ref() {
                    inner.invoke(order);
                }
            });
            self.backend.attach_templated_reorder(id, cb);
        }

        // WinUI container-recycling events (ContainerContentChanging) fire
        // during scrolling, outside any render pass. The realize/recycle
        // closures enqueue work and then ask the host to render, so the queue
        // is drained on the next UI-thread frame. `request_render` coalesces,
        // so a burst of scroll events costs about one reconcile per frame.
        let queue_r = Rc::clone(&self.realization_queue);
        let queue_c = Rc::clone(&self.realization_queue);
        let rerender_r = Rc::clone(&self.host.request_rerender);
        let rerender_c = Rc::clone(&self.host.request_rerender);
        let list_id = id;
        let realize: Rc<dyn Fn(usize)> = Rc::new(move |row_idx: usize| {
            queue_r
                .borrow_mut()
                .push(RealizationRequest::Realize { list_id, row_idx });
            (rerender_r)();
        });
        let recycle: Rc<dyn Fn(usize)> = Rc::new(move |row_idx: usize| {
            queue_c
                .borrow_mut()
                .push(RealizationRequest::Recycle { list_id, row_idx });
            (rerender_c)();
        });
        self.backend
            .attach_templated_realization(id, realize, recycle);

        self.backend.set_templated_item_count(id, count);
        self.backend
            .set_templated_selection_mode(id, tl.selection_mode);
        if tl.can_drag_items {
            self.backend.set_templated_can_drag_items(id, true);
        }
        if tl.can_reorder_items {
            self.backend.set_templated_can_reorder_items(id, true);
        }
        if tl.allow_drop {
            self.backend.set_templated_allow_drop(id, true);
        }
        let sel = tl.selected_index();
        if sel >= 0 {
            self.backend.set_templated_selected_index(id, sel);
        }

        // FlipView is not a ListViewBase and has no container-recycling
        // events, so it can't self-virtualize. Realize every row up front; it
        // shows one item at a time, so the working set stays tiny anyway.
        if matches!(tl.kind, TemplatedKind::FlipView) {
            let mut q = self.realization_queue.borrow_mut();
            for row_idx in 0..count {
                q.push(RealizationRequest::Realize {
                    list_id: id,
                    row_idx,
                });
            }
        }

        id
    }

    pub fn update_templated_list(
        &mut self,
        old: &TemplatedListElement,
        new: &TemplatedListElement,
        id: ControlId,
    ) {
        self.diff_modifiers(id, &old.modifiers, &new.modifiers);

        if let Some(state) = self.templated_lists.get_mut(&id) {
            state.element = new.clone();
        }

        if let Some(cell) = self.selection_callbacks.get(&id) {
            *cell.borrow_mut() = new.raw_selection_callback();
        }

        if let Some(cell) = self.reorder_callbacks.get(&id) {
            *cell.borrow_mut() = new.raw_reorder_callback();
        }

        if old.selected_index() != new.selected_index() {
            self.backend
                .set_templated_selected_index(id, new.selected_index());
        }

        if old.selection_mode != new.selection_mode {
            self.backend
                .set_templated_selection_mode(id, new.selection_mode);
        }

        if old.can_drag_items != new.can_drag_items {
            self.backend
                .set_templated_can_drag_items(id, new.can_drag_items);
        }

        if old.can_reorder_items != new.can_reorder_items {
            self.backend
                .set_templated_can_reorder_items(id, new.can_reorder_items);
        }

        if old.allow_drop != new.allow_drop {
            self.backend.set_templated_allow_drop(id, new.allow_drop);
        }

        let old_count = old.item_count();
        let new_count = new.item_count();
        if old_count != new_count {
            let to_unmount: Vec<(usize, ControlId)> = {
                let state = self.templated_lists.get_mut(&id).unwrap();
                let mut out = Vec::new();
                if new_count < state.rows.len() {
                    for (i, row) in state.rows.drain(new_count..).enumerate() {
                        if let Some(row) = row {
                            out.push((new_count + i, row.content_id));
                        }
                    }
                }
                state.rows.resize_with(new_count, || None);
                out
            };
            for (row_idx, cid) in to_unmount {
                self.backend.set_templated_row_content(id, row_idx, None);
                self.tree.clear_parent(cid, id);
                self.unmount(cid);
            }
            self.backend.set_templated_item_count(id, new_count);

            // FlipView doesn't self-virtualize, so realize any rows added by
            // the growth (ListView/GridView get these from WinUI recycling).
            if matches!(new.kind, TemplatedKind::FlipView) && new_count > old_count {
                let mut q = self.realization_queue.borrow_mut();
                for row_idx in old_count..new_count {
                    q.push(RealizationRequest::Realize {
                        list_id: id,
                        row_idx,
                    });
                }
            }
        }

        if !old.same_items_as(new) && !self.remap_keyed_realized_rows(id, old, new) {
            self.refresh_realized_rows(id, new);
        }
    }

    /// Preserves realized row controls across an equal-length keyed reorder.
    ///
    /// The backend's item source remains an identity vector of native slots. Moving a logical item
    /// therefore means detaching its realized control from the old slot and attaching it to the
    /// slot where the same key now appears. Count changes stay on the positional path because they
    /// can synchronously change WinUI realization while the item source is being resized.
    fn remap_keyed_realized_rows(
        &mut self,
        id: ControlId,
        old: &TemplatedListElement,
        new: &TemplatedListElement,
    ) -> bool {
        let count = old.item_count();
        if count != new.item_count() || matches!(new.kind, TemplatedKind::FlipView) {
            return false;
        }

        let realized_indices: Vec<usize> = {
            let state = self.templated_lists.get(&id).unwrap();
            state
                .rows
                .iter()
                .enumerate()
                .filter_map(|(idx, row)| row.as_ref().map(|_| idx))
                .collect()
        };
        if realized_indices.is_empty() {
            return false;
        }

        // Content-only updates are common and should remain proportional to the realized window.
        // A full key map is only needed when a visible slot actually changed identity.
        let visible_order_changed = realized_indices.iter().copied().any(|idx| {
            let old_key = old.item_key(idx);
            let new_key = new.item_key(idx);
            old_key.is_none() || new_key.is_none() || old_key != new_key
        });
        if !visible_order_changed {
            return false;
        }

        let mut realized_slots = vec![false; count];
        for idx in realized_indices {
            realized_slots[idx] = true;
        }

        let mut new_indices = FxHashMap::default();
        for new_idx in 0..count {
            let Some(key) = new.item_key(new_idx) else {
                return false;
            };
            if new_indices.insert(key, new_idx).is_some() {
                return false;
            }
        }

        let mut old_keys = rustc_hash::FxHashSet::default();
        let mut old_to_new = Vec::with_capacity(count);
        let mut order_changed = false;
        for old_idx in 0..count {
            let Some(key) = old.item_key(old_idx) else {
                return false;
            };
            let Some(new_idx) = new_indices.get(&key).copied() else {
                return false;
            };
            if !old_keys.insert(key) {
                return false;
            }
            order_changed |= old_idx != new_idx;
            old_to_new.push(new_idx);
        }
        if !order_changed {
            return false;
        }

        let old_rows = {
            let state = self.templated_lists.get_mut(&id).unwrap();
            std::mem::take(&mut state.rows)
        };
        let mut new_rows: Vec<Option<RealizedRow>> = Vec::with_capacity(count);
        new_rows.resize_with(count, || None);
        let mut moved_into = vec![false; count];
        let mut dropped = Vec::new();

        for (old_idx, row) in old_rows.into_iter().enumerate() {
            let Some(row) = row else { continue };
            let new_idx = old_to_new[old_idx];
            if realized_slots[new_idx] {
                new_rows[new_idx] = Some(row);
                moved_into[new_idx] = old_idx != new_idx;
            } else {
                dropped.push(row.content_id);
            }
        }

        for (row_idx, realized) in realized_slots.iter().copied().enumerate() {
            if realized && old_to_new[row_idx] != row_idx {
                self.backend.set_templated_row_content(id, row_idx, None);
            }
        }
        for content_id in dropped {
            self.dispatch_disappeared(content_id);
            self.tree.clear_parent(content_id, id);
            self.unmount(content_id);
        }

        if let Some(state) = self.templated_lists.get_mut(&id) {
            state.rows = new_rows;
        }

        for (row_idx, realized) in realized_slots.into_iter().enumerate() {
            if !realized {
                continue;
            }
            let existing = self
                .templated_lists
                .get_mut(&id)
                .and_then(|state| state.rows[row_idx].take());
            let new_el = new.build_item_view(row_idx);

            if let Some(row) = existing {
                let new_id = self.update(&row.rendered, &new_el, row.content_id);
                if let Some(content_id) = new_id {
                    self.tree.set_parent(content_id, id);
                    if moved_into[row_idx] || content_id != row.content_id {
                        self.backend
                            .set_templated_row_content(id, row_idx, Some(content_id));
                    }
                    if let Some(state) = self.templated_lists.get_mut(&id) {
                        state.rows[row_idx] = Some(RealizedRow {
                            rendered: new_el,
                            content_id,
                        });
                    }
                } else {
                    self.backend.set_templated_row_content(id, row_idx, None);
                }
            } else if let Some(content_id) = self.mount(&new_el) {
                self.tree.set_parent(content_id, id);
                self.backend
                    .set_templated_row_content(id, row_idx, Some(content_id));
                if let Some(state) = self.templated_lists.get_mut(&id) {
                    state.rows[row_idx] = Some(RealizedRow {
                        rendered: new_el,
                        content_id,
                    });
                }
                self.dispatch_appeared(content_id);
            }
        }

        true
    }

    fn refresh_realized_rows(&mut self, id: ControlId, new: &TemplatedListElement) {
        let realized_indices: Vec<usize> = {
            let state = self.templated_lists.get(&id).unwrap();
            state
                .rows
                .iter()
                .enumerate()
                .filter_map(|(i, r)| r.as_ref().map(|_| i))
                .collect()
        };

        for row_idx in realized_indices {
            if row_idx >= new.item_count() {
                continue;
            }
            let (old_el, content_id) = {
                let state = self.templated_lists.get(&id).unwrap();
                let row = state.rows[row_idx].as_ref().unwrap();
                (row.rendered.clone(), row.content_id)
            };
            let new_el = new.build_item_view(row_idx);

            if can_skip_update(&old_el, &new_el) && !self.is_control_forced(content_id) {
                self.debug_elements_skipped += 1;
                if let Some(state) = self.templated_lists.get_mut(&id)
                    && let Some(Some(row)) = state.rows.get_mut(row_idx)
                {
                    row.rendered = new_el;
                }
                continue;
            }

            let new_id = self.update(&old_el, &new_el, content_id);
            if let Some(nid) = new_id {
                self.tree.set_parent(nid, id);
                if let Some(state) = self.templated_lists.get_mut(&id)
                    && let Some(slot) = state.rows.get_mut(row_idx)
                {
                    *slot = Some(RealizedRow {
                        rendered: new_el,
                        content_id: nid,
                    });
                }
                if nid != content_id {
                    self.backend
                        .set_templated_row_content(id, row_idx, Some(nid));
                }
            } else {
                if let Some(state) = self.templated_lists.get_mut(&id)
                    && let Some(slot) = state.rows.get_mut(row_idx)
                {
                    *slot = None;
                }
                self.backend.set_templated_row_content(id, row_idx, None);
            }
        }
    }

    pub fn drain_realizations(&mut self) {
        let drained = {
            let mut q = self.realization_queue.borrow_mut();
            std::mem::take(&mut *q)
        };
        for req in drained {
            match req {
                RealizationRequest::Realize { list_id, row_idx } => {
                    self.realize_row_inner(list_id, row_idx);
                }
                RealizationRequest::Recycle { list_id, row_idx } => {
                    self.recycle_row_inner(list_id, row_idx);
                }
            }
        }
    }

    fn realize_row_inner(&mut self, list_id: ControlId, row_idx: usize) {
        let (rendered, count) = {
            let Some(state) = self.templated_lists.get(&list_id) else {
                return;
            };
            if row_idx >= state.element.item_count() {
                return;
            }
            (
                state.element.build_item_view(row_idx),
                state.element.item_count(),
            )
        };

        if let Some(existing) = self.clear_row_realized_state(list_id, row_idx) {
            self.unmount(existing);
        }

        let Some(content_id) = self.mount(&rendered) else {
            return;
        };

        self.tree.set_parent(content_id, list_id);
        self.backend
            .set_templated_row_content(list_id, row_idx, Some(content_id));
        if let Some(state) = self.templated_lists.get_mut(&list_id) {
            if state.rows.len() < count {
                state.rows.resize_with(count, || None);
            }
            state.rows[row_idx] = Some(RealizedRow {
                rendered,
                content_id,
            });
        }

        self.dispatch_appeared(content_id);
    }

    fn recycle_row_inner(&mut self, list_id: ControlId, row_idx: usize) {
        let Some(existing) = self.clear_row_realized_state(list_id, row_idx) else {
            return;
        };

        self.dispatch_disappeared(existing);
        self.backend
            .set_templated_row_content(list_id, row_idx, None);
        if self.defer_templated_unmounts {
            self.deferred_unmounts.push(existing);
        } else {
            self.unmount(existing);
        }
    }

    fn clear_row_realized_state(
        &mut self,
        list_id: ControlId,
        row_idx: usize,
    ) -> Option<ControlId> {
        let state = self.templated_lists.get_mut(&list_id)?;
        let row = state.rows.get_mut(row_idx)?;
        let content_id = row.take().map(|r| r.content_id)?;
        self.tree.clear_parent(content_id, list_id);
        Some(content_id)
    }

    fn dispatch_appeared(&mut self, id: ControlId) {
        if self.appeared_listener_count == 0 {
            return;
        }
        let subtree = self.collect_subtree_ids(id);
        for node in subtree {
            if let Some(node_ids) = self.components_by_control.get(&node) {
                for node_id in node_ids.as_slice().iter().rev() {
                    if let Some(inst) = self.component_instances.get_mut(node_id)
                        && inst.last_obj.has_on_appeared()
                    {
                        inst.render_cx
                            .set_context_stack(Rc::clone(&self.host.context_stack));
                        inst.last_obj.invoke_appeared(&mut inst.render_cx);
                    }
                }
            }
        }
    }

    fn dispatch_disappeared(&mut self, id: ControlId) {
        if self.disappeared_listener_count == 0 {
            return;
        }
        let subtree = self.collect_subtree_ids(id);
        for node in subtree {
            if let Some(node_ids) = self.components_by_control.get(&node) {
                for node_id in node_ids.as_slice() {
                    if let Some(inst) = self.component_instances.get_mut(node_id)
                        && inst.last_obj.has_on_disappeared()
                    {
                        inst.render_cx
                            .set_context_stack(Rc::clone(&self.host.context_stack));
                        inst.last_obj.invoke_disappeared(&mut inst.render_cx);
                    }
                }
            }
        }
    }

    fn collect_subtree_ids(&self, id: ControlId) -> Vec<ControlId> {
        let mut out = Vec::new();
        let mut stack = vec![id];
        while let Some(node) = stack.pop() {
            out.push(node);
            if let Some(children) = self.tree.children.get(&node) {
                for child in children.iter().rev() {
                    stack.push(*child);
                }
            }
        }
        out
    }
}
