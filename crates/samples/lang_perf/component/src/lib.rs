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
        Ok(Class.into())
    }
}

#[implement(bindings::Class, bindings::INonDefault)]
struct Class;

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
}

impl bindings::INonDefault_Impl for Class_Impl {
    fn Value(&self) -> Result<i32> {
        Ok(0)
    }
}
