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
    pub const RequestEntityTooLarge: Self = Self(0i32);
    pub const AttributeNotSupported: Self = Self(1i32);
    pub const AttributeValuesNotSupported: Self = Self(2i32);
    pub const AttributeNotSettable: Self = Self(3i32);
    pub const ConflictingAttributes: Self = Self(4i32);
}
#[repr(transparent)]
pub struct IppAttributeValue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IppAttributeValueKind(pub i32);
impl IppAttributeValueKind {
    pub const Unsupported: Self = Self(0i32);
    pub const Unknown: Self = Self(1i32);
    pub const NoValue: Self = Self(2i32);
    pub const Integer: Self = Self(3i32);
    pub const Boolean: Self = Self(4i32);
    pub const Enum: Self = Self(5i32);
    pub const OctetString: Self = Self(6i32);
    pub const DateTime: Self = Self(7i32);
    pub const Resolution: Self = Self(8i32);
    pub const RangeOfInteger: Self = Self(9i32);
    pub const Collection: Self = Self(10i32);
    pub const TextWithLanguage: Self = Self(11i32);
    pub const NameWithLanguage: Self = Self(12i32);
    pub const TextWithoutLanguage: Self = Self(13i32);
    pub const NameWithoutLanguage: Self = Self(14i32);
    pub const Keyword: Self = Self(15i32);
    pub const Uri: Self = Self(16i32);
    pub const UriSchema: Self = Self(17i32);
    pub const Charset: Self = Self(18i32);
    pub const NaturalLanguage: Self = Self(19i32);
    pub const MimeMediaType: Self = Self(20i32);
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
    pub const DotsPerInch: Self = Self(0i32);
    pub const DotsPerCentimeter: Self = Self(1i32);
}
#[repr(transparent)]
pub struct IppSetAttributesResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IppTextWithLanguage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Print3DDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintSchema(pub *mut ::core::ffi::c_void);
