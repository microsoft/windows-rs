mod bindings;
use std::sync::*;
use windows::{core::*, Foundation::*, Win32::Foundation::*, Win32::System::WinRT::*};

#[implement(bindings::Class)]
struct Class(RwLock<i32>);

impl bindings::IClass_Impl for Class_Impl {
    fn Property(&self) -> Result<i32> {
        let reader = self.0.read().unwrap();
        Ok(*reader)
    }
    fn SetProperty(&self, value: i32) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        *writer = value;
        Ok(())
    }
    fn Flags(&self) -> Result<bindings::Flags> {
        Ok(bindings::Flags::Ok)
    }
    fn Int32Array(&self, a: &[i32], b: &mut [i32], c: &mut Array<i32>) -> Result<Array<i32>> {
        assert_eq!(a.len(), b.len());
        assert!(c.is_empty());
        b.copy_from_slice(a);
        *c = Array::from_slice(a);
        Ok(Array::from_slice(a))
    }
    fn StringArray(
        &self,
        a: &[HSTRING],
        b: &mut [HSTRING],
        c: &mut Array<HSTRING>,
    ) -> Result<Array<HSTRING>> {
        assert_eq!(a.len(), b.len());
        assert!(c.is_empty());
        b.clone_from_slice(a);
        *c = Array::from_slice(a);
        Ok(Array::from_slice(a))
    }
    fn Input(
        &self,
        a: Ref<IInspectable>,
        b: Ref<bindings::Class>,
        c: Ref<IStringable>,
        d: Ref<bindings::Callback>,
    ) -> Result<()> {
        let a = a.ok()?;
        let b = b.ok()?;
        let c = c.ok()?;
        let d = d.ok()?;

        let a: IUnknown = a.cast()?;
        let b: IUnknown = b.cast()?;
        assert_eq!(a, b);
        assert_eq!(c.ToString()?, "client");
        assert_eq!(d.Invoke(123)?, 123);

        Ok(())
    }
}

#[implement(IActivationFactory)]
struct ClassFactory;

impl IActivationFactory_Impl for ClassFactory_Impl {
    fn ActivateInstance(&self) -> Result<IInspectable> {
        Ok(Class(RwLock::new(0)).into())
    }
}

// HRESULT __stdcall DllGetActivationFactory(HSTRING, IActivationFactory**)
#[no_mangle]
unsafe extern "system" fn DllGetActivationFactory(
    name: Ref<HSTRING>,
    factory: OutRef<IActivationFactory>,
) -> HRESULT {
    if *name == "test_component.Class" {
        factory.write(Some(ClassFactory.into())).into()
    } else {
        _ = factory.write(None);
        CLASS_E_CLASSNOTAVAILABLE
    }
}
