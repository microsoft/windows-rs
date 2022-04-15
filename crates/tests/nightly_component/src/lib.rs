mod bindings {
    pub mod microsoft_windows_system_power;
    pub mod test_nightly_component;
}

use std::mem::*;
use std::sync::*;
use windows::{core::*, Win32::Foundation::*, Win32::System::WinRT::*};

#[implement(bindings::test_nightly_component::Class)]
struct Class(RwLock<i32>);

impl bindings::test_nightly_component::IClass_Impl for Class {
    fn Property(&self) -> Result<i32> {
        let reader = self.0.read().unwrap();
        Ok(*reader)
    }
    fn SetProperty(&self, value: i32) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        *writer = value;
        Ok(())
    }
}

#[implement(IActivationFactory)]
struct ClassFactory;

impl IActivationFactory_Impl for ClassFactory {
    fn ActivateInstance(&self) -> Result<IInspectable> {
        Ok(Class(RwLock::new(0)).into())
    }
}

#[implement(bindings::microsoft_windows_system_power::PowerManager)]
struct PowerManager(RwLock<i32>);

impl bindings::microsoft_windows_system_power::IPowerManager_Impl for PowerManager {
    fn Property(&self) -> Result<i32> {
        let reader = self.0.read().unwrap();
        Ok(*reader)
    }
    fn SetProperty(&self, value: i32) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        *writer = value;
        Ok(())
    }
}

#[implement(IActivationFactory)]
struct PowerManagerFactory;

impl IActivationFactory_Impl for PowerManagerFactory {
    fn ActivateInstance(&self) -> Result<IInspectable> {
        Ok(PowerManager(RwLock::new(0)).into())
    }
}

#[no_mangle]
unsafe extern "stdcall" fn DllGetActivationFactory(name: ManuallyDrop<HSTRING>, result: *mut *mut std::ffi::c_void) -> HRESULT {
    let factory: Option<IActivationFactory> = match name.to_string().as_str() {
        "test_nightly_component.Class" => Some(ClassFactory.into()),
        "Microsoft.Windows.System.Power.PowerManager" => Some(PowerManagerFactory.into()),
        _ => None,
    };

    if let Some(factory) = factory {
        *result = transmute(factory);
        S_OK
    } else {
        *result = std::ptr::null_mut();
        CLASS_E_CLASSNOTAVAILABLE
    }
}
