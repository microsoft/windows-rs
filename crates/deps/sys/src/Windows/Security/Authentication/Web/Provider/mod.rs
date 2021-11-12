#![allow(non_snake_case, non_camel_case_types)]
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
#[repr(C)]
pub struct WebAccountClientViewType(i32);
#[repr(transparent)]
pub struct WebAccountProviderAddAccountOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebAccountProviderDeleteAccountOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebAccountProviderGetTokenSilentOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebAccountProviderManageAccountOperation(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct WebAccountProviderOperationKind(i32);
#[repr(transparent)]
pub struct WebAccountProviderRequestTokenOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebAccountProviderRetrieveCookiesOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebAccountProviderSignOutAccountOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebAccountProviderTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct WebAccountScope(i32);
#[repr(C)]
pub struct WebAccountSelectionOptions(i32);
#[repr(transparent)]
pub struct WebProviderTokenRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebProviderTokenResponse(pub *mut ::core::ffi::c_void);
