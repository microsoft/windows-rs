#[cfg(feature = "Devices_Printers_Extensions")]
pub mod Extensions;
#[doc(hidden)]
#[repr(transparent)]
pub struct IIppAttributeError(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIppAttributeError {
    type Vtable = IIppAttributeError_Vtbl;
}
impl ::core::clone::Clone for IIppAttributeError {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IIppAttributeError {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x750feda1_9eef_5c39_93e4_46149bbcef27);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppAttributeError_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IppAttributeErrorReason) -> ::windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetUnsupportedValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetUnsupportedValues: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIppAttributeValue(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIppAttributeValue {
    type Vtable = IIppAttributeValue_Vtbl;
}
impl ::core::clone::Clone for IIppAttributeValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IIppAttributeValue {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x99407fed_e2bb_59a3_988b_28a974052a26);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppAttributeValue_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IppAttributeValueKind) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetIntegerArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetIntegerArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetBooleanArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetBooleanArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetEnumArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetEnumArray: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub GetOctetStringArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    GetOctetStringArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetDateTimeArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetDateTimeArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetResolutionArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetResolutionArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetRangeOfIntegerArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetRangeOfIntegerArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCollectionArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCollectionArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetTextWithLanguageArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetTextWithLanguageArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetNameWithLanguageArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetNameWithLanguageArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetTextWithoutLanguageArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetTextWithoutLanguageArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetNameWithoutLanguageArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetNameWithoutLanguageArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetKeywordArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetKeywordArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetUriArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetUriArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetUriSchemaArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetUriSchemaArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCharsetArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCharsetArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetNaturalLanguageArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetNaturalLanguageArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetMimeMediaTypeArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetMimeMediaTypeArray: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIppAttributeValueStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIppAttributeValueStatics {
    type Vtable = IIppAttributeValueStatics_Vtbl;
}
impl ::core::clone::Clone for IIppAttributeValueStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IIppAttributeValueStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x10d43942_dd94_5998_b235_afafb6fa7935);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppAttributeValueStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateUnsupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateUnknown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateNoValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateInteger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateIntegerArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateIntegerArray: usize,
    pub CreateBoolean: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateBooleanArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateBooleanArray: usize,
    pub CreateEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateEnumArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateEnumArray: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateOctetString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateOctetString: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub CreateOctetStringArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    CreateOctetStringArray: usize,
    #[cfg(feature = "Foundation")]
    pub CreateDateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateDateTime: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateDateTimeArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateDateTimeArray: usize,
    pub CreateResolution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateResolutionArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateResolutionArray: usize,
    pub CreateRangeOfInteger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateRangeOfIntegerArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateRangeOfIntegerArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memberattributes: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateCollection: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateCollectionArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memberattributesarray: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateCollectionArray: usize,
    pub CreateTextWithLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateTextWithLanguageArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateTextWithLanguageArray: usize,
    pub CreateNameWithLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateNameWithLanguageArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateNameWithLanguageArray: usize,
    pub CreateTextWithoutLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateTextWithoutLanguageArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateTextWithoutLanguageArray: usize,
    pub CreateNameWithoutLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateNameWithoutLanguageArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateNameWithoutLanguageArray: usize,
    pub CreateKeyword: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateKeywordArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateKeywordArray: usize,
    #[cfg(feature = "Foundation")]
    pub CreateUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateUri: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateUriArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateUriArray: usize,
    pub CreateUriSchema: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateUriSchemaArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateUriSchemaArray: usize,
    pub CreateCharset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateCharsetArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateCharsetArray: usize,
    pub CreateNaturalLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateNaturalLanguageArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateNaturalLanguageArray: usize,
    pub CreateMimeMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateMimeMediaArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateMimeMediaArray: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIppIntegerRange(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIppIntegerRange {
    type Vtable = IIppIntegerRange_Vtbl;
}
impl ::core::clone::Clone for IIppIntegerRange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IIppIntegerRange {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x92907346_c3ea_5ed6_bdb1_3752c62c6f7f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppIntegerRange_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub End: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIppIntegerRangeFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIppIntegerRangeFactory {
    type Vtable = IIppIntegerRangeFactory_Vtbl;
}
impl ::core::clone::Clone for IIppIntegerRangeFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IIppIntegerRangeFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x75d4ecae_f87e_54ad_b5d0_465204db7553);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppIntegerRangeFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, start: i32, end: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIppPrintDevice(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIppPrintDevice {
    type Vtable = IIppPrintDevice_Vtbl;
}
impl ::core::clone::Clone for IIppPrintDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IIppPrintDevice {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd748ac56_76f3_5dc6_afd4_c2a8686b9359);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppPrintDevice_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PrinterName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PrinterUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PrinterUri: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub GetPrinterAttributesAsBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributenames: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    GetPrinterAttributesAsBuffer: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetPrinterAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributenames: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetPrinterAttributes: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetPrinterAttributesFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printerattributesbuffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetPrinterAttributesFromBuffer: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetPrinterAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printerattributes: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetPrinterAttributes: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIppPrintDevice2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIppPrintDevice2 {
    type Vtable = IIppPrintDevice2_Vtbl;
}
impl ::core::clone::Clone for IIppPrintDevice2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IIppPrintDevice2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf7c844c9_9d21_5c63_ac20_3676915be2d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppPrintDevice2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetMaxSupportedPdfSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub GetMaxSupportedPdfVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsPdlPassthroughSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdlcontenttype: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
    pub GetPdlPassthroughProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIppPrintDeviceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIppPrintDeviceStatics {
    type Vtable = IIppPrintDeviceStatics_Vtbl;
}
impl ::core::clone::Clone for IIppPrintDeviceStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IIppPrintDeviceStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7dc19f08_7f20_52ab_94a7_894b83b2a17e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppPrintDeviceStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FromId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FromPrinterName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printername: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub IsIppPrinter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printername: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIppResolution(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIppResolution {
    type Vtable = IIppResolution_Vtbl;
}
impl ::core::clone::Clone for IIppResolution {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IIppResolution {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcb493f86_6bf3_56f5_86ce_263d08aead63);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppResolution_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub Unit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IppResolutionUnit) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIppResolutionFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIppResolutionFactory {
    type Vtable = IIppResolutionFactory_Vtbl;
}
impl ::core::clone::Clone for IIppResolutionFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IIppResolutionFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe481c2ae_251a_5326_b173_95543ed99a35);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppResolutionFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: i32, height: i32, unit: IppResolutionUnit, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIppSetAttributesResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIppSetAttributesResult {
    type Vtable = IIppSetAttributesResult_Vtbl;
}
impl ::core::clone::Clone for IIppSetAttributesResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IIppSetAttributesResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7d1c7f55_aa9d_58a3_90e9_17bdc5281f07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppSetAttributesResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Succeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AttributeErrors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AttributeErrors: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIppTextWithLanguage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIppTextWithLanguage {
    type Vtable = IIppTextWithLanguage_Vtbl;
}
impl ::core::clone::Clone for IIppTextWithLanguage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IIppTextWithLanguage {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x326447a6_5149_5936_90e8_0c736036bf77);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppTextWithLanguage_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIppTextWithLanguageFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIppTextWithLanguageFactory {
    type Vtable = IIppTextWithLanguageFactory_Vtbl;
}
impl ::core::clone::Clone for IIppTextWithLanguageFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IIppTextWithLanguageFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xca4a1e8d_2968_5775_997c_8a46f1a574ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppTextWithLanguageFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: ::std::mem::MaybeUninit<::windows_core::HSTRING>, text: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPageConfigurationSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPageConfigurationSettings {
    type Vtable = IPageConfigurationSettings_Vtbl;
}
impl ::core::clone::Clone for IPageConfigurationSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPageConfigurationSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb6fc1e02_5331_54ff_95a0_1fcb76bb97a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPageConfigurationSettings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OrientationSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PageConfigurationSource) -> ::windows_core::HRESULT,
    pub SetOrientationSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PageConfigurationSource) -> ::windows_core::HRESULT,
    pub SizeSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PageConfigurationSource) -> ::windows_core::HRESULT,
    pub SetSizeSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PageConfigurationSource) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPdlPassthroughProvider(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPdlPassthroughProvider {
    type Vtable = IPdlPassthroughProvider_Vtbl;
}
impl ::core::clone::Clone for IPdlPassthroughProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPdlPassthroughProvider {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x23c71dd2_6117_553f_9378_180af5849a49);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPdlPassthroughProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedPdlContentTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedPdlContentTypes: usize,
    #[cfg(feature = "Graphics_Printing")]
    pub StartPrintJobWithTaskOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, jobname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, pdlcontenttype: ::std::mem::MaybeUninit<::windows_core::HSTRING>, taskoptions: *mut ::core::ffi::c_void, pageconfigurationsettings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing"))]
    StartPrintJobWithTaskOptions: usize,
    #[cfg(feature = "Storage_Streams")]
    pub StartPrintJobWithPrintTicket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, jobname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, pdlcontenttype: ::std::mem::MaybeUninit<::windows_core::HSTRING>, printticket: *mut ::core::ffi::c_void, pageconfigurationsettings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    StartPrintJobWithPrintTicket: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPdlPassthroughTarget(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPdlPassthroughTarget {
    type Vtable = IPdlPassthroughTarget_Vtbl;
}
impl ::core::clone::Clone for IPdlPassthroughTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPdlPassthroughTarget {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9840be79_67f8_5385_a5b9_e8c96e0fca76);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPdlPassthroughTarget_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PrintJobId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetOutputStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetOutputStream: usize,
    pub Submit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrint3DDevice(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrint3DDevice {
    type Vtable = IPrint3DDevice_Vtbl;
}
impl ::core::clone::Clone for IPrint3DDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPrint3DDevice {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x041c3d19_9713_42a2_9813_7dc3337428d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint3DDevice_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PrintSchema: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrint3DDeviceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrint3DDeviceStatics {
    type Vtable = IPrint3DDeviceStatics_Vtbl;
}
impl ::core::clone::Clone for IPrint3DDeviceStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPrint3DDeviceStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfde3620a_67cd_41b7_a344_5150a1fd75b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint3DDeviceStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintSchema(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintSchema {
    type Vtable = IPrintSchema_Vtbl;
}
impl ::core::clone::Clone for IPrintSchema {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPrintSchema {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc2b98316_26b8_4bfb_8138_9f962c22a35b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSchema_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetDefaultPrintTicketAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetDefaultPrintTicketAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetCapabilitiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, constrainticket: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetCapabilitiesAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub MergeAndValidateWithDefaultPrintTicketAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deltaticket: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    MergeAndValidateWithDefaultPrintTicketAsync: usize,
}
#[doc = "*Required features: `\"Devices_Printers\"`*"]
#[repr(transparent)]
pub struct IppAttributeError(::windows_core::IUnknown);
impl IppAttributeError {
    pub fn Reason(&self) -> ::windows_core::Result<IppAttributeErrorReason> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Reason)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetUnsupportedValues(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<IppAttributeValue>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetUnsupportedValues)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for IppAttributeError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IppAttributeError {}
impl ::core::fmt::Debug for IppAttributeError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IppAttributeError").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IppAttributeError {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.IppAttributeError;{750feda1-9eef-5c39-93e4-46149bbcef27})");
}
impl ::core::clone::Clone for IppAttributeError {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for IppAttributeError {
    type Vtable = IIppAttributeError_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IppAttributeError {
    const IID: ::windows_core::GUID = <IIppAttributeError as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for IppAttributeError {
    const NAME: &'static str = "Windows.Devices.Printers.IppAttributeError";
}
::windows_core::imp::interface_hierarchy!(IppAttributeError, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for IppAttributeError {}
unsafe impl ::core::marker::Sync for IppAttributeError {}
#[doc = "*Required features: `\"Devices_Printers\"`*"]
#[repr(transparent)]
pub struct IppAttributeValue(::windows_core::IUnknown);
impl IppAttributeValue {
    pub fn Kind(&self) -> ::windows_core::Result<IppAttributeValueKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetIntegerArray(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetIntegerArray)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetBooleanArray(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetBooleanArray)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetEnumArray(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetEnumArray)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn GetOctetStringArray(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetOctetStringArray)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetDateTimeArray(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDateTimeArray)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetResolutionArray(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<IppResolution>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetResolutionArray)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetRangeOfIntegerArray(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<IppIntegerRange>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetRangeOfIntegerArray)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCollectionArray(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, IppAttributeValue>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetCollectionArray)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetTextWithLanguageArray(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<IppTextWithLanguage>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetTextWithLanguageArray)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetNameWithLanguageArray(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<IppTextWithLanguage>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetNameWithLanguageArray)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetTextWithoutLanguageArray(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetTextWithoutLanguageArray)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetNameWithoutLanguageArray(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetNameWithoutLanguageArray)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetKeywordArray(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetKeywordArray)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetUriArray(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetUriArray)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetUriSchemaArray(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetUriSchemaArray)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCharsetArray(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetCharsetArray)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetNaturalLanguageArray(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetNaturalLanguageArray)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMimeMediaTypeArray(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMimeMediaTypeArray)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateUnsupported() -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateUnsupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateUnknown() -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateUnknown)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateNoValue() -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateNoValue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateInteger(value: i32) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInteger)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateIntegerArray<P0>(values: P0) -> ::windows_core::Result<IppAttributeValue>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<i32>>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateIntegerArray)(::windows_core::Interface::as_raw(this), values.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateBoolean(value: bool) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateBoolean)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateBooleanArray<P0>(values: P0) -> ::windows_core::Result<IppAttributeValue>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<bool>>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateBooleanArray)(::windows_core::Interface::as_raw(this), values.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateEnum(value: i32) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateEnum)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateEnumArray<P0>(values: P0) -> ::windows_core::Result<IppAttributeValue>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<i32>>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateEnumArray)(::windows_core::Interface::as_raw(this), values.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateOctetString<P0>(value: P0) -> ::windows_core::Result<IppAttributeValue>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateOctetString)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn CreateOctetStringArray<P0>(values: P0) -> ::windows_core::Result<IppAttributeValue>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Storage::Streams::IBuffer>>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateOctetStringArray)(::windows_core::Interface::as_raw(this), values.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateDateTime(value: super::super::Foundation::DateTime) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateDateTime)(::windows_core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateDateTimeArray<P0>(values: P0) -> ::windows_core::Result<IppAttributeValue>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::DateTime>>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateDateTimeArray)(::windows_core::Interface::as_raw(this), values.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateResolution<P0>(value: P0) -> ::windows_core::Result<IppAttributeValue>
    where
        P0: ::windows_core::IntoParam<IppResolution>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateResolution)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateResolutionArray<P0>(values: P0) -> ::windows_core::Result<IppAttributeValue>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<IppResolution>>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateResolutionArray)(::windows_core::Interface::as_raw(this), values.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateRangeOfInteger<P0>(value: P0) -> ::windows_core::Result<IppAttributeValue>
    where
        P0: ::windows_core::IntoParam<IppIntegerRange>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateRangeOfInteger)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateRangeOfIntegerArray<P0>(values: P0) -> ::windows_core::Result<IppAttributeValue>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<IppIntegerRange>>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateRangeOfIntegerArray)(::windows_core::Interface::as_raw(this), values.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateCollection<P0>(memberattributes: P0) -> ::windows_core::Result<IppAttributeValue>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, IppAttributeValue>>>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateCollection)(::windows_core::Interface::as_raw(this), memberattributes.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateCollectionArray<P0>(memberattributesarray: P0) -> ::windows_core::Result<IppAttributeValue>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, IppAttributeValue>>>>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateCollectionArray)(::windows_core::Interface::as_raw(this), memberattributesarray.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateTextWithLanguage<P0>(value: P0) -> ::windows_core::Result<IppAttributeValue>
    where
        P0: ::windows_core::IntoParam<IppTextWithLanguage>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateTextWithLanguage)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateTextWithLanguageArray<P0>(values: P0) -> ::windows_core::Result<IppAttributeValue>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<IppTextWithLanguage>>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateTextWithLanguageArray)(::windows_core::Interface::as_raw(this), values.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateNameWithLanguage<P0>(value: P0) -> ::windows_core::Result<IppAttributeValue>
    where
        P0: ::windows_core::IntoParam<IppTextWithLanguage>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateNameWithLanguage)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateNameWithLanguageArray<P0>(values: P0) -> ::windows_core::Result<IppAttributeValue>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<IppTextWithLanguage>>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateNameWithLanguageArray)(::windows_core::Interface::as_raw(this), values.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateTextWithoutLanguage(value: &::windows_core::HSTRING) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateTextWithoutLanguage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateTextWithoutLanguageArray<P0>(values: P0) -> ::windows_core::Result<IppAttributeValue>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateTextWithoutLanguageArray)(::windows_core::Interface::as_raw(this), values.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateNameWithoutLanguage(value: &::windows_core::HSTRING) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateNameWithoutLanguage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateNameWithoutLanguageArray<P0>(values: P0) -> ::windows_core::Result<IppAttributeValue>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateNameWithoutLanguageArray)(::windows_core::Interface::as_raw(this), values.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateKeyword(value: &::windows_core::HSTRING) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateKeyword)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateKeywordArray<P0>(values: P0) -> ::windows_core::Result<IppAttributeValue>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateKeywordArray)(::windows_core::Interface::as_raw(this), values.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateUri<P0>(value: P0) -> ::windows_core::Result<IppAttributeValue>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateUri)(::windows_core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateUriArray<P0>(values: P0) -> ::windows_core::Result<IppAttributeValue>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateUriArray)(::windows_core::Interface::as_raw(this), values.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateUriSchema(value: &::windows_core::HSTRING) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateUriSchema)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateUriSchemaArray<P0>(values: P0) -> ::windows_core::Result<IppAttributeValue>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateUriSchemaArray)(::windows_core::Interface::as_raw(this), values.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateCharset(value: &::windows_core::HSTRING) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateCharset)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateCharsetArray<P0>(values: P0) -> ::windows_core::Result<IppAttributeValue>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateCharsetArray)(::windows_core::Interface::as_raw(this), values.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateNaturalLanguage(value: &::windows_core::HSTRING) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateNaturalLanguage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateNaturalLanguageArray<P0>(values: P0) -> ::windows_core::Result<IppAttributeValue>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateNaturalLanguageArray)(::windows_core::Interface::as_raw(this), values.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateMimeMedia(value: &::windows_core::HSTRING) -> ::windows_core::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateMimeMedia)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateMimeMediaArray<P0>(values: P0) -> ::windows_core::Result<IppAttributeValue>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateMimeMediaArray)(::windows_core::Interface::as_raw(this), values.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IIppAttributeValueStatics<R, F: FnOnce(&IIppAttributeValueStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<IppAttributeValue, IIppAttributeValueStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for IppAttributeValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IppAttributeValue {}
impl ::core::fmt::Debug for IppAttributeValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IppAttributeValue").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IppAttributeValue {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.IppAttributeValue;{99407fed-e2bb-59a3-988b-28a974052a26})");
}
impl ::core::clone::Clone for IppAttributeValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for IppAttributeValue {
    type Vtable = IIppAttributeValue_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IppAttributeValue {
    const IID: ::windows_core::GUID = <IIppAttributeValue as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for IppAttributeValue {
    const NAME: &'static str = "Windows.Devices.Printers.IppAttributeValue";
}
::windows_core::imp::interface_hierarchy!(IppAttributeValue, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for IppAttributeValue {}
unsafe impl ::core::marker::Sync for IppAttributeValue {}
#[doc = "*Required features: `\"Devices_Printers\"`*"]
#[repr(transparent)]
pub struct IppIntegerRange(::windows_core::IUnknown);
impl IppIntegerRange {
    pub fn Start(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn End(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).End)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateInstance(start: i32, end: i32) -> ::windows_core::Result<IppIntegerRange> {
        Self::IIppIntegerRangeFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), start, end, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IIppIntegerRangeFactory<R, F: FnOnce(&IIppIntegerRangeFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<IppIntegerRange, IIppIntegerRangeFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for IppIntegerRange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IppIntegerRange {}
impl ::core::fmt::Debug for IppIntegerRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IppIntegerRange").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IppIntegerRange {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.IppIntegerRange;{92907346-c3ea-5ed6-bdb1-3752c62c6f7f})");
}
impl ::core::clone::Clone for IppIntegerRange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for IppIntegerRange {
    type Vtable = IIppIntegerRange_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IppIntegerRange {
    const IID: ::windows_core::GUID = <IIppIntegerRange as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for IppIntegerRange {
    const NAME: &'static str = "Windows.Devices.Printers.IppIntegerRange";
}
::windows_core::imp::interface_hierarchy!(IppIntegerRange, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for IppIntegerRange {}
unsafe impl ::core::marker::Sync for IppIntegerRange {}
#[doc = "*Required features: `\"Devices_Printers\"`*"]
#[repr(transparent)]
pub struct IppPrintDevice(::windows_core::IUnknown);
impl IppPrintDevice {
    pub fn PrinterName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PrinterName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PrinterUri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PrinterUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn GetPrinterAttributesAsBuffer<P0>(&self, attributenames: P0) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPrinterAttributesAsBuffer)(::windows_core::Interface::as_raw(this), attributenames.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetPrinterAttributes<P0>(&self, attributenames: P0) -> ::windows_core::Result<super::super::Foundation::Collections::IMap<::windows_core::HSTRING, IppAttributeValue>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPrinterAttributes)(::windows_core::Interface::as_raw(this), attributenames.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetPrinterAttributesFromBuffer<P0>(&self, printerattributesbuffer: P0) -> ::windows_core::Result<IppSetAttributesResult>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetPrinterAttributesFromBuffer)(::windows_core::Interface::as_raw(this), printerattributesbuffer.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetPrinterAttributes<P0>(&self, printerattributes: P0) -> ::windows_core::Result<IppSetAttributesResult>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, IppAttributeValue>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetPrinterAttributes)(::windows_core::Interface::as_raw(this), printerattributes.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetMaxSupportedPdfSize(&self) -> ::windows_core::Result<u64> {
        let this = &::windows_core::ComInterface::cast::<IIppPrintDevice2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMaxSupportedPdfSize)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetMaxSupportedPdfVersion(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IIppPrintDevice2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMaxSupportedPdfVersion)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsPdlPassthroughSupported(&self, pdlcontenttype: &::windows_core::HSTRING) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IIppPrintDevice2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsPdlPassthroughSupported)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(pdlcontenttype), &mut result__).from_abi(result__)
        }
    }
    pub fn GetPdlPassthroughProvider(&self) -> ::windows_core::Result<PdlPassthroughProvider> {
        let this = &::windows_core::ComInterface::cast::<IIppPrintDevice2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPdlPassthroughProvider)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDeviceSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IIppPrintDeviceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn FromId(deviceid: &::windows_core::HSTRING) -> ::windows_core::Result<IppPrintDevice> {
        Self::IIppPrintDeviceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FromId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), &mut result__).from_abi(result__)
        })
    }
    pub fn FromPrinterName(printername: &::windows_core::HSTRING) -> ::windows_core::Result<IppPrintDevice> {
        Self::IIppPrintDeviceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FromPrinterName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(printername), &mut result__).from_abi(result__)
        })
    }
    pub fn IsIppPrinter(printername: &::windows_core::HSTRING) -> ::windows_core::Result<bool> {
        Self::IIppPrintDeviceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsIppPrinter)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(printername), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IIppPrintDeviceStatics<R, F: FnOnce(&IIppPrintDeviceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<IppPrintDevice, IIppPrintDeviceStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for IppPrintDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IppPrintDevice {}
impl ::core::fmt::Debug for IppPrintDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IppPrintDevice").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IppPrintDevice {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.IppPrintDevice;{d748ac56-76f3-5dc6-afd4-c2a8686b9359})");
}
impl ::core::clone::Clone for IppPrintDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for IppPrintDevice {
    type Vtable = IIppPrintDevice_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IppPrintDevice {
    const IID: ::windows_core::GUID = <IIppPrintDevice as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for IppPrintDevice {
    const NAME: &'static str = "Windows.Devices.Printers.IppPrintDevice";
}
::windows_core::imp::interface_hierarchy!(IppPrintDevice, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for IppPrintDevice {}
unsafe impl ::core::marker::Sync for IppPrintDevice {}
#[doc = "*Required features: `\"Devices_Printers\"`*"]
#[repr(transparent)]
pub struct IppResolution(::windows_core::IUnknown);
impl IppResolution {
    pub fn Width(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Width)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Height(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Height)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Unit(&self) -> ::windows_core::Result<IppResolutionUnit> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Unit)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateInstance(width: i32, height: i32, unit: IppResolutionUnit) -> ::windows_core::Result<IppResolution> {
        Self::IIppResolutionFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), width, height, unit, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IIppResolutionFactory<R, F: FnOnce(&IIppResolutionFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<IppResolution, IIppResolutionFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for IppResolution {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IppResolution {}
impl ::core::fmt::Debug for IppResolution {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IppResolution").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IppResolution {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.IppResolution;{cb493f86-6bf3-56f5-86ce-263d08aead63})");
}
impl ::core::clone::Clone for IppResolution {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for IppResolution {
    type Vtable = IIppResolution_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IppResolution {
    const IID: ::windows_core::GUID = <IIppResolution as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for IppResolution {
    const NAME: &'static str = "Windows.Devices.Printers.IppResolution";
}
::windows_core::imp::interface_hierarchy!(IppResolution, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for IppResolution {}
unsafe impl ::core::marker::Sync for IppResolution {}
#[doc = "*Required features: `\"Devices_Printers\"`*"]
#[repr(transparent)]
pub struct IppSetAttributesResult(::windows_core::IUnknown);
impl IppSetAttributesResult {
    pub fn Succeeded(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Succeeded)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AttributeErrors(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, IppAttributeError>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AttributeErrors)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for IppSetAttributesResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IppSetAttributesResult {}
impl ::core::fmt::Debug for IppSetAttributesResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IppSetAttributesResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IppSetAttributesResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.IppSetAttributesResult;{7d1c7f55-aa9d-58a3-90e9-17bdc5281f07})");
}
impl ::core::clone::Clone for IppSetAttributesResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for IppSetAttributesResult {
    type Vtable = IIppSetAttributesResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IppSetAttributesResult {
    const IID: ::windows_core::GUID = <IIppSetAttributesResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for IppSetAttributesResult {
    const NAME: &'static str = "Windows.Devices.Printers.IppSetAttributesResult";
}
::windows_core::imp::interface_hierarchy!(IppSetAttributesResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for IppSetAttributesResult {}
unsafe impl ::core::marker::Sync for IppSetAttributesResult {}
#[doc = "*Required features: `\"Devices_Printers\"`*"]
#[repr(transparent)]
pub struct IppTextWithLanguage(::windows_core::IUnknown);
impl IppTextWithLanguage {
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateInstance(language: &::windows_core::HSTRING, text: &::windows_core::HSTRING) -> ::windows_core::Result<IppTextWithLanguage> {
        Self::IIppTextWithLanguageFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(language), ::core::mem::transmute_copy(text), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IIppTextWithLanguageFactory<R, F: FnOnce(&IIppTextWithLanguageFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<IppTextWithLanguage, IIppTextWithLanguageFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for IppTextWithLanguage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IppTextWithLanguage {}
impl ::core::fmt::Debug for IppTextWithLanguage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IppTextWithLanguage").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IppTextWithLanguage {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.IppTextWithLanguage;{326447a6-5149-5936-90e8-0c736036bf77})");
}
impl ::core::clone::Clone for IppTextWithLanguage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for IppTextWithLanguage {
    type Vtable = IIppTextWithLanguage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IppTextWithLanguage {
    const IID: ::windows_core::GUID = <IIppTextWithLanguage as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for IppTextWithLanguage {
    const NAME: &'static str = "Windows.Devices.Printers.IppTextWithLanguage";
}
::windows_core::imp::interface_hierarchy!(IppTextWithLanguage, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for IppTextWithLanguage {}
unsafe impl ::core::marker::Sync for IppTextWithLanguage {}
#[doc = "*Required features: `\"Devices_Printers\"`*"]
#[repr(transparent)]
pub struct PageConfigurationSettings(::windows_core::IUnknown);
impl PageConfigurationSettings {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PageConfigurationSettings, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn OrientationSource(&self) -> ::windows_core::Result<PageConfigurationSource> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OrientationSource)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOrientationSource(&self, value: PageConfigurationSource) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOrientationSource)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SizeSource(&self) -> ::windows_core::Result<PageConfigurationSource> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SizeSource)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSizeSource(&self, value: PageConfigurationSource) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSizeSource)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for PageConfigurationSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PageConfigurationSettings {}
impl ::core::fmt::Debug for PageConfigurationSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PageConfigurationSettings").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PageConfigurationSettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.PageConfigurationSettings;{b6fc1e02-5331-54ff-95a0-1fcb76bb97a9})");
}
impl ::core::clone::Clone for PageConfigurationSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PageConfigurationSettings {
    type Vtable = IPageConfigurationSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PageConfigurationSettings {
    const IID: ::windows_core::GUID = <IPageConfigurationSettings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PageConfigurationSettings {
    const NAME: &'static str = "Windows.Devices.Printers.PageConfigurationSettings";
}
::windows_core::imp::interface_hierarchy!(PageConfigurationSettings, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PageConfigurationSettings {}
unsafe impl ::core::marker::Sync for PageConfigurationSettings {}
#[doc = "*Required features: `\"Devices_Printers\"`*"]
#[repr(transparent)]
pub struct PdlPassthroughProvider(::windows_core::IUnknown);
impl PdlPassthroughProvider {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedPdlContentTypes(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedPdlContentTypes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing\"`*"]
    #[cfg(feature = "Graphics_Printing")]
    pub fn StartPrintJobWithTaskOptions<P0, P1>(&self, jobname: &::windows_core::HSTRING, pdlcontenttype: &::windows_core::HSTRING, taskoptions: P0, pageconfigurationsettings: P1) -> ::windows_core::Result<PdlPassthroughTarget>
    where
        P0: ::windows_core::IntoParam<super::super::Graphics::Printing::PrintTaskOptions>,
        P1: ::windows_core::IntoParam<PageConfigurationSettings>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartPrintJobWithTaskOptions)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(jobname), ::core::mem::transmute_copy(pdlcontenttype), taskoptions.into_param().abi(), pageconfigurationsettings.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn StartPrintJobWithPrintTicket<P0, P1>(&self, jobname: &::windows_core::HSTRING, pdlcontenttype: &::windows_core::HSTRING, printticket: P0, pageconfigurationsettings: P1) -> ::windows_core::Result<PdlPassthroughTarget>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IInputStream>,
        P1: ::windows_core::IntoParam<PageConfigurationSettings>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartPrintJobWithPrintTicket)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(jobname), ::core::mem::transmute_copy(pdlcontenttype), printticket.try_into_param()?.abi(), pageconfigurationsettings.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PdlPassthroughProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PdlPassthroughProvider {}
impl ::core::fmt::Debug for PdlPassthroughProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PdlPassthroughProvider").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PdlPassthroughProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.PdlPassthroughProvider;{23c71dd2-6117-553f-9378-180af5849a49})");
}
impl ::core::clone::Clone for PdlPassthroughProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PdlPassthroughProvider {
    type Vtable = IPdlPassthroughProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PdlPassthroughProvider {
    const IID: ::windows_core::GUID = <IPdlPassthroughProvider as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PdlPassthroughProvider {
    const NAME: &'static str = "Windows.Devices.Printers.PdlPassthroughProvider";
}
::windows_core::imp::interface_hierarchy!(PdlPassthroughProvider, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PdlPassthroughProvider {}
unsafe impl ::core::marker::Sync for PdlPassthroughProvider {}
#[doc = "*Required features: `\"Devices_Printers\"`*"]
#[repr(transparent)]
pub struct PdlPassthroughTarget(::windows_core::IUnknown);
impl PdlPassthroughTarget {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn PrintJobId(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PrintJobId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetOutputStream(&self) -> ::windows_core::Result<super::super::Storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetOutputStream)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Submit(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Submit)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::cmp::PartialEq for PdlPassthroughTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PdlPassthroughTarget {}
impl ::core::fmt::Debug for PdlPassthroughTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PdlPassthroughTarget").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PdlPassthroughTarget {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.PdlPassthroughTarget;{9840be79-67f8-5385-a5b9-e8c96e0fca76})");
}
impl ::core::clone::Clone for PdlPassthroughTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PdlPassthroughTarget {
    type Vtable = IPdlPassthroughTarget_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PdlPassthroughTarget {
    const IID: ::windows_core::GUID = <IPdlPassthroughTarget as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PdlPassthroughTarget {
    const NAME: &'static str = "Windows.Devices.Printers.PdlPassthroughTarget";
}
::windows_core::imp::interface_hierarchy!(PdlPassthroughTarget, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for PdlPassthroughTarget {}
unsafe impl ::core::marker::Send for PdlPassthroughTarget {}
unsafe impl ::core::marker::Sync for PdlPassthroughTarget {}
#[doc = "*Required features: `\"Devices_Printers\"`*"]
#[repr(transparent)]
pub struct Print3DDevice(::windows_core::IUnknown);
impl Print3DDevice {
    pub fn PrintSchema(&self) -> ::windows_core::Result<PrintSchema> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PrintSchema)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(deviceid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<Print3DDevice>> {
        Self::IPrint3DDeviceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), &mut result__).from_abi(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IPrint3DDeviceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPrint3DDeviceStatics<R, F: FnOnce(&IPrint3DDeviceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<Print3DDevice, IPrint3DDeviceStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for Print3DDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Print3DDevice {}
impl ::core::fmt::Debug for Print3DDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Print3DDevice").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for Print3DDevice {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.Print3DDevice;{041c3d19-9713-42a2-9813-7dc3337428d3})");
}
impl ::core::clone::Clone for Print3DDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for Print3DDevice {
    type Vtable = IPrint3DDevice_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Print3DDevice {
    const IID: ::windows_core::GUID = <IPrint3DDevice as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Print3DDevice {
    const NAME: &'static str = "Windows.Devices.Printers.Print3DDevice";
}
::windows_core::imp::interface_hierarchy!(Print3DDevice, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for Print3DDevice {}
unsafe impl ::core::marker::Sync for Print3DDevice {}
#[doc = "*Required features: `\"Devices_Printers\"`*"]
#[repr(transparent)]
pub struct PrintSchema(::windows_core::IUnknown);
impl PrintSchema {
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetDefaultPrintTicketAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStreamWithContentType>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDefaultPrintTicketAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetCapabilitiesAsync<P0>(&self, constrainticket: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStreamWithContentType>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStreamWithContentType>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetCapabilitiesAsync)(::windows_core::Interface::as_raw(this), constrainticket.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn MergeAndValidateWithDefaultPrintTicketAsync<P0>(&self, deltaticket: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStreamWithContentType>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStreamWithContentType>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MergeAndValidateWithDefaultPrintTicketAsync)(::windows_core::Interface::as_raw(this), deltaticket.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintSchema {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintSchema {}
impl ::core::fmt::Debug for PrintSchema {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintSchema").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PrintSchema {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Printers.PrintSchema;{c2b98316-26b8-4bfb-8138-9f962c22a35b})");
}
impl ::core::clone::Clone for PrintSchema {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PrintSchema {
    type Vtable = IPrintSchema_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintSchema {
    const IID: ::windows_core::GUID = <IPrintSchema as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintSchema {
    const NAME: &'static str = "Windows.Devices.Printers.PrintSchema";
}
::windows_core::imp::interface_hierarchy!(PrintSchema, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PrintSchema {}
unsafe impl ::core::marker::Sync for PrintSchema {}
#[doc = "*Required features: `\"Devices_Printers\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IppAttributeErrorReason(pub i32);
impl IppAttributeErrorReason {
    pub const RequestEntityTooLarge: Self = Self(0i32);
    pub const AttributeNotSupported: Self = Self(1i32);
    pub const AttributeValuesNotSupported: Self = Self(2i32);
    pub const AttributeNotSettable: Self = Self(3i32);
    pub const ConflictingAttributes: Self = Self(4i32);
}
impl ::core::marker::Copy for IppAttributeErrorReason {}
impl ::core::clone::Clone for IppAttributeErrorReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IppAttributeErrorReason {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for IppAttributeErrorReason {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IppAttributeErrorReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IppAttributeErrorReason").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IppAttributeErrorReason {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Printers.IppAttributeErrorReason;i4)");
}
#[doc = "*Required features: `\"Devices_Printers\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::marker::Copy for IppAttributeValueKind {}
impl ::core::clone::Clone for IppAttributeValueKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IppAttributeValueKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for IppAttributeValueKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IppAttributeValueKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IppAttributeValueKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IppAttributeValueKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Printers.IppAttributeValueKind;i4)");
}
#[doc = "*Required features: `\"Devices_Printers\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IppResolutionUnit(pub i32);
impl IppResolutionUnit {
    pub const DotsPerInch: Self = Self(0i32);
    pub const DotsPerCentimeter: Self = Self(1i32);
}
impl ::core::marker::Copy for IppResolutionUnit {}
impl ::core::clone::Clone for IppResolutionUnit {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IppResolutionUnit {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for IppResolutionUnit {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IppResolutionUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IppResolutionUnit").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IppResolutionUnit {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Printers.IppResolutionUnit;i4)");
}
#[doc = "*Required features: `\"Devices_Printers\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PageConfigurationSource(pub i32);
impl PageConfigurationSource {
    pub const PrintJobConfiguration: Self = Self(0i32);
    pub const PdlContent: Self = Self(1i32);
}
impl ::core::marker::Copy for PageConfigurationSource {}
impl ::core::clone::Clone for PageConfigurationSource {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PageConfigurationSource {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PageConfigurationSource {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PageConfigurationSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PageConfigurationSource").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PageConfigurationSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Printers.PageConfigurationSource;i4)");
}
