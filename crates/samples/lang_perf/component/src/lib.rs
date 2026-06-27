#[allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
mod bindings;

use std::cell::{Cell, RefCell};
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

// A deliberately trivial WinRT runtime class: each accessor does the minimum work
// possible so a tight calling loop measures the projection overhead of the caller's
// language rather than the cost of the callee.
#[implement(bindings::Class)]
#[derive(Default)]
struct Class {
    int32: Cell<i32>,
    string: RefCell<HSTRING>,
    object: RefCell<Option<IInspectable>>,
}

impl bindings::IClass_Impl for Class_Impl {
    fn Int32Property(&self) -> Result<i32> {
        Ok(self.int32.get())
    }

    fn SetInt32Property(&self, value: i32) -> Result<()> {
        self.int32.set(value);
        Ok(())
    }

    fn StringProperty(&self) -> Result<HSTRING> {
        Ok(self.string.borrow().clone())
    }

    fn SetStringProperty(&self, value: &HSTRING) -> Result<()> {
        *self.string.borrow_mut() = value.clone();
        Ok(())
    }

    fn ObjectProperty(&self) -> Result<IInspectable> {
        self.object.borrow().clone().ok_or_else(|| E_POINTER.into())
    }

    fn SetObjectProperty(&self, value: Ref<IInspectable>) -> Result<()> {
        *self.object.borrow_mut() = value.cloned();
        Ok(())
    }

    fn NewObject(&self) -> Result<IInspectable> {
        Ok(NonDefault.into())
    }
}

#[implement(bindings::INonDefault)]
struct NonDefault;

impl bindings::INonDefault_Impl for NonDefault_Impl {
    fn Value(&self) -> Result<i32> {
        Ok(0)
    }
}
