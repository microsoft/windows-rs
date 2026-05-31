use super::*;

impl<B: Backend + 'static> Reconciler<B> {
    pub(crate) fn mount_widget(&mut self, w: &dyn Widget) -> ControlId {
        let id = self.acquire_control(w.kind());
        self.apply_props(id, &w.bindings());
        self.apply_modifiers(id, w.modifiers());
        self.apply_attached(id, w.attached());
        self.mount_widget_children(id, w.children());
        if let Some(hdr) = w.header_element()
            && let Some(hdr_id) = self.mount(hdr) {
                self.backend.set_header_element(id, Some(hdr_id));
                self.header_elements.insert(id, hdr_id);
            }
        if let Some(pane) = w.pane_element()
            && let Some(pane_id) = self.mount(pane) {
                self.backend.set_pane_element(id, Some(pane_id));
                self.pane_elements.insert(id, pane_id);
            }
        id
    }

    pub(crate) fn update_widget(&mut self, old: &dyn Widget, new: &dyn Widget, id: ControlId) {
        self.diff_props(id, &old.bindings(), &new.bindings());
        self.diff_modifiers(id, old.modifiers(), new.modifiers());
        self.diff_attached(id, old.attached(), new.attached());
        self.update_widget_children(id, old.children(), new.children());
        self.update_header_element(id, old.header_element(), new.header_element());
        self.update_pane_element(id, old.pane_element(), new.pane_element());
    }

    fn mount_widget_children(&mut self, id: ControlId, children: Children<'_>) {
        match children {
            Children::None => {}
            Children::PositionalSingle(child) => {
                if let Some(child_id) = self.mount(child) {
                    self.append_child_tracked(id, child_id);
                }
            }
            Children::Keyed(list) => {
                for child in crate::core::reconciler::collect_live(list) {
                    if let Some(child_id) = self.mount(child) {
                        self.append_child_tracked(id, child_id);
                    }
                }
            }
            Children::Tabs(tabs) => {
                for tab in tabs {
                    self.mount_tab_item(id, tab);
                }
            }
            Children::PivotItems(items) => {
                for item in items {
                    self.mount_pivot_item(id, item);
                }
            }
        }
    }

    fn update_widget_children(&mut self, id: ControlId, old: Children<'_>, new: Children<'_>) {
        match (old, new) {
            (Children::None, Children::None) => {}
            (Children::PositionalSingle(o), Children::PositionalSingle(n)) => {
                let oc = std::slice::from_ref(o);
                let nc = std::slice::from_ref(n);
                self.reconcile_children_positional(id, oc, nc);
            }
            (Children::Keyed(o), Children::Keyed(n)) => {
                self.reconcile_children(id, o, n);
            }
            (Children::Tabs(o), Children::Tabs(n)) => {
                self.reconcile_tabs(id, o, n);
            }
            (Children::PivotItems(o), Children::PivotItems(n)) => {
                self.reconcile_pivot_items(id, o, n);
            }
            _ => {
                debug_assert!(
                    false,
                    "update_widget_children: child-layout shape changed across update"
                );
            }
        }
    }

    fn update_header_element(
        &mut self,
        id: ControlId,
        old: Option<&Element>,
        new: Option<&Element>,
    ) {
        match (old, new) {
            (None, None) => {}
            (None, Some(hdr)) => {
                if let Some(hdr_id) = self.mount(hdr) {
                    self.backend.set_header_element(id, Some(hdr_id));
                    self.header_elements.insert(id, hdr_id);
                }
            }
            (Some(_), None) => {
                if let Some(hdr_id) = self.header_elements.remove(&id) {
                    self.backend
                        .set_header_element(id, Option::<ControlId>::None);
                    self.unmount(hdr_id);
                }
            }
            (Some(old_el), Some(new_el)) => {
                // Reconcile in-place when possible to preserve focus/state.
                if let Some(hdr_id) = self.header_elements.get(&id).copied() {
                    if old_el.kind_matches(new_el) {
                        let new_id = self.update(old_el, new_el, hdr_id);
                        match new_id {
                            Some(nid) if nid != hdr_id => {
                                self.backend.set_header_element(id, Some(nid));
                                self.header_elements.insert(id, nid);
                            }
                            None => {
                                self.backend
                                    .set_header_element(id, Option::<ControlId>::None);
                                self.header_elements.remove(&id);
                            }
                            _ => {}
                        }
                        return;
                    }
                    self.header_elements.remove(&id);
                    self.unmount(hdr_id);
                }
                if let Some(hdr_id) = self.mount(new_el) {
                    self.backend.set_header_element(id, Some(hdr_id));
                    self.header_elements.insert(id, hdr_id);
                }
            }
        }
    }

    fn update_pane_element(&mut self, id: ControlId, old: Option<&Element>, new: Option<&Element>) {
        match (old, new) {
            (None, None) => {}
            (None, Some(pane)) => {
                if let Some(pane_id) = self.mount(pane) {
                    self.backend.set_pane_element(id, Some(pane_id));
                    self.pane_elements.insert(id, pane_id);
                }
            }
            (Some(_), None) => {
                if let Some(pane_id) = self.pane_elements.remove(&id) {
                    self.backend.set_pane_element(id, Option::<ControlId>::None);
                    self.unmount(pane_id);
                }
            }
            (Some(old_el), Some(new_el)) => {
                // Reconcile in-place when possible to preserve focus/state.
                if let Some(pane_id) = self.pane_elements.get(&id).copied() {
                    if old_el.kind_matches(new_el) {
                        let new_id = self.update(old_el, new_el, pane_id);
                        match new_id {
                            Some(nid) if nid != pane_id => {
                                self.backend.set_pane_element(id, Some(nid));
                                self.pane_elements.insert(id, nid);
                            }
                            None => {
                                self.backend
                                    .set_pane_element(id, Option::<ControlId>::None);
                                self.pane_elements.remove(&id);
                            }
                            _ => {}
                        }
                        return;
                    }
                    self.pane_elements.remove(&id);
                    self.unmount(pane_id);
                }
                if let Some(pane_id) = self.mount(new_el) {
                    self.backend.set_pane_element(id, Some(pane_id));
                    self.pane_elements.insert(id, pane_id);
                }
            }
        }
    }

    fn mount_tab_item(&mut self, parent: ControlId, tab: &TabItem) {
        let tab_id = self.acquire_control(crate::core::backend::ControlKind::TabViewItem);
        self.backend
            .set_prop(tab_id, Prop::TabHeader, PropValue::Str(tab.header.clone()));
        if let Some(key) = &tab.key {
            self.backend
                .set_prop(tab_id, Prop::TabItemKey, PropValue::Str(key.clone()));
        }
        if let Some(closable) = tab.is_closable {
            self.backend
                .set_prop(tab_id, Prop::IsClosable, PropValue::Bool(closable));
        }
        if let Some(content_id) = self.mount(&tab.content) {
            self.append_child_tracked(tab_id, content_id);
        }
        self.append_child_tracked(parent, tab_id);
    }

    fn mount_pivot_item(&mut self, parent: ControlId, item: &PivotItem) {
        let item_id = self.acquire_control(crate::core::backend::ControlKind::PivotItem);
        self.backend.set_prop(
            item_id,
            Prop::PivotItemHeader,
            PropValue::Str(item.header.clone()),
        );
        if let Some(content_id) = self.mount(&item.content) {
            self.append_child_tracked(item_id, content_id);
        }
        self.append_child_tracked(parent, item_id);
    }

    fn reconcile_tabs(&mut self, parent: ControlId, old: &[TabItem], new: &[TabItem]) {
        let common = old.len().min(new.len());

        for i in 0..common {
            let Some(tab_id) = self.child_at(parent, i) else {
                continue;
            };
            let o = &old[i];
            let n = &new[i];
            if o.header != n.header {
                self.backend
                    .set_prop(tab_id, Prop::TabHeader, PropValue::Str(n.header.clone()));
            }
            if o.key != n.key
                && let Some(key) = &n.key {
                    self.backend
                        .set_prop(tab_id, Prop::TabItemKey, PropValue::Str(key.clone()));
                }
            if o.is_closable != n.is_closable {
                // Either explicit value (set new), or transition to default
                // (re-enable platform default = true).
                let v = n.is_closable.unwrap_or(true);
                self.backend
                    .set_prop(tab_id, Prop::IsClosable, PropValue::Bool(v));
            }
            let oc = std::slice::from_ref(&o.content);
            let nc = std::slice::from_ref(&n.content);
            self.reconcile_children_positional(tab_id, oc, nc);
        }

        if new.len() > old.len() {
            for n in &new[old.len()..] {
                self.mount_tab_item(parent, n);
            }
        }

        while self.child_at(parent, new.len()).is_some() {
            let extra_id = self.child_at(parent, new.len()).unwrap();
            self.remove_child_tracked(parent, new.len());
            self.unmount(extra_id);
        }
    }

    fn reconcile_pivot_items(&mut self, parent: ControlId, old: &[PivotItem], new: &[PivotItem]) {
        let common = old.len().min(new.len());

        for i in 0..common {
            let Some(item_id) = self.child_at(parent, i) else {
                continue;
            };
            let o = &old[i];
            let n = &new[i];
            if o.header != n.header {
                self.backend.set_prop(
                    item_id,
                    Prop::PivotItemHeader,
                    PropValue::Str(n.header.clone()),
                );
            }
            let oc = std::slice::from_ref(&o.content);
            let nc = std::slice::from_ref(&n.content);
            self.reconcile_children_positional(item_id, oc, nc);
        }

        if new.len() > old.len() {
            for n in &new[old.len()..] {
                self.mount_pivot_item(parent, n);
            }
        }

        while self.child_at(parent, new.len()).is_some() {
            let extra_id = self.child_at(parent, new.len()).unwrap();
            self.remove_child_tracked(parent, new.len());
            self.unmount(extra_id);
        }
    }

    pub(crate) fn apply_attached(&mut self, id: ControlId, attached: Option<&AttachedProps>) {
        let Some(att) = attached else { return };
        // GridPlacement is now on Modifiers::grid — handled by apply_modifiers.
        if let Some(p) = att.get::<crate::core::widgets::CanvasPosition>() {
            self.apply_canvas_position(id, *p);
        }
        if let Some(p) = att.get::<crate::core::widgets::RelativePanelAlignment>() {
            self.apply_relative_panel_alignment(id, *p);
        }
    }

    pub(crate) fn apply_grid_placement(&mut self, id: ControlId, p: GridPlacement) {
        if p.row != 0 {
            self.backend
                .set_prop(id, Prop::AttachedGridRow, PropValue::I32(p.row));
        }
        if p.column != 0 {
            self.backend
                .set_prop(id, Prop::AttachedGridColumn, PropValue::I32(p.column));
        }
        if p.row_span > 1 {
            self.backend
                .set_prop(id, Prop::AttachedGridRowSpan, PropValue::I32(p.row_span));
        }
        if p.column_span > 1 {
            self.backend.set_prop(
                id,
                Prop::AttachedGridColumnSpan,
                PropValue::I32(p.column_span),
            );
        }
    }

    /// Unconditionally emits all four grid attached props — used in the diff
    /// path to clear stale values when placement changes or is removed.
    pub(crate) fn apply_grid_placement_full(&mut self, id: ControlId, p: GridPlacement) {
        self.backend
            .set_prop(id, Prop::AttachedGridRow, PropValue::I32(p.row));
        self.backend
            .set_prop(id, Prop::AttachedGridColumn, PropValue::I32(p.column));
        self.backend
            .set_prop(id, Prop::AttachedGridRowSpan, PropValue::I32(p.row_span));
        self.backend.set_prop(
            id,
            Prop::AttachedGridColumnSpan,
            PropValue::I32(p.column_span),
        );
    }

    pub(crate) fn apply_canvas_position(
        &mut self,
        id: ControlId,
        p: crate::core::widgets::CanvasPosition,
    ) {
        // Canvas defaults are 0.0 — only emit when non-zero on mount;
        // the diff path always emits to overwrite the previous value.
        if p.left != 0.0 {
            self.backend
                .set_prop(id, Prop::AttachedCanvasLeft, PropValue::F64(p.left));
        }
        if p.top != 0.0 {
            self.backend
                .set_prop(id, Prop::AttachedCanvasTop, PropValue::F64(p.top));
        }
        if p.z_index != 0 {
            self.backend
                .set_prop(id, Prop::AttachedCanvasZIndex, PropValue::I32(p.z_index));
        }
    }

    pub(crate) fn diff_attached(
        &mut self,
        id: ControlId,
        old: Option<&AttachedProps>,
        new: Option<&AttachedProps>,
    ) {
        // GridPlacement is now on Modifiers::grid — handled by diff_modifiers.

        let old_canvas = old
            .and_then(|a| a.get::<crate::core::widgets::CanvasPosition>())
            .copied();
        let new_canvas = new
            .and_then(|a| a.get::<crate::core::widgets::CanvasPosition>())
            .copied();
        if old_canvas != new_canvas {
            let p = new_canvas.unwrap_or_default();
            self.backend
                .set_prop(id, Prop::AttachedCanvasLeft, PropValue::F64(p.left));
            self.backend
                .set_prop(id, Prop::AttachedCanvasTop, PropValue::F64(p.top));
            self.backend
                .set_prop(id, Prop::AttachedCanvasZIndex, PropValue::I32(p.z_index));
        }

        let old_rp = old
            .and_then(|a| a.get::<crate::core::widgets::RelativePanelAlignment>())
            .copied();
        let new_rp = new
            .and_then(|a| a.get::<crate::core::widgets::RelativePanelAlignment>())
            .copied();
        if old_rp != new_rp {
            let p = new_rp.unwrap_or_default();
            self.apply_relative_panel_alignment_full(id, p);
        }
    }

    pub(crate) fn apply_relative_panel_alignment(
        &mut self,
        id: ControlId,
        p: crate::core::widgets::RelativePanelAlignment,
    ) {
        if p.align_left_with_panel {
            self.backend.set_prop(
                id,
                Prop::RelativePanelAlignLeftWithPanel,
                PropValue::Bool(true),
            );
        }
        if p.align_right_with_panel {
            self.backend.set_prop(
                id,
                Prop::RelativePanelAlignRightWithPanel,
                PropValue::Bool(true),
            );
        }
        if p.align_top_with_panel {
            self.backend.set_prop(
                id,
                Prop::RelativePanelAlignTopWithPanel,
                PropValue::Bool(true),
            );
        }
        if p.align_bottom_with_panel {
            self.backend.set_prop(
                id,
                Prop::RelativePanelAlignBottomWithPanel,
                PropValue::Bool(true),
            );
        }
        if p.align_h_center_with_panel {
            self.backend.set_prop(
                id,
                Prop::RelativePanelAlignHCenterWithPanel,
                PropValue::Bool(true),
            );
        }
        if p.align_v_center_with_panel {
            self.backend.set_prop(
                id,
                Prop::RelativePanelAlignVCenterWithPanel,
                PropValue::Bool(true),
            );
        }
    }

    fn apply_relative_panel_alignment_full(
        &mut self,
        id: ControlId,
        p: crate::core::widgets::RelativePanelAlignment,
    ) {
        self.backend.set_prop(
            id,
            Prop::RelativePanelAlignLeftWithPanel,
            PropValue::Bool(p.align_left_with_panel),
        );
        self.backend.set_prop(
            id,
            Prop::RelativePanelAlignRightWithPanel,
            PropValue::Bool(p.align_right_with_panel),
        );
        self.backend.set_prop(
            id,
            Prop::RelativePanelAlignTopWithPanel,
            PropValue::Bool(p.align_top_with_panel),
        );
        self.backend.set_prop(
            id,
            Prop::RelativePanelAlignBottomWithPanel,
            PropValue::Bool(p.align_bottom_with_panel),
        );
        self.backend.set_prop(
            id,
            Prop::RelativePanelAlignHCenterWithPanel,
            PropValue::Bool(p.align_h_center_with_panel),
        );
        self.backend.set_prop(
            id,
            Prop::RelativePanelAlignVCenterWithPanel,
            PropValue::Bool(p.align_v_center_with_panel),
        );
    }
}
