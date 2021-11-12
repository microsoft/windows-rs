#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IItemEnumerator(i32);
pub struct ISettingsContext(i32);
pub struct ISettingsEngine(i32);
pub struct ISettingsIdentity(i32);
pub struct ISettingsItem(i32);
pub struct ISettingsNamespace(i32);
pub struct ISettingsResult(i32);
pub struct ITargetInfo(i32);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const LIMITED_VALIDATION_MODE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const LINK_STORE_TO_ENGINE_INSTANCE: u32 = 1u32;
pub struct SettingsEngine(i32);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_ABORTOPERATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255384i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_ASSERTIONFAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255398i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_ATTRIBUTENOTALLOWED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255420i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_ATTRIBUTENOTFOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255421i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_CONFLICTINGASSERTION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255399i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_CYCLICREFERENCE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255389i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_DUPLICATENAME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255397i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_EXPRESSIONNOTFOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255408i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_HANDLERNOTFOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255394i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_INTERNALERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255424i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_INVALIDATTRIBUTECOMBINATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255385i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_INVALIDDATATYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255416i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_INVALIDEXPRESSIONSYNTAX: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255401i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_INVALIDHANDLERSYNTAX: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255393i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_INVALIDKEY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255396i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_INVALIDLANGUAGEFORMAT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255410i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_INVALIDPATH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255413i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_INVALIDPROCESSORFORMAT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255382i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_INVALIDSTREAM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255395i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_INVALIDVALUE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255419i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_INVALIDVALUEFORMAT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255418i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_INVALIDVERSIONFORMAT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255411i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_KEYNOTCHANGEABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255409i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_MANIFESTCOMPILATIONFAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255390i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_MISSINGCONFIGURATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255383i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_MIXTYPEASSERTION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255388i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_NAMESPACEALREADYREGISTERED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255403i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_NAMESPACENOTFOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255404i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_NOTIFICATIONNOTFOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255400i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_NOTPOSITIONED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255415i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_NOTSUPPORTEDFUNCTION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255387i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_READONLYITEM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255414i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_RESTRICTIONFAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255391i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_SOURCEMANEMPTYVALUE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255381i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_STATENODENOTALLOWED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255422i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_STATENODENOTFOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255423i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_STORECORRUPTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255402i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_SUBSTITUTIONNOTFOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255407i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_TYPENOTSPECIFIED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255417i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_UNKNOWNRESULT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145251325i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_USERALREADYREGISTERED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255406i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_USERNOTFOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255405i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_VALIDATIONFAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255392i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_VALUETOOBIG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255386i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_E_WRONGESCAPESTRING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255412i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_SETTINGS_ID_FLAG_DEFINITION: u32 = 1u32;
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_SETTINGS_ID_FLAG_REFERENCE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_S_ATTRIBUTENOTALLOWED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2232325i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_S_ATTRIBUTENOTFOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2232321i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_S_INTERNALERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2232320i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_S_INVALIDATTRIBUTECOMBINATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2232324i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_S_LEGACYSETTINGWARNING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2232322i32 as _);
#[doc = "*Required features: `Win32_System_SettingsManagementInfrastructure`*"]
pub const WCM_S_NAMESPACENOTFOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2232326i32 as _);
pub struct WcmDataType(i32);
pub struct WcmNamespaceAccess(i32);
pub struct WcmNamespaceEnumerationFlags(i32);
pub struct WcmRestrictionFacets(i32);
pub struct WcmSettingType(i32);
pub struct WcmTargetMode(i32);
pub struct WcmUserStatus(i32);
