#![cfg(windows)]
#[allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
mod bindings;

use windows::{Foundation::*, Win32::Foundation::*, Win32::System::WinRT::*, core::*};

#[unsafe(no_mangle)]
unsafe extern "system" fn DllGetActivationFactory(
    name: Ref<HSTRING>,
    factory: OutRef<IActivationFactory>,
) -> HRESULT {
    if name.unwrap() == "test_reference.Reference" {
        factory.write(Some(ReferenceFactory.into())).into()
    } else {
        _ = factory.write(None);
        CLASS_E_CLASSNOTAVAILABLE
    }
}

#[implement(IActivationFactory)]
struct ReferenceFactory;

impl IActivationFactory_Impl for ReferenceFactory_Impl {
    fn ActivateInstance(&self) -> Result<IInspectable> {
        Ok(Reference.into())
    }
}

#[implement(IStringable, bindings::IReference)]
struct Reference;

impl IStringable_Impl for Reference_Impl {
    fn ToString(&self) -> Result<HSTRING> {
        Ok(h!("Reference").clone())
    }
}

impl bindings::IReference_Impl for Reference_Impl {
    fn Method(&self, stringable: Ref<IStringable>) -> Result<HSTRING> {
        stringable.unwrap().ToString()
    }
}
