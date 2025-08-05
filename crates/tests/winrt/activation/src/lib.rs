mod bindings;

use windows::{core::*, Win32::Foundation::*, Win32::System::WinRT::*};

#[no_mangle]
unsafe extern "system" fn DllGetActivationFactory(
    name: Ref<HSTRING>,
    factory: OutRef<IActivationFactory>,
) -> HRESULT {
    if *name == "test_activation.One.Instance" {
        factory.write(Some(InstanceFactory.into())).into()
    } else if *name == "test_activation.One.Two.Three.Four.Static" {
        factory.write(Some(StaticFactory.into())).into()
    } else {
        _ = factory.write(None);
        CLASS_E_CLASSNOTAVAILABLE
    }
}

#[implement(IActivationFactory)]
struct InstanceFactory;

impl IActivationFactory_Impl for InstanceFactory_Impl {
    fn ActivateInstance(&self) -> Result<IInspectable> {
        Ok(Instance.into())
    }
}

#[implement(bindings::Instance)]
struct Instance;

impl bindings::IInstance_Impl for Instance_Impl {
    fn Property(&self) -> Result<i32> {
        Ok(123)
    }
}

#[implement(IActivationFactory, bindings::IStaticStatics)]
struct StaticFactory;

impl IActivationFactory_Impl for StaticFactory_Impl {
    fn ActivateInstance(&self) -> Result<IInspectable> {
        Err(Error::from_hresult(E_NOTIMPL))
    }
}

impl bindings::IStaticStatics_Impl for StaticFactory_Impl {
    fn Property(&self) -> Result<i32> {
        Ok(456)
    }
}
