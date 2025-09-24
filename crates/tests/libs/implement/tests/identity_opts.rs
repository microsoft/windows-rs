use windows_core::*;

#[interface("e2a0713e-6f6b-4b0c-9a2f-9a0b4a7a2b88")]
unsafe trait ITest: IUnknown {
    unsafe fn Ping(&self) -> HRESULT;
}

#[implement(ITest, Identity = IUnknown, Agile = false)]
struct UnknownOnly;

impl ITest_Impl for UnknownOnly_Impl {
    unsafe fn Ping(&self) -> HRESULT {
        HRESULT(0)
    }
}

#[test]
fn identity_iunknown_no_inspectable_no_agile() {
    unsafe {
        // Create object implementing ITest, with IUnknown identity and without IAgileObject.
        let test: ITest = UnknownOnly.into();

        // IInspectable should not be supported.
        assert!(test.cast::<IInspectable>().is_err());

        // IAgileObject should not be supported.
        assert!(test.cast::<windows_core::imp::IAgileObject>().is_err());

        // IUnknown should always work (already true by interface hierarchy).
        let _base: IUnknown = test.clone().into();
        let _ = _base; // silence unused warning
    }
}

