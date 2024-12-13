mod bindings;
use windows::{core::*, Foundation::*, Win32::Foundation::*, Win32::System::WinRT::*};

static CLASS_FACTORY: StaticComObject<ClassFactory> = ClassFactory::new().into_static();

#[no_mangle]
unsafe extern "system" fn DllGetActivationFactory(
    name: Ref<HSTRING>,
    factory: OutRef<IActivationFactory>,
) -> HRESULT {
    if *name == "test_events.Class" {
        factory.write(Some(CLASS_FACTORY.to_interface())).into()
    } else {
        _ = factory.write(None);
        CLASS_E_CLASSNOTAVAILABLE
    }
}

#[implement(IActivationFactory, bindings::IClassStatics)]
struct ClassFactory(Event<EventHandler<i32>>);

impl ClassFactory {
    const fn new() -> Self {
        Self(Event::new())
    }
}

impl bindings::IClassStatics_Impl for ClassFactory_Impl {
    fn StaticSignal(&self, value: i32) -> Result<i32> {
        let mut counter = 0;
        self.0.call(|delegate| {
            counter += 1;
            delegate.Invoke(self.as_interface(), value)
        });
        Ok(counter)
    }

    fn StaticEvent(&self, handler: Ref<EventHandler<i32>>) -> Result<i64> {
        self.0.add(handler.unwrap())
    }

    fn RemoveStaticEvent(&self, token: i64) -> Result<()> {
        self.0.remove(token);
        Ok(())
    }
}

impl IActivationFactory_Impl for ClassFactory_Impl {
    fn ActivateInstance(&self) -> Result<IInspectable> {
        Ok(Class::new().into())
    }
}

#[implement(bindings::Class)]
struct Class(Event<TypedEventHandler<bindings::Class, i32>>);

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
        handler: Ref<TypedEventHandler<bindings::Class, i32>>,
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
