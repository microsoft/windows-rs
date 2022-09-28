use super::*;
use std::mem::*;

// A streamlined version of the IActivationFactory interface used by WinRT class factories used internally by the windows crate
// to simplify code generation. Components should implement the `IActivationFactory` interface published by the windows crate.
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq)]
pub struct IGenericFactory(IUnknown);

impl IGenericFactory {
    pub fn ActivateInstance<I: Interface>(&self) -> Result<I> {
        unsafe {
            let mut result__ = zeroed();
            (Vtable::vtable(self).ActivateInstance)(transmute_copy(self), &mut result__ as *mut _ as *mut _).from_abi::<IInspectable>(result__)?.cast()
        }
    }
}

#[repr(C)]
pub struct IGenericFactory_Vtbl {
    pub base__: IInspectable_Vtbl,
    pub ActivateInstance: unsafe extern "system" fn(this: *mut std::ffi::c_void, instance: *mut *mut std::ffi::c_void) -> HRESULT,
}

unsafe impl Vtable for IGenericFactory {
    type Vtable = IGenericFactory_Vtbl;
}

unsafe impl Interface for IGenericFactory {
    const IID: GUID = GUID::from_u128(0x00000035_0000_0000_c000_000000000046);
}
