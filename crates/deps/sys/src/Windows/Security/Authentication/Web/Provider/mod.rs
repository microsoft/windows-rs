#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IWebAccountClientView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebAccountClientView {}
impl ::core::clone::Clone for IWebAccountClientView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebAccountClientViewFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebAccountClientViewFactory {}
impl ::core::clone::Clone for IWebAccountClientViewFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebAccountManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebAccountManagerStatics {}
impl ::core::clone::Clone for IWebAccountManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebAccountManagerStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebAccountManagerStatics2 {}
impl ::core::clone::Clone for IWebAccountManagerStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebAccountManagerStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebAccountManagerStatics3 {}
impl ::core::clone::Clone for IWebAccountManagerStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebAccountManagerStatics4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebAccountManagerStatics4 {}
impl ::core::clone::Clone for IWebAccountManagerStatics4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebAccountMapManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebAccountMapManagerStatics {}
impl ::core::clone::Clone for IWebAccountMapManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebAccountProviderAddAccountOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebAccountProviderAddAccountOperation {}
impl ::core::clone::Clone for IWebAccountProviderAddAccountOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebAccountProviderBaseReportOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebAccountProviderBaseReportOperation {}
impl ::core::clone::Clone for IWebAccountProviderBaseReportOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebAccountProviderDeleteAccountOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebAccountProviderDeleteAccountOperation {}
impl ::core::clone::Clone for IWebAccountProviderDeleteAccountOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebAccountProviderManageAccountOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebAccountProviderManageAccountOperation {}
impl ::core::clone::Clone for IWebAccountProviderManageAccountOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebAccountProviderOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebAccountProviderOperation {}
impl ::core::clone::Clone for IWebAccountProviderOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebAccountProviderRetrieveCookiesOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebAccountProviderRetrieveCookiesOperation {}
impl ::core::clone::Clone for IWebAccountProviderRetrieveCookiesOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebAccountProviderSignOutAccountOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebAccountProviderSignOutAccountOperation {}
impl ::core::clone::Clone for IWebAccountProviderSignOutAccountOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebAccountProviderSilentReportOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebAccountProviderSilentReportOperation {}
impl ::core::clone::Clone for IWebAccountProviderSilentReportOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebAccountProviderTokenObjects(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebAccountProviderTokenObjects {}
impl ::core::clone::Clone for IWebAccountProviderTokenObjects {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebAccountProviderTokenObjects2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebAccountProviderTokenObjects2 {}
impl ::core::clone::Clone for IWebAccountProviderTokenObjects2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebAccountProviderTokenOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebAccountProviderTokenOperation {}
impl ::core::clone::Clone for IWebAccountProviderTokenOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebAccountProviderUIReportOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebAccountProviderUIReportOperation {}
impl ::core::clone::Clone for IWebAccountProviderUIReportOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebAccountScopeManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebAccountScopeManagerStatics {}
impl ::core::clone::Clone for IWebAccountScopeManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebProviderTokenRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebProviderTokenRequest {}
impl ::core::clone::Clone for IWebProviderTokenRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebProviderTokenRequest2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebProviderTokenRequest2 {}
impl ::core::clone::Clone for IWebProviderTokenRequest2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebProviderTokenRequest3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebProviderTokenRequest3 {}
impl ::core::clone::Clone for IWebProviderTokenRequest3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebProviderTokenResponse(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebProviderTokenResponse {}
impl ::core::clone::Clone for IWebProviderTokenResponse {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebProviderTokenResponseFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebProviderTokenResponseFactory {}
impl ::core::clone::Clone for IWebProviderTokenResponseFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebAccountClientView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebAccountClientView {}
impl ::core::clone::Clone for WebAccountClientView {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for WebAccountProviderAddAccountOperation {}
impl ::core::clone::Clone for WebAccountProviderAddAccountOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebAccountProviderDeleteAccountOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebAccountProviderDeleteAccountOperation {}
impl ::core::clone::Clone for WebAccountProviderDeleteAccountOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebAccountProviderGetTokenSilentOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebAccountProviderGetTokenSilentOperation {}
impl ::core::clone::Clone for WebAccountProviderGetTokenSilentOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebAccountProviderManageAccountOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebAccountProviderManageAccountOperation {}
impl ::core::clone::Clone for WebAccountProviderManageAccountOperation {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for WebAccountProviderRequestTokenOperation {}
impl ::core::clone::Clone for WebAccountProviderRequestTokenOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebAccountProviderRetrieveCookiesOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebAccountProviderRetrieveCookiesOperation {}
impl ::core::clone::Clone for WebAccountProviderRetrieveCookiesOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebAccountProviderSignOutAccountOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebAccountProviderSignOutAccountOperation {}
impl ::core::clone::Clone for WebAccountProviderSignOutAccountOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebAccountProviderTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebAccountProviderTriggerDetails {}
impl ::core::clone::Clone for WebAccountProviderTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for WebProviderTokenRequest {}
impl ::core::clone::Clone for WebProviderTokenRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebProviderTokenResponse(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebProviderTokenResponse {}
impl ::core::clone::Clone for WebProviderTokenResponse {
    fn clone(&self) -> Self {
        *self
    }
}
