pub const AllEnumeration: WcmNamespaceEnumerationFlags = 3i32;
pub const LIMITED_VALIDATION_MODE: u32 = 1u32;
pub const LINK_STORE_TO_ENGINE_INSTANCE: u32 = 1u32;
pub const OfflineMode: WcmTargetMode = 1i32;
pub const OnlineMode: WcmTargetMode = 2i32;
pub const ReadOnlyAccess: WcmNamespaceAccess = 1i32;
pub const ReadWriteAccess: WcmNamespaceAccess = 2i32;
pub const SharedEnumeration: WcmNamespaceEnumerationFlags = 1i32;
pub const UnknownStatus: WcmUserStatus = 0i32;
pub const UserEnumeration: WcmNamespaceEnumerationFlags = 2i32;
pub const UserLoaded: WcmUserStatus = 3i32;
pub const UserRegistered: WcmUserStatus = 1i32;
pub const UserUnloaded: WcmUserStatus = 4i32;
pub const UserUnregistered: WcmUserStatus = 2i32;
pub const WCM_E_ABORTOPERATION: windows_core::HRESULT = 0x80220028_u32 as _;
pub const WCM_E_ASSERTIONFAILED: windows_core::HRESULT = 0x8022001A_u32 as _;
pub const WCM_E_ATTRIBUTENOTALLOWED: windows_core::HRESULT = 0x80220004_u32 as _;
pub const WCM_E_ATTRIBUTENOTFOUND: windows_core::HRESULT = 0x80220003_u32 as _;
pub const WCM_E_CONFLICTINGASSERTION: windows_core::HRESULT = 0x80220019_u32 as _;
pub const WCM_E_CYCLICREFERENCE: windows_core::HRESULT = 0x80220023_u32 as _;
pub const WCM_E_DUPLICATENAME: windows_core::HRESULT = 0x8022001B_u32 as _;
pub const WCM_E_EXPRESSIONNOTFOUND: windows_core::HRESULT = 0x80220010_u32 as _;
pub const WCM_E_HANDLERNOTFOUND: windows_core::HRESULT = 0x8022001E_u32 as _;
pub const WCM_E_INTERNALERROR: windows_core::HRESULT = 0x80220000_u32 as _;
pub const WCM_E_INVALIDATTRIBUTECOMBINATION: windows_core::HRESULT = 0x80220027_u32 as _;
pub const WCM_E_INVALIDDATATYPE: windows_core::HRESULT = 0x80220008_u32 as _;
pub const WCM_E_INVALIDEXPRESSIONSYNTAX: windows_core::HRESULT = 0x80220017_u32 as _;
pub const WCM_E_INVALIDHANDLERSYNTAX: windows_core::HRESULT = 0x8022001F_u32 as _;
pub const WCM_E_INVALIDKEY: windows_core::HRESULT = 0x8022001C_u32 as _;
pub const WCM_E_INVALIDLANGUAGEFORMAT: windows_core::HRESULT = 0x8022000E_u32 as _;
pub const WCM_E_INVALIDPATH: windows_core::HRESULT = 0x8022000B_u32 as _;
pub const WCM_E_INVALIDPROCESSORFORMAT: windows_core::HRESULT = 0x8022002A_u32 as _;
pub const WCM_E_INVALIDSTREAM: windows_core::HRESULT = 0x8022001D_u32 as _;
pub const WCM_E_INVALIDVALUE: windows_core::HRESULT = 0x80220005_u32 as _;
pub const WCM_E_INVALIDVALUEFORMAT: windows_core::HRESULT = 0x80220006_u32 as _;
pub const WCM_E_INVALIDVERSIONFORMAT: windows_core::HRESULT = 0x8022000D_u32 as _;
pub const WCM_E_KEYNOTCHANGEABLE: windows_core::HRESULT = 0x8022000F_u32 as _;
pub const WCM_E_MANIFESTCOMPILATIONFAILED: windows_core::HRESULT = 0x80220022_u32 as _;
pub const WCM_E_MISSINGCONFIGURATION: windows_core::HRESULT = 0x80220029_u32 as _;
pub const WCM_E_MIXTYPEASSERTION: windows_core::HRESULT = 0x80220024_u32 as _;
pub const WCM_E_NAMESPACEALREADYREGISTERED: windows_core::HRESULT = 0x80220015_u32 as _;
pub const WCM_E_NAMESPACENOTFOUND: windows_core::HRESULT = 0x80220014_u32 as _;
pub const WCM_E_NOTIFICATIONNOTFOUND: windows_core::HRESULT = 0x80220018_u32 as _;
pub const WCM_E_NOTPOSITIONED: windows_core::HRESULT = 0x80220009_u32 as _;
pub const WCM_E_NOTSUPPORTEDFUNCTION: windows_core::HRESULT = 0x80220025_u32 as _;
pub const WCM_E_READONLYITEM: windows_core::HRESULT = 0x8022000A_u32 as _;
pub const WCM_E_RESTRICTIONFAILED: windows_core::HRESULT = 0x80220021_u32 as _;
pub const WCM_E_SOURCEMANEMPTYVALUE: windows_core::HRESULT = 0x8022002B_u32 as _;
pub const WCM_E_STATENODENOTALLOWED: windows_core::HRESULT = 0x80220002_u32 as _;
pub const WCM_E_STATENODENOTFOUND: windows_core::HRESULT = 0x80220001_u32 as _;
pub const WCM_E_STORECORRUPTED: windows_core::HRESULT = 0x80220016_u32 as _;
pub const WCM_E_SUBSTITUTIONNOTFOUND: windows_core::HRESULT = 0x80220011_u32 as _;
pub const WCM_E_TYPENOTSPECIFIED: windows_core::HRESULT = 0x80220007_u32 as _;
pub const WCM_E_UNKNOWNRESULT: windows_core::HRESULT = 0x80221003_u32 as _;
pub const WCM_E_USERALREADYREGISTERED: windows_core::HRESULT = 0x80220012_u32 as _;
pub const WCM_E_USERNOTFOUND: windows_core::HRESULT = 0x80220013_u32 as _;
pub const WCM_E_VALIDATIONFAILED: windows_core::HRESULT = 0x80220020_u32 as _;
pub const WCM_E_VALUETOOBIG: windows_core::HRESULT = 0x80220026_u32 as _;
pub const WCM_E_WRONGESCAPESTRING: windows_core::HRESULT = 0x8022000C_u32 as _;
pub const WCM_SETTINGS_ID_ARCHITECTURE: windows_core::PCWSTR = windows_core::w!("architecture");
pub const WCM_SETTINGS_ID_FLAG_DEFINITION: u32 = 1u32;
pub const WCM_SETTINGS_ID_FLAG_REFERENCE: u32 = 0u32;
pub const WCM_SETTINGS_ID_LANGUAGE: windows_core::PCWSTR = windows_core::w!("language");
pub const WCM_SETTINGS_ID_NAME: windows_core::PCWSTR = windows_core::w!("name");
pub const WCM_SETTINGS_ID_TOKEN: windows_core::PCWSTR = windows_core::w!("token");
pub const WCM_SETTINGS_ID_URI: windows_core::PCWSTR = windows_core::w!("uri");
pub const WCM_SETTINGS_ID_VERSION: windows_core::PCWSTR = windows_core::w!("version");
pub const WCM_SETTINGS_ID_VERSION_SCOPE: windows_core::PCWSTR = windows_core::w!("versionScope");
pub const WCM_S_ATTRIBUTENOTALLOWED: windows_core::HRESULT = 0x221005_u32 as _;
pub const WCM_S_ATTRIBUTENOTFOUND: windows_core::HRESULT = 0x221001_u32 as _;
pub const WCM_S_INTERNALERROR: windows_core::HRESULT = 0x221000_u32 as _;
pub const WCM_S_INVALIDATTRIBUTECOMBINATION: windows_core::HRESULT = 0x221004_u32 as _;
pub const WCM_S_LEGACYSETTINGWARNING: windows_core::HRESULT = 0x221002_u32 as _;
pub const WCM_S_NAMESPACENOTFOUND: windows_core::HRESULT = 0x221006_u32 as _;
pub const dataTypeBoolean: WcmDataType = 11i32;
pub const dataTypeByte: WcmDataType = 1i32;
pub const dataTypeFlagArray: WcmDataType = 32768i32;
pub const dataTypeInt16: WcmDataType = 4i32;
pub const dataTypeInt32: WcmDataType = 6i32;
pub const dataTypeInt64: WcmDataType = 8i32;
pub const dataTypeSByte: WcmDataType = 2i32;
pub const dataTypeString: WcmDataType = 12i32;
pub const dataTypeUInt16: WcmDataType = 3i32;
pub const dataTypeUInt32: WcmDataType = 5i32;
pub const dataTypeUInt64: WcmDataType = 7i32;
pub const restrictionFacetEnumeration: WcmRestrictionFacets = 2i32;
pub const restrictionFacetMaxInclusive: WcmRestrictionFacets = 4i32;
pub const restrictionFacetMaxLength: WcmRestrictionFacets = 1i32;
pub const restrictionFacetMinInclusive: WcmRestrictionFacets = 8i32;
pub const settingTypeComplex: WcmSettingType = 2i32;
pub const settingTypeList: WcmSettingType = 3i32;
pub const settingTypeScalar: WcmSettingType = 1i32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WcmDataType(pub i32);
impl windows_core::TypeKind for WcmDataType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WcmNamespaceAccess(pub i32);
impl windows_core::TypeKind for WcmNamespaceAccess {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WcmNamespaceEnumerationFlags(pub i32);
impl windows_core::TypeKind for WcmNamespaceEnumerationFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WcmRestrictionFacets(pub i32);
impl windows_core::TypeKind for WcmRestrictionFacets {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WcmSettingType(pub i32);
impl windows_core::TypeKind for WcmSettingType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WcmTargetMode(pub i32);
impl windows_core::TypeKind for WcmTargetMode {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WcmUserStatus(pub i32);
impl windows_core::TypeKind for WcmUserStatus {
    type TypeKind = windows_core::CopyType;
}
pub const SettingsEngine: windows_core::GUID = windows_core::GUID::from_u128(0x9f7d7bb5_20b3_11da_81a5_0030f1642e3c);
