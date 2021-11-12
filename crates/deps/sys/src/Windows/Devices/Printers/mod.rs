#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Devices_Printers_Extensions")]
pub mod Extensions;
#[link(name = "windows")]
extern "system" {}
pub struct IIppAttributeError(pub *mut ::core::ffi::c_void);
pub struct IIppAttributeValue(pub *mut ::core::ffi::c_void);
pub struct IIppAttributeValueStatics(pub *mut ::core::ffi::c_void);
pub struct IIppIntegerRange(pub *mut ::core::ffi::c_void);
pub struct IIppIntegerRangeFactory(pub *mut ::core::ffi::c_void);
pub struct IIppPrintDevice(pub *mut ::core::ffi::c_void);
pub struct IIppResolution(pub *mut ::core::ffi::c_void);
pub struct IIppResolutionFactory(pub *mut ::core::ffi::c_void);
pub struct IIppSetAttributesResult(pub *mut ::core::ffi::c_void);
pub struct IIppTextWithLanguage(pub *mut ::core::ffi::c_void);
pub struct IIppTextWithLanguageFactory(pub *mut ::core::ffi::c_void);
pub struct IPrint3DDevice(pub *mut ::core::ffi::c_void);
pub struct IPrint3DDeviceStatics(pub *mut ::core::ffi::c_void);
pub struct IPrintSchema(pub *mut ::core::ffi::c_void);
pub struct IppAttributeError(i32);
pub struct IppAttributeErrorReason(i32);
pub struct IppAttributeValue(i32);
pub struct IppAttributeValueKind(i32);
pub struct IppIntegerRange(i32);
pub struct IppPrintDevice(i32);
pub struct IppResolution(i32);
pub struct IppResolutionUnit(i32);
pub struct IppSetAttributesResult(i32);
pub struct IppTextWithLanguage(i32);
pub struct Print3DDevice(i32);
pub struct PrintSchema(i32);
pub struct PrintersContract(i32);
