#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct ACCOUNTINGPROPERTIES(i32);
pub struct ATTRIBUTEFILTER(i32);
pub struct ATTRIBUTEID(i32);
pub struct ATTRIBUTEINFO(i32);
pub struct ATTRIBUTEPROPERTIES(i32);
pub struct ATTRIBUTERESTRICTIONS(i32);
pub struct ATTRIBUTESYNTAX(i32);
pub struct AUTHENTICATION_TYPE(i32);
pub struct CLIENTPROPERTIES(i32);
pub struct CONDITIONPROPERTIES(i32);
pub struct DICTIONARYPROPERTIES(i32);
pub struct IASCOMMONPROPERTIES(i32);
pub struct IASCOMPONENTPROPERTIES(i32);
pub struct IASDATASTORE(i32);
pub struct IASDOMAINTYPE(i32);
pub struct IASOSTYPE(i32);
pub struct IASPROPERTIES(i32);
pub struct IDENTITY_TYPE(i32);
pub struct IPFILTERPROPERTIES(i32);
#[repr(transparent)]
pub struct ISdo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISdoCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISdoDictionaryOld(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISdoMachine(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISdoMachine2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISdoServiceControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITemplateSdo(pub *mut ::core::ffi::c_void);
pub struct NAMESPROPERTIES(i32);
pub struct NAPPROPERTIES(i32);
pub struct NEW_LOG_FILE_FREQUENCY(i32);
pub struct NTEVENTLOGPROPERTIES(i32);
pub struct NTSAMPROPERTIES(i32);
pub struct POLICYPROPERTIES(i32);
pub struct PRADIUS_EXTENSION_FREE_ATTRIBUTES(i32);
pub struct PRADIUS_EXTENSION_INIT(i32);
pub struct PRADIUS_EXTENSION_PROCESS(i32);
pub struct PRADIUS_EXTENSION_PROCESS_2(i32);
pub struct PRADIUS_EXTENSION_PROCESS_EX(i32);
pub struct PRADIUS_EXTENSION_TERM(i32);
pub struct PROFILEPROPERTIES(i32);
pub struct PROTOCOLPROPERTIES(i32);
pub struct RADIUSPROPERTIES(i32);
pub struct RADIUSPROXYPROPERTIES(i32);
pub struct RADIUSSERVERGROUPPROPERTIES(i32);
pub struct RADIUSSERVERPROPERTIES(i32);
pub struct RADIUS_ACTION(i32);
pub struct RADIUS_ATTRIBUTE(i32);
pub struct RADIUS_ATTRIBUTE_ARRAY(i32);
pub struct RADIUS_ATTRIBUTE_TYPE(i32);
pub struct RADIUS_AUTHENTICATION_PROVIDER(i32);
pub struct RADIUS_CODE(i32);
pub struct RADIUS_DATA_TYPE(i32);
pub struct RADIUS_EXTENSION_CONTROL_BLOCK(i32);
pub struct RADIUS_EXTENSION_POINT(i32);
#[doc = "*Required features: `Win32_NetworkManagement_NetworkPolicyServer`*"]
pub const RADIUS_EXTENSION_VERSION: u32 = 1u32;
pub struct RADIUS_REJECT_REASON_CODE(i32);
pub struct RADIUS_VSA_FORMAT(i32);
pub struct REMEDIATIONSERVERGROUPPROPERTIES(i32);
pub struct REMEDIATIONSERVERPROPERTIES(i32);
pub struct REMEDIATIONSERVERSPROPERTIES(i32);
pub struct SERVICE_TYPE(i32);
pub struct SHAREDSECRETPROPERTIES(i32);
pub struct SHVTEMPLATEPROPERTIES(i32);
pub struct SHV_COMBINATION_TYPE(i32);
pub struct SdoMachine(i32);
pub struct TEMPLATESPROPERTIES(i32);
pub struct USERPROPERTIES(i32);
pub struct VENDORPROPERTIES(i32);
