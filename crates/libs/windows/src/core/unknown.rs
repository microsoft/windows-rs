use super::*;

/// All COM interfaces (and thus WinRT classes and interfaces) implement
/// [IUnknown](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nn-unknwn-iunknown)
/// under the hood to provide reference-counted lifetime management as well as the ability
/// to query for additional interfaces that the object may implement.
#[repr(transparent)]
pub struct IUnknown(core::ptr::NonNull<core::ffi::c_void>);

#[doc(hidden)]
#[repr(C)]
pub struct IUnknownVtbl {
    pub QueryInterface: unsafe extern "system" fn(this: RawPtr, iid: &GUID, interface: *mut RawPtr) -> HRESULT,
    pub AddRef: unsafe extern "system" fn(this: RawPtr) -> u32,
    pub Release: unsafe extern "system" fn(this: RawPtr) -> u32,
}

unsafe impl Interface for IUnknown {
    type Vtable = IUnknownVtbl;

    const IID: GUID = GUID::from_u128(0x00000000_0000_0000_c000_000000000046);
}

impl Clone for IUnknown {
    fn clone(&self) -> Self {
        unsafe {
            (self.vtable().AddRef)(core::mem::transmute_copy(self));
        }

        Self(self.0)
    }
}

impl Drop for IUnknown {
    fn drop(&mut self) {
        unsafe {
            (self.vtable().Release)(core::mem::transmute_copy(self));
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

#[doc(hidden)]
pub trait IUnknownImpl {
    fn get_impl(&mut self) -> RawPtr;

    fn QueryInterface(&mut self, iid: &GUID, interface: *mut RawPtr) -> HRESULT;
    fn AddRef(&mut self) -> u32;
    fn Release(&mut self) -> u32;
}

#[cfg(feature = "implement")]
impl IUnknownVtbl {
    pub const fn new<T: IUnknownImpl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInterface<T: IUnknownImpl, const OFFSET: isize>(this: RawPtr, iid: &GUID, interface: *mut RawPtr) -> HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut T;
            (*this).QueryInterface(iid, interface)
        }
        unsafe extern "system" fn AddRef<T: IUnknownImpl, const OFFSET: isize>(this: RawPtr) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut T;
            (*this).AddRef()
        }
        unsafe extern "system" fn Release<T: IUnknownImpl, const OFFSET: isize>(this: RawPtr) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut T;
            (*this).Release()
        }
        Self { QueryInterface: QueryInterface::<T, OFFSET>, AddRef: AddRef::<T, OFFSET>, Release: Release::<T, OFFSET> }
    }
}
