#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
pub struct ApplicationProfile {}
impl ApplicationProfile {
    pub fn Modes() -> ::windows::core::Result<ApplicationProfileModes> {
        Self::IApplicationProfileStatics(|this| unsafe {
            let mut result__: ApplicationProfileModes = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationProfileModes>(result__)
        })
    }
    pub fn IApplicationProfileStatics<R, F: FnOnce(&IApplicationProfileStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ApplicationProfile, IApplicationProfileStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for ApplicationProfile {
    const NAME: &'static str = "Windows.Phone.ApplicationModel.ApplicationProfile";
}
#[repr(transparent)]
pub struct ApplicationProfileModes(pub u32);
impl ApplicationProfileModes {
    pub const Default: Self = Self(0u32);
    pub const Alternate: Self = Self(1u32);
}
impl ::core::marker::Copy for ApplicationProfileModes {}
impl ::core::clone::Clone for ApplicationProfileModes {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ApplicationProfileModes {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ApplicationProfileModes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ApplicationProfileModes {}
unsafe impl ::windows::core::RuntimeType for ApplicationProfileModes {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Phone.ApplicationModel.ApplicationProfileModes;u4)");
}
impl ::windows::core::DefaultType for ApplicationProfileModes {
    type DefaultType = Self;
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IApplicationProfileStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IApplicationProfileStatics {
    type Vtable = IApplicationProfileStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5008ab4_7e7a_11e1_a7f2_b0a14824019b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationProfileStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ApplicationProfileModes) -> ::windows::core::HRESULT,
);
