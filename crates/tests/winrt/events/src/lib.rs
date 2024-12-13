mod bindings;

use windows::{core::*, Win32::Foundation::*, Win32::System::WinRT::*};

#[no_mangle]
unsafe extern "system" fn DllGetActivationFactory(
    name: Ref<HSTRING>,
    factory: OutRef<IActivationFactory>,
) -> HRESULT {
    if *name == "test_events.Class" {
        factory.write(Some(ClassFactory.into())).into()
    } else {
        _ = factory.write(None);
        CLASS_E_CLASSNOTAVAILABLE
    }
}

#[implement(IActivationFactory)]
struct ClassFactory;

impl IActivationFactory_Impl for ClassFactory_Impl {
    fn ActivateInstance(&self) -> Result<IInspectable> {
        Ok(Class::new().into())
    }
}

#[implement(bindings::Class)]
struct Class(Event<windows::Foundation::TypedEventHandler<bindings::Class, i32>>);

impl bindings::IClass_Impl for Class_Impl {
    fn Signal(&self, value: i32) -> Result<i32> {
        let mut counter = 0;
        self.0.call(|delegate| {
            counter += 1;
            delegate.Invoke(self.as_interface(), value)
        });
        Ok(counter)
    }

    fn Event(
        &self,
        handler: Ref<windows::Foundation::TypedEventHandler<bindings::Class, i32>>,
    ) -> windows_core::Result<i64> {
        self.0.add(handler.unwrap())
    }

    fn RemoveEvent(&self, token: i64) -> windows_core::Result<()> {
        self.0.remove(token);
        Ok(())
    }
}

impl Class {
    fn new() -> Self {
        Self(Event::new())
    }
}
