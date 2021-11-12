#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct ACCOUNT_STATE(i32);
#[repr(transparent)]
pub struct AsyncIAssociatedIdentityProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AsyncIConnectedIdentityProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AsyncIIdentityAdvise(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AsyncIIdentityAuthentication(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AsyncIIdentityProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AsyncIIdentityStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AsyncIIdentityStoreEx(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct CIdentityProfileHandler(i32);
#[repr(C)]
pub struct CoClassIdentityStore(i32);
#[repr(transparent)]
pub struct IAssociatedIdentityProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConnectedIdentityProvider(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct IDENTITY_TYPE(i32);
#[repr(C)]
pub struct IDENTITY_URL(i32);
#[repr(transparent)]
pub struct IIdentityAdvise(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIdentityAuthentication(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIdentityProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIdentityStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIdentityStoreEx(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct IdentityUpdateEvent(i32);
pub const OID_OAssociatedIdentityProviderObject: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2563089373,
    data2: 56168,
    data3: 20250,
    data4: [141, 43, 144, 121, 205, 254, 175, 97],
};
