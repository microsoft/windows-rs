mod bindings;

use windows::{core::*, Win32::Foundation::*, Win32::System::WinRT::*};

#[no_mangle]
unsafe extern "system" fn DllGetActivationFactory(
    name: Ref<HSTRING>,
    factory: OutRef<IActivationFactory>,
) -> HRESULT {
    if *name == "test_constructors.Activatable" {
        factory.write(Some(ActivatableFactory.into())).into()
    } else if *name == "test_constructors.Composable" {
        factory.write(Some(ComposableFactory.into())).into()
    } else {
        _ = factory.write(None);
        CLASS_E_CLASSNOTAVAILABLE
    }
}

#[implement(IActivationFactory, bindings::IActivatableFactory)]
struct ActivatableFactory;

impl IActivationFactory_Impl for ActivatableFactory_Impl {
    fn ActivateInstance(&self) -> Result<IInspectable> {
        // `Activatable` doesn't have a default constructor so this should never get called.
        Err(E_NOTIMPL.into())
    }
}

impl bindings::IActivatableFactory_Impl for ActivatableFactory_Impl {
    fn CreateInstance(&self, arg: i32) -> Result<bindings::Activatable> {
        Ok(Activatable::new(arg).into())
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
    fn new(arg: i32) -> Self {
        Self(arg)
    }
}

#[implement(IActivationFactory, bindings::IComposableFactory)]
struct ComposableFactory;

impl IActivationFactory_Impl for ComposableFactory_Impl {
    fn ActivateInstance(&self) -> Result<IInspectable> {
        // `Composable` doesn't have a default constructor so this should never get called.
        Err(E_NOTIMPL.into())
    }
}

impl bindings::IComposableFactory_Impl for ComposableFactory_Impl {
    fn CreateInstance(
        &self,
        arg: i32,
        base: Option<&windows_core::IInspectable>,
        inner: &mut Option<windows_core::IInspectable>,
    ) -> Result<bindings::Composable> {
        // windows-rs doesn't support binary composition
        if base.is_some() || inner.is_some() {
            Err(CLASS_E_NOAGGREGATION.into())
        } else {
            Ok(Composable::new(arg).into())
        }
    }
}

#[implement(bindings::Composable)]
struct Composable(i32);

impl bindings::IComposable_Impl for Composable_Impl {
    fn Property(&self) -> Result<i32> {
        Ok(self.0)
    }
}

impl Composable {
    fn new(arg: i32) -> Self {
        Self(arg)
    }
}
