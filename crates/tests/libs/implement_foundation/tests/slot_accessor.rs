//! Step 2b prep: exercises `Outer::as_slot_interface` for **non-head**
//! interface positions. The tuple `(I0, I1)` only provides
//! `Implements<I0>` (see `crates/libs/core/src/imp/implement/list.rs:62-68`),
//! so reaching the `I1` vtable slot of an `Outer<T, (I0, I1)>` requires
//! the unsafe slot-indexed accessor. The reskinned `implement_decl!`
//! (Step 2b) will emit per-declared-interface `ComObjectInterface<Ik>`
//! using exactly this accessor.

use windows_core::imp::{Agile, Implemented, Outer};
use windows_core::{ComObjectInner, IUnknown, Interface};

// Two distinct fake interfaces; both inherit from IUnknown, both empty.

#[windows_core::interface("aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa")]
unsafe trait IFirst: IUnknown {}

#[windows_core::interface("bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb")]
unsafe trait ISecond: IUnknown {}

struct Pair;

impl Implemented for Pair {
    type Interfaces = (IFirst, ISecond);
    type Agility = Agile;
}

// User-supplied method bodies (empty traits, but the impls are required
// for the `Outer<Pair, (IFirst, ISecond)>` to type-check).
impl IFirst_Impl for Outer<Pair, (IFirst, ISecond)> {}
impl ISecond_Impl for Outer<Pair, (IFirst, ISecond)> {}

#[test]
fn slot_one_returns_first_vtable() {
    let obj = Pair.into_object();
    let outer: &Outer<Pair, (IFirst, ISecond)> = &*obj;

    // Slot 1 is the first declared interface — IFirst.
    let first_ref = unsafe { outer.as_slot_interface::<IFirst, 1>() };

    // Round-trip through the raw pointer to confirm we got a live vtable
    // pointing at IFirst's IID via QueryInterface.
    let first: IFirst = first_ref.to_owned();
    let _: IFirst = first.cast().unwrap(); // QI to self succeeds
    let _: IUnknown = first.cast().unwrap();
    // QI to ISecond also succeeds because both belong to the same outer.
    let _: ISecond = first.cast().unwrap();
}

#[test]
fn slot_two_returns_second_vtable() {
    let obj = Pair.into_object();
    let outer: &Outer<Pair, (IFirst, ISecond)> = &*obj;

    // Slot 2 is the second declared interface — ISecond.
    let second_ref = unsafe { outer.as_slot_interface::<ISecond, 2>() };
    let second: ISecond = second_ref.to_owned();

    // QI back to IFirst proves the slot really belonged to the same outer.
    let _: IFirst = second.cast().unwrap();
}

#[test]
fn slots_one_and_two_point_at_different_vtables() {
    // The two slots must yield distinct pointers (different vtable
    // storage cells) — otherwise the slot arithmetic is wrong and one
    // of the per-declared-interface impls would alias the other.
    let obj = Pair.into_object();
    let outer: &Outer<Pair, (IFirst, ISecond)> = &*obj;

    let first_ref = unsafe { outer.as_slot_interface::<IFirst, 1>() };
    let second_ref = unsafe { outer.as_slot_interface::<ISecond, 2>() };

    let p1: *const IFirst = &*first_ref as *const _;
    let p2: *const ISecond = &*second_ref as *const _;
    assert_ne!(p1 as usize, p2 as usize);
}
