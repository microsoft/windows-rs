#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IItemEnumerator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IItemEnumerator {}
impl ::core::clone::Clone for IItemEnumerator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISettingsContext(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISettingsContext {}
impl ::core::clone::Clone for ISettingsContext {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISettingsEngine(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISettingsEngine {}
impl ::core::clone::Clone for ISettingsEngine {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISettingsIdentity(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISettingsIdentity {}
impl ::core::clone::Clone for ISettingsIdentity {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISettingsItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISettingsItem {}
impl ::core::clone::Clone for ISettingsItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISettingsNamespace(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISettingsNamespace {}
impl ::core::clone::Clone for ISettingsNamespace {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISettingsResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISettingsResult {}
impl ::core::clone::Clone for ISettingsResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITargetInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITargetInfo {}
impl ::core::clone::Clone for ITargetInfo {
    fn clone(&self) -> Self {
        *self
    }
}
pub const LIMITED_VALIDATION_MODE: u32 = 1u32;
pub const LINK_STORE_TO_ENGINE_INSTANCE: u32 = 1u32;
pub const SettingsEngine: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2675801013, data2: 8371, data3: 4570, data4: [129, 165, 0, 48, 241, 100, 46, 60] };
pub const WCM_E_ABORTOPERATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255384i32 as _);
pub const WCM_E_ASSERTIONFAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255398i32 as _);
pub const WCM_E_ATTRIBUTENOTALLOWED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255420i32 as _);
pub const WCM_E_ATTRIBUTENOTFOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255421i32 as _);
pub const WCM_E_CONFLICTINGASSERTION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255399i32 as _);
pub const WCM_E_CYCLICREFERENCE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255389i32 as _);
pub const WCM_E_DUPLICATENAME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255397i32 as _);
pub const WCM_E_EXPRESSIONNOTFOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255408i32 as _);
pub const WCM_E_HANDLERNOTFOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255394i32 as _);
pub const WCM_E_INTERNALERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255424i32 as _);
pub const WCM_E_INVALIDATTRIBUTECOMBINATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255385i32 as _);
pub const WCM_E_INVALIDDATATYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255416i32 as _);
pub const WCM_E_INVALIDEXPRESSIONSYNTAX: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255401i32 as _);
pub const WCM_E_INVALIDHANDLERSYNTAX: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255393i32 as _);
pub const WCM_E_INVALIDKEY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255396i32 as _);
pub const WCM_E_INVALIDLANGUAGEFORMAT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255410i32 as _);
pub const WCM_E_INVALIDPATH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255413i32 as _);
pub const WCM_E_INVALIDPROCESSORFORMAT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255382i32 as _);
pub const WCM_E_INVALIDSTREAM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255395i32 as _);
pub const WCM_E_INVALIDVALUE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255419i32 as _);
pub const WCM_E_INVALIDVALUEFORMAT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255418i32 as _);
pub const WCM_E_INVALIDVERSIONFORMAT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255411i32 as _);
pub const WCM_E_KEYNOTCHANGEABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255409i32 as _);
pub const WCM_E_MANIFESTCOMPILATIONFAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255390i32 as _);
pub const WCM_E_MISSINGCONFIGURATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255383i32 as _);
pub const WCM_E_MIXTYPEASSERTION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255388i32 as _);
pub const WCM_E_NAMESPACEALREADYREGISTERED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255403i32 as _);
pub const WCM_E_NAMESPACENOTFOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255404i32 as _);
pub const WCM_E_NOTIFICATIONNOTFOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255400i32 as _);
pub const WCM_E_NOTPOSITIONED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255415i32 as _);
pub const WCM_E_NOTSUPPORTEDFUNCTION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255387i32 as _);
pub const WCM_E_READONLYITEM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255414i32 as _);
pub const WCM_E_RESTRICTIONFAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255391i32 as _);
pub const WCM_E_SOURCEMANEMPTYVALUE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255381i32 as _);
pub const WCM_E_STATENODENOTALLOWED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255422i32 as _);
pub const WCM_E_STATENODENOTFOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255423i32 as _);
pub const WCM_E_STORECORRUPTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255402i32 as _);
pub const WCM_E_SUBSTITUTIONNOTFOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255407i32 as _);
pub const WCM_E_TYPENOTSPECIFIED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255417i32 as _);
pub const WCM_E_UNKNOWNRESULT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145251325i32 as _);
pub const WCM_E_USERALREADYREGISTERED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255406i32 as _);
pub const WCM_E_USERNOTFOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255405i32 as _);
pub const WCM_E_VALIDATIONFAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255392i32 as _);
pub const WCM_E_VALUETOOBIG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255386i32 as _);
pub const WCM_E_WRONGESCAPESTRING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2145255412i32 as _);
pub const WCM_SETTINGS_ID_FLAG_DEFINITION: u32 = 1u32;
pub const WCM_SETTINGS_ID_FLAG_REFERENCE: u32 = 0u32;
pub const WCM_S_ATTRIBUTENOTALLOWED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2232325i32 as _);
pub const WCM_S_ATTRIBUTENOTFOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2232321i32 as _);
pub const WCM_S_INTERNALERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2232320i32 as _);
pub const WCM_S_INVALIDATTRIBUTECOMBINATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2232324i32 as _);
pub const WCM_S_LEGACYSETTINGWARNING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2232322i32 as _);
pub const WCM_S_NAMESPACENOTFOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2232326i32 as _);
pub const dataTypeByte: i32 = 1i32;
pub const dataTypeSByte: i32 = 2i32;
pub const dataTypeUInt16: i32 = 3i32;
pub const dataTypeInt16: i32 = 4i32;
pub const dataTypeUInt32: i32 = 5i32;
pub const dataTypeInt32: i32 = 6i32;
pub const dataTypeUInt64: i32 = 7i32;
pub const dataTypeInt64: i32 = 8i32;
pub const dataTypeBoolean: i32 = 11i32;
pub const dataTypeString: i32 = 12i32;
pub const dataTypeFlagArray: i32 = 32768i32;
pub const ReadOnlyAccess: i32 = 1i32;
pub const ReadWriteAccess: i32 = 2i32;
pub const SharedEnumeration: i32 = 1i32;
pub const UserEnumeration: i32 = 2i32;
pub const AllEnumeration: i32 = 3i32;
pub const restrictionFacetMaxLength: i32 = 1i32;
pub const restrictionFacetEnumeration: i32 = 2i32;
pub const restrictionFacetMaxInclusive: i32 = 4i32;
pub const restrictionFacetMinInclusive: i32 = 8i32;
pub const settingTypeScalar: i32 = 1i32;
pub const settingTypeComplex: i32 = 2i32;
pub const settingTypeList: i32 = 3i32;
pub const OfflineMode: i32 = 1i32;
pub const OnlineMode: i32 = 2i32;
pub const UnknownStatus: i32 = 0i32;
pub const UserRegistered: i32 = 1i32;
pub const UserUnregistered: i32 = 2i32;
pub const UserLoaded: i32 = 3i32;
pub const UserUnloaded: i32 = 4i32;
