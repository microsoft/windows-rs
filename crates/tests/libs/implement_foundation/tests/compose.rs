//! Tests the `Compose` blanket added in Step 2a. The blanket lives at
//! `crates/libs/core/src/imp/implement/runtime.rs` and replaces the
//! per-use-site `Compose` impl that today's `#[implement]` and
//! `implement_decl!` emit (mirrors `crates/libs/core/src/implement_macro.rs:456`
//! and `crates/libs/implement/src/gen.rs:292`).

use test_implement_foundation::sample::foundation_path::Foo;
use windows_core::{Compose, IInspectable, Interface};

#[test]
fn compose_returns_inspectable_and_writable_base_slot() {
    // SAFETY: we hold the returned `IInspectable` for the entire lifetime
    // of the borrow into the base slot, satisfying the safety contract
    // documented on `Compose::compose`.
    let (inspectable, base) = unsafe { Compose::compose(Foo { x: 99 }) };

    // The base slot starts empty (set only when the WinRT runtime writes
    // back the inner non-delegating `IInspectable` during aggregation).
    assert!(base.is_none());

    // We can write to the base slot — proves it's a live mutable reference
    // into the boxed outer object, not a stale or read-only view.
    *base = Some(inspectable.clone());
    assert!(base.is_some());

    // The returned `IInspectable` points at the identity vtable of the
    // outer object — round-tripping it as `IInspectable` keeps the same
    // raw pointer. (Casting back to a non-identity interface would go
    // through `QueryInterface`; here we stay on the identity slot.)
    let raw1 = Interface::as_raw(&inspectable);
    let inspectable2: IInspectable = inspectable.clone();
    let raw2 = Interface::as_raw(&inspectable2);
    assert_eq!(raw1, raw2);

    // Drop the surplus references so the object can free.
    drop(inspectable2);
    *base = None;
    drop(inspectable);
}
