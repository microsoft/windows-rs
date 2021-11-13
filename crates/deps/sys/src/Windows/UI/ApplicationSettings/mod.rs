#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AccountsSettingsPane(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AccountsSettingsPane {}
impl ::core::clone::Clone for AccountsSettingsPane {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AccountsSettingsPaneCommandsRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AccountsSettingsPaneCommandsRequestedEventArgs {}
impl ::core::clone::Clone for AccountsSettingsPaneCommandsRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AccountsSettingsPaneEventDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AccountsSettingsPaneEventDeferral {}
impl ::core::clone::Clone for AccountsSettingsPaneEventDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CredentialCommand(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CredentialCommand {}
impl ::core::clone::Clone for CredentialCommand {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CredentialCommandCredentialDeletedHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CredentialCommandCredentialDeletedHandler {}
impl ::core::clone::Clone for CredentialCommandCredentialDeletedHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAccountsSettingsPane(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAccountsSettingsPane {}
impl ::core::clone::Clone for IAccountsSettingsPane {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAccountsSettingsPaneCommandsRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAccountsSettingsPaneCommandsRequestedEventArgs {}
impl ::core::clone::Clone for IAccountsSettingsPaneCommandsRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAccountsSettingsPaneCommandsRequestedEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAccountsSettingsPaneCommandsRequestedEventArgs2 {}
impl ::core::clone::Clone for IAccountsSettingsPaneCommandsRequestedEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAccountsSettingsPaneEventDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAccountsSettingsPaneEventDeferral {}
impl ::core::clone::Clone for IAccountsSettingsPaneEventDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAccountsSettingsPaneStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAccountsSettingsPaneStatics {}
impl ::core::clone::Clone for IAccountsSettingsPaneStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAccountsSettingsPaneStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAccountsSettingsPaneStatics2 {}
impl ::core::clone::Clone for IAccountsSettingsPaneStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAccountsSettingsPaneStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAccountsSettingsPaneStatics3 {}
impl ::core::clone::Clone for IAccountsSettingsPaneStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICredentialCommand(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICredentialCommand {}
impl ::core::clone::Clone for ICredentialCommand {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICredentialCommandFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICredentialCommandFactory {}
impl ::core::clone::Clone for ICredentialCommandFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISettingsCommandFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISettingsCommandFactory {}
impl ::core::clone::Clone for ISettingsCommandFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISettingsCommandStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISettingsCommandStatics {}
impl ::core::clone::Clone for ISettingsCommandStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISettingsPane(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISettingsPane {}
impl ::core::clone::Clone for ISettingsPane {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISettingsPaneCommandsRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISettingsPaneCommandsRequest {}
impl ::core::clone::Clone for ISettingsPaneCommandsRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISettingsPaneCommandsRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISettingsPaneCommandsRequestedEventArgs {}
impl ::core::clone::Clone for ISettingsPaneCommandsRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISettingsPaneStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISettingsPaneStatics {}
impl ::core::clone::Clone for ISettingsPaneStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebAccountCommand(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebAccountCommand {}
impl ::core::clone::Clone for IWebAccountCommand {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebAccountCommandFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebAccountCommandFactory {}
impl ::core::clone::Clone for IWebAccountCommandFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebAccountInvokedArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebAccountInvokedArgs {}
impl ::core::clone::Clone for IWebAccountInvokedArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebAccountProviderCommand(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebAccountProviderCommand {}
impl ::core::clone::Clone for IWebAccountProviderCommand {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebAccountProviderCommandFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebAccountProviderCommandFactory {}
impl ::core::clone::Clone for IWebAccountProviderCommandFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SettingsCommand(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SettingsCommand {}
impl ::core::clone::Clone for SettingsCommand {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for SettingsPane {}
impl ::core::clone::Clone for SettingsPane {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SettingsPaneCommandsRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SettingsPaneCommandsRequest {}
impl ::core::clone::Clone for SettingsPaneCommandsRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SettingsPaneCommandsRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SettingsPaneCommandsRequestedEventArgs {}
impl ::core::clone::Clone for SettingsPaneCommandsRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for WebAccountCommand {}
impl ::core::clone::Clone for WebAccountCommand {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebAccountCommandInvokedHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebAccountCommandInvokedHandler {}
impl ::core::clone::Clone for WebAccountCommandInvokedHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebAccountInvokedArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebAccountInvokedArgs {}
impl ::core::clone::Clone for WebAccountInvokedArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebAccountProviderCommand(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebAccountProviderCommand {}
impl ::core::clone::Clone for WebAccountProviderCommand {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebAccountProviderCommandInvokedHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebAccountProviderCommandInvokedHandler {}
impl ::core::clone::Clone for WebAccountProviderCommandInvokedHandler {
    fn clone(&self) -> Self {
        *self
    }
}
