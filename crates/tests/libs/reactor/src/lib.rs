use std::collections::VecDeque;
use std::rc::Rc;

use windows_reactor::*;

/// Backend call that can be selected for deterministic fail-before-operation testing.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BackendOperation {
    Create,
    SetProp,
    AppendChild,
    RemoveChild,
    ReplaceChild,
    MoveChild,
    InsertChild,
    Destroy,
    AttachEvent,
    DetachEvent,
    SetTemplatedItemCount,
    SetTemplatedSelectedIndex,
    SetTemplatedSelectionMode,
    SetTemplatedCanDragItems,
    SetTemplatedCanReorderItems,
    SetTemplatedAllowDrop,
    AttachTemplatedSelectionChanged,
    AttachTemplatedRealization,
    AttachTemplatedReorder,
    SetHeaderElement,
    SetPaneElement,
}

struct BackendFailure {
    operation: BackendOperation,
    remaining: usize,
}

/// Recorded backend operation, used by [`RecordingBackend`] for tests.
#[derive(Clone, Debug)]
pub enum Op {
    Create {
        id: ControlId,
        kind: ControlKind,
    },
    SetProp {
        id: ControlId,
        prop: Prop,
        value: PropValue,
    },
    AppendChild {
        parent: ControlId,
        child: ControlId,
    },
    RemoveChild {
        parent: ControlId,
        index: usize,
    },
    ReplaceChild {
        parent: ControlId,
        index: usize,
        new: ControlId,
    },
    MoveChild {
        parent: ControlId,
        from: usize,
        to: usize,
    },
    InsertChild {
        parent: ControlId,
        index: usize,
        child: ControlId,
    },
    Destroy {
        id: ControlId,
    },
    AttachEvent {
        id: ControlId,
        event: Event,
        handler: EventHandler,
    },
    DetachEvent {
        id: ControlId,
        event: Event,
    },
    SetTemplatedItemCount {
        list_id: ControlId,
        count: usize,
    },
    MountRowContent {
        list_id: ControlId,
        row_idx: usize,
        content: ControlId,
    },
    ClearRowContent {
        list_id: ControlId,
        row_idx: usize,
    },
    SetTemplatedSelectedIndex {
        id: ControlId,
        index: i32,
    },
    SetTemplatedSelectionMode {
        id: ControlId,
        mode: SelectionMode,
    },
    SetTemplatedCanDragItems {
        id: ControlId,
        value: bool,
    },
    SetTemplatedCanReorderItems {
        id: ControlId,
        value: bool,
    },
    SetTemplatedAllowDrop {
        id: ControlId,
        value: bool,
    },
    SetHeaderElement {
        id: ControlId,
        header_id: Option<ControlId>,
    },
    SetPaneElement {
        id: ControlId,
        pane_id: Option<ControlId>,
    },
    ScrollTemplatedToIndex {
        id: ControlId,
        index: i32,
    },
    AttachTemplatedSelectionChanged {
        id: ControlId,
    },
    AttachTemplatedRealization {
        id: ControlId,
    },
    AttachTemplatedReorder {
        id: ControlId,
    },
    ApplyThemeBindings {
        id: ControlId,
        kind: ControlKind,
        bindings: Vec<(Prop, ThemeRef)>,
        cache_hit: bool,
    },
    OnThemeChanged,
    SetImplicitTransitions {
        id: ControlId,
        transitions: Option<ImplicitTransitions>,
    },
    SetLayoutAnimation {
        id: ControlId,
        config: Option<LayoutAnimationConfig>,
    },
    RunPropertyAnimation {
        id: ControlId,
        config: Option<AnimationConfig>,
    },
    SetElementTransitions {
        id: ControlId,
        enter: Option<AnimationConfig>,
        exit: Option<AnimationConfig>,
    },
    SetRichTextParagraphs {
        id: ControlId,
        paragraphs: Vec<RichTextParagraph>,
    },
    SetAccessibility {
        id: ControlId,
        accessibility: AccessibilityModifiers,
    },
    SetKeyboardAccelerators {
        id: ControlId,
        accelerators: Vec<KeyboardAccelerator>,
    },
    SetTooltip {
        id: ControlId,
        tooltip: Option<Tooltip>,
    },
    SetPointerHandlers {
        id: ControlId,
        handlers: Option<PointerHandlers>,
    },
}

/// In-memory [`Backend`] that records every operation as an [`Op`] and
/// can replay events back into the recorded handlers; used by the test
/// suite as the deterministic counterpart to `WinUIBackend`.
#[derive(Default)]
pub struct RecordingBackend {
    pub ops: Vec<Op>,
    next_id: u32,
    live_controls: rustc_hash::FxHashSet<ControlId>,
    children: rustc_hash::FxHashMap<ControlId, Vec<ControlId>>,
    headers: rustc_hash::FxHashMap<ControlId, ControlId>,
    panes: rustc_hash::FxHashMap<ControlId, ControlId>,
    handlers: rustc_hash::FxHashMap<(ControlId, Event), EventHandler>,
    row_contents: rustc_hash::FxHashMap<ControlId, rustc_hash::FxHashMap<usize, ControlId>>,
    item_counts: rustc_hash::FxHashMap<ControlId, usize>,
    realization_handlers: rustc_hash::FxHashMap<ControlId, (Rc<dyn Fn(usize)>, Rc<dyn Fn(usize)>)>,
    selection_handlers: rustc_hash::FxHashMap<ControlId, Callback<i32>>,
    reorder_handlers: rustc_hash::FxHashMap<ControlId, Callback<Vec<usize>>>,
    theme_style_cache: rustc_hash::FxHashSet<(ControlKind, Vec<(Prop, ThemeRef)>)>,
    theme_bindings_live: rustc_hash::FxHashMap<ControlId, Vec<(Prop, ThemeRef)>>,
    failures: VecDeque<BackendFailure>,
    /// A stand-in [`IInspectable`] fabricated for every control so that
    /// [`get_native_element`](Backend::get_native_element) returns `Some`,
    /// mirroring the real WinUI backend (which returns the live XAML element).
    /// This lets `on_mounted` / `on_unmounted` callbacks fire in tests. The
    /// entry is removed in [`destroy`](Backend::destroy), so a destroyed
    /// control reports `None` again.
    native_elements: rustc_hash::FxHashMap<ControlId, windows_core::IInspectable>,
}

/// Builds a stand-in native [`IInspectable`] for [`RecordingBackend`]. Any
/// concrete `IInspectable` works because the reconciler forwards it opaquely to
/// mount and unmount callbacks, so a boxed `IReference<i32>` is the cheapest
/// dependency-free choice (the real WinUI backend returns the XAML element).
fn stub_native_element() -> windows_core::IInspectable {
    windows_reference::IReference::<i32>::from(0).into()
}

impl RecordingBackend {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn op_count(&self) -> usize {
        self.ops.len()
    }

    pub fn clear_ops(&mut self) {
        self.ops.clear();
    }

    /// Panics immediately before the next matching backend operation mutates state.
    pub fn fail_next(&mut self, operation: BackendOperation) {
        self.fail_on(operation, 1);
    }

    /// Panics immediately before the `occurrence`th matching backend operation mutates state.
    pub fn fail_on(&mut self, operation: BackendOperation, occurrence: usize) {
        self.fail_sequence([(operation, occurrence)]);
    }

    /// Panics before each requested operation in order.
    pub fn fail_sequence(&mut self, failures: impl IntoIterator<Item = (BackendOperation, usize)>) {
        self.failures = failures
            .into_iter()
            .map(|(operation, remaining)| {
                assert!(
                    remaining != 0,
                    "backend failure occurrence must be non-zero"
                );
                BackendFailure {
                    operation,
                    remaining,
                }
            })
            .collect();
    }

    fn maybe_fail(&mut self, operation: BackendOperation) {
        let Some(failure) = self.failures.front_mut() else {
            return;
        };
        if failure.operation != operation {
            return;
        }
        failure.remaining -= 1;
        if failure.remaining == 0 {
            self.failures.pop_front();
            panic!("injected backend failure before {operation:?}");
        }
    }

    pub fn children_of(&self, parent: ControlId) -> &[ControlId] {
        self.children.get(&parent).map_or(&[], Vec::as_slice)
    }

    pub fn fire(&self, id: ControlId, event: Event) {
        let h = self
            .handlers
            .get(&(id, event))
            .unwrap_or_else(|| panic!("no handler for ({id}, {event:?})"));
        h.invoke();
    }

    pub fn fire_bool(&self, id: ControlId, event: Event, v: bool) {
        let h = self
            .handlers
            .get(&(id, event))
            .unwrap_or_else(|| panic!("no handler for ({id}, {event:?})"));
        h.invoke_bool(v);
    }

    pub fn fire_string(&self, id: ControlId, event: Event, s: String) {
        let h = self
            .handlers
            .get(&(id, event))
            .unwrap_or_else(|| panic!("no handler for ({id}, {event:?})"));
        h.invoke_string(s);
    }

    pub fn fire_f64(&self, id: ControlId, event: Event, v: f64) {
        let h = self
            .handlers
            .get(&(id, event))
            .unwrap_or_else(|| panic!("no handler for ({id}, {event:?})"));
        h.invoke_f64(v);
    }

    pub fn fire_i32(&self, id: ControlId, event: Event, v: i32) {
        let h = self
            .handlers
            .get(&(id, event))
            .unwrap_or_else(|| panic!("no handler for ({id}, {event:?})"));
        h.invoke_i32(v);
    }

    pub fn fire_navigation_display_mode(
        &self,
        id: ControlId,
        event: Event,
        mode: NavigationViewDisplayMode,
    ) {
        let h = self
            .handlers
            .get(&(id, event))
            .unwrap_or_else(|| panic!("no handler for ({id}, {event:?})"));
        h.invoke_navigation_display_mode(mode);
    }

    pub fn fire_datetime(&self, id: ControlId, event: Event, dt: DateTime) {
        let h = self
            .handlers
            .get(&(id, event))
            .unwrap_or_else(|| panic!("no handler for ({id}, {event:?})"));
        h.invoke_datetime(dt);
    }

    pub fn fire_timespan(&self, id: ControlId, event: Event, ts: TimeSpan) {
        let h = self
            .handlers
            .get(&(id, event))
            .unwrap_or_else(|| panic!("no handler for ({id}, {event:?})"));
        h.invoke_timespan(ts);
    }

    pub fn row_contents_of(&self, list_id: ControlId) -> rustc_hash::FxHashMap<usize, ControlId> {
        self.row_contents.get(&list_id).cloned().unwrap_or_default()
    }

    pub fn item_count_of(&self, list_id: ControlId) -> Option<usize> {
        self.item_counts.get(&list_id).copied()
    }

    pub fn simulate_prepare_row(&self, list_id: ControlId, row_idx: usize) {
        let (realize, _recycle) = self
            .realization_handlers
            .get(&list_id)
            .unwrap_or_else(|| panic!("no realization handler attached to {list_id}"));
        realize(row_idx);
    }

    pub fn simulate_clear_row(&self, list_id: ControlId, row_idx: usize) {
        let (_realize, recycle) = self
            .realization_handlers
            .get(&list_id)
            .unwrap_or_else(|| panic!("no realization handler attached to {list_id}"));
        recycle(row_idx);
    }

    pub fn fire_templated_selection_changed(&self, list_id: ControlId, index: i32) {
        let cb = self
            .selection_handlers
            .get(&list_id)
            .unwrap_or_else(|| panic!("no selection handler on {list_id}"));
        cb.invoke(index);
    }

    /// Simulates a completed drag-reorder by invoking the list's reorder
    /// handler with `order`, the new permutation of original item indices.
    pub fn simulate_reorder(&self, list_id: ControlId, order: Vec<usize>) {
        let cb = self
            .reorder_handlers
            .get(&list_id)
            .unwrap_or_else(|| panic!("no reorder handler on {list_id}"));
        cb.invoke(order);
    }

    pub fn theme_cache_size(&self) -> usize {
        self.theme_style_cache.len()
    }

    pub fn theme_bindings_of(&self, id: ControlId) -> Vec<(Prop, ThemeRef)> {
        self.theme_bindings_live
            .get(&id)
            .cloned()
            .unwrap_or_default()
    }

    pub fn live_control_count(&self) -> usize {
        self.live_controls.len()
    }

    /// Verifies the backend model after a reconciliation boundary.
    pub fn assert_consistent(&self) {
        let native: rustc_hash::FxHashSet<_> = self.native_elements.keys().copied().collect();
        assert_eq!(
            native, self.live_controls,
            "native element registry disagrees with live controls"
        );

        let mut owners = rustc_hash::FxHashMap::<ControlId, String>::default();
        let mut claim = |child: ControlId, owner: String| {
            assert!(
                self.live_controls.contains(&child),
                "{owner} owns destroyed or unknown control {child}"
            );
            if let Some(previous) = owners.insert(child, owner.clone()) {
                panic!("{child} has multiple owners: {previous} and {owner}");
            }
        };

        for (parent, children) in &self.children {
            assert!(
                self.live_controls.contains(parent),
                "destroyed or unknown parent {parent} retains children"
            );
            for (index, child) in children.iter().copied().enumerate() {
                claim(child, format!("child {index} of {parent}"));
            }
        }
        for (parent, child) in &self.headers {
            assert!(
                self.live_controls.contains(parent),
                "destroyed or unknown parent {parent} retains a header"
            );
            claim(*child, format!("header of {parent}"));
        }
        for (parent, child) in &self.panes {
            assert!(
                self.live_controls.contains(parent),
                "destroyed or unknown parent {parent} retains a pane"
            );
            claim(*child, format!("pane of {parent}"));
        }
        for (list, rows) in &self.row_contents {
            assert!(
                self.live_controls.contains(list),
                "destroyed or unknown list {list} retains row content"
            );
            for (index, child) in rows {
                claim(*child, format!("row {index} of {list}"));
            }
        }

        let assert_live_owner = |id: &ControlId, state: &str| {
            assert!(
                self.live_controls.contains(id),
                "destroyed or unknown control {id} retains {state}"
            );
        };
        for id in self.handlers.keys().map(|(id, _)| id) {
            assert_live_owner(id, "an event handler");
        }
        for id in self.item_counts.keys() {
            assert_live_owner(id, "a templated item count");
        }
        for id in self.realization_handlers.keys() {
            assert_live_owner(id, "templated realization handlers");
        }
        for id in self.selection_handlers.keys() {
            assert_live_owner(id, "a templated selection handler");
        }
        for id in self.reorder_handlers.keys() {
            assert_live_owner(id, "a templated reorder handler");
        }
        for id in self.theme_bindings_live.keys() {
            assert_live_owner(id, "theme bindings");
        }
    }
}

impl Backend for RecordingBackend {
    fn create(&mut self, kind: ControlKind) -> ControlId {
        self.maybe_fail(BackendOperation::Create);
        self.next_id += 1;
        let id = ControlId::new(self.next_id);
        self.ops.push(Op::Create { id, kind });
        self.live_controls.insert(id);
        self.native_elements.insert(id, stub_native_element());
        id
    }

    fn set_prop(&mut self, id: ControlId, prop: Prop, value: &PropValue) {
        self.maybe_fail(BackendOperation::SetProp);
        self.ops.push(Op::SetProp {
            id,
            prop,
            value: value.clone(),
        });
    }

    fn append_child(&mut self, parent: ControlId, child: ControlId) {
        self.maybe_fail(BackendOperation::AppendChild);
        self.children.entry(parent).or_default().push(child);
        self.ops.push(Op::AppendChild { parent, child });
    }

    fn remove_child(&mut self, parent: ControlId, index: usize) {
        self.maybe_fail(BackendOperation::RemoveChild);
        let list = self.children.entry(parent).or_default();
        assert!(
            index < list.len(),
            "remove_child: index {index} out of bounds for parent {parent} (len={})",
            list.len()
        );
        list.remove(index);
        self.ops.push(Op::RemoveChild { parent, index });
    }

    fn replace_child(&mut self, parent: ControlId, index: usize, new: ControlId) {
        self.maybe_fail(BackendOperation::ReplaceChild);
        let list = self.children.entry(parent).or_default();
        assert!(
            index < list.len(),
            "replace_child: index {index} out of bounds for parent {parent} (len={})",
            list.len()
        );
        list[index] = new;
        self.ops.push(Op::ReplaceChild { parent, index, new });
    }

    fn move_child(&mut self, parent: ControlId, from: usize, to: usize) {
        self.maybe_fail(BackendOperation::MoveChild);
        let list = self.children.entry(parent).or_default();
        assert!(
            from < list.len(),
            "move_child: from-index {from} out of bounds for parent {parent} (len={})",
            list.len()
        );
        assert!(
            to < list.len(),
            "move_child: to-index {to} out of bounds for parent {parent} (len={})",
            list.len()
        );
        if from != to {
            let item = list.remove(from);
            list.insert(to, item);
        }
        self.ops.push(Op::MoveChild { parent, from, to });
    }

    fn insert_child(&mut self, parent: ControlId, index: usize, child: ControlId) {
        self.maybe_fail(BackendOperation::InsertChild);
        let list = self.children.entry(parent).or_default();
        assert!(
            index <= list.len(),
            "insert_child: index {index} out of bounds for parent {parent} (len={})",
            list.len()
        );
        list.insert(index, child);
        self.ops.push(Op::InsertChild {
            parent,
            index,
            child,
        });
    }

    fn destroy(&mut self, id: ControlId) {
        self.maybe_fail(BackendOperation::Destroy);
        assert!(
            self.live_controls.remove(&id),
            "destroy: unknown or already destroyed control {id}"
        );
        self.children.remove(&id);
        self.headers.remove(&id);
        self.panes.remove(&id);
        self.native_elements.remove(&id);
        self.row_contents.remove(&id);
        self.item_counts.remove(&id);
        self.realization_handlers.remove(&id);
        self.selection_handlers.remove(&id);
        self.reorder_handlers.remove(&id);
        self.theme_bindings_live.remove(&id);
        self.handlers.retain(|(hid, _), _| *hid != id);
        self.ops.push(Op::Destroy { id });
    }

    fn attach_event(&mut self, id: ControlId, event: Event, handler: EventHandler) {
        self.maybe_fail(BackendOperation::AttachEvent);
        self.handlers.insert((id, event), handler.clone());
        self.ops.push(Op::AttachEvent { id, event, handler });
    }

    fn detach_event(&mut self, id: ControlId, event: Event) {
        self.maybe_fail(BackendOperation::DetachEvent);
        self.handlers.remove(&(id, event));
        self.ops.push(Op::DetachEvent { id, event });
    }

    fn set_templated_item_count(&mut self, id: ControlId, count: usize) {
        self.maybe_fail(BackendOperation::SetTemplatedItemCount);
        self.item_counts.insert(id, count);
        self.ops
            .push(Op::SetTemplatedItemCount { list_id: id, count });
    }

    fn set_templated_row_content(
        &mut self,
        list_id: ControlId,
        row_idx: usize,
        content: Option<ControlId>,
    ) {
        let slot = self.row_contents.entry(list_id).or_default();
        if let Some(c) = content {
            slot.insert(row_idx, c);
            self.ops.push(Op::MountRowContent {
                list_id,
                row_idx,
                content: c,
            });
        } else {
            slot.remove(&row_idx);
            self.ops.push(Op::ClearRowContent { list_id, row_idx });
        }
    }

    fn set_templated_selected_index(&mut self, id: ControlId, index: i32) {
        self.maybe_fail(BackendOperation::SetTemplatedSelectedIndex);
        self.ops.push(Op::SetTemplatedSelectedIndex { id, index });
    }

    fn set_templated_selection_mode(&mut self, id: ControlId, mode: SelectionMode) {
        self.maybe_fail(BackendOperation::SetTemplatedSelectionMode);
        self.ops.push(Op::SetTemplatedSelectionMode { id, mode });
    }

    fn set_templated_can_drag_items(&mut self, id: ControlId, value: bool) {
        self.maybe_fail(BackendOperation::SetTemplatedCanDragItems);
        self.ops.push(Op::SetTemplatedCanDragItems { id, value });
    }

    fn set_templated_can_reorder_items(&mut self, id: ControlId, value: bool) {
        self.maybe_fail(BackendOperation::SetTemplatedCanReorderItems);
        self.ops.push(Op::SetTemplatedCanReorderItems { id, value });
    }

    fn set_templated_allow_drop(&mut self, id: ControlId, value: bool) {
        self.maybe_fail(BackendOperation::SetTemplatedAllowDrop);
        self.ops.push(Op::SetTemplatedAllowDrop { id, value });
    }

    fn set_header_element(&mut self, id: ControlId, header_id: Option<ControlId>) {
        self.maybe_fail(BackendOperation::SetHeaderElement);
        if let Some(header_id) = header_id {
            self.headers.insert(id, header_id);
        } else {
            self.headers.remove(&id);
        }
        self.ops.push(Op::SetHeaderElement { id, header_id });
    }

    fn set_pane_element(&mut self, id: ControlId, pane_id: Option<ControlId>) {
        self.maybe_fail(BackendOperation::SetPaneElement);
        if let Some(pane_id) = pane_id {
            self.panes.insert(id, pane_id);
        } else {
            self.panes.remove(&id);
        }
        self.ops.push(Op::SetPaneElement { id, pane_id });
    }

    fn scroll_templated_to_index(&mut self, id: ControlId, index: i32) {
        self.ops.push(Op::ScrollTemplatedToIndex { id, index });
    }

    fn attach_templated_selection_changed(&mut self, id: ControlId, handler: Callback<i32>) {
        self.maybe_fail(BackendOperation::AttachTemplatedSelectionChanged);
        self.selection_handlers.insert(id, handler);
        self.ops.push(Op::AttachTemplatedSelectionChanged { id });
    }

    fn attach_templated_realization(
        &mut self,
        id: ControlId,
        realize: Rc<dyn Fn(usize)>,
        recycle: Rc<dyn Fn(usize)>,
    ) {
        self.maybe_fail(BackendOperation::AttachTemplatedRealization);
        self.realization_handlers.insert(id, (realize, recycle));
        self.ops.push(Op::AttachTemplatedRealization { id });
    }

    fn attach_templated_reorder(&mut self, id: ControlId, handler: Callback<Vec<usize>>) {
        self.maybe_fail(BackendOperation::AttachTemplatedReorder);
        self.reorder_handlers.insert(id, handler);
        self.ops.push(Op::AttachTemplatedReorder { id });
    }

    fn set_theme_bindings(
        &mut self,
        id: ControlId,
        kind: ControlKind,
        bindings: &[(Prop, ThemeRef)],
    ) {
        let mut canonical: Vec<(Prop, ThemeRef)> = bindings.to_vec();
        canonical.sort_by(|a, b| format!("{:?}", a.0).cmp(&format!("{:?}", b.0)));

        let cache_hit = !self.theme_style_cache.insert((kind, canonical.clone()));
        self.theme_bindings_live.insert(id, canonical.clone());
        self.ops.push(Op::ApplyThemeBindings {
            id,
            kind,
            bindings: canonical,
            cache_hit,
        });
    }

    fn on_theme_changed(&mut self) {
        self.theme_style_cache.clear();

        self.ops.push(Op::OnThemeChanged);
    }

    fn set_implicit_transitions(
        &mut self,
        id: ControlId,
        transitions: Option<ImplicitTransitions>,
    ) {
        self.ops
            .push(Op::SetImplicitTransitions { id, transitions });
    }

    fn set_layout_animation(&mut self, id: ControlId, config: Option<LayoutAnimationConfig>) {
        self.ops.push(Op::SetLayoutAnimation { id, config });
    }

    fn run_property_animation(&mut self, id: ControlId, config: Option<AnimationConfig>) {
        self.ops.push(Op::RunPropertyAnimation { id, config });
    }

    fn set_element_transitions(
        &mut self,
        id: ControlId,
        enter: Option<AnimationConfig>,
        exit: Option<AnimationConfig>,
    ) {
        self.ops.push(Op::SetElementTransitions { id, enter, exit });
    }

    fn set_rich_text_paragraphs(&mut self, id: ControlId, paragraphs: &[RichTextParagraph]) {
        self.ops.push(Op::SetRichTextParagraphs {
            id,
            paragraphs: paragraphs.to_vec(),
        });
    }

    fn set_accessibility(&mut self, id: ControlId, accessibility: &AccessibilityModifiers) {
        self.ops.push(Op::SetAccessibility {
            id,
            accessibility: accessibility.clone(),
        });
    }

    fn set_keyboard_accelerators(&mut self, id: ControlId, accelerators: &[KeyboardAccelerator]) {
        self.ops.push(Op::SetKeyboardAccelerators {
            id,
            accelerators: accelerators.to_vec(),
        });
    }

    fn set_tooltip(&mut self, id: ControlId, tooltip: Option<&Tooltip>) {
        self.ops.push(Op::SetTooltip {
            id,
            tooltip: tooltip.cloned(),
        });
    }

    fn set_pointer_handlers(&mut self, id: ControlId, handlers: Option<&PointerHandlers>) {
        self.ops.push(Op::SetPointerHandlers {
            id,
            handlers: handlers.cloned(),
        });
    }

    fn get_native_element(&self, id: ControlId) -> Option<windows_core::IInspectable> {
        self.native_elements.get(&id).cloned()
    }
}
