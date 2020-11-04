use crate::*;

#[allow(non_camel_case_types)]
pub type IUnknown_QueryInterface =
    extern "system" fn(this: RawPtr, iid: &Guid, interface: *mut RawPtr) -> ErrorCode;

#[allow(non_camel_case_types)]
pub type IUnknown_AddRef = extern "system" fn(this: RawPtr) -> u32;

#[allow(non_camel_case_types)]
pub type IUnknown_Release = extern "system" fn(this: RawPtr) -> u32;

#[repr(C)]
pub struct IUnknown(RawPtr);

#[repr(C)]
pub struct IUnknown_vtable(
    pub IUnknown_QueryInterface,
    pub IUnknown_AddRef,
    pub IUnknown_Release,
);

unsafe impl ComInterface for IUnknown {
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

unsafe impl AbiTransferable for IUnknown {
    type Abi = RawPtr;

    unsafe fn get_abi(&self) -> RawPtr {
        self.0
    }

    unsafe fn set_abi(&mut self) -> *mut RawPtr {
        (self.vtable().2)(self.0); // Release
        self.0 = std::ptr::null_mut();
        &mut self.0
    }
}

impl Clone for IUnknown {
    fn clone(&self) -> Self {
        unsafe {
            (self.vtable().1)(self.0); // AddRef
        }

        Self(self.0)
    }
}

impl Drop for IUnknown {
    fn drop(&mut self) {
        unsafe {
            (self.vtable().2)(self.0); // Release
        }
    }
}
