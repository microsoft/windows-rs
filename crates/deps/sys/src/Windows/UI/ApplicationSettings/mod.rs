#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AccountsSettingsPane(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AccountsSettingsPaneCommandsRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AccountsSettingsPaneEventDeferral(pub *mut ::core::ffi::c_void);
pub struct ApplicationsSettingsContract(i32);
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
pub struct SettingsEdgeLocation(i32);
#[repr(transparent)]
pub struct SettingsPane(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SettingsPaneCommandsRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SettingsPaneCommandsRequestedEventArgs(pub *mut ::core::ffi::c_void);
pub struct SupportedWebAccountActions(i32);
pub struct WebAccountAction(i32);
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
