use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use rustc_hash::FxHashMap;

use super::*;

/// Visual modifiers shared by every widget; carried on each element struct
/// and applied uniformly via `FrameworkElement`-level setters at the
/// backend.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Modifiers {
    pub margin: Option<Thickness>,
    pub padding: Option<Thickness>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub min_width: Option<f64>,
    pub max_width: Option<f64>,
    pub min_height: Option<f64>,
    pub max_height: Option<f64>,
    pub horizontal_alignment: Option<HorizontalAlignment>,
    pub vertical_alignment: Option<VerticalAlignment>,
    pub opacity: Option<f64>,
    pub background: Option<Brush>,
    pub foreground: Option<Brush>,
    pub font_family: Option<String>,
    pub font_size: Option<f64>,
    pub theme_bindings: Option<Box<rustc_hash::FxHashMap<crate::core::backend::Prop, ThemeRef>>>,
    pub animations: Option<Box<crate::core::animation::AnimationModifiers>>,
    pub attached: Option<AttachedProps>,
    pub accessibility: Option<Box<AccessibilityModifiers>>,
    pub keyboard_accelerators: Option<Box<Vec<KeyboardAccelerator>>>,
    pub tooltip: Option<Box<crate::core::tooltip::Tooltip>>,
    pub pointer_handlers: Option<Box<crate::core::pointer::PointerHandlers>>,
    /// Fast-path for grid row/column placement — avoids the `AttachedProps`
    /// HashMap + Box + thread_local overhead for the most common attached prop.
    pub grid: Option<GridPlacement>,
    pub resources: Option<Box<HashMap<String, String>>>,
}

impl Modifiers {
    pub fn is_empty(&self) -> bool {
        self.margin.is_none()
            && self.padding.is_none()
            && self.width.is_none()
            && self.height.is_none()
            && self.min_width.is_none()
            && self.max_width.is_none()
            && self.min_height.is_none()
            && self.max_height.is_none()
            && self.horizontal_alignment.is_none()
            && self.vertical_alignment.is_none()
            && self.opacity.is_none()
            && self.background.is_none()
            && self.foreground.is_none()
            && self.font_family.is_none()
            && self.font_size.is_none()
            && self.theme_bindings.as_ref().is_none_or(|m| m.is_empty())
            && self.animations.as_ref().is_none_or(|a| a.is_empty())
            && self.attached.as_ref().is_none_or(|a| a.is_empty())
            && self.accessibility.as_deref().is_none_or(|a| a.is_empty())
            && self
                .keyboard_accelerators
                .as_deref()
                .is_none_or(|a| a.is_empty())
            && self.tooltip.is_none()
            && self
                .pointer_handlers
                .as_deref()
                .is_none_or(|p| p.is_empty())
            && self.grid.is_none()
            && self.resources.as_deref().is_none_or(|r| r.is_empty())
    }
}

/// Type-erased bag of attached properties (e.g. [`GridPlacement`]) keyed
/// by [`TypeId`]; values must be inserted via [`AttachedProps::set`].
#[derive(Default, Debug)]
pub struct AttachedProps(FxHashMap<TypeId, Box<dyn Any>>);

impl Clone for AttachedProps {
    fn clone(&self) -> Self {
        let mut copy = FxHashMap::default();
        for (k, v) in &self.0 {
            let (clone_fn, _) = ops_for(*k).unwrap();
            copy.insert(*k, clone_fn(&**v));
        }
        Self(copy)
    }
}

impl PartialEq for AttachedProps {
    fn eq(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }
        for (k, v) in &self.0 {
            let Some(ov) = other.0.get(k) else {
                return false;
            };
            let (_, eq_fn) = ops_for(*k).unwrap();
            if !eq_fn(&**v, &**ov) {
                return false;
            }
        }
        true
    }
}

impl AttachedProps {
    pub fn set<T: Clone + PartialEq + 'static>(&mut self, v: T) {
        register_ops::<T>();
        self.0.insert(TypeId::of::<T>(), Box::new(v));
    }
    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.0
            .get(&TypeId::of::<T>())
            .and_then(|b| b.downcast_ref::<T>())
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GridPlacement {
    pub row: i32,
    pub column: i32,
    pub row_span: i32,
    pub column_span: i32,
}

impl Default for GridPlacement {
    fn default() -> Self {
        Self {
            row: 0,
            column: 0,
            row_span: 1,
            column_span: 1,
        }
    }
}

type CloneFn = fn(&dyn Any) -> Box<dyn Any>;
type EqFn = fn(&dyn Any, &dyn Any) -> bool;

#[derive(Copy, Clone)]
struct AttachedOps {
    clone: CloneFn,
    eq: EqFn,
}

thread_local! {
    static OPS_REGISTRY: std::cell::RefCell<FxHashMap<TypeId, AttachedOps>> =
        std::cell::RefCell::new(FxHashMap::default());
}

fn register_ops<T: Clone + PartialEq + 'static>() {
    let tid = TypeId::of::<T>();
    OPS_REGISTRY.with(|r| {
        r.borrow_mut().entry(tid).or_insert_with(|| AttachedOps {
            clone: |src| Box::new(src.downcast_ref::<T>().unwrap().clone()),
            eq: |a, b| match (a.downcast_ref::<T>(), b.downcast_ref::<T>()) {
                (Some(a), Some(b)) => a == b,
                _ => false,
            },
        });
    });
}

fn ops_for(tid: TypeId) -> Option<(CloneFn, EqFn)> {
    OPS_REGISTRY.with(|r| r.borrow().get(&tid).map(|ops| (ops.clone, ops.eq)))
}
