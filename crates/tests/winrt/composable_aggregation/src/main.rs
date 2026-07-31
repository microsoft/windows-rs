//! Linux-runnable composable aggregation test with a mock `IFooFactory`.

#[allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
mod bindings;
use bindings::{Foo, IFoo, IFoo_Impl, IFooFactory, IFooFactory_Impl};
use std::sync::atomic::{AtomicUsize, Ordering};
use windows_core::{Compose, IInspectable, Interface, OutRef, Ref, Result};
use windows_implement::implement;

const DERIVED_VALUE: i32 = 42;
const INNER_VALUE: i32 = 7;

static DERIVED_DROPS: AtomicUsize = AtomicUsize::new(0);
static INNER_DROPS: AtomicUsize = AtomicUsize::new(0);

// The inner implements `IFoo` to exercise `aggregation_query` fallback.
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

#[implement(IFooFactory)]
struct MockFactory;

impl IFooFactory_Impl for MockFactory_Impl {
    fn CreateInstance(&self, outer: Ref<IInspectable>, inner: OutRef<IInspectable>) -> Result<Foo> {
        let inner_inspectable: IInspectable = Inner.into();
        inner.write(Some(inner_inspectable))?;

        let outer = outer.ok()?;
        outer.cast::<Foo>()
    }
}

fn main() -> Result<()> {
    let factory: IFooFactory = MockFactory.into();

    let derived = Derived;
    // SAFETY: `outer` is kept alive across the factory call below; `base_slot`
    // points into the heap allocation that backs `outer`.
    let (outer, base_slot) = unsafe { Compose::compose(derived) };

    let foo_outer = factory.CreateInstance(&outer, base_slot)?;
    // The factory writes through `base_slot`, which points into `outer`.
    let _ = &outer;

    assert!(
        base_slot.is_some(),
        "factory must have populated inner slot"
    );

    let n = foo_outer.Hello()?;
    assert_eq!(
        n, DERIVED_VALUE,
        "Hello() on outer should dispatch to derived (got {n}, expected {DERIVED_VALUE})"
    );

    let _: IInspectable = foo_outer.cast()?;

    // An unknown IID reaches the generated `aggregation_query` fallback.
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

    // The final outer release must also release the inner stored in `ComposeBase`.
    drop(foo_outer);
    assert_eq!(
        DERIVED_DROPS.load(Ordering::SeqCst),
        0,
        "Derived must still be alive after dropping foo_outer (outer still referenced)"
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

    println!("composable aggregation OK: foo_outer.Hello() = {n}");
    Ok(())
}
