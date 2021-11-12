#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ACCOUNT_STATE(pub i32);
pub const NOT_CONNECTED: ACCOUNT_STATE = ACCOUNT_STATE(0i32);
pub const CONNECTING: ACCOUNT_STATE = ACCOUNT_STATE(1i32);
pub const CONNECT_COMPLETED: ACCOUNT_STATE = ACCOUNT_STATE(2i32);
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
#[repr(transparent)]
pub struct IDENTITY_TYPE(pub i32);
pub const IDENTITIES_ALL: IDENTITY_TYPE = IDENTITY_TYPE(0i32);
pub const IDENTITIES_ME_ONLY: IDENTITY_TYPE = IDENTITY_TYPE(1i32);
#[repr(transparent)]
pub struct IDENTITY_URL(pub i32);
pub const IDENTITY_URL_CREATE_ACCOUNT_WIZARD: IDENTITY_URL = IDENTITY_URL(0i32);
pub const IDENTITY_URL_SIGN_IN_WIZARD: IDENTITY_URL = IDENTITY_URL(1i32);
pub const IDENTITY_URL_CHANGE_PASSWORD_WIZARD: IDENTITY_URL = IDENTITY_URL(2i32);
pub const IDENTITY_URL_IFEXISTS_WIZARD: IDENTITY_URL = IDENTITY_URL(3i32);
pub const IDENTITY_URL_ACCOUNT_SETTINGS: IDENTITY_URL = IDENTITY_URL(4i32);
pub const IDENTITY_URL_RESTORE_WIZARD: IDENTITY_URL = IDENTITY_URL(5i32);
pub const IDENTITY_URL_CONNECT_WIZARD: IDENTITY_URL = IDENTITY_URL(6i32);
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
#[repr(transparent)]
pub struct IdentityUpdateEvent(pub u32);
pub const IDENTITY_ASSOCIATED: IdentityUpdateEvent = IdentityUpdateEvent(1u32);
pub const IDENTITY_DISASSOCIATED: IdentityUpdateEvent = IdentityUpdateEvent(2u32);
pub const IDENTITY_CREATED: IdentityUpdateEvent = IdentityUpdateEvent(4u32);
pub const IDENTITY_IMPORTED: IdentityUpdateEvent = IdentityUpdateEvent(8u32);
pub const IDENTITY_DELETED: IdentityUpdateEvent = IdentityUpdateEvent(16u32);
pub const IDENTITY_PROPCHANGED: IdentityUpdateEvent = IdentityUpdateEvent(32u32);
pub const IDENTITY_CONNECTED: IdentityUpdateEvent = IdentityUpdateEvent(64u32);
pub const IDENTITY_DISCONNECTED: IdentityUpdateEvent = IdentityUpdateEvent(128u32);
pub const OID_OAssociatedIdentityProviderObject: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2563089373,
    data2: 56168,
    data3: 20250,
    data4: [141, 43, 144, 121, 205, 254, 175, 97],
};
