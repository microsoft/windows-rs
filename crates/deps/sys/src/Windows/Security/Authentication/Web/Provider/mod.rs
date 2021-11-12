#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IWebAccountClientView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAccountClientViewFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAccountManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAccountManagerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAccountManagerStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAccountManagerStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAccountMapManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAccountProviderAddAccountOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAccountProviderBaseReportOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAccountProviderDeleteAccountOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAccountProviderManageAccountOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAccountProviderOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAccountProviderRetrieveCookiesOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAccountProviderSignOutAccountOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAccountProviderSilentReportOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAccountProviderTokenObjects(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAccountProviderTokenObjects2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAccountProviderTokenOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAccountProviderUIReportOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAccountScopeManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebProviderTokenRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebProviderTokenRequest2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebProviderTokenRequest3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebProviderTokenResponse(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebProviderTokenResponseFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebAccountClientView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebAccountClientViewType(pub i32);
impl WebAccountClientViewType {
    pub const IdOnly: Self = Self(0i32);
    pub const IdAndProperties: Self = Self(1i32);
}
impl ::core::marker::Copy for WebAccountClientViewType {}
impl ::core::clone::Clone for WebAccountClientViewType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebAccountProviderAddAccountOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebAccountProviderDeleteAccountOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebAccountProviderGetTokenSilentOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebAccountProviderManageAccountOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebAccountProviderOperationKind(pub i32);
impl WebAccountProviderOperationKind {
    pub const RequestToken: Self = Self(0i32);
    pub const GetTokenSilently: Self = Self(1i32);
    pub const AddAccount: Self = Self(2i32);
    pub const ManageAccount: Self = Self(3i32);
    pub const DeleteAccount: Self = Self(4i32);
    pub const RetrieveCookies: Self = Self(5i32);
    pub const SignOutAccount: Self = Self(6i32);
}
impl ::core::marker::Copy for WebAccountProviderOperationKind {}
impl ::core::clone::Clone for WebAccountProviderOperationKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebAccountProviderRequestTokenOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebAccountProviderRetrieveCookiesOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebAccountProviderSignOutAccountOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebAccountProviderTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebAccountScope(pub i32);
impl WebAccountScope {
    pub const PerUser: Self = Self(0i32);
    pub const PerApplication: Self = Self(1i32);
}
impl ::core::marker::Copy for WebAccountScope {}
impl ::core::clone::Clone for WebAccountScope {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebAccountSelectionOptions(pub u32);
impl WebAccountSelectionOptions {
    pub const Default: Self = Self(0u32);
    pub const New: Self = Self(1u32);
}
impl ::core::marker::Copy for WebAccountSelectionOptions {}
impl ::core::clone::Clone for WebAccountSelectionOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebProviderTokenRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebProviderTokenResponse(pub *mut ::core::ffi::c_void);
