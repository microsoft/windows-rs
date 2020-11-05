use crate::*;

#[allow(non_camel_case_types)]
pub type IUnknown_QueryInterface =
    extern "system" fn(this: RawPtr, iid: &Guid, interface: *mut RawPtr) -> ErrorCode;

#[allow(non_camel_case_types)]
pub type IUnknown_AddRef = extern "system" fn(this: RawPtr) -> u32;

#[allow(non_camel_case_types)]
pub type IUnknown_Release = extern "system" fn(this: RawPtr) -> u32;

#[repr(transparent)]
pub struct IUnknown(RawComPtr);

#[repr(C)]
pub struct IUnknown_vtable(
    pub IUnknown_QueryInterface,
    pub IUnknown_AddRef,
    pub IUnknown_Release,
);

unsafe impl Interface for IUnknown {
    type Vtable = IUnknown_vtable;

    const IID: Guid = {
        Guid::from_values(
            0x0000_0000,
            0x0000,
            0x0000,
            [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
        )
    };
}

impl Clone for IUnknown {
    fn clone(&self) -> Self {
        unsafe {
            (self.vtable().1)(self.get_abi()); // AddRef
        }

        Self(self.0)
    }
}

impl Drop for IUnknown {
    fn drop(&mut self) {
        unsafe {
            (self.vtable().2)(self.get_abi()); // Release
        }
    }
}

impl PartialEq for IUnknown {
    fn eq(&self, other: &Self) -> bool {
        self.try_query::<IUnknown>() == other.try_query::<IUnknown>()
    }
}
