use crate::*;

/// All COM interfaces (and thus WinRT classes and interfaces) implement
/// [IUnknown](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nn-unknwn-iunknown)
/// under the hood to provide reference-counted lifetime management as well as the ability
/// to query for additional interfaces that the object may implement.
#[repr(transparent)]
pub struct IUnknown(std::ptr::NonNull<std::ffi::c_void>);

#[repr(C)]
pub struct IUnknown_vtable(
    pub unsafe extern "system" fn(this: RawPtr, iid: &Guid, interface: *mut RawPtr) -> ErrorCode,
    pub unsafe extern "system" fn(this: RawPtr) -> u32,
    pub unsafe extern "system" fn(this: RawPtr) -> u32,
);

unsafe impl Interface for IUnknown {
    type Vtable = IUnknown_vtable;

    const IID: Guid = Guid::from_values(
        0x0000_0000,
        0x0000,
        0x0000,
        [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
    );
}

impl Clone for IUnknown {
    fn clone(&self) -> Self {
        unsafe {
            (self.vtable().1)(self.abi()); // AddRef
        }

        Self(self.0)
    }
}

impl Drop for IUnknown {
    fn drop(&mut self) {
        unsafe {
            (self.vtable().2)(self.abi()); // Release
        }
    }
}

impl PartialEq for IUnknown {
    fn eq(&self, other: &Self) -> bool {
        // Since COM objects may implement multiple intefaces, COM identity can only
        // be determined by querying for `IUnknown` explicitly and then comparing the
        // pointer values. This works since `QueryInterface` is required to return
        // the same pointer value for queries for `IUnknown`.
        self.cast::<IUnknown>().unwrap().0 == other.cast::<IUnknown>().unwrap().0
    }
}

impl Eq for IUnknown {}

impl std::fmt::Debug for IUnknown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
