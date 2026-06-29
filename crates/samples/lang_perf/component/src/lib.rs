#[allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
mod bindings;

use windows::{Win32::Foundation::*, Win32::System::WinRT::*, core::*};

#[unsafe(no_mangle)]
extern "system" fn DllGetActivationFactory(
    name: Ref<HSTRING>,
    factory: OutRef<IActivationFactory>,
) -> HRESULT {
    if *name == "LangPerf.Class" {
        factory.write(Some(CLASS_FACTORY.to_interface())).into()
    } else {
        _ = factory.write(None);
        CLASS_E_CLASSNOTAVAILABLE
    }
}

static CLASS_FACTORY: StaticComObject<ClassFactory> = ClassFactory.into_static();

#[implement(IActivationFactory)]
struct ClassFactory;

impl IActivationFactory_Impl for ClassFactory_Impl {
    fn ActivateInstance(&self) -> Result<IInspectable> {
        Ok(Class::default().into())
    }
}

#[implement(bindings::Class, bindings::INonDefault)]
#[derive(Default)]
struct Class {
    event: Event<bindings::Handler>,
}

impl bindings::IClass_Impl for Class_Impl {
    fn Int32Property(&self) -> Result<i32> {
        Ok(0)
    }

    fn SetInt32Property(&self, _value: i32) -> Result<()> {
        Ok(())
    }

    fn StringProperty(&self) -> Result<HSTRING> {
        Ok(HSTRING::new())
    }

    fn SetStringProperty(&self, _value: &HSTRING) -> Result<()> {
        Ok(())
    }

    fn ObjectProperty(&self) -> Result<IInspectable> {
        Ok(self.to_interface())
    }

    fn SetObjectProperty(&self, _value: Ref<IInspectable>) -> Result<()> {
        Ok(())
    }

    fn Next(&self) -> Result<i32> {
        Err(E_BOUNDS.into())
    }

    fn Lang(&self) -> Result<HSTRING> {
        Ok(h!("Rust").to_owned())
    }

    fn Event(&self, handler: Ref<bindings::Handler>) -> Result<i64> {
        self.event.add(handler.ok()?)
    }

    fn RemoveEvent(&self, token: i64) -> Result<()> {
        self.event.remove(token);
        Ok(())
    }

    fn Raise(&self) -> Result<()> {
        let sender: IInspectable = self.to_interface();
        self.event.call(|handler| handler.Invoke(&sender, 0));
        Ok(())
    }

    fn Items(&self, count: u32) -> Result<windows_collections::IVector<i32>> {
        Ok(windows_collections::IVector::<i32>::from(
            (0..count as i32).collect::<Vec<i32>>(),
        ))
    }

    fn Map(&self, count: u32) -> Result<windows_collections::IMap<HSTRING, i32>> {
        let pairs: std::collections::BTreeMap<HSTRING, i32> = (0..count as i32)
            .map(|i| (i.to_string().into(), i))
            .collect();
        Ok(windows_collections::IMap::<HSTRING, i32>::from(pairs))
    }

    fn Operation(&self) -> Result<windows_future::IAsyncOperation<i32>> {
        Ok(windows_future::IAsyncOperation::<i32>::ready(Ok(0)))
    }

    fn Reference(&self) -> Result<windows_reference::IReference<i32>> {
        Ok(windows_reference::IReference::<i32>::from(0))
    }
}

impl bindings::INonDefault_Impl for Class_Impl {
    fn Value(&self) -> Result<i32> {
        Ok(0)
    }
}
