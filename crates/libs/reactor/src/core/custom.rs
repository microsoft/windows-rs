//! Extension hatch for downstream widgets that aren't part of the core
//! [`Element`](crate::core::element::Element) enum. Leaf-only in step 1a.

use super::*;

use std::any::{Any, TypeId};

/// Out-of-tree widget definition managed by the reconciler via
/// [`Element::Custom`](crate::core::element::Element::Custom).
pub trait CustomElement: 'static {
    /// Standard `Any` accessor; the implementor's body is almost always `self`.
    fn as_any(&self) -> &dyn Any;

    /// Stable type identity used by the reconciler to decide
    /// mount-vs-update; defaults to the underlying `Any::type_id`.
    fn type_id(&self) -> TypeId {
        self.as_any().type_id()
    }

    /// Human-readable name surfaced in diagnostics and `Element::kind_name`.
    fn kind_name(&self) -> &'static str;

    /// Optional key for keyed reconciliation inside multi-child containers.
    fn key(&self) -> Option<&str> {
        None
    }

    /// Structural equality against another element of the same `type_id`;
    /// returning `false` is always safe but skips an `update` short-circuit.
    fn eq_dyn(&self, other: &dyn CustomElement) -> bool;

    /// Boxed clone so [`Element`](crate::core::element::Element) stays `Clone`.
    fn clone_dyn(&self) -> Box<dyn CustomElement>;

    /// Create the underlying control via `backend` and return its id.
    fn mount(&self, backend: &mut dyn Backend) -> ControlId;

    /// Apply the diff from `prev` (same `type_id`) to the live control `id`.
    fn update(&self, prev: &dyn CustomElement, id: ControlId, backend: &mut dyn Backend);

    /// Hook fired just before [`Backend::destroy`]; defaults to no-op.
    fn before_destroy(&self, _id: ControlId, _backend: &mut dyn Backend) {}
}

/// Boxed [`CustomElement`] with `Clone` / `Debug` / `PartialEq` so it can
/// live inside the `Element` enum.
pub struct CustomElementHandle(pub Box<dyn CustomElement>);

impl CustomElementHandle {
    pub fn new<C: CustomElement>(c: C) -> Self {
        Self(Box::new(c))
    }
    pub fn get(&self) -> &dyn CustomElement {
        &*self.0
    }
}

impl Clone for CustomElementHandle {
    fn clone(&self) -> Self {
        Self(self.0.clone_dyn())
    }
}

impl std::fmt::Debug for CustomElementHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CustomElement")
            .field("kind", &self.0.kind_name())
            .field("type_id", &CustomElement::type_id(&*self.0))
            .field("key", &self.0.key())
            .finish()
    }
}

impl PartialEq for CustomElementHandle {
    fn eq(&self, other: &Self) -> bool {
        if CustomElement::type_id(&*self.0) != CustomElement::type_id(&*other.0) {
            return false;
        }
        if self.0.key() != other.0.key() {
            return false;
        }
        self.0.eq_dyn(&*other.0)
    }
}
