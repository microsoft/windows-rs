use crate::{interface_generic as full, interface_generic_minimal as minimal};
use windows_core::{GUID, IInspectable, IUnknown, Interface, RuntimeName, RuntimeType};

type FullPair = full::IPair<full::IVector<i32>, full::IValue>;
type MinimalPair = minimal::IPair<minimal::IVector<i32>, minimal::IValue>;

#[test]
fn generic_interface_metadata() {
    let signature = concat!(
        "pinterface({02b51929-c1c4-4a7e-8940-0312b5c18500};",
        "pinterface({1b2776f1-623d-5a07-b5d8-9a225dfc1901};i4);",
        "{11111111-2222-3333-4444-555555555555})"
    );
    assert_eq!(FullPair::SIGNATURE.as_slice(), signature.as_bytes());
    assert_eq!(MinimalPair::SIGNATURE.as_slice(), signature.as_bytes());
    // UUID v5 of the signature in the WinRT parameterized-interface namespace.
    let iid = GUID::from_u128(0x564e964a_7b94_57d4_9fee_15d6a865f3d4);
    assert_eq!(FullPair::IID, iid);
    assert_eq!(MinimalPair::IID, iid);
    assert_eq!(
        <FullPair as RuntimeType>::NAME.as_slice(),
        b"Test.IPair`2<Test.IVector`1<Int32>, Test.IValue>"
    );
    assert_eq!(
        FullPair::RUNTIME_CLASS_NAME.as_slice(),
        <FullPair as RuntimeType>::NAME.as_slice()
    );
    assert_eq!(
        <full::IValue as RuntimeType>::NAME.as_slice(),
        b"Test.IValue"
    );
    assert!(<MinimalPair as RuntimeType>::NAME.as_slice().is_empty());
    assert!(minimal::IValue::NAME.as_slice().is_empty());
    assert_eq!(
        full::IValue::SIGNATURE.as_slice(),
        b"{11111111-2222-3333-4444-555555555555}"
    );
    assert_eq!(
        full::IValue::SIGNATURE.as_slice(),
        minimal::IValue::SIGNATURE.as_slice()
    );
    assert_eq!(size_of::<FullPair>(), size_of::<IUnknown>());
    assert_eq!(align_of::<FullPair>(), align_of::<IUnknown>());
    assert_eq!(size_of::<Option<MinimalPair>>(), size_of::<IUnknown>());
}

#[test]
fn generic_interface_identity_and_debug() {
    #[windows_core::implement(full::IPair<full::IVector<i32>, full::IValue>)]
    struct Pair;
    impl full::IPair_Impl<full::IVector<i32>, full::IValue> for Pair_Impl {}

    let pair: FullPair = Pair.into();
    let minimal: MinimalPair = pair.cast().unwrap();
    assert_eq!(pair.as_raw(), minimal.as_raw());
    assert_eq!(pair, pair.clone());
    let unknown = pair.cast::<IUnknown>().unwrap();
    let inspectable = pair.cast::<IInspectable>().unwrap();
    assert_eq!(unknown.as_raw(), inspectable.as_raw());
    #[cfg(all(windows, not(miri)))]
    assert_eq!(
        inspectable.GetRuntimeClassName().unwrap(),
        "Test.IPair`2<Test.IVector`1<Int32>, Test.IValue>"
    );
    unsafe {
        assert_eq!(
            windows_core::Param::<IInspectable>::param(&pair).abi(),
            pair.as_raw()
        );
        assert_eq!(
            windows_core::Param::<IUnknown>::param(&minimal).abi(),
            minimal.as_raw()
        );
    }
}
