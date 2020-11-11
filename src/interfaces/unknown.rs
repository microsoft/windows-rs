use crate::*;

#[repr(transparent)]
pub struct IUnknown(RawComPtr);

#[repr(C)]
pub struct IUnknown_vtable(
    pub extern "system" fn(this: RawPtr, iid: &Guid, interface: *mut RawPtr) -> ErrorCode,
    pub extern "system" fn(this: RawPtr) -> u32,
    pub extern "system" fn(this: RawPtr) -> u32,
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
        self.cast::<IUnknown>().unwrap() == other.cast::<IUnknown>().unwrap()
    }
}

impl std::fmt::Debug for IUnknown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
