//! Library-based foundation for `#[implement]` — see `docs/option-d.md`.
//!
//! The proc macro emits a `#[repr(C)]` outer struct, vtable consts, an
//! `IUnknownImpl` impl with a hand-rolled `QueryInterface`, and per-interface
//! `From` / `ComObjectInterface<I>` / `AsImpl` impls for every `#[implement]`
//! use site. This module hosts the same machinery as ordinary generic code:
//! [`Outer<T, L>`] is the byte-identical replacement for the macro's
//! `Foo_Impl`, and the blanket impls below replace each per-use-site
//! emission.
//!
//! The proc macro is **not** rewritten yet — that's Step 2 in
//! `docs/option-d.md`'s migration plan. For now the foundation lives
//! alongside the macro and is exercised by the integration tests under
//! `crates/tests/libs/implement_foundation`. Hand-written use of the
//! foundation works without invoking any proc macro, which is the path
//! that `implement_decl!` (Step 3) targets.

mod agility;
mod list;
mod outer;
mod runtime;
mod sealed;
mod storage;
mod vtbl;

pub use agility::{Agile, Agility, NonAgile};
pub use list::{Implements, InterfaceList, ListVtables};
pub use outer::{Implemented, Outer};
pub use storage::{VCons, VNil};
pub use vtbl::VtableCtor;
