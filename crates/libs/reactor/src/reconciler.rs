use std::cell::{Cell, RefCell};
use std::rc::Rc;

use rustc_hash::FxHashMap;

pub use super::*;

mod child;
mod diff_helpers;
mod templated;
mod widget_dispatch;
mod wrappers;

pub use self::child::compute_lis;
pub use self::templated::TemplatedListState;
pub use self::templated::{RealizationQueue, RealizationRequest, new_realization_queue};

/// Diff/apply engine that drives a [`Backend`] from successive
/// [`Element`] trees. Owns the bookkeeping needed to reuse controls
/// across renders (children mirror, component instances, custom
/// handles, templated-list state, …).
pub struct Reconciler<B: Backend> {
    pub backend: B,
    pub debug_elements_skipped: u64,
    pub debug_elements_diffed: u64,
    pub debug_ui_elements_created: u64,
    pub children_mirror: FxHashMap<ControlId, Vec<ControlId>>,
    pub id_kinds: FxHashMap<ControlId, ControlKind>,
    pub context_stack: Rc<ContextStack>,
    pub component_instances: FxHashMap<ControlId, ComponentInstance>,
    pub appeared_listener_count: usize,
    pub disappeared_listener_count: usize,
    pub force_component_rerender: bool,
    pub forced_components: rustc_hash::FxHashSet<ControlId>,
    pub error_boundary_fallbacks: rustc_hash::FxHashSet<ControlId>,
    pub templated_lists: FxHashMap<ControlId, TemplatedListState>,
    pub custom_handles: FxHashMap<ControlId, Box<dyn CustomElement>>,
    pub defer_templated_unmounts: bool,
    pub deferred_unmounts: Vec<ControlId>,
    pub realization_queue: RealizationQueue,
    pub selection_callbacks: FxHashMap<ControlId, Rc<RefCell<Option<Callback<i32>>>>>,
    pub reorder_callbacks: FxHashMap<ControlId, Rc<RefCell<Option<Callback<Vec<usize>>>>>>,
    /// Pre-unmount callbacks keyed by control id, invoked with the native
    /// element (or `None`) just before the control is destroyed (see
    /// [`Widget::on_unmounted_callback`]).
    pub unmount_callbacks: FxHashMap<ControlId, Callback<Option<windows_core::IInspectable>>>,
    /// Tracks header element control IDs for widgets that use header_element().
    pub header_elements: FxHashMap<ControlId, ControlId>,
    /// Tracks pane element control IDs for widgets that use pane_element().
    pub pane_elements: FxHashMap<ControlId, ControlId>,
    /// UI marshaller propagated into every nested component's
    /// [`RenderCx`] for [`RenderCx::use_async_state`].
    pub marshaller: Option<UiMarshaller>,
    pub inner_size: Rc<Cell<WindowSize>>,
    pub dpi: Rc<Cell<u32>>,
    /// Rerender hook propagated from the render host; child components
    /// clone this into their `RenderCx` so `SetState` triggers re-render.
    pub request_rerender: Rc<dyn Fn()>,
}

pub struct ComponentInstance {
    pub render_cx: RenderCx,
    pub last_rendered: Element,
    pub last_obj: Rc<dyn ComponentObject>,
    pub read_contexts: rustc_hash::FxHashSet<ContextId>,
}

impl<B: Backend + 'static> Reconciler<B> {
    pub fn new(backend: B) -> Self {
        Self {
            backend,
            debug_elements_skipped: 0,
            debug_elements_diffed: 0,
            debug_ui_elements_created: 0,
            children_mirror: FxHashMap::default(),
            id_kinds: FxHashMap::default(),
            context_stack: Rc::new(ContextStack::new()),
            component_instances: FxHashMap::default(),
            appeared_listener_count: 0,
            disappeared_listener_count: 0,
            force_component_rerender: false,
            forced_components: rustc_hash::FxHashSet::default(),
            error_boundary_fallbacks: rustc_hash::FxHashSet::default(),
            templated_lists: FxHashMap::default(),
            custom_handles: FxHashMap::default(),
            realization_queue: new_realization_queue(),
            selection_callbacks: FxHashMap::default(),
            reorder_callbacks: FxHashMap::default(),
            unmount_callbacks: FxHashMap::default(),
            header_elements: FxHashMap::default(),
            pane_elements: FxHashMap::default(),
            defer_templated_unmounts: false,
            deferred_unmounts: Vec::new(),
            marshaller: None,
            inner_size: Rc::new(Cell::new(WindowSize::default())),
            dpi: Rc::new(Cell::new(96_u32)),
            request_rerender: Rc::new(|| {}),
        }
    }

    /// Install the [`UiMarshaller`] propagated into nested
    /// [`RenderCx`] instances.
    pub fn set_marshaller(&mut self, marshaller: Option<UiMarshaller>) {
        self.marshaller = marshaller;
    }

    #[cfg(feature = "test")]
    pub fn flush_deferred_unmounts(&mut self) {
        let drained = std::mem::take(&mut self.deferred_unmounts);
        for cid in drained {
            self.unmount(cid);
        }
    }

    pub fn context_stack_handle(&self) -> Rc<ContextStack> {
        Rc::clone(&self.context_stack)
    }

    /// Returns `true` if the component instance at `id` has had state
    /// modified since its last render (i.e. a `SetState`/`Updater`/`Dispatch`
    /// fired). Used by child reconciliation to bypass `can_skip_update`.
    /// Does NOT clear the flag — `update_component` consumes it.
    pub fn is_component_state_dirty(&self, id: ControlId) -> bool {
        self.component_instances
            .get(&id)
            .is_some_and(|inst| inst.render_cx.peek_state_dirty())
    }

    pub fn reset_stats(&mut self) {
        self.debug_elements_skipped = 0;
        self.debug_elements_diffed = 0;
        self.debug_ui_elements_created = 0;
    }

    #[cfg(feature = "test")]
    pub fn debug_forced_components_len(&self) -> usize {
        self.forced_components.len()
    }

    pub fn acquire_control(&mut self, kind: ControlKind) -> ControlId {
        self.debug_ui_elements_created += 1;
        let id = self.backend.create(kind);

        if let Some(stale) = self.component_instances.remove(&id) {
            self.unregister_component_listeners(&stale);
        }
        self.templated_lists.remove(&id);
        self.selection_callbacks.remove(&id);
        self.reorder_callbacks.remove(&id);
        self.error_boundary_fallbacks.remove(&id);
        self.children_mirror.remove(&id);
        self.custom_handles.remove(&id);
        self.id_kinds.insert(id, kind);
        id
    }

    pub fn register_component_instance(
        &mut self,
        id: ControlId,
        inst: ComponentInstance,
    ) -> Option<ComponentInstance> {
        if inst.last_obj.has_on_appeared() {
            self.appeared_listener_count += 1;
        }
        if inst.last_obj.has_on_disappeared() {
            self.disappeared_listener_count += 1;
        }
        let displaced = self.component_instances.insert(id, inst);
        if let Some(prev) = &displaced {
            self.unregister_component_listeners(prev);
        }
        displaced
    }

    pub fn take_component_instance(&mut self, id: ControlId) -> Option<ComponentInstance> {
        let inst = self.component_instances.remove(&id)?;
        self.unregister_component_listeners(&inst);
        Some(inst)
    }

    fn unregister_component_listeners(&mut self, inst: &ComponentInstance) {
        if inst.last_obj.has_on_appeared() {
            debug_assert!(
                self.appeared_listener_count > 0,
                "appeared_listener_count underflow: register/take are mismatched"
            );
            self.appeared_listener_count -= 1;
        }
        if inst.last_obj.has_on_disappeared() {
            debug_assert!(
                self.disappeared_listener_count > 0,
                "disappeared_listener_count underflow: register/take are mismatched"
            );
            self.disappeared_listener_count -= 1;
        }
    }

    #[cfg(feature = "test")]
    pub fn debug_appeared_listener_count(&self) -> usize {
        self.appeared_listener_count
    }

    #[cfg(feature = "test")]
    pub fn debug_disappeared_listener_count(&self) -> usize {
        self.disappeared_listener_count
    }

    pub fn reconcile(
        &mut self,
        old: Option<&Element>,
        new: &Element,
        existing: Option<ControlId>,
        request_rerender: Rc<dyn Fn()>,
    ) -> Option<ControlId> {
        self.request_rerender = request_rerender;
        match (existing, old) {
            (None, _) => self.mount(new),
            (Some(id), Some(old_el)) => {
                let seeded = self.force_state_dirty_components();
                let result = self.update(old_el, new, id);
                debug_assert!(
                    seeded
                        .iter()
                        .all(|cid| !self.is_component_state_dirty(*cid)),
                    "a state-dirty component was not re-rendered by the pass"
                );
                result
            }
            (Some(_id), None) => self.mount(new),
        }
    }

    /// Seed [`Self::forced_components`] with every mounted component instance
    /// whose own `use_state` was written since the last pass, and enable
    /// [`Self::force_component_rerender`] so `can_skip_update` pruning cannot
    /// drop a dirty component nested under unchanged parents. Returns the seeded
    /// ids. Mirrors [`Self::force_context_subscribers`] for plain state writes.
    fn force_state_dirty_components(&mut self) -> Vec<ControlId> {
        let dirty: Vec<ControlId> = self
            .component_instances
            .iter()
            .filter(|(_, inst)| inst.render_cx.peek_state_dirty())
            .map(|(id, _)| *id)
            .collect();
        if !dirty.is_empty() {
            self.force_component_rerender = true;
            self.forced_components.extend(dirty.iter().copied());
        }
        dirty
    }

    pub fn mount(&mut self, el: &Element) -> Option<ControlId> {
        match el {
            Element::Component(ce) => return self.mount_component(ce),
            Element::ErrorBoundary(eb) => return self.mount_error_boundary(eb),
            Element::Provider(pe) => return self.mount_provider(pe),
            Element::TemplatedList(tl) => return Some(self.mount_templated_list(tl)),
            Element::Custom(c) => return Some(self.mount_custom(c)),
            Element::Group(_) => {
                panic!(
                    "Element::Group can only appear inside a multi-child container's child list. \
                     A Group at a single-child position (e.g. as a Component's render output, \
                     or as the sole child of Border / ScrollViewer) is not supported. \
                     Wrap the Group in a StackPanel, return its single non-empty child, or place it \
                     directly inside a StackPanel/Grid that owns it as one of many children."
                );
            }
            Element::Empty => return None,
            _ => {}
        }
        let widget = el.as_widget().unwrap();
        let id = self.mount_widget(widget);
        if let Element::RichTextBlock(rt) = el
            && !rt.paragraphs.is_empty()
        {
            self.backend.set_rich_text_paragraphs(id, &rt.paragraphs);
        }
        Some(id)
    }

    pub fn update(&mut self, old: &Element, new: &Element, id: ControlId) -> Option<ControlId> {
        if !self.force_component_rerender && can_skip_update(old, new) {
            // A component with dirty internal state must still be re-rendered
            // even when its element is structurally identical.
            if !self.is_component_state_dirty(id) {
                self.debug_elements_skipped += 1;
                return Some(id);
            }
        }
        self.debug_elements_diffed += 1;

        if !old.kind_matches(new) {
            self.unmount(id);
            return self.mount(new);
        }

        match (old, new) {
            (Element::Component(o), Element::Component(n)) => {
                return self.update_component(o, n, id);
            }
            (Element::ErrorBoundary(o), Element::ErrorBoundary(n)) => {
                return self.update_error_boundary(o, n, id);
            }
            (Element::Provider(o), Element::Provider(n)) => return self.update_provider(o, n, id),
            (Element::TemplatedList(o), Element::TemplatedList(n)) => {
                self.update_templated_list(o, n, id);
                return Some(id);
            }
            (Element::Custom(o), Element::Custom(n)) => {
                return Some(self.update_custom(o, n, id));
            }
            (Element::Group(_), Element::Group(_)) => {
                panic!(
                    "Element::Group reached update() with a ControlId. \
                     Group is a fragment and cannot own a control. \
                     This usually means a Group was placed at a single-child \
                     position; see Element::Group docs."
                );
            }
            (Element::Empty, Element::Empty) => return None,
            _ => {}
        }

        let (Some(ow), Some(nw)) = (old.as_widget(), new.as_widget()) else {
            unreachable!("kind_matches guarantees same variant; non-widget variants handled above");
        };
        self.update_widget(ow, nw, id);
        if let (Element::RichTextBlock(o), Element::RichTextBlock(n)) = (old, new)
            && o.paragraphs != n.paragraphs
        {
            self.backend.set_rich_text_paragraphs(id, &n.paragraphs);
        }
        Some(id)
    }

    pub fn unmount(&mut self, id: ControlId) {
        let mut to_release = Vec::new();
        self.collect_subtree(id, &mut to_release);
        for node in to_release.into_iter().rev() {
            if let Some(mut inst) = self.take_component_instance(node) {
                inst.render_cx.run_cleanups();
            }

            if let Some(state) = self.templated_lists.remove(&node) {
                for row in state.rows.into_iter().flatten() {
                    self.unmount(row.content_id);
                }
            }

            // Unmount header/pane element subtrees that are tracked outside
            // children_mirror (mounted via Widget::header_element / pane_element).
            if let Some(hdr_id) = self.header_elements.remove(&node) {
                self.unmount(hdr_id);
            }
            if let Some(pane_id) = self.pane_elements.remove(&node) {
                self.unmount(pane_id);
            }

            self.selection_callbacks.remove(&node);
            self.reorder_callbacks.remove(&node);

            // Let the control tear down resources bound to it (e.g. join a
            // render thread) while the native control still exists, before it
            // is destroyed. The callback always runs when registered; the
            // native element is passed when available, else `None`.
            if let Some(cb) = self.unmount_callbacks.remove(&node) {
                cb.invoke(self.backend.get_native_element(node));
            }

            self.error_boundary_fallbacks.remove(&node);

            if let Some(handle) = self.custom_handles.remove(&node) {
                handle.before_destroy(node, &mut self.backend);
            }

            self.children_mirror.remove(&node);
            self.id_kinds.remove(&node);
            self.backend.destroy(node);
        }
    }

    fn collect_subtree(&self, id: ControlId, out: &mut Vec<ControlId>) {
        let mut stack = vec![id];
        while let Some(node) = stack.pop() {
            out.push(node);
            if let Some(children) = self.children_mirror.get(&node) {
                for child in children.iter().rev() {
                    stack.push(*child);
                }
            }
        }
    }

    pub fn append_child_tracked(&mut self, parent: ControlId, child: ControlId) {
        self.children_mirror.entry(parent).or_default().push(child);
        self.backend.append_child(parent, child);
    }

    pub fn remove_child_tracked(&mut self, parent: ControlId, index: usize) {
        if let Some(list) = self.children_mirror.get_mut(&parent)
            && index < list.len()
        {
            list.remove(index);
        }
        self.backend.remove_child(parent, index);
    }

    pub fn replace_child_tracked(&mut self, parent: ControlId, index: usize, new: ControlId) {
        if let Some(list) = self.children_mirror.get_mut(&parent)
            && index < list.len()
        {
            list[index] = new;
        }
        self.backend.replace_child(parent, index, new);
    }

    pub fn move_child_tracked(&mut self, parent: ControlId, from: usize, to: usize) {
        if from == to {
            return;
        }
        if let Some(list) = self.children_mirror.get_mut(&parent)
            && from < list.len()
            && to < list.len()
        {
            let item = list.remove(from);
            list.insert(to, item);
        }
        self.backend.move_child(parent, from, to);
    }

    pub fn insert_child_tracked(&mut self, parent: ControlId, index: usize, child: ControlId) {
        let list = self.children_mirror.entry(parent).or_default();
        let clamped = index.min(list.len());
        list.insert(clamped, child);
        self.backend.insert_child(parent, clamped, child);
    }

    pub fn child_at(&self, parent: ControlId, i: usize) -> Option<ControlId> {
        self.children_mirror
            .get(&parent)
            .and_then(|v| v.get(i).copied())
    }

    pub fn apply_modifiers(&mut self, id: ControlId, mods: &Modifiers) {
        if mods.is_empty() {
            return;
        }
        if let Some(v) = mods.margin {
            self.backend
                .set_prop(id, Prop::Margin, &PropValue::Thickness(v));
        }
        if let Some(v) = mods.padding {
            self.backend
                .set_prop(id, Prop::Padding, &PropValue::Thickness(v));
        }
        if let Some(v) = mods.width {
            self.backend.set_prop(id, Prop::Width, &PropValue::F64(v));
        }
        if let Some(v) = mods.height {
            self.backend.set_prop(id, Prop::Height, &PropValue::F64(v));
        }
        if let Some(v) = mods.min_width {
            self.backend
                .set_prop(id, Prop::MinWidth, &PropValue::F64(v));
        }
        if let Some(v) = mods.max_width {
            self.backend
                .set_prop(id, Prop::MaxWidth, &PropValue::F64(v));
        }
        if let Some(v) = mods.min_height {
            self.backend
                .set_prop(id, Prop::MinHeight, &PropValue::F64(v));
        }
        if let Some(v) = mods.max_height {
            self.backend
                .set_prop(id, Prop::MaxHeight, &PropValue::F64(v));
        }
        if let Some(v) = mods.horizontal_alignment {
            self.backend
                .set_prop(id, Prop::HorizontalAlignment, &PropValue::I32(v.0));
        }
        if let Some(v) = mods.vertical_alignment {
            self.backend
                .set_prop(id, Prop::VerticalAlignment, &PropValue::I32(v.0));
        }
        if let Some(v) = mods.opacity {
            self.backend.set_prop(id, Prop::Opacity, &PropValue::F64(v));
        }
        if let Some(v) = &mods.background {
            self.backend
                .set_prop(id, Prop::Background, &PropValue::Color(*v));
        }
        if let Some(v) = &mods.foreground {
            self.backend
                .set_prop(id, Prop::Foreground, &PropValue::Color(*v));
        }
        if let Some(v) = &mods.font_family {
            self.backend
                .set_prop(id, Prop::FontFamily, &PropValue::Str(v.clone()));
        }
        if let Some(v) = mods.font_size {
            self.backend
                .set_prop(id, Prop::FontSize, &PropValue::F64(v));
        }

        if let Some(v) = mods.allow_drop {
            self.backend
                .set_prop(id, Prop::AllowDrop, &PropValue::Bool(v));
        }

        self.apply_theme_bindings_for(id, mods);
        self.apply_animations_for(id, mods);
        self.apply_accessibility_for(id, mods);
        self.apply_keyboard_accelerators_for(id, mods);
        self.apply_tooltip_for(id, mods);
        self.apply_pointer_handlers_for(id, mods);
        self.apply_drag_handlers_for(id, mods);

        if let Some(p) = mods.grid {
            self.apply_grid_placement(id, p);
        }

        if !mods.resources.is_empty() {
            self.backend.set_prop(
                id,
                Prop::Resources,
                &PropValue::Resources(mods.resources.clone()),
            );
        }
    }

    fn apply_tooltip_for(&mut self, id: ControlId, mods: &Modifiers) {
        let Some(tt) = mods.tooltip.as_deref() else {
            return;
        };
        self.backend.set_tooltip(id, Some(tt));
    }

    fn apply_pointer_handlers_for(&mut self, id: ControlId, mods: &Modifiers) {
        let Some(ph) = mods.pointer_handlers.as_deref() else {
            return;
        };
        if ph.is_empty() {
            return;
        }
        self.backend.set_pointer_handlers(id, Some(ph));
    }

    fn apply_drag_handlers_for(&mut self, id: ControlId, mods: &Modifiers) {
        let Some(dh) = mods.drag_handlers.as_deref() else {
            return;
        };
        if dh.is_empty() {
            return;
        }
        self.backend.set_drag_handlers(id, Some(dh));
    }

    fn apply_accessibility_for(&mut self, id: ControlId, mods: &Modifiers) {
        let Some(acc) = mods.accessibility.as_deref() else {
            return;
        };
        if acc.is_empty() {
            return;
        }
        self.backend.set_accessibility(id, acc);
    }

    fn apply_keyboard_accelerators_for(&mut self, id: ControlId, mods: &Modifiers) {
        if mods.keyboard_accelerators.is_empty() {
            return;
        }
        self.backend
            .set_keyboard_accelerators(id, &mods.keyboard_accelerators);
    }

    fn apply_animations_for(&mut self, id: ControlId, mods: &Modifiers) {
        let Some(anim) = mods.animations.as_deref() else {
            return;
        };
        if anim.is_empty() {
            return;
        }
        if let Some(it) = anim.implicit_transitions
            && !it.is_empty()
        {
            self.backend.set_implicit_transitions(id, Some(it));
        }
        if let Some(la) = anim.layout_animation {
            self.backend.set_layout_animation(id, Some(la));
        }

        if let Some(p) = anim.property_animation {
            self.backend.run_property_animation(id, Some(p));
        } else if let Some(enter) = anim.enter_transition {
            self.backend.run_property_animation(id, Some(enter));
        }
    }

    fn diff_animations_for(
        &mut self,
        id: ControlId,
        old: Option<&AnimationModifiers>,
        new: Option<&AnimationModifiers>,
    ) {
        let old_it = old.and_then(|a| a.implicit_transitions);
        let new_it = new.and_then(|a| a.implicit_transitions);
        if old_it != new_it {
            self.backend
                .set_implicit_transitions(id, new_it.filter(|t| !t.is_empty()));
        }
        let old_la = old.and_then(|a| a.layout_animation);
        let new_la = new.and_then(|a| a.layout_animation);
        if old_la != new_la {
            self.backend.set_layout_animation(id, new_la);
        }

        let old_pa = old.and_then(|a| a.property_animation);
        let new_pa = new.and_then(|a| a.property_animation);
        if old_pa != new_pa {
            self.backend.run_property_animation(id, new_pa);
        }
    }

    fn apply_theme_bindings_for(&mut self, id: ControlId, mods: &Modifiers) {
        let Some(map) = mods.theme_bindings.as_deref() else {
            return;
        };
        if map.is_empty() {
            return;
        }
        let kind = match self.id_kinds.get(&id) {
            Some(k) => *k,
            None => return,
        };
        let bindings: Vec<(Prop, ThemeRef)> = map.iter().map(|(p, t)| (*p, t.clone())).collect();
        self.backend.set_theme_bindings(id, kind, &bindings);
    }

    pub fn diff_modifiers(&mut self, id: ControlId, old: &Modifiers, new: &Modifiers) {
        self.diff_opt_copy(
            id,
            Prop::Margin,
            old.margin,
            new.margin,
            PropValue::Thickness,
        );
        self.diff_opt_copy(
            id,
            Prop::Padding,
            old.padding,
            new.padding,
            PropValue::Thickness,
        );
        self.diff_opt_f64(id, Prop::Width, old.width, new.width);
        self.diff_opt_f64(id, Prop::Height, old.height, new.height);
        self.diff_opt_f64(id, Prop::MinWidth, old.min_width, new.min_width);
        self.diff_opt_f64(id, Prop::MaxWidth, old.max_width, new.max_width);
        self.diff_opt_f64(id, Prop::MinHeight, old.min_height, new.min_height);
        self.diff_opt_f64(id, Prop::MaxHeight, old.max_height, new.max_height);
        self.diff_opt_copy(
            id,
            Prop::HorizontalAlignment,
            old.horizontal_alignment,
            new.horizontal_alignment,
            |v: HorizontalAlignment| PropValue::I32(v.0),
        );
        self.diff_opt_copy(
            id,
            Prop::VerticalAlignment,
            old.vertical_alignment,
            new.vertical_alignment,
            |v: VerticalAlignment| PropValue::I32(v.0),
        );
        self.diff_opt_f64(id, Prop::Opacity, old.opacity, new.opacity);
        self.diff_opt_clone(
            id,
            Prop::Background,
            &old.background,
            &new.background,
            PropValue::Color,
        );
        self.diff_opt_clone(
            id,
            Prop::Foreground,
            &old.foreground,
            &new.foreground,
            PropValue::Color,
        );
        self.diff_opt_clone(
            id,
            Prop::FontFamily,
            &old.font_family,
            &new.font_family,
            PropValue::Str,
        );
        self.diff_opt_f64(id, Prop::FontSize, old.font_size, new.font_size);

        if old.theme_bindings != new.theme_bindings {
            let kind = self.id_kinds.get(&id).copied();
            if let Some(kind) = kind {
                let bindings: Vec<(Prop, ThemeRef)> = new
                    .theme_bindings
                    .as_deref()
                    .map(|m| m.iter().map(|(p, t)| (*p, t.clone())).collect())
                    .unwrap_or_default();
                self.backend.set_theme_bindings(id, kind, &bindings);
            }
        }

        let old_anim = old.animations.as_deref();
        let new_anim = new.animations.as_deref();
        if old_anim != new_anim {
            self.diff_animations_for(id, old_anim, new_anim);
        }

        let old_acc = old.accessibility.as_deref();
        let new_acc = new.accessibility.as_deref();
        if old_acc != new_acc {
            static EMPTY: AccessibilityModifiers = AccessibilityModifiers {
                automation_name: None,
                automation_id: None,
                help_text: None,
                live_setting: None,
                heading_level: None,
            };
            let new_acc = new_acc.unwrap_or(&EMPTY);
            self.backend.set_accessibility(id, new_acc);
        }

        let old_ka = &old.keyboard_accelerators;
        let new_ka = &new.keyboard_accelerators;
        if old_ka != new_ka {
            self.backend.set_keyboard_accelerators(id, new_ka);
        }

        // Tooltip is a `ToolTipService` attached property and survives
        // re-renders; emit a `None` clear on Some→None transitions.
        let old_tt = old.tooltip.as_deref();
        let new_tt = new.tooltip.as_deref();
        if old_tt != new_tt {
            self.backend.set_tooltip(id, new_tt);
        }

        // Pointer / tap handlers: clear on Some→None so we don't leak
        // previously-attached event tokens.
        let old_ph = old.pointer_handlers.as_deref();
        let new_ph = new.pointer_handlers.as_deref();
        if old_ph != new_ph {
            let new_ph = new_ph.filter(|p| !p.is_empty());
            self.backend.set_pointer_handlers(id, new_ph);
        }

        if old.allow_drop != new.allow_drop {
            self.backend.set_prop(
                id,
                Prop::AllowDrop,
                &PropValue::Bool(new.allow_drop.unwrap_or(false)),
            );
        }

        let old_dh = old.drag_handlers.as_deref();
        let new_dh = new.drag_handlers.as_deref();
        if old_dh != new_dh {
            let new_dh = new_dh.filter(|d| !d.is_empty());
            self.backend.set_drag_handlers(id, new_dh);
        }

        // Grid placement — fast path (avoids AttachedProps dynamic dispatch).
        // Always emit all four props on change so stale values are cleared.
        if old.grid != new.grid {
            self.apply_grid_placement_full(id, new.grid.unwrap_or_default());
        }

        if old.resources != new.resources && !new.resources.is_empty() {
            self.backend.set_prop(
                id,
                Prop::Resources,
                &PropValue::Resources(new.resources.clone()),
            );
        }
    }

    pub fn collect_affected_components(
        &self,
        root_id: ControlId,
        changed: &rustc_hash::FxHashSet<ContextId>,
    ) -> Vec<ControlId> {
        let mut result = Vec::new();
        let mut stack = vec![root_id];
        while let Some(id) = stack.pop() {
            if let Some(inst) = self.component_instances.get(&id)
                && inst.read_contexts.iter().any(|c| changed.contains(c))
            {
                result.push(id);
            }
            if let Some(kids) = self.children_mirror.get(&id) {
                for k in kids {
                    stack.push(*k);
                }
            }
        }
        result
    }

    pub fn notify_theme_changed(&mut self) {
        self.backend.on_theme_changed();
    }

    pub fn reconcile_children_positional(
        &mut self,
        parent: ControlId,
        old: &[Element],
        new: &[Element],
    ) {
        let old_live = LiveChildren::from_slice(old);
        let new_live = LiveChildren::from_slice(new);
        child::reconcile_positional(self, parent, old_live.as_ref(), new_live.as_ref());
    }

    pub fn reconcile_children(&mut self, parent: ControlId, old: &[Element], new: &[Element]) {
        let old_live = LiveChildren::from_slice(old);
        let new_live = LiveChildren::from_slice(new);
        child::reconcile(self, parent, old_live.as_ref(), new_live.as_ref());
    }

    pub fn set_inner_size_cell(&mut self, cell: Rc<Cell<WindowSize>>) {
        self.inner_size = cell;
    }

    pub fn set_dpi_cell(&mut self, cell: Rc<Cell<u32>>) {
        self.dpi = cell;
    }

    pub fn force_context_subscribers(
        &mut self,
        root_id: ControlId,
        context_ids: &rustc_hash::FxHashSet<ContextId>,
    ) {
        let affected = self.collect_affected_components(root_id, context_ids);
        if !affected.is_empty() {
            self.force_component_rerender = true;
            self.forced_components.extend(affected);
        }
    }

    pub fn clear_forced_component_rerender(&mut self) {
        self.force_component_rerender = false;
        self.forced_components.clear();
    }
}

/// A cow-like container that avoids allocating a `Vec<&Element>` when the
/// input slice has no `Empty` or `Group` elements (the common fast-path for
/// grids and most stack panels).
enum LiveChildren<'a> {
    /// All elements are live — borrow the original slice directly.
    Flat(&'a [Element]),
    /// Contains Empty/Group elements that need filtering.
    Filtered(Vec<&'a Element>),
}

impl<'a> LiveChildren<'a> {
    fn from_slice(slice: &'a [Element]) -> Self {
        let needs_filter = slice
            .iter()
            .any(|e| matches!(e, Element::Empty | Element::Group(_)));
        if needs_filter {
            LiveChildren::Filtered(collect_live(slice))
        } else {
            LiveChildren::Flat(slice)
        }
    }

    fn as_ref(&self) -> LiveChildrenRef<'_> {
        match self {
            LiveChildren::Flat(s) => LiveChildrenRef::Flat(s),
            LiveChildren::Filtered(v) => LiveChildrenRef::Filtered(v),
        }
    }
}

/// Borrowable view into either flat or filtered children, providing
/// slice-like access via `len`, `get`, and `iter`.
pub enum LiveChildrenRef<'a> {
    Flat(&'a [Element]),
    Filtered(&'a [&'a Element]),
}

impl<'a> LiveChildrenRef<'a> {
    pub fn len(&self) -> usize {
        match self {
            LiveChildrenRef::Flat(s) => s.len(),
            LiveChildrenRef::Filtered(v) => v.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, i: usize) -> Option<&'a Element> {
        match self {
            LiveChildrenRef::Flat(s) => s.get(i),
            LiveChildrenRef::Filtered(v) => v.get(i).copied(),
        }
    }

    pub fn any_has_key(&self) -> bool {
        match self {
            LiveChildrenRef::Flat(s) => s.iter().any(|e| e.key().is_some()),
            LiveChildrenRef::Filtered(v) => v.iter().any(|e| e.key().is_some()),
        }
    }
}

/// Flatten a child slice for reconciliation: drops `Element::Empty` and
/// recursively splices `Element::Group` children into the output.
pub fn collect_live(slice: &[Element]) -> Vec<&Element> {
    let mut out = Vec::with_capacity(slice.len());
    for el in slice {
        push_live(el, &mut out);
    }
    out
}

fn push_live<'a>(el: &'a Element, out: &mut Vec<&'a Element>) {
    match el {
        Element::Empty => {}
        Element::Group(g) => {
            for child in &g.children {
                push_live(child, out);
            }
        }
        other => out.push(other),
    }
}
