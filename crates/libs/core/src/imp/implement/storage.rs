//! HList storage cells.
//!
//! Runtime backing for [`Outer<T, L>::vtables`]. `VCons` is `#[repr(C)]` with
//! `vtable: &'static I::Vtable` first and `rest: R` second. Because
//! `&'static T` is pointer-sized and pointer-aligned on every supported
//! target and `repr(C)` lays out fields in declaration order with natural
//! alignment, a `VCons<I0, VCons<I1, VNil>>` has exactly the same in-memory
//! representation as two adjacent `&'static _` fields — i.e. byte-identical
//! to today's `Foo_Impl` chain fields.
//!
//! `VNil` is zero-sized: under `repr(C)` and as the tail of a sequence of
//! larger fields, the terminal `rest: VNil` contributes zero bytes.
//!
//! [`Outer<T, L>::vtables`]: super::Outer

use crate::Interface;

/// Empty list terminator. Zero-sized.
#[repr(C)]
pub struct VNil;

/// A non-empty list cell — `vtable` for one interface chain, then `rest` for
/// the remaining chains.
#[repr(C)]
pub struct VCons<I: Interface + 'static, R: 'static> {
    pub vtable: &'static I::Vtable,
    pub rest: R,
}
