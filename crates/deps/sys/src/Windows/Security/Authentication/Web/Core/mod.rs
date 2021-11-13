#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct FindAllAccountsResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FindAllAccountsResult {}
impl ::core::clone::Clone for FindAllAccountsResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FindAllWebAccountsStatus(pub i32);
impl FindAllWebAccountsStatus {
    pub const Success: Self = Self(0i32);
    pub const NotAllowedByProvider: Self = Self(1i32);
    pub const NotSupportedByProvider: Self = Self(2i32);
    pub const ProviderError: Self = Self(3i32);
}
impl ::core::marker::Copy for FindAllWebAccountsStatus {}
impl ::core::clone::Clone for FindAllWebAccountsStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFindAllAccountsResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFindAllAccountsResult {}
impl ::core::clone::Clone for IFindAllAccountsResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebAccountEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebAccountEventArgs {}
impl ::core::clone::Clone for IWebAccountEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebAccountMonitor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebAccountMonitor {}
impl ::core::clone::Clone for IWebAccountMonitor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebAccountMonitor2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebAccountMonitor2 {}
impl ::core::clone::Clone for IWebAccountMonitor2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebAuthenticationCoreManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebAuthenticationCoreManagerStatics {}
impl ::core::clone::Clone for IWebAuthenticationCoreManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebAuthenticationCoreManagerStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebAuthenticationCoreManagerStatics2 {}
impl ::core::clone::Clone for IWebAuthenticationCoreManagerStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebAuthenticationCoreManagerStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebAuthenticationCoreManagerStatics3 {}
impl ::core::clone::Clone for IWebAuthenticationCoreManagerStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebAuthenticationCoreManagerStatics4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebAuthenticationCoreManagerStatics4 {}
impl ::core::clone::Clone for IWebAuthenticationCoreManagerStatics4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebProviderError(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebProviderError {}
impl ::core::clone::Clone for IWebProviderError {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebProviderErrorFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebProviderErrorFactory {}
impl ::core::clone::Clone for IWebProviderErrorFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebTokenRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebTokenRequest {}
impl ::core::clone::Clone for IWebTokenRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebTokenRequest2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebTokenRequest2 {}
impl ::core::clone::Clone for IWebTokenRequest2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebTokenRequest3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebTokenRequest3 {}
impl ::core::clone::Clone for IWebTokenRequest3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebTokenRequestFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebTokenRequestFactory {}
impl ::core::clone::Clone for IWebTokenRequestFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebTokenRequestResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebTokenRequestResult {}
impl ::core::clone::Clone for IWebTokenRequestResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebTokenResponse(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebTokenResponse {}
impl ::core::clone::Clone for IWebTokenResponse {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebTokenResponseFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebTokenResponseFactory {}
impl ::core::clone::Clone for IWebTokenResponseFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebAccountEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebAccountEventArgs {}
impl ::core::clone::Clone for WebAccountEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebAccountMonitor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebAccountMonitor {}
impl ::core::clone::Clone for WebAccountMonitor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebProviderError(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebProviderError {}
impl ::core::clone::Clone for WebProviderError {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebTokenRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebTokenRequest {}
impl ::core::clone::Clone for WebTokenRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebTokenRequestPromptType(pub i32);
impl WebTokenRequestPromptType {
    pub const Default: Self = Self(0i32);
    pub const ForceAuthentication: Self = Self(1i32);
}
impl ::core::marker::Copy for WebTokenRequestPromptType {}
impl ::core::clone::Clone for WebTokenRequestPromptType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebTokenRequestResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebTokenRequestResult {}
impl ::core::clone::Clone for WebTokenRequestResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebTokenRequestStatus(pub i32);
impl WebTokenRequestStatus {
    pub const Success: Self = Self(0i32);
    pub const UserCancel: Self = Self(1i32);
    pub const AccountSwitch: Self = Self(2i32);
    pub const UserInteractionRequired: Self = Self(3i32);
    pub const AccountProviderNotAvailable: Self = Self(4i32);
    pub const ProviderError: Self = Self(5i32);
}
impl ::core::marker::Copy for WebTokenRequestStatus {}
impl ::core::clone::Clone for WebTokenRequestStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebTokenResponse(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebTokenResponse {}
impl ::core::clone::Clone for WebTokenResponse {
    fn clone(&self) -> Self {
        *self
    }
}
