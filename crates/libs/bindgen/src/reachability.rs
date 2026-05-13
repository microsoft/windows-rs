//! Method-level reachability pruning for `--minimal` mode (B1).
//!
//! `TypeMap::filter` already produces a *type-level* transitive closure
//! from the explicit `--filter` roots. That closure intentionally pulls
//! in every base interface, every parameter / return / generic-argument
//! type, and every default interface of a kept class — so far so good.
//!
//! In `--minimal` mode, however, we don't want every kept type to drag in
//! every one of *its* methods as a `pub fn` forwarder + live vtable slot.
//! We only want methods that are reachable from something the user
//! actually named. This pass computes that closure and registers the
//! result on [`Filter`] so the existing per-method demotion path in
//! `Interface::get_methods` / `CppInterface::get_methods` (which already
//! turns a method into an opaque `Slot: usize`) does the rest.
//!
//! Algorithm (worklist over `TypeName`s of kept WinRT/COM interfaces):
//!
//! 1. **Roots.** For every `Type` retained by `TypeMap::filter`:
//!    * `Interface` / `CppInterface` whose `TypeName` matches an
//!      explicit `--filter` rule (fully-qualified or namespace-prefix),
//!      or which is matched by `--implements`, or `--implement` is set:
//!      seed all of its methods.
//!    * `Class` whose `TypeName` matches an explicit rule: cascade to
//!      every interface returned by `Class::required_interfaces` and
//!      every base class in `Class::bases`, seeding those interfaces'
//!      methods (mirrors the class-level method-filter cascade in
//!      `filter::push_method_filter`).
//!    * If the user supplied an allowlist (`MethodFilter::Keep`) for a
//!      type, the user's set wins and reachability defers — that type
//!      is treated as "already handled" and not added here.
//!
//! 2. **Edges.** For each kept method `M` on type `T`, look at every
//!    immediate type in `M.signature.types()` (decayed):
//!    * a referenced `Interface` or `CppInterface` `U` becomes a newly
//!      reached type — all of `U`'s methods become newly kept;
//!    * a referenced `Class` cascades to its required interfaces and
//!      bases (parallels the root rule);
//!    * generic arguments nested in `Type::Interface(_)` / `Delegate(_)`
//!      are recursed into.
//!
//! 3. **Fixpoint.** A reached interface is processed at most once — the
//!    universe is bounded by `TypeMap::filter`'s output.
//!
//! Whole-type pruning (the second tier in the design) is deferred: for
//! the first cut, types whose entire method set is opaque slots are
//! still emitted (with their IID, vtable layout, and `interface_hierarchy!`
//! invocation), so `Interface::cast::<T>()` keeps working.

use super::*;
use std::collections::{BTreeSet, HashMap, HashSet};

/// Compute the per-type method keep set for `--minimal` mode.
pub fn compute(
    reader: &Reader,
    types: &TypeMap,
    filter: &Filter,
    implements: &Implements,
    pin_all: bool,
) -> HashMap<(String, String), BTreeSet<String>> {
    let mut keep: HashMap<(String, String), BTreeSet<String>> = HashMap::new();
    let mut reached: HashSet<TypeName> = HashSet::new();
    let mut worklist: Vec<TypeName> = Vec::new();

    let pin = |tn: TypeName| pin_all || implements.matches(tn);

    // Seed roots from explicit filter rules and class cascades.
    for (tn, type_set) in types.iter() {
        let is_explicit = filter.has_explicit_type_rule(*tn);
        if !is_explicit && !pin(*tn) {
            continue;
        }

        for ty in type_set {
            match ty {
                Type::Interface(_) | Type::CppInterface(_) => {
                    reach(*tn, &mut reached, &mut worklist);
                }
                Type::Class(class) => {
                    seed_class(class, reader, &mut reached, &mut worklist);
                }
                _ => {}
            }
        }
    }

    // Fixpoint walk.
    while let Some(tn) = worklist.pop() {
        let Some(type_set) = types.get(&tn) else {
            continue;
        };

        for ty in type_set {
            match ty {
                Type::Interface(iface) => {
                    walk_interface(
                        iface,
                        reader,
                        filter,
                        &mut keep,
                        &mut reached,
                        &mut worklist,
                    );
                }
                Type::CppInterface(cpp) => {
                    walk_cpp_interface(
                        cpp,
                        reader,
                        filter,
                        &mut keep,
                        &mut reached,
                        &mut worklist,
                    );
                }
                _ => {}
            }
        }
    }

    keep
}

fn reach(tn: TypeName, reached: &mut HashSet<TypeName>, worklist: &mut Vec<TypeName>) {
    if reached.insert(tn) {
        worklist.push(tn);
    }
}

fn seed_class(
    class: &Class,
    reader: &Reader,
    reached: &mut HashSet<TypeName>,
    worklist: &mut Vec<TypeName>,
) {
    // `Class::required_interfaces` already walks base classes' interface
    // impls (and tags activation/static/composable factories), so this
    // single call covers the full forwarder surface a class would emit.
    for iface in class.required_interfaces(reader) {
        reach(iface.def.type_name(), reached, worklist);
    }
}

fn walk_interface(
    iface: &Interface,
    reader: &Reader,
    filter: &Filter,
    keep: &mut HashMap<(String, String), BTreeSet<String>>,
    reached: &mut HashSet<TypeName>,
    worklist: &mut Vec<TypeName>,
) {
    let tn = iface.def.type_name();
    let ns_name = (tn.namespace().to_string(), tn.name().to_string());

    // If the user supplied a method-level filter, defer to it entirely.
    if filter.has_user_methods(tn.namespace(), tn.name()) {
        return;
    }

    let entry = keep.entry(ns_name).or_default();

    for def in iface.def.methods() {
        let name = def.name();
        if !entry.insert(name.to_string()) {
            // Already kept on a prior visit (e.g. a different generic
            // instantiation of the same generic interface).
            continue;
        }

        // Walk the signature's immediate types and reach any referenced
        // interface/class types.
        let signature = def.method_signature(iface.def.namespace(), &iface.generics, reader);
        for ty in signature.types() {
            reach_from_type(ty, reader, reached, worklist);
        }
    }

    // Base interfaces themselves are *type-level* reachable but their
    // methods are not automatically kept — only signature mentions can
    // promote them. This is the whole point of the pass. So we do NOT
    // walk `iface.required_interfaces(reader)` here.
}

fn walk_cpp_interface(
    cpp: &CppInterface,
    reader: &Reader,
    filter: &Filter,
    keep: &mut HashMap<(String, String), BTreeSet<String>>,
    reached: &mut HashSet<TypeName>,
    worklist: &mut Vec<TypeName>,
) {
    let tn = cpp.def.type_name();
    let ns_name = (tn.namespace().to_string(), tn.name().to_string());

    if filter.has_user_methods(tn.namespace(), tn.name()) {
        return;
    }

    let entry = keep.entry(ns_name).or_default();
    let namespace = cpp.def.namespace();

    for def in cpp.def.methods() {
        let name = def.name();
        if !entry.insert(name.to_string()) {
            continue;
        }

        let method = CppMethod::new(def, namespace, reader);
        for ty in method.signature.types() {
            reach_from_type(ty, reader, reached, worklist);
        }
    }
}

/// Recursively extract reachable interface / class type names from a
/// signature `Type`. `ty` has already been `decay`'d by `Signature::types`.
fn reach_from_type(
    ty: &Type,
    reader: &Reader,
    reached: &mut HashSet<TypeName>,
    worklist: &mut Vec<TypeName>,
) {
    let ty = ty.decay();
    match ty {
        Type::Interface(iface) => {
            reach(iface.def.type_name(), reached, worklist);
            for g in &iface.generics {
                reach_from_type(g, reader, reached, worklist);
            }
        }
        Type::CppInterface(cpp) => {
            reach(cpp.def.type_name(), reached, worklist);
        }
        Type::Class(class) => {
            // Cascade to the class's required interfaces — callers can
            // get one of these out of a return value and immediately
            // call methods on it.
            seed_class(class, reader, reached, worklist);
        }
        Type::Delegate(d) => {
            for g in &d.generics {
                reach_from_type(g, reader, reached, worklist);
            }
        }
        _ => {}
    }
}
