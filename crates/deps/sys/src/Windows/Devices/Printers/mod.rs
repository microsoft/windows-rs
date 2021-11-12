#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Devices_Printers_Extensions")]
pub mod Extensions;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IIppAttributeError(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIppAttributeValue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIppAttributeValueStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIppIntegerRange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIppIntegerRangeFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIppPrintDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIppResolution(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIppResolutionFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIppSetAttributesResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIppTextWithLanguage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIppTextWithLanguageFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrint3DDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrint3DDeviceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintSchema(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IppAttributeError(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct IppAttributeErrorReason(i32);
#[repr(transparent)]
pub struct IppAttributeValue(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct IppAttributeValueKind(i32);
#[repr(transparent)]
pub struct IppIntegerRange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IppPrintDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IppResolution(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct IppResolutionUnit(i32);
#[repr(transparent)]
pub struct IppSetAttributesResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IppTextWithLanguage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Print3DDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintSchema(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PrintersContract(i32);
