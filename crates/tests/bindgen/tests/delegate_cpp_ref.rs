use test_bindgen::delegate_cpp_ref::*;
use windows_core::*;

#[test]
fn test_factory() {
    static FACTORY: StaticComObject<Factory> = Factory.into_static();

    #[implement(IActivationFactory)]
    struct Factory;

    impl IActivationFactory_Impl for Factory_Impl {
        fn ActivateInstance(&self) -> Result<IInspectable> {
            todo!()
        }
    }

    unsafe extern "system" fn callback(
        name: Ref<windows_core::HSTRING>,
        factory: OutRef<IActivationFactory>,
    ) -> HRESULT {
        if *name == "Factory" {
            factory.write(Some(FACTORY.to_interface())).into()
        } else {
            _ = factory.write(None);
            CLASS_E_CLASSNOTAVAILABLE
        }
    }

    // This is a roundabout way to test that PFNGETACTIVATIONFACTORY can be both implemented and called in Rust.
    let _callback: PFNGETACTIVATIONFACTORY = Some(callback);

    unsafe {
        let mut factory: Option<IActivationFactory> = None;

        assert_eq!(
            CLASS_E_CLASSNOTAVAILABLE,
            callback(
                // transmute_copy is required here since Ref doesn't have a lifetime parameter and thus is unsafe
                // to implement From or construct a Ref without unsafe code.
                std::mem::transmute_copy(h!("invalid")),
                // OutRef provides a safe From implementation since it includes a lifetime parameter.
                OutRef::from(&mut factory)
            )
        );

        assert!(factory.is_none());

        assert_eq!(
            S_OK,
            callback(
                std::mem::transmute_copy(h!("Factory")),
                OutRef::from(&mut factory)
            )
        );

        assert_eq!(
            FACTORY.to_interface::<IActivationFactory>(),
            factory.unwrap()
        );
    }
}
