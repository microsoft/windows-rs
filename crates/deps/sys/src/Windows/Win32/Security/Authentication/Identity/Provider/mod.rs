#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
pub const NOT_CONNECTED: i32 = 0i32;
pub const CONNECTING: i32 = 1i32;
pub const CONNECT_COMPLETED: i32 = 2i32;
#[repr(transparent)]
pub struct AsyncIAssociatedIdentityProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AsyncIAssociatedIdentityProvider {}
impl ::core::clone::Clone for AsyncIAssociatedIdentityProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AsyncIConnectedIdentityProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AsyncIConnectedIdentityProvider {}
impl ::core::clone::Clone for AsyncIConnectedIdentityProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AsyncIIdentityAdvise(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AsyncIIdentityAdvise {}
impl ::core::clone::Clone for AsyncIIdentityAdvise {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AsyncIIdentityAuthentication(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AsyncIIdentityAuthentication {}
impl ::core::clone::Clone for AsyncIIdentityAuthentication {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AsyncIIdentityProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AsyncIIdentityProvider {}
impl ::core::clone::Clone for AsyncIIdentityProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AsyncIIdentityStore(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AsyncIIdentityStore {}
impl ::core::clone::Clone for AsyncIIdentityStore {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AsyncIIdentityStoreEx(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AsyncIIdentityStoreEx {}
impl ::core::clone::Clone for AsyncIIdentityStoreEx {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CIdentityProfileHandler: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3975528262,
    data2: 58294,
    data3: 17562,
    data4: [181, 107, 67, 245, 143, 134, 120, 20],
};
pub const CoClassIdentityStore: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 819237446,
    data2: 53783,
    data3: 18015,
    data4: [176, 11, 172, 157, 221, 101, 46, 183],
};
#[repr(transparent)]
pub struct IAssociatedIdentityProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAssociatedIdentityProvider {}
impl ::core::clone::Clone for IAssociatedIdentityProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IConnectedIdentityProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IConnectedIdentityProvider {}
impl ::core::clone::Clone for IConnectedIdentityProvider {
    fn clone(&self) -> Self {
        *self
    }
}
pub const IDENTITIES_ALL: i32 = 0i32;
pub const IDENTITIES_ME_ONLY: i32 = 1i32;
pub const IDENTITY_URL_CREATE_ACCOUNT_WIZARD: i32 = 0i32;
pub const IDENTITY_URL_SIGN_IN_WIZARD: i32 = 1i32;
pub const IDENTITY_URL_CHANGE_PASSWORD_WIZARD: i32 = 2i32;
pub const IDENTITY_URL_IFEXISTS_WIZARD: i32 = 3i32;
pub const IDENTITY_URL_ACCOUNT_SETTINGS: i32 = 4i32;
pub const IDENTITY_URL_RESTORE_WIZARD: i32 = 5i32;
pub const IDENTITY_URL_CONNECT_WIZARD: i32 = 6i32;
#[repr(transparent)]
pub struct IIdentityAdvise(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIdentityAdvise {}
impl ::core::clone::Clone for IIdentityAdvise {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIdentityAuthentication(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIdentityAuthentication {}
impl ::core::clone::Clone for IIdentityAuthentication {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIdentityProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIdentityProvider {}
impl ::core::clone::Clone for IIdentityProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIdentityStore(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIdentityStore {}
impl ::core::clone::Clone for IIdentityStore {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIdentityStoreEx(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIdentityStoreEx {}
impl ::core::clone::Clone for IIdentityStoreEx {
    fn clone(&self) -> Self {
        *self
    }
}
pub const IDENTITY_ASSOCIATED: u32 = 1u32;
pub const IDENTITY_DISASSOCIATED: u32 = 2u32;
pub const IDENTITY_CREATED: u32 = 4u32;
pub const IDENTITY_IMPORTED: u32 = 8u32;
pub const IDENTITY_DELETED: u32 = 16u32;
pub const IDENTITY_PROPCHANGED: u32 = 32u32;
pub const IDENTITY_CONNECTED: u32 = 64u32;
pub const IDENTITY_DISCONNECTED: u32 = 128u32;
pub const OID_OAssociatedIdentityProviderObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2563089373,
    data2: 56168,
    data3: 20250,
    data4: [141, 43, 144, 121, 205, 254, 175, 97],
};
