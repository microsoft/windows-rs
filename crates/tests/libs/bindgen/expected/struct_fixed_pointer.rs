pub type Address64 = __windows_bindgen_Pointer64<*mut u32>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Addresses {
    pub address: Address64,
    pub point: __windows_bindgen_Pointer64<*const Point>,
    pub array: [__windows_bindgen_Pointer64<*mut core::ffi::c_void>; 3],
    pub mixed: *const __windows_bindgen_Pointer64<*mut u8>,
    pub deep: *mut *const __windows_bindgen_Pointer64<*mut Point>,
}
impl Default for Addresses {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DeepOnly {
    pub deep: *mut *const __windows_bindgen_Pointer64<*mut Point>,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Point {
    pub value: i32,
}
#[doc = r" A fixed 64-bit address, not a native Rust pointer."]
#[doc(hidden)]
#[repr(C)]
pub struct __windows_bindgen_Pointer64<P> {
    address: u64,
    pointer: core::marker::PhantomData<fn() -> P>,
}
impl<P> __windows_bindgen_Pointer64<P> {
    pub const fn from_address(address: u64) -> Self {
        Self {
            address,
            pointer: core::marker::PhantomData,
        }
    }
    pub const fn address(self) -> u64 {
        self.address
    }
}
impl<P> Copy for __windows_bindgen_Pointer64<P> {}
impl<P> Clone for __windows_bindgen_Pointer64<P> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<P> Default for __windows_bindgen_Pointer64<P> {
    fn default() -> Self {
        Self::from_address(0)
    }
}
impl<P> PartialEq for __windows_bindgen_Pointer64<P> {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
    }
}
impl<P> Eq for __windows_bindgen_Pointer64<P> {}
impl<P> core::fmt::Debug for __windows_bindgen_Pointer64<P> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Pointer64").field(&self.address).finish()
    }
}
