use super::*;
use Composition::*;
use Windows::Foundation::{IStringable, IStringable_Impl};
use core::sync::atomic::{AtomicI32, Ordering};

static DERIVED: AtomicI32 = AtomicI32::new(0);

// Overrides `Name` and adds an interface that the base class does not implement.
#[implement(IBaseOverrides, IStringable)]
struct Derived;

impl Derived {
    fn new() -> Self {
        DERIVED.fetch_add(1, Ordering::Relaxed);
        Self
    }
}

impl Drop for Derived {
    fn drop(&mut self) {
        DERIVED.fetch_sub(1, Ordering::Relaxed);
    }
}

impl IBaseOverrides_Impl for Derived_Impl {
    fn Name(&self) -> Result<HSTRING> {
        Ok("Rust".into())
    }
}

impl IStringable_Impl for Derived_Impl {
    fn ToString(&self) -> Result<HSTRING> {
        Ok("Stringable".into())
    }
}

// Derives without overriding anything.
#[implement]
struct Plain;

pub fn test() -> Result<()> {
    assert_eq!(Base::Instances()?, 0);

    let base = Base::WithValue(5)?;
    assert_eq!(base.Value()?, 5);
    assert_eq!(base.Describe()?, "Base(5)");
    assert_eq!(base.Stringify()?, "");
    assert_eq!(Base::Instances()?, 1);
    drop(base);
    assert_eq!(Base::Instances()?, 0);

    let base = Base::WithValue_compose(7, Derived::new())?;

    // The base calls the override through the outer object.
    assert_eq!(base.Value()?, 7);
    assert_eq!(base.Describe()?, "Rust(7)");
    assert_eq!(base.Stringify()?, "Stringable");

    // Rust has no protected members, so the projection makes them public.
    assert_eq!(base.Doubled()?, 14);

    // QueryInterface reaches the derived and base interfaces from either side, with one identity.
    let stringable: IStringable = base.cast()?;
    assert_eq!(stringable.ToString()?, "Stringable");
    assert_eq!(base.Name()?, "Rust");
    assert_eq!(stringable.cast::<Base>()?.Value()?, 7);
    assert_eq!(base.cast::<IUnknown>()?, stringable.cast::<IUnknown>()?);

    assert_eq!(Base::Instances()?, 1);
    assert_eq!(DERIVED.load(Ordering::Relaxed), 1);
    drop(stringable);
    drop(base);
    assert_eq!(Base::Instances()?, 0);
    assert_eq!(DERIVED.load(Ordering::Relaxed), 0);

    let base = Base::compose(Plain)?;
    assert_eq!(base.Describe()?, "Base(0)");
    assert_eq!(base.Stringify()?, "");
    assert_eq!(base.Name()?, "Base");
    assert!(base.cast::<IStringable>().is_err());
    assert_eq!(Base::Instances()?, 1);
    drop(base);
    assert_eq!(Base::Instances()?, 0);

    Ok(())
}
