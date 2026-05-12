//! HList storage cells.
//!
//! These are the runtime backing for `Outer<T, L>::vtables`. Each `VCons` cell
//! is `#[repr(C)]` with `vtable: &'static I::Vtable` first and `rest: R`
//! second. Because `&'static T` is pointer-sized and pointer-aligned on every
//! supported target, and because `repr(C)` lays out fields in declaration order
//! with natural alignment, a `VCons<I0, VCons<I1, VNil>>` has exactly the same
//! in-memory representation as two adjacent `&'static _` fields — i.e.
//! identical to today's `Foo_Impl` chain fields.
//!
//! `VNil` is zero-sized: unit-style empty structs (`pub struct VNil;`) have
//! size and alignment 1, *but* — under `repr(C)` and as the tail of a
//! sequence of larger fields — Rust does not add trailing padding for a
//! 1-byte alignment, so the terminal `rest: VNil` in a
//! `VCons<.., VCons<.., VNil>>` contributes zero bytes to the outer struct's
//! layout. The whole tower folds down to a pure sequence of `&'static _`
//! references. This is asserted byte-for-byte in `tests/layout.rs`.
//!
//! This is the central layout claim of `docs/option-d.md`. The `tests/layout.rs`
//! integration test asserts it byte-for-byte against today's macro output.

use windows_core::Interface;

/// Empty list terminator. Zero-sized, zero-alignment.
#[repr(C)]
pub struct VNil;

/// A non-empty list cell — `vtable` for one interface chain, then `rest` for
/// the remaining chains.
///
/// Critically `repr(C)` and `vtable` first: the byte at offset 0 of any
/// non-empty `VCons<...>` tower is the first chain's vtable pointer, exactly
/// matching today's `Foo_Impl::interface1` field.
#[repr(C)]
pub struct VCons<I: Interface + 'static, R: 'static> {
    pub vtable: &'static I::Vtable,
    pub rest: R,
}
