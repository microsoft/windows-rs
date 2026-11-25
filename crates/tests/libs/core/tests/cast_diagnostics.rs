// Runs only when built with `RUSTFLAGS=--cfg windows_cast_diagnostics`.
#![cfg(all(windows, windows_cast_diagnostics))]
#![expect(non_snake_case)]

use test_core::*;
use windows_core::*;

#[interface("8a4b0e8e-0ec1-4f4f-9d1a-0f5c2f6f4a10")]
unsafe trait IBase: IUnknown {
    unsafe fn Base(&self) -> u32;
}

#[interface("8a4b0e8e-0ec1-4f4f-9d1a-0f5c2f6f4a11")]
unsafe trait IDerived: IBase {
    unsafe fn Derived(&self) -> u32;
}

#[interface("8a4b0e8e-0ec1-4f4f-9d1a-0f5c2f6f4a12")]
unsafe trait IOther: IUnknown {
    unsafe fn Other(&self) -> u32;
}

#[interface("8a4b0e8e-0ec1-4f4f-9d1a-0f5c2f6f4a13")]
unsafe trait IMissing: IUnknown {}

#[implement(IDerived, IOther)]
struct Test;

impl IBase_Impl for Test_Impl {
    unsafe fn Base(&self) -> u32 {
        1
    }
}

impl IDerived_Impl for Test_Impl {
    unsafe fn Derived(&self) -> u32 {
        2
    }
}

impl IOther_Impl for Test_Impl {
    unsafe fn Other(&self) -> u32 {
        3
    }
}

fn count<S, T>(counts: &[imp::CastCount], line: u32) -> u64 {
    let source = std::any::type_name::<S>();
    let target = std::any::type_name::<T>();
    counts
        .iter()
        .filter(|count| {
            count.location.line() == line && count.source == source && count.target == target
        })
        .map(|count| count.count)
        .sum()
}

#[test]
fn counts_casts_per_call_site() -> Result<()> {
    let derived: IDerived = Test.into();
    let reference: IPropertyValue = PropertyValue::CreateInt32(1)?.cast()?;
    let line = line!() + 2;
    for _ in 0..3 {
        let other: IOther = derived.cast()?;
        assert!(other.cast::<IMissing>().is_err());
        let base: IBase = derived.cast()?;
        let _: IDerived = base.cast()?;
        let _: IDerived = derived.cast()?;
        let _: IInspectable = reference.cast()?;
    }

    let counts = imp::take_cast_counts();
    assert_eq!(count::<IDerived, IOther>(&counts, line), 3);
    assert_eq!(count::<IOther, IMissing>(&counts, line + 1), 3);
    assert_eq!(count::<IDerived, IBase>(&counts, line + 2), 3);
    assert_eq!(count::<IBase, IDerived>(&counts, line + 3), 3);
    assert!(imp::take_cast_counts().is_empty());

    // Same-IID and IInspectable casts are avoidable. The upcast is only a hint, since the same
    // pointer for a shorter vtable does not prove inheritance.
    let file = file!();
    let mut avoidable: Vec<_> = imp::avoidable_casts()
        .iter()
        .filter(|cast| cast.location.file() == file)
        .map(|cast| (cast.location.line(), cast.target))
        .collect();
    avoidable.sort();
    assert_eq!(
        avoidable,
        [
            (line + 4, std::any::type_name::<IDerived>()),
            (line + 5, std::any::type_name::<IInspectable>()),
        ]
    );
    Ok(())
}
