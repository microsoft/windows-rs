#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(transparent)]
pub struct IppAttributeErrorReason(pub i32);
impl IppAttributeErrorReason {
    pub const RequestEntityTooLarge: IppAttributeErrorReason = IppAttributeErrorReason(0i32);
    pub const AttributeNotSupported: IppAttributeErrorReason = IppAttributeErrorReason(1i32);
    pub const AttributeValuesNotSupported: IppAttributeErrorReason = IppAttributeErrorReason(2i32);
    pub const AttributeNotSettable: IppAttributeErrorReason = IppAttributeErrorReason(3i32);
    pub const ConflictingAttributes: IppAttributeErrorReason = IppAttributeErrorReason(4i32);
}
#[repr(transparent)]
pub struct IppAttributeValue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IppAttributeValueKind(pub i32);
impl IppAttributeValueKind {
    pub const Unsupported: IppAttributeValueKind = IppAttributeValueKind(0i32);
    pub const Unknown: IppAttributeValueKind = IppAttributeValueKind(1i32);
    pub const NoValue: IppAttributeValueKind = IppAttributeValueKind(2i32);
    pub const Integer: IppAttributeValueKind = IppAttributeValueKind(3i32);
    pub const Boolean: IppAttributeValueKind = IppAttributeValueKind(4i32);
    pub const Enum: IppAttributeValueKind = IppAttributeValueKind(5i32);
    pub const OctetString: IppAttributeValueKind = IppAttributeValueKind(6i32);
    pub const DateTime: IppAttributeValueKind = IppAttributeValueKind(7i32);
    pub const Resolution: IppAttributeValueKind = IppAttributeValueKind(8i32);
    pub const RangeOfInteger: IppAttributeValueKind = IppAttributeValueKind(9i32);
    pub const Collection: IppAttributeValueKind = IppAttributeValueKind(10i32);
    pub const TextWithLanguage: IppAttributeValueKind = IppAttributeValueKind(11i32);
    pub const NameWithLanguage: IppAttributeValueKind = IppAttributeValueKind(12i32);
    pub const TextWithoutLanguage: IppAttributeValueKind = IppAttributeValueKind(13i32);
    pub const NameWithoutLanguage: IppAttributeValueKind = IppAttributeValueKind(14i32);
    pub const Keyword: IppAttributeValueKind = IppAttributeValueKind(15i32);
    pub const Uri: IppAttributeValueKind = IppAttributeValueKind(16i32);
    pub const UriSchema: IppAttributeValueKind = IppAttributeValueKind(17i32);
    pub const Charset: IppAttributeValueKind = IppAttributeValueKind(18i32);
    pub const NaturalLanguage: IppAttributeValueKind = IppAttributeValueKind(19i32);
    pub const MimeMediaType: IppAttributeValueKind = IppAttributeValueKind(20i32);
}
#[repr(transparent)]
pub struct IppIntegerRange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IppPrintDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IppResolution(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IppResolutionUnit(pub i32);
impl IppResolutionUnit {
    pub const DotsPerInch: IppResolutionUnit = IppResolutionUnit(0i32);
    pub const DotsPerCentimeter: IppResolutionUnit = IppResolutionUnit(1i32);
}
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
