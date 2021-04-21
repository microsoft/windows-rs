use crate::*;

#[repr(transparent)]
#[derive(Clone, PartialEq, Eq)]
pub struct IWeakReference(IUnknown);

impl IWeakReference {
    pub fn upgrade<I: Interface>(&self) -> Option<I> {
        unsafe {
            let mut result = None;
            let _ = (self.vtable().3)(self.abi(), &I::IID, &mut result as *mut _ as _);
            result
        }
    }
}

#[repr(C)]
pub struct IWeakReference_vtable(
    pub unsafe extern "system" fn(this: RawPtr, iid: &Guid, interface: *mut RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> u32,
    pub unsafe extern "system" fn(this: RawPtr) -> u32,
    pub unsafe extern "system" fn(this: RawPtr, iid: &Guid, interface: *mut RawPtr) -> HRESULT,
);

unsafe impl Interface for IWeakReference {
    type Vtable = IWeakReference_vtable;

    const IID: Guid = Guid::from_values(
        0x0000_0037,
        0x0000,
        0x0000,
        [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
    );
}
