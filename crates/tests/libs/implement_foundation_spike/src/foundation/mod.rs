//! The proposed library-based foundation for `#[implement]`.
//!
//! Each item here corresponds 1:1 to a piece of `docs/option-d.md`. Names match
//! the doc verbatim so a reviewer can cross-reference each declaration.
//!
//! Hierarchy:
//!
//! * [`storage`] — `VNil` / `VCons<I, R>` HList cells (the runtime storage).
//! * [`list`]    — `InterfaceList` + `Implements<I>` + tuple impls for arity 0..=4.
//! * [`agility`] — sealed `Agility` marker + `Agile` / `NonAgile`.
//! * [`vtbl`]    — `VtableCtor<T, OFFSET>` + the per-`_Vtbl` opt-in pattern.
//! * [`outer`]   — `Outer<T, L>` and the `Implemented` user trait.
//!
//! Sealing pattern (used throughout): a private module exports a `Sealed` marker
//! trait; public traits inherit from it; downstream crates cannot extend them
//! without also extending `Sealed`, which they cannot reach.

pub mod agility;
pub mod list;
pub mod outer;
pub mod sealed;
pub mod storage;
pub mod vtbl;

pub use agility::{Agile, Agility, NonAgile};
pub use list::{Implements, InterfaceList};
pub use outer::{Implemented, Outer};
pub use storage::{VCons, VNil};
pub use vtbl::{HasVtblCtor, VtableCtor};
