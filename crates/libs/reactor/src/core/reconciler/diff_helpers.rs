use super::*;

impl<B: Backend> Reconciler<B> {
    pub(crate) fn diff_opt_f64(
        &mut self,
        id: ControlId,
        prop: Prop,
        old: Option<f64>,
        new: Option<f64>,
    ) {
        if old == new {
            return;
        }
        match new {
            Some(v) => self.backend.set_prop(id, prop, PropValue::F64(v)),
            None => self.backend.set_prop(id, prop, PropValue::Unset),
        }
    }

    pub(crate) fn diff_opt_copy<T: Copy + PartialEq>(
        &mut self,
        id: ControlId,
        prop: Prop,
        old: Option<T>,
        new: Option<T>,
        wrap: fn(T) -> PropValue,
    ) {
        if old == new {
            return;
        }
        match new {
            Some(v) => self.backend.set_prop(id, prop, wrap(v)),
            None => self.backend.set_prop(id, prop, PropValue::Unset),
        }
    }

    pub(crate) fn diff_opt_clone<T: Clone + PartialEq>(
        &mut self,
        id: ControlId,
        prop: Prop,
        old: &Option<T>,
        new: &Option<T>,
        wrap: fn(T) -> PropValue,
    ) {
        if old == new {
            return;
        }
        match new {
            Some(v) => self.backend.set_prop(id, prop, wrap(v.clone())),
            None => self.backend.set_prop(id, prop, PropValue::Unset),
        }
    }

    pub(crate) fn apply_props(&mut self, id: ControlId, bindings: &[Binding]) {
        for b in bindings {
            match b {
                Binding::Prop(p, v) => self.backend.set_prop(id, *p, v.clone()),
                Binding::Event(e, Some(h)) => self.backend.attach_event(id, *e, h.clone()),
                Binding::Event(_, None) => {}
            }
        }
    }

    /// Process only Event bindings (props handled by typed path).
    pub(crate) fn apply_events(&mut self, id: ControlId, bindings: &[Binding]) {
        for b in bindings {
            if let Binding::Event(e, Some(h)) = b {
                self.backend.attach_event(id, *e, h.clone());
            }
        }
    }

    pub(crate) fn diff_props(&mut self, id: ControlId, old: &[Binding], new: &[Binding]) {
        for b in new {
            match b {
                Binding::Prop(p, v) => match find_prop(old, *p) {
                    Some(ov) if ov == v => {}
                    _ => self.backend.set_prop(id, *p, v.clone()),
                },
                Binding::Event(e, new_h) => {
                    let old_inner: Option<&_> = find_event(old, *e).and_then(|o| o.as_ref());
                    if old_inner == new_h.as_ref() {
                        continue;
                    }
                    match new_h {
                        Some(h) => self.backend.attach_event(id, *e, h.clone()),
                        None => self.backend.detach_event(id, *e),
                    }
                }
            }
        }
        for b in old {
            match b {
                Binding::Prop(p, _) => {
                    if find_prop(new, *p).is_none() {
                        self.backend.set_prop(id, *p, PropValue::Unset);
                    }
                }
                Binding::Event(e, old_h) => {
                    if find_event(new, *e).is_none() && old_h.is_some() {
                        self.backend.detach_event(id, *e);
                    }
                }
            }
        }
    }

    /// Diff only Event bindings (props handled by typed path).
    pub(crate) fn diff_events(&mut self, id: ControlId, old: &[Binding], new: &[Binding]) {
        for b in new {
            if let Binding::Event(e, new_h) = b {
                let old_inner: Option<&_> = find_event(old, *e).and_then(|o| o.as_ref());
                if old_inner == new_h.as_ref() {
                    continue;
                }
                match new_h {
                    Some(h) => self.backend.attach_event(id, *e, h.clone()),
                    None => self.backend.detach_event(id, *e),
                }
            }
        }
        for b in old {
            if let Binding::Event(e, old_h) = b
                && find_event(new, *e).is_none() && old_h.is_some()
            {
                self.backend.detach_event(id, *e);
            }
        }
    }
}
