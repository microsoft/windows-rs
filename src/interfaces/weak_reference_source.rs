use super::*;

#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IWeakReferenceSource(IUnknown);

impl IWeakReferenceSource {
    pub fn downgrade(&self) -> Result<IWeakReference> {
        unsafe {
            let mut result = None;
            (self.vtable().3)(self.abi(), &mut result as *mut _ as _).and_some(result)
        }
    }
}

#[repr(C)]
pub struct IWeakReferenceSource_vtable(
    pub unsafe extern "system" fn(this: RawPtr, iid: &Guid, interface: *mut RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> u32,
    pub unsafe extern "system" fn(this: RawPtr) -> u32,
    pub unsafe extern "system" fn(this: RawPtr, interface: *mut RawPtr) -> HRESULT,
);

unsafe impl Interface for IWeakReferenceSource {
    type Vtable = IWeakReferenceSource_vtable;

    const IID: Guid = Guid::from_values(
        0x0000_0038,
        0x0000,
        0x0000,
        [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
    );
}
