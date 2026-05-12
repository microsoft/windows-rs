//! Step 0 spike for `docs/option-d.md` — a library-based foundation for `#[implement]`.
//!
//! This crate proves that the proposed `Outer<T, L>` shape:
//!
//!  1. Compiles on stable Rust with no nightly features (no `generic_const_exprs`,
//!     no specialisation, no GAT lifetimes).
//!  2. Yields a `repr(C)` layout byte-identical to today's `#[implement]`-emitted
//!     `Foo_Impl` struct.
//!  3. Accepts a `VtableCtor<T, OFFSET>` opt-in for an arbitrary, hand-rolled
//!     interface vtbl (here, `IValue_Vtbl`) without changes to `windows-interface`
//!     or `windows-bindgen`.
//!
//! What this spike intentionally does **not** do (deferred to phase 2 of step 0):
//!
//!  * Implement `IUnknownImpl` for `Outer<T, L>` (the generic `QueryInterface`
//!    body, agile/IMarshal handling, dynamic-cast carve-out, aggregation
//!    fall-through, refcount lifecycle).
//!  * The OQ-4 microbenchmark.
//!
//! Phase 1 below is what answers "does the type system accept this design?", which
//! is the load-bearing question for OQ-2 (where `VtableCtor` lives) and OQ-6
//! (`new` / `new_generic` constructor split). Phase 2 is what answers OQ-4.
//!
//! Layout proof lives in `tests/layout.rs`.

#![no_std]
#![allow(non_camel_case_types, non_snake_case, dead_code)]

extern crate alloc;

pub mod foundation;
pub mod sample;
