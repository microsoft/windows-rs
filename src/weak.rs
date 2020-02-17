use crate::*;

#[com::com_interface("00000037-0000-0000-C000-000000000046")]
pub trait IWeakReference: com::interfaces::iunknown::IUnknown {
    unsafe fn resolve(&self, guid: *const com::sys::IID, ptr: *mut *mut <dyn IInspectable as com::ComInterface>::VTable) -> ErrorCode;
}

#[com::com_interface("00000038-0000-0000-C000-000000000046")]
pub trait IWeakReferenceSource: com::interfaces::iunknown::IUnknown {
    unsafe fn get_weak_reference(&self, ptr: *mut *mut <dyn IWeakReference as com::ComInterface>::VTable) -> ErrorCode;
}
