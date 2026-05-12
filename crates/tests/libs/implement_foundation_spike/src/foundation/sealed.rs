//! Sealing module — see `docs/option-d.md` ("`InterfaceList` is `Sealed` so we
//! can extend it without breaking users").
//!
//! Public traits in this crate that should not be extensible by downstream
//! crates inherit from `Sealed`. `Sealed` has no public constructors; only this
//! crate can implement it for a given type.

pub trait Sealed {}
