//! Sealed `Agility` marker trait + the two implementors `Agile` / `NonAgile`.
//!
//! See `docs/option-d.md` (resolution of OQ-3 — sealed marker types over
//! const generics).

use super::sealed::Sealed;

/// Sealed marker that selects the agility axis on a `#[implement]` type.
pub trait Agility: Sealed {
    /// Whether `QueryInterface(IID_IAgileObject)` succeeds.
    const IS_AGILE: bool;

    /// Whether the object exposes `IMarshal` via the framework's standard
    /// free-threaded marshaler. Forced to `false` outside Windows.
    const HAS_MARSHAL: bool;
}

/// Default agility: agile, with `IMarshal` exposed on Windows.
pub struct Agile;
impl Sealed for Agile {}
impl Agility for Agile {
    const IS_AGILE: bool = true;
    const HAS_MARSHAL: bool = cfg!(windows);
}

/// Opt-out: object is not agile, no `IMarshal`.
pub struct NonAgile;
impl Sealed for NonAgile {}
impl Agility for NonAgile {
    const IS_AGILE: bool = false;
    const HAS_MARSHAL: bool = false;
}
