#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AccountsSettingsPane(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AccountsSettingsPaneCommandsRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AccountsSettingsPaneEventDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CredentialCommand(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CredentialCommandCredentialDeletedHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAccountsSettingsPane(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAccountsSettingsPaneCommandsRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAccountsSettingsPaneCommandsRequestedEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAccountsSettingsPaneEventDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAccountsSettingsPaneStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAccountsSettingsPaneStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAccountsSettingsPaneStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICredentialCommand(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICredentialCommandFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISettingsCommandFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISettingsCommandStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISettingsPane(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISettingsPaneCommandsRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISettingsPaneCommandsRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISettingsPaneStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAccountCommand(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAccountCommandFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAccountInvokedArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAccountProviderCommand(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAccountProviderCommandFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SettingsCommand(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SettingsEdgeLocation(pub i32);
impl SettingsEdgeLocation {
    pub const Right: Self = Self(0i32);
    pub const Left: Self = Self(1i32);
}
impl ::core::marker::Copy for SettingsEdgeLocation {}
impl ::core::clone::Clone for SettingsEdgeLocation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SettingsPane(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SettingsPaneCommandsRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SettingsPaneCommandsRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SupportedWebAccountActions(pub u32);
impl SupportedWebAccountActions {
    pub const None: Self = Self(0u32);
    pub const Reconnect: Self = Self(1u32);
    pub const Remove: Self = Self(2u32);
    pub const ViewDetails: Self = Self(4u32);
    pub const Manage: Self = Self(8u32);
    pub const More: Self = Self(16u32);
}
impl ::core::marker::Copy for SupportedWebAccountActions {}
impl ::core::clone::Clone for SupportedWebAccountActions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebAccountAction(pub i32);
impl WebAccountAction {
    pub const Reconnect: Self = Self(0i32);
    pub const Remove: Self = Self(1i32);
    pub const ViewDetails: Self = Self(2i32);
    pub const Manage: Self = Self(3i32);
    pub const More: Self = Self(4i32);
}
impl ::core::marker::Copy for WebAccountAction {}
impl ::core::clone::Clone for WebAccountAction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebAccountCommand(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebAccountCommandInvokedHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebAccountInvokedArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebAccountProviderCommand(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebAccountProviderCommandInvokedHandler(pub *mut ::core::ffi::c_void);
