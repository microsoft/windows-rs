//! Step 1 integration tests for the `windows_core::imp::implement` foundation.
//!
//! `windows_core::imp::implement` re-exports the foundation types
//! (`Outer`, `Implemented`, `InterfaceList`, `Implements`, `ListVtables`,
//! `Agile`, `NonAgile`, `Agility`, `VCons`, `VNil`, `VtableCtor`)
//! `#[doc(hidden)]`. This crate consumes them as if it were a downstream
//! user — including the per-`_Vtbl` `VtableCtor` opt-in that
//! `windows-interface` / `windows-bindgen` will emit in Step 1b.
//!
//! Tests live under `tests/`:
//!
//! * `layout.rs`  — byte-identical layout proof against today's
//!   `#[implement(IValue)] Foo`'s `Foo_Impl`.
//! * `runtime.rs` — construct, AddRef/Release, QI dispatch for identity /
//!   declared / unknown IIDs, From-conversions, method-dispatch round-trip.
//! * `bench_qi.rs` — OQ-4 microbenchmark (ignored by default).

#![allow(non_camel_case_types)]

pub mod sample;
