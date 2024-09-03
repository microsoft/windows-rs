mod bindings;

use windows::{core::*, Win32::System::WinRT::*, Win32::Foundation::*};

#[no_mangle]
unsafe extern "system" fn DllGetActivationFactory(
    name: Ref<HSTRING>,
    factory: OutRef<IActivationFactory>,
) -> HRESULT {
    if *name == "Namespace.Activatable" {
        factory.write(Some(ActivatableFactory.into())).into()
    } else {
        _ = factory.write(None);
        CLASS_E_CLASSNOTAVAILABLE
    }
}

#[implement(IActivationFactory)]
struct ActivatableFactory;

impl IActivationFactory_Impl for ActivatableFactory_Impl {
    fn ActivateInstance(&self) -> Result<IInspectable> {
        Ok(Activatable::new().into())
    }
}

#[implement(bindings::Activatable)]
struct Activatable(i32);

impl bindings::IActivatable_Impl for Activatable_Impl {
    fn Property(&self) -> Result<i32> {
        Ok(self.0)
    }
}

impl Activatable {
    fn new() -> Self {
        Self(0)
    }
}