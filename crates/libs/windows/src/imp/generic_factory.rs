use super::*;
use windows::core::ComInterface;

// A streamlined version of the IActivationFactory interface used by WinRT class factories used internally by the windows crate
// to simplify code generation. Components should implement the `IActivationFactory` interface published by the windows crate.
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq)]
pub struct IGenericFactory(core::IUnknown);

impl IGenericFactory {
    pub fn ActivateInstance<I: core::ComInterface>(&self) -> core::Result<I> {
        unsafe {
            let mut result__ = core::zeroed::<I>();
            (core::Interface::vtable(self).ActivateInstance)(std::mem::transmute_copy(self), &mut result__ as *mut _ as *mut _).from_abi::<core::IInspectable>(result__)?.cast()
        }
    }
}

#[repr(C)]
pub struct IGenericFactory_Vtbl {
    pub base__: core::IInspectable_Vtbl,
    pub ActivateInstance: unsafe extern "system" fn(this: *mut std::ffi::c_void, instance: *mut *mut std::ffi::c_void) -> core::HRESULT,
}

unsafe impl core::Interface for IGenericFactory {
    type Vtable = IGenericFactory_Vtbl;
}

unsafe impl core::ComInterface for IGenericFactory {
    const IID: core::GUID = core::GUID::from_u128(0x00000035_0000_0000_c000_000000000046);
}
