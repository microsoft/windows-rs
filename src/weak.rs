use crate::*;

#[com::com_iterface("00000037-0000-0000-C000-000000000046")]
pub(crate) trait IWeakReference: com::interfaces::iunknown::IUnknown {
    unsafe fn resolve(&self, guid: *const Guid, ptr: *mut *mut IInspectable::VTable) -> ErrorCode;
}

#[com::com_iterface("00000038-0000-0000-C000-000000000046")]
pub(crate) trait IWeakReferenceSource: com::interfaces::iunknown::IUnknown {
    unsafe fn get_weak_reference(&self, ptr: *mut *mut IWeakReference::VTable) -> ErrorCode;
}