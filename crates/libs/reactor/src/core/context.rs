use std::any::{Any, TypeId};
use std::rc::Rc;
use std::sync::atomic::{AtomicU32, Ordering};

use rustc_hash::FxHashMap;

use super::element::Element;

/// Process-wide unique identifier for a [`Context`] of any value type.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ContextId(u32);

static NEXT_CONTEXT_ID: AtomicU32 = AtomicU32::new(1);

impl ContextId {
    pub fn new() -> Self {
        Self(NEXT_CONTEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

impl Default for ContextId {
    fn default() -> Self {
        Self::new()
    }
}

/// Typed lookup key + default for a value made available to descendants
/// via [`ProviderElement`] and [`RenderCx::use_context()`](crate::RenderCx::use_context).
pub struct Context<T> {
    pub default: T,
    pub id: ContextId,
}

impl<T> Context<T> {
    pub fn new(default: T) -> Self {
        Self {
            default,
            id: ContextId::new(),
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Context<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Context")
            .field("id", &self.id)
            .field("default", &self.default)
            .finish()
    }
}

struct ContextEntry {
    value_type_id: TypeId,
    value: Rc<dyn Any>,
}

#[derive(Default)]
pub struct ContextStack {
    entries: std::cell::RefCell<FxHashMap<ContextId, Vec<ContextEntry>>>,
    push_order: std::cell::RefCell<Vec<ContextId>>,
}

impl ContextStack {
    pub fn new() -> Self {
        Self::default()
    }

    #[cfg(test)]
    pub(crate) fn push<T: 'static>(&self, context_id: ContextId, value: T) -> ContextScope<'_> {
        self.push_raw_retain(context_id, TypeId::of::<T>(), Rc::new(value));
        ContextScope { stack: self }
    }

    pub(crate) fn push_raw_retain(
        &self,
        context_id: ContextId,
        value_type_id: TypeId,
        value: Rc<dyn Any>,
    ) {
        self.entries
            .borrow_mut()
            .entry(context_id)
            .or_default()
            .push(ContextEntry {
                value_type_id,
                value,
            });
        self.push_order.borrow_mut().push(context_id);
    }

    pub(crate) fn pop_raw(&self) {
        if let Some(id) = self.push_order.borrow_mut().pop() {
            let mut entries = self.entries.borrow_mut();
            if let Some(stack) = entries.get_mut(&id) {
                stack.pop();
                if stack.is_empty() {
                    entries.remove(&id);
                }
            }
        }
    }

    pub fn get<T: 'static + Clone>(&self, context_id: ContextId) -> Option<T> {
        let entries = self.entries.borrow();
        let entry = entries.get(&context_id)?.last()?;
        assert!(
            entry.value_type_id == TypeId::of::<T>(),
            "context type mismatch for context {:?}: provided as {:?}, consumed as {:?}",
            context_id,
            entry.value_type_id,
            TypeId::of::<T>()
        );
        entry.value.downcast_ref::<T>().cloned()
    }

    #[cfg(test)]
    fn pop(&self) {
        self.pop_raw();
    }
}

#[cfg(test)]
pub(crate) struct ContextScope<'a> {
    stack: &'a ContextStack,
}

#[cfg(test)]
impl Drop for ContextScope<'_> {
    fn drop(&mut self) {
        self.stack.pop();
    }
}

#[derive(Clone)]
pub struct ContextProvision {
    pub(crate) context_id: ContextId,
    pub(crate) value_type_id: TypeId,
    pub(crate) value: Rc<dyn Any>,
    eq_fn: fn(&dyn Any, &dyn Any) -> bool,
}

impl ContextProvision {
    pub fn new<T: Clone + PartialEq + 'static>(id: ContextId, value: T) -> Self {
        Self {
            context_id: id,
            value_type_id: TypeId::of::<T>(),
            value: Rc::new(value),
            eq_fn: |a, b| match (a.downcast_ref::<T>(), b.downcast_ref::<T>()) {
                (Some(a), Some(b)) => a == b,
                _ => false,
            },
        }
    }
}

impl std::fmt::Debug for ContextProvision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ContextProvision")
            .field("context_id", &self.context_id)
            .field("value_type_id", &self.value_type_id)
            .finish()
    }
}

impl PartialEq for ContextProvision {
    fn eq(&self, other: &Self) -> bool {
        if self.context_id != other.context_id || self.value_type_id != other.value_type_id {
            return false;
        }
        if Rc::ptr_eq(&self.value, &other.value) {
            return true;
        }
        (self.eq_fn)(self.value.as_ref(), other.value.as_ref())
    }
}

#[derive(Clone, Debug)]
pub struct ProviderElement {
    pub key: Option<String>,
    pub provisions: Vec<ContextProvision>,
    pub child: Box<Element>,
}

impl PartialEq for ProviderElement {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.provisions == other.provisions && self.child == other.child
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_ids_are_unique() {
        let a = ContextId::new();
        let b = ContextId::new();
        assert_ne!(a, b);
    }

    #[test]
    fn context_new_assigns_unique_id() {
        let a = Context::<i32>::new(0);
        let b = Context::<i32>::new(0);
        assert_ne!(a.id, b.id);
    }

    #[test]
    fn stack_get_returns_none_when_empty() {
        let s = ContextStack::new();
        let id = ContextId::new();
        assert!(s.get::<i32>(id).is_none());
    }

    #[test]
    fn stack_push_pop_shadows_lookup() {
        let s = ContextStack::new();
        let id = ContextId::new();
        assert!(s.get::<i32>(id).is_none());
        {
            let _g = s.push(id, 42_i32);
            assert_eq!(s.get::<i32>(id), Some(42));
        }
        assert!(s.get::<i32>(id).is_none());
    }

    #[test]
    fn nested_provides_return_innermost() {
        let s = ContextStack::new();
        let id = ContextId::new();
        let _a = s.push(id, "outer".to_string());
        {
            let _b = s.push(id, "inner".to_string());
            assert_eq!(s.get::<String>(id), Some("inner".into()));
        }
        assert_eq!(s.get::<String>(id), Some("outer".into()));
    }

    #[test]
    fn distinct_ids_dont_shadow() {
        let s = ContextStack::new();
        let id_a = ContextId::new();
        let id_b = ContextId::new();
        let _a = s.push(id_a, 1_i32);
        let _b = s.push(id_b, 2_i32);
        assert_eq!(s.get::<i32>(id_a), Some(1));
        assert_eq!(s.get::<i32>(id_b), Some(2));
    }

    #[test]
    #[should_panic(expected = "context type mismatch")]
    fn mismatched_types_panic() {
        let s = ContextStack::new();
        let id = ContextId::new();
        let _g = s.push(id, 1_i32);
        let _ = s.get::<String>(id);
    }
}
