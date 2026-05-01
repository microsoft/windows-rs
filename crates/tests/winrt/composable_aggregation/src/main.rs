//! Linux-runnable harness for windows-rs composable / aggregation support.
//!
//! Mirrors the bindgen-emitted shape from
//! `crates/tests/fixtures/harness/data/bindgen/composable_class/expected.rs`,
//! but bypasses `FactoryCache::call` (which requires `RoGetActivationFactory`)
//! by instantiating a mock `IFooFactory` directly. This lets us exercise the
//! `Compose::compose` path end-to-end on a non-Windows host.

mod bindings;
use bindings::{Foo, IFoo, IFooFactory, IFooFactory_Impl, IFoo_Impl};
use std::sync::atomic::{AtomicUsize, Ordering};
use windows_core::{Compose, IInspectable, Interface, OutRef, Ref, Result};
use windows_implement::implement;

const DERIVED_VALUE: i32 = 42;
const INNER_VALUE: i32 = 7;

// Drop counters let us verify outer/inner teardown order. The aggregation
// contract is: dropping the outer must `Release` the inner stored in
// `ComposeBase`, so `INNER_DROPS` must reach 1 before the program exits, and
// it must do so as a side effect of the outer (`DERIVED_DROPS`) being dropped.
static DERIVED_DROPS: AtomicUsize = AtomicUsize::new(0);
static INNER_DROPS: AtomicUsize = AtomicUsize::new(0);

/// Stands in for the "inner non-delegating IInspectable" that a real WinRT
/// composable factory would create. It implements `IFoo` so we can detect when
/// `QueryInterface` falls through from the derived (outer) to the inner via
/// `aggregation_query` in the `#[implement]`-generated QueryInterface.
#[implement(IFoo)]
struct Inner;

impl IFoo_Impl for Inner_Impl {
    fn Hello(&self) -> Result<i32> {
        Ok(INNER_VALUE)
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        INNER_DROPS.fetch_add(1, Ordering::SeqCst);
        eprintln!("Inner::drop");
    }
}

/// Rust derived type that aggregates `Foo`. Hello returns `DERIVED_VALUE` so we
/// can tell which side of the aggregate dispatched a call.
#[implement(IFoo)]
struct Derived;

impl IFoo_Impl for Derived_Impl {
    fn Hello(&self) -> Result<i32> {
        Ok(DERIVED_VALUE)
    }
}

impl Drop for Derived {
    fn drop(&mut self) {
        DERIVED_DROPS.fetch_add(1, Ordering::SeqCst);
        eprintln!("Derived::drop");
    }
}

/// Mock CRT-side factory. Plays the role normally played by the WinRT runtime
/// class implementation living in (e.g.) `Windows.UI.Xaml.dll`.
#[implement(IFooFactory)]
struct MockFactory;

impl IFooFactory_Impl for MockFactory_Impl {
    fn CreateInstance(&self, outer: Ref<IInspectable>, inner: OutRef<IInspectable>) -> Result<Foo> {
        // Aggregation contract:
        //   * `outer` is the controlling-unknown supplied by the derived caller.
        //   * `inner` receives the freshly-allocated non-delegating IInspectable
        //     that the outer will use to resolve QI for interfaces it doesn't
        //     handle locally.
        //   * Return value is the outer's default interface for the runtime
        //     class (here, `IFoo` reinterpreted as `Foo`).
        let inner_inspectable: IInspectable = Inner.into();
        inner.write(Some(inner_inspectable))?;

        let outer = outer.ok()?;
        outer.cast::<Foo>()
    }
}

fn main() -> Result<()> {
    let factory: IFooFactory = MockFactory.into();

    // Manually drive what `Foo::compose::<Derived>(Derived)` would do, minus
    // the `FactoryCache::call` step that depends on `RoGetActivationFactory`.
    let derived = Derived;
    // SAFETY: `outer` is kept alive across the factory call below; `base_slot`
    // points into the heap allocation that backs `outer`.
    let (outer, base_slot) = unsafe { Compose::compose(derived) };

    let foo = factory.CreateInstance(&outer, base_slot)?;
    // Keep `outer` alive until after the factory has populated `base_slot`.
    let _ = &outer;

    // The base slot must now contain the inner produced by the factory.
    assert!(
        base_slot.is_some(),
        "factory must have populated inner slot"
    );

    // 1. Hello() on the returned Foo (outer's IFoo) dispatches to the derived
    //    implementation, not the inner.
    let n = foo.Hello()?;
    assert_eq!(
        n, DERIVED_VALUE,
        "Hello() on outer should dispatch to derived (got {n}, expected {DERIVED_VALUE})"
    );

    // 2. QI for IInspectable from the outer succeeds locally (identity_query).
    let _: IInspectable = foo.cast()?;

    // 3. QI fall-through: the outer (Derived_Impl) has only IFoo + IInspectable
    //    + IUnknown in its interface chain. Issuing a raw QueryInterface for an
    //    arbitrary IID that *neither* outer nor inner implements should reach
    //    the `aggregation_query` branch in the generated QueryInterface, which
    //    calls `query` on the inner stored in `ComposeBase`. The inner returns
    //    E_NOINTERFACE for unknown IIDs. We just need to confirm we don't
    //    crash and we get a clean E_NOINTERFACE.
    let unknown_iid = windows_core::GUID::from_u128(0xdead_beef_dead_beef_dead_beef_dead_beef);
    unsafe {
        use windows_core::HRESULT;
        let mut sink: *mut core::ffi::c_void = core::ptr::null_mut();
        let outer_unk: &windows_core::IUnknown = (&outer).into();
        let hr: HRESULT = (Interface::vtable(outer_unk).QueryInterface)(
            Interface::as_raw(outer_unk),
            &unknown_iid,
            &mut sink,
        );
        assert!(
            hr.is_err(),
            "expected QI for bogus IID to fail; got hr=0x{:08x} sink={:p}",
            hr.0,
            sink
        );
        assert!(sink.is_null(), "failed QI must leave out-pointer null");
    }

    // 4. Drop order: drop `foo` first (releases one strong ref on outer), then
    //    drop `outer`. That last drop must run `Derived::drop` *and* release
    //    the inner stored in `ComposeBase`, which in turn runs `Inner::drop`.
    drop(foo);
    assert_eq!(
        DERIVED_DROPS.load(Ordering::SeqCst),
        0,
        "Derived must still be alive after dropping foo (outer still referenced)"
    );
    assert_eq!(
        INNER_DROPS.load(Ordering::SeqCst),
        0,
        "Inner must outlive the outer"
    );

    drop(outer);
    assert_eq!(
        DERIVED_DROPS.load(Ordering::SeqCst),
        1,
        "Derived must be dropped exactly once after the last outer ref is gone"
    );
    assert_eq!(
        INNER_DROPS.load(Ordering::SeqCst),
        1,
        "Inner must be dropped as a side-effect of dropping the outer"
    );

    println!("composable aggregation OK: foo.Hello() = {n}");
    Ok(())
}
