use super::*;

/// All COM interfaces (and thus WinRT classes and interfaces) implement
/// [IUnknown](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nn-unknwn-iunknown)
/// under the hood to provide reference-counted lifetime management as well as the ability
/// to query for additional interfaces that the object may implement.
#[repr(transparent)]
pub struct IUnknown(core::ptr::NonNull<core::ffi::c_void>);

#[doc(hidden)]
#[repr(C)]
pub struct IUnknownVtbl(pub unsafe extern "system" fn(this: RawPtr, iid: *const GUID, interface: *mut RawPtr) -> HRESULT, pub unsafe extern "system" fn(this: RawPtr) -> u32, pub unsafe extern "system" fn(this: RawPtr) -> u32);

unsafe impl Interface for IUnknown {
    type Vtable = IUnknownVtbl;

    const IID: GUID = GUID::from_u128(0x00000000_0000_0000_c000_000000000046);
}

impl Clone for IUnknown {
    fn clone(&self) -> Self {
        unsafe {
            (self.vtable().1)(core::mem::transmute_copy(self)); // AddRef
        }

        Self(self.0)
    }
}

impl Drop for IUnknown {
    fn drop(&mut self) {
        unsafe {
            (self.vtable().2)(core::mem::transmute_copy(self)); // Release
        }
    }
}

impl PartialEq for IUnknown {
    fn eq(&self, other: &Self) -> bool {
        // Since COM objects may implement multiple interfaces, COM identity can only
        // be determined by querying for `IUnknown` explicitly and then comparing the
        // pointer values. This works since `QueryInterface` is required to return
        // the same pointer value for queries for `IUnknown`.
        self.cast::<IUnknown>().unwrap().0 == other.cast::<IUnknown>().unwrap().0
    }
}

impl Eq for IUnknown {}

impl core::fmt::Debug for IUnknown {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IUnknown").field(&self.0).finish()
    }
}
