#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct ACCOUNTINGPROPERTIES(i32);
#[repr(C)]
pub struct ATTRIBUTEFILTER(i32);
#[repr(C)]
pub struct ATTRIBUTEID(i32);
#[repr(C)]
pub struct ATTRIBUTEINFO(i32);
#[repr(C)]
pub struct ATTRIBUTEPROPERTIES(i32);
#[repr(C)]
pub struct ATTRIBUTERESTRICTIONS(i32);
#[repr(C)]
pub struct ATTRIBUTESYNTAX(i32);
#[repr(C)]
pub struct AUTHENTICATION_TYPE(i32);
#[repr(C)]
pub struct CLIENTPROPERTIES(i32);
#[repr(C)]
pub struct CONDITIONPROPERTIES(i32);
#[repr(C)]
pub struct DICTIONARYPROPERTIES(i32);
#[repr(C)]
pub struct IASCOMMONPROPERTIES(i32);
#[repr(C)]
pub struct IASCOMPONENTPROPERTIES(i32);
#[repr(C)]
pub struct IASDATASTORE(i32);
#[repr(C)]
pub struct IASDOMAINTYPE(i32);
#[repr(C)]
pub struct IASOSTYPE(i32);
#[repr(C)]
pub struct IASPROPERTIES(i32);
#[repr(C)]
pub struct IDENTITY_TYPE(i32);
#[repr(C)]
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
#[repr(C)]
pub struct NAMESPROPERTIES(i32);
#[repr(C)]
pub struct NAPPROPERTIES(i32);
#[repr(C)]
pub struct NEW_LOG_FILE_FREQUENCY(i32);
#[repr(C)]
pub struct NTEVENTLOGPROPERTIES(i32);
#[repr(C)]
pub struct NTSAMPROPERTIES(i32);
#[repr(C)]
pub struct POLICYPROPERTIES(i32);
#[repr(C)]
pub struct PRADIUS_EXTENSION_FREE_ATTRIBUTES(i32);
#[repr(C)]
pub struct PRADIUS_EXTENSION_INIT(i32);
#[repr(C)]
pub struct PRADIUS_EXTENSION_PROCESS(i32);
#[repr(C)]
pub struct PRADIUS_EXTENSION_PROCESS_2(i32);
#[repr(C)]
pub struct PRADIUS_EXTENSION_PROCESS_EX(i32);
#[repr(C)]
pub struct PRADIUS_EXTENSION_TERM(i32);
#[repr(C)]
pub struct PROFILEPROPERTIES(i32);
#[repr(C)]
pub struct PROTOCOLPROPERTIES(i32);
#[repr(C)]
pub struct RADIUSPROPERTIES(i32);
#[repr(C)]
pub struct RADIUSPROXYPROPERTIES(i32);
#[repr(C)]
pub struct RADIUSSERVERGROUPPROPERTIES(i32);
#[repr(C)]
pub struct RADIUSSERVERPROPERTIES(i32);
#[repr(C)]
pub struct RADIUS_ACTION(i32);
#[repr(C)]
pub struct RADIUS_ATTRIBUTE(i32);
#[repr(C)]
pub struct RADIUS_ATTRIBUTE_ARRAY(i32);
#[repr(C)]
pub struct RADIUS_ATTRIBUTE_TYPE(i32);
#[repr(C)]
pub struct RADIUS_AUTHENTICATION_PROVIDER(i32);
#[repr(C)]
pub struct RADIUS_CODE(i32);
#[repr(C)]
pub struct RADIUS_DATA_TYPE(i32);
#[repr(C)]
pub struct RADIUS_EXTENSION_CONTROL_BLOCK(i32);
#[repr(C)]
pub struct RADIUS_EXTENSION_POINT(i32);
pub const RADIUS_EXTENSION_VERSION: u32 = 1u32;
#[repr(C)]
pub struct RADIUS_REJECT_REASON_CODE(i32);
#[repr(C)]
pub struct RADIUS_VSA_FORMAT(i32);
#[repr(C)]
pub struct REMEDIATIONSERVERGROUPPROPERTIES(i32);
#[repr(C)]
pub struct REMEDIATIONSERVERPROPERTIES(i32);
#[repr(C)]
pub struct REMEDIATIONSERVERSPROPERTIES(i32);
#[repr(C)]
pub struct SERVICE_TYPE(i32);
#[repr(C)]
pub struct SHAREDSECRETPROPERTIES(i32);
#[repr(C)]
pub struct SHVTEMPLATEPROPERTIES(i32);
#[repr(C)]
pub struct SHV_COMBINATION_TYPE(i32);
#[repr(C)]
pub struct SdoMachine(i32);
#[repr(C)]
pub struct TEMPLATESPROPERTIES(i32);
#[repr(C)]
pub struct USERPROPERTIES(i32);
#[repr(C)]
pub struct VENDORPROPERTIES(i32);
