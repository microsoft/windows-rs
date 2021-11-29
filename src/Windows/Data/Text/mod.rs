#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AlternateNormalizationFormat(pub i32);
impl AlternateNormalizationFormat {
    pub const NotNormalized: AlternateNormalizationFormat = AlternateNormalizationFormat(0i32);
    pub const Number: AlternateNormalizationFormat = AlternateNormalizationFormat(1i32);
    pub const Currency: AlternateNormalizationFormat = AlternateNormalizationFormat(3i32);
    pub const Date: AlternateNormalizationFormat = AlternateNormalizationFormat(4i32);
    pub const Time: AlternateNormalizationFormat = AlternateNormalizationFormat(5i32);
}
impl ::core::convert::From<i32> for AlternateNormalizationFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AlternateNormalizationFormat {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AlternateNormalizationFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Data.Text.AlternateNormalizationFormat;i4)");
}
impl ::windows::core::DefaultType for AlternateNormalizationFormat {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AlternateWordForm(pub ::windows::core::IInspectable);
impl AlternateWordForm {
    pub fn SourceTextSegment(&self) -> ::windows::core::Result<TextSegment> {
        let this = self;
        unsafe {
            let mut result__: TextSegment = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TextSegment>(result__)
        }
    }
    pub fn AlternateText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn NormalizationFormat(&self) -> ::windows::core::Result<AlternateNormalizationFormat> {
        let this = self;
        unsafe {
            let mut result__: AlternateNormalizationFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AlternateNormalizationFormat>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AlternateWordForm {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Text.AlternateWordForm;{47396c1e-51b9-4207-9146-248e636a1d1d})");
}
unsafe impl ::windows::core::Interface for AlternateWordForm {
    type Vtable = IAlternateWordForm_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47396c1e_51b9_4207_9146_248e636a1d1d);
}
impl ::windows::core::RuntimeName for AlternateWordForm {
    const NAME: &'static str = "Windows.Data.Text.AlternateWordForm";
}
impl ::core::convert::From<AlternateWordForm> for ::windows::core::IUnknown {
    fn from(value: AlternateWordForm) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AlternateWordForm> for ::windows::core::IUnknown {
    fn from(value: &AlternateWordForm) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AlternateWordForm {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AlternateWordForm {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AlternateWordForm> for ::windows::core::IInspectable {
    fn from(value: AlternateWordForm) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AlternateWordForm> for ::windows::core::IInspectable {
    fn from(value: &AlternateWordForm) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AlternateWordForm {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AlternateWordForm {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AlternateWordForm {}
unsafe impl ::core::marker::Sync for AlternateWordForm {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IAlternateWordForm(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAlternateWordForm {
    type Vtable = IAlternateWordForm_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47396c1e_51b9_4207_9146_248e636a1d1d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAlternateWordForm_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut TextSegment) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AlternateNormalizationFormat) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISelectableWordSegment(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISelectableWordSegment {
    type Vtable = ISelectableWordSegment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x916a4cb7_8aa7_4c78_b374_5dedb752e60b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectableWordSegment_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut TextSegment) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISelectableWordsSegmenter(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISelectableWordsSegmenter {
    type Vtable = ISelectableWordsSegmenter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6dc31e7_4b13_45c5_8897_7d71269e085d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectableWordsSegmenter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, startindex: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, startindex: u32, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISelectableWordsSegmenterFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISelectableWordsSegmenterFactory {
    type Vtable = ISelectableWordsSegmenterFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c7a7648_6057_4339_bc70_f210010a4150);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectableWordsSegmenterFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, language: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISemanticTextQuery(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISemanticTextQuery {
    type Vtable = ISemanticTextQuery_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a1cab51_1fb2_4909_80b8_35731a2b3e7f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISemanticTextQuery_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, content: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propertycontent: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISemanticTextQueryFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISemanticTextQueryFactory {
    type Vtable = ISemanticTextQueryFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x238c0503_f995_4587_8777_a2b7d80acfef);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISemanticTextQueryFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, aqsfilter: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, aqsfilter: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, filterlanguage: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITextConversionGenerator(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITextConversionGenerator {
    type Vtable = ITextConversionGenerator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03606a5e_2aa9_4ab6_af8b_a562b63a8992);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextConversionGenerator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, maxcandidates: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITextConversionGeneratorFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITextConversionGeneratorFactory {
    type Vtable = ITextConversionGeneratorFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcaa3781_3083_49ab_be15_56dfbbb74d6f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextConversionGeneratorFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, languagetag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITextPhoneme(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITextPhoneme {
    type Vtable = ITextPhoneme_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9362a40a_9b7a_4569_94cf_d84f2f38cf9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextPhoneme_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITextPredictionGenerator(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITextPredictionGenerator {
    type Vtable = ITextPredictionGenerator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5eacab07_abf1_4cb6_9d9e_326f2b468756);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextPredictionGenerator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, maxcandidates: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITextPredictionGenerator2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITextPredictionGenerator2 {
    type Vtable = ITextPredictionGenerator2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb84723b8_2c77_486a_900a_a3453eedc15d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextPredictionGenerator2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, maxcandidates: u32, predictionoptions: TextPredictionOptions, previousstrings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, maxcandidates: u32, previousstrings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "UI_Text_Core")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::UI::Text::Core::CoreTextInputScope) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text_Core"))] usize,
    #[cfg(feature = "UI_Text_Core")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::UI::Text::Core::CoreTextInputScope) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text_Core"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITextPredictionGeneratorFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITextPredictionGeneratorFactory {
    type Vtable = ITextPredictionGeneratorFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7257b416_8ba2_4751_9d30_9d85435653a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextPredictionGeneratorFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, languagetag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITextReverseConversionGenerator(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITextReverseConversionGenerator {
    type Vtable = ITextReverseConversionGenerator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51e7f514_9c51_4d86_ae1b_b498fbad8313);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextReverseConversionGenerator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITextReverseConversionGenerator2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITextReverseConversionGenerator2 {
    type Vtable = ITextReverseConversionGenerator2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1aafd2ec_85d6_46fd_828a_3a4830fa6e18);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextReverseConversionGenerator2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITextReverseConversionGeneratorFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITextReverseConversionGeneratorFactory {
    type Vtable = ITextReverseConversionGeneratorFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63bed326_1fda_41f6_89d5_23ddea3c729a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextReverseConversionGeneratorFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, languagetag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUnicodeCharactersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUnicodeCharactersStatics {
    type Vtable = IUnicodeCharactersStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97909e87_9291_4f91_b6c8_b6e359d7a7fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnicodeCharactersStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, highsurrogate: u32, lowsurrogate: u32, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, codepoint: u32, highsurrogate: *mut u16, lowsurrogate: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, codepoint: u32, result__: *mut UnicodeNumericType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, codepoint: u32, result__: *mut UnicodeGeneralCategory) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWordSegment(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWordSegment {
    type Vtable = IWordSegment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2d4ba6d_987c_4cc0_b6bd_d49a11b38f9a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWordSegment_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut TextSegment) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWordsSegmenter(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWordsSegmenter {
    type Vtable = IWordsSegmenter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86b4d4d1_b2fe_4e34_a81d_66640300454f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWordsSegmenter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, startindex: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, startindex: u32, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWordsSegmenterFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWordsSegmenterFactory {
    type Vtable = IWordsSegmenterFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6977274_fc35_455c_8bfb_6d7f4653ca97);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWordsSegmenterFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, language: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SelectableWordSegment(pub ::windows::core::IInspectable);
impl SelectableWordSegment {
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SourceTextSegment(&self) -> ::windows::core::Result<TextSegment> {
        let this = self;
        unsafe {
            let mut result__: TextSegment = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TextSegment>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SelectableWordSegment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Text.SelectableWordSegment;{916a4cb7-8aa7-4c78-b374-5dedb752e60b})");
}
unsafe impl ::windows::core::Interface for SelectableWordSegment {
    type Vtable = ISelectableWordSegment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x916a4cb7_8aa7_4c78_b374_5dedb752e60b);
}
impl ::windows::core::RuntimeName for SelectableWordSegment {
    const NAME: &'static str = "Windows.Data.Text.SelectableWordSegment";
}
impl ::core::convert::From<SelectableWordSegment> for ::windows::core::IUnknown {
    fn from(value: SelectableWordSegment) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SelectableWordSegment> for ::windows::core::IUnknown {
    fn from(value: &SelectableWordSegment) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SelectableWordSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SelectableWordSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SelectableWordSegment> for ::windows::core::IInspectable {
    fn from(value: SelectableWordSegment) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SelectableWordSegment> for ::windows::core::IInspectable {
    fn from(value: &SelectableWordSegment) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SelectableWordSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SelectableWordSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SelectableWordSegment {}
unsafe impl ::core::marker::Sync for SelectableWordSegment {}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SelectableWordSegmentsTokenizingHandler(::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl SelectableWordSegmentsTokenizingHandler {
    pub fn new<F: FnMut(&::core::option::Option<super::super::Foundation::Collections::IIterable<SelectableWordSegment>>, &::core::option::Option<super::super::Foundation::Collections::IIterable<SelectableWordSegment>>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = SelectableWordSegmentsTokenizingHandler_box::<F> { vtable: &SelectableWordSegmentsTokenizingHandler_box::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<SelectableWordSegment>>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<SelectableWordSegment>>>(&self, precedingwords: Param0, words: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), precedingwords.into_param().abi(), words.into_param().abi()).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for SelectableWordSegmentsTokenizingHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({3a3dfc9c-aede-4dc7-9e6c-41c044bd3592})");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for SelectableWordSegmentsTokenizingHandler {
    type Vtable = SelectableWordSegmentsTokenizingHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a3dfc9c_aede_4dc7_9e6c_41c044bd3592);
}
#[cfg(feature = "Foundation_Collections")]
#[repr(C)]
#[doc(hidden)]
pub struct SelectableWordSegmentsTokenizingHandler_abi(pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr, precedingwords: ::windows::core::RawPtr, words: ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[cfg(feature = "Foundation_Collections")]
#[repr(C)]
struct SelectableWordSegmentsTokenizingHandler_box<F: FnMut(&::core::option::Option<super::super::Foundation::Collections::IIterable<SelectableWordSegment>>, &::core::option::Option<super::super::Foundation::Collections::IIterable<SelectableWordSegment>>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const SelectableWordSegmentsTokenizingHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
#[cfg(feature = "Foundation_Collections")]
impl<F: FnMut(&::core::option::Option<super::super::Foundation::Collections::IIterable<SelectableWordSegment>>, &::core::option::Option<super::super::Foundation::Collections::IIterable<SelectableWordSegment>>) -> ::windows::core::Result<()> + 'static> SelectableWordSegmentsTokenizingHandler_box<F> {
    const VTABLE: SelectableWordSegmentsTokenizingHandler_abi = SelectableWordSegmentsTokenizingHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<SelectableWordSegmentsTokenizingHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, precedingwords: ::windows::core::RawPtr, words: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&precedingwords as *const <super::super::Foundation::Collections::IIterable<SelectableWordSegment> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<SelectableWordSegment> as ::windows::core::DefaultType>::DefaultType),
            &*(&words as *const <super::super::Foundation::Collections::IIterable<SelectableWordSegment> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<SelectableWordSegment> as ::windows::core::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SelectableWordsSegmenter(pub ::windows::core::IInspectable);
impl SelectableWordsSegmenter {
    pub fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn GetTokenAt<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0, startindex: u32) -> ::windows::core::Result<SelectableWordSegment> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), text.into_param().abi(), startindex, &mut result__).from_abi::<SelectableWordSegment>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetTokens<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SelectableWordSegment>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<SelectableWordSegment>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Tokenize<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, SelectableWordSegmentsTokenizingHandler>>(&self, text: Param0, startindex: u32, handler: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), text.into_param().abi(), startindex, handler.into_param().abi()).ok() }
    }
    pub fn CreateWithLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(language: Param0) -> ::windows::core::Result<SelectableWordsSegmenter> {
        Self::ISelectableWordsSegmenterFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), language.into_param().abi(), &mut result__).from_abi::<SelectableWordsSegmenter>(result__)
        })
    }
    pub fn ISelectableWordsSegmenterFactory<R, F: FnOnce(&ISelectableWordsSegmenterFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SelectableWordsSegmenter, ISelectableWordsSegmenterFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for SelectableWordsSegmenter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Text.SelectableWordsSegmenter;{f6dc31e7-4b13-45c5-8897-7d71269e085d})");
}
unsafe impl ::windows::core::Interface for SelectableWordsSegmenter {
    type Vtable = ISelectableWordsSegmenter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6dc31e7_4b13_45c5_8897_7d71269e085d);
}
impl ::windows::core::RuntimeName for SelectableWordsSegmenter {
    const NAME: &'static str = "Windows.Data.Text.SelectableWordsSegmenter";
}
impl ::core::convert::From<SelectableWordsSegmenter> for ::windows::core::IUnknown {
    fn from(value: SelectableWordsSegmenter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SelectableWordsSegmenter> for ::windows::core::IUnknown {
    fn from(value: &SelectableWordsSegmenter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SelectableWordsSegmenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SelectableWordsSegmenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SelectableWordsSegmenter> for ::windows::core::IInspectable {
    fn from(value: SelectableWordsSegmenter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SelectableWordsSegmenter> for ::windows::core::IInspectable {
    fn from(value: &SelectableWordsSegmenter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SelectableWordsSegmenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SelectableWordsSegmenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SelectableWordsSegmenter {}
unsafe impl ::core::marker::Sync for SelectableWordsSegmenter {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SemanticTextQuery(pub ::windows::core::IInspectable);
impl SemanticTextQuery {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Find<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, content: Param0) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TextSegment>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), content.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<TextSegment>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindInProperty<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, propertycontent: Param0, propertyname: Param1) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TextSegment>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), propertycontent.into_param().abi(), propertyname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<TextSegment>>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(aqsfilter: Param0) -> ::windows::core::Result<SemanticTextQuery> {
        Self::ISemanticTextQueryFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), aqsfilter.into_param().abi(), &mut result__).from_abi::<SemanticTextQuery>(result__)
        })
    }
    pub fn CreateWithLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(aqsfilter: Param0, filterlanguage: Param1) -> ::windows::core::Result<SemanticTextQuery> {
        Self::ISemanticTextQueryFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), aqsfilter.into_param().abi(), filterlanguage.into_param().abi(), &mut result__).from_abi::<SemanticTextQuery>(result__)
        })
    }
    pub fn ISemanticTextQueryFactory<R, F: FnOnce(&ISemanticTextQueryFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SemanticTextQuery, ISemanticTextQueryFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for SemanticTextQuery {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Text.SemanticTextQuery;{6a1cab51-1fb2-4909-80b8-35731a2b3e7f})");
}
unsafe impl ::windows::core::Interface for SemanticTextQuery {
    type Vtable = ISemanticTextQuery_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a1cab51_1fb2_4909_80b8_35731a2b3e7f);
}
impl ::windows::core::RuntimeName for SemanticTextQuery {
    const NAME: &'static str = "Windows.Data.Text.SemanticTextQuery";
}
impl ::core::convert::From<SemanticTextQuery> for ::windows::core::IUnknown {
    fn from(value: SemanticTextQuery) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SemanticTextQuery> for ::windows::core::IUnknown {
    fn from(value: &SemanticTextQuery) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SemanticTextQuery {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SemanticTextQuery {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SemanticTextQuery> for ::windows::core::IInspectable {
    fn from(value: SemanticTextQuery) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SemanticTextQuery> for ::windows::core::IInspectable {
    fn from(value: &SemanticTextQuery) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SemanticTextQuery {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SemanticTextQuery {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SemanticTextQuery {}
unsafe impl ::core::marker::Sync for SemanticTextQuery {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TextConversionGenerator(pub ::windows::core::IInspectable);
impl TextConversionGenerator {
    pub fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn LanguageAvailableButNotInstalled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetCandidatesAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, input: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetCandidatesWithMaxCountAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, input: Param0, maxcandidates: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), input.into_param().abi(), maxcandidates, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(languagetag: Param0) -> ::windows::core::Result<TextConversionGenerator> {
        Self::ITextConversionGeneratorFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), languagetag.into_param().abi(), &mut result__).from_abi::<TextConversionGenerator>(result__)
        })
    }
    pub fn ITextConversionGeneratorFactory<R, F: FnOnce(&ITextConversionGeneratorFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TextConversionGenerator, ITextConversionGeneratorFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for TextConversionGenerator {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Text.TextConversionGenerator;{03606a5e-2aa9-4ab6-af8b-a562b63a8992})");
}
unsafe impl ::windows::core::Interface for TextConversionGenerator {
    type Vtable = ITextConversionGenerator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03606a5e_2aa9_4ab6_af8b_a562b63a8992);
}
impl ::windows::core::RuntimeName for TextConversionGenerator {
    const NAME: &'static str = "Windows.Data.Text.TextConversionGenerator";
}
impl ::core::convert::From<TextConversionGenerator> for ::windows::core::IUnknown {
    fn from(value: TextConversionGenerator) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TextConversionGenerator> for ::windows::core::IUnknown {
    fn from(value: &TextConversionGenerator) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TextConversionGenerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TextConversionGenerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TextConversionGenerator> for ::windows::core::IInspectable {
    fn from(value: TextConversionGenerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TextConversionGenerator> for ::windows::core::IInspectable {
    fn from(value: &TextConversionGenerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TextConversionGenerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TextConversionGenerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for TextConversionGenerator {}
unsafe impl ::core::marker::Sync for TextConversionGenerator {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TextPhoneme(pub ::windows::core::IInspectable);
impl TextPhoneme {
    pub fn DisplayText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ReadingText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for TextPhoneme {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Text.TextPhoneme;{9362a40a-9b7a-4569-94cf-d84f2f38cf9b})");
}
unsafe impl ::windows::core::Interface for TextPhoneme {
    type Vtable = ITextPhoneme_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9362a40a_9b7a_4569_94cf_d84f2f38cf9b);
}
impl ::windows::core::RuntimeName for TextPhoneme {
    const NAME: &'static str = "Windows.Data.Text.TextPhoneme";
}
impl ::core::convert::From<TextPhoneme> for ::windows::core::IUnknown {
    fn from(value: TextPhoneme) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TextPhoneme> for ::windows::core::IUnknown {
    fn from(value: &TextPhoneme) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TextPhoneme {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TextPhoneme {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TextPhoneme> for ::windows::core::IInspectable {
    fn from(value: TextPhoneme) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TextPhoneme> for ::windows::core::IInspectable {
    fn from(value: &TextPhoneme) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TextPhoneme {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TextPhoneme {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for TextPhoneme {}
unsafe impl ::core::marker::Sync for TextPhoneme {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TextPredictionGenerator(pub ::windows::core::IInspectable);
impl TextPredictionGenerator {
    pub fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn LanguageAvailableButNotInstalled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetCandidatesAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, input: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetCandidatesWithMaxCountAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, input: Param0, maxcandidates: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), input.into_param().abi(), maxcandidates, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(languagetag: Param0) -> ::windows::core::Result<TextPredictionGenerator> {
        Self::ITextPredictionGeneratorFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), languagetag.into_param().abi(), &mut result__).from_abi::<TextPredictionGenerator>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetCandidatesWithParametersAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, input: Param0, maxcandidates: u32, predictionoptions: TextPredictionOptions, previousstrings: Param3) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>> {
        let this = &::windows::core::Interface::cast::<ITextPredictionGenerator2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), input.into_param().abi(), maxcandidates, predictionoptions, previousstrings.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetNextWordCandidatesAsync<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, maxcandidates: u32, previousstrings: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>> {
        let this = &::windows::core::Interface::cast::<ITextPredictionGenerator2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), maxcandidates, previousstrings.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>(result__)
        }
    }
    #[cfg(feature = "UI_Text_Core")]
    pub fn InputScope(&self) -> ::windows::core::Result<super::super::UI::Text::Core::CoreTextInputScope> {
        let this = &::windows::core::Interface::cast::<ITextPredictionGenerator2>(self)?;
        unsafe {
            let mut result__: super::super::UI::Text::Core::CoreTextInputScope = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Text::Core::CoreTextInputScope>(result__)
        }
    }
    #[cfg(feature = "UI_Text_Core")]
    pub fn SetInputScope(&self, value: super::super::UI::Text::Core::CoreTextInputScope) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextPredictionGenerator2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ITextPredictionGeneratorFactory<R, F: FnOnce(&ITextPredictionGeneratorFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TextPredictionGenerator, ITextPredictionGeneratorFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for TextPredictionGenerator {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Text.TextPredictionGenerator;{5eacab07-abf1-4cb6-9d9e-326f2b468756})");
}
unsafe impl ::windows::core::Interface for TextPredictionGenerator {
    type Vtable = ITextPredictionGenerator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5eacab07_abf1_4cb6_9d9e_326f2b468756);
}
impl ::windows::core::RuntimeName for TextPredictionGenerator {
    const NAME: &'static str = "Windows.Data.Text.TextPredictionGenerator";
}
impl ::core::convert::From<TextPredictionGenerator> for ::windows::core::IUnknown {
    fn from(value: TextPredictionGenerator) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TextPredictionGenerator> for ::windows::core::IUnknown {
    fn from(value: &TextPredictionGenerator) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TextPredictionGenerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TextPredictionGenerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TextPredictionGenerator> for ::windows::core::IInspectable {
    fn from(value: TextPredictionGenerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TextPredictionGenerator> for ::windows::core::IInspectable {
    fn from(value: &TextPredictionGenerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TextPredictionGenerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TextPredictionGenerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for TextPredictionGenerator {}
unsafe impl ::core::marker::Sync for TextPredictionGenerator {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TextPredictionOptions(pub u32);
impl TextPredictionOptions {
    pub const None: TextPredictionOptions = TextPredictionOptions(0u32);
    pub const Predictions: TextPredictionOptions = TextPredictionOptions(1u32);
    pub const Corrections: TextPredictionOptions = TextPredictionOptions(2u32);
}
impl ::core::convert::From<u32> for TextPredictionOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for TextPredictionOptions {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for TextPredictionOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Data.Text.TextPredictionOptions;u4)");
}
impl ::windows::core::DefaultType for TextPredictionOptions {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for TextPredictionOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for TextPredictionOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for TextPredictionOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for TextPredictionOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for TextPredictionOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TextReverseConversionGenerator(pub ::windows::core::IInspectable);
impl TextReverseConversionGenerator {
    pub fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn LanguageAvailableButNotInstalled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ConvertBackAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, input: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(languagetag: Param0) -> ::windows::core::Result<TextReverseConversionGenerator> {
        Self::ITextReverseConversionGeneratorFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), languagetag.into_param().abi(), &mut result__).from_abi::<TextReverseConversionGenerator>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetPhonemesAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, input: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<TextPhoneme>>> {
        let this = &::windows::core::Interface::cast::<ITextReverseConversionGenerator2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<TextPhoneme>>>(result__)
        }
    }
    pub fn ITextReverseConversionGeneratorFactory<R, F: FnOnce(&ITextReverseConversionGeneratorFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TextReverseConversionGenerator, ITextReverseConversionGeneratorFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for TextReverseConversionGenerator {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Text.TextReverseConversionGenerator;{51e7f514-9c51-4d86-ae1b-b498fbad8313})");
}
unsafe impl ::windows::core::Interface for TextReverseConversionGenerator {
    type Vtable = ITextReverseConversionGenerator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51e7f514_9c51_4d86_ae1b_b498fbad8313);
}
impl ::windows::core::RuntimeName for TextReverseConversionGenerator {
    const NAME: &'static str = "Windows.Data.Text.TextReverseConversionGenerator";
}
impl ::core::convert::From<TextReverseConversionGenerator> for ::windows::core::IUnknown {
    fn from(value: TextReverseConversionGenerator) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TextReverseConversionGenerator> for ::windows::core::IUnknown {
    fn from(value: &TextReverseConversionGenerator) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TextReverseConversionGenerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TextReverseConversionGenerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TextReverseConversionGenerator> for ::windows::core::IInspectable {
    fn from(value: TextReverseConversionGenerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TextReverseConversionGenerator> for ::windows::core::IInspectable {
    fn from(value: &TextReverseConversionGenerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TextReverseConversionGenerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TextReverseConversionGenerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for TextReverseConversionGenerator {}
unsafe impl ::core::marker::Sync for TextReverseConversionGenerator {}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct TextSegment {
    pub StartPosition: u32,
    pub Length: u32,
}
impl TextSegment {}
impl ::core::default::Default for TextSegment {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for TextSegment {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TextSegment").field("StartPosition", &self.StartPosition).field("Length", &self.Length).finish()
    }
}
impl ::core::cmp::PartialEq for TextSegment {
    fn eq(&self, other: &Self) -> bool {
        self.StartPosition == other.StartPosition && self.Length == other.Length
    }
}
impl ::core::cmp::Eq for TextSegment {}
unsafe impl ::windows::core::Abi for TextSegment {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for TextSegment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Data.Text.TextSegment;u4;u4)");
}
impl ::windows::core::DefaultType for TextSegment {
    type DefaultType = Self;
}
pub struct UnicodeCharacters {}
impl UnicodeCharacters {
    pub fn GetCodepointFromSurrogatePair(highsurrogate: u32, lowsurrogate: u32) -> ::windows::core::Result<u32> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), highsurrogate, lowsurrogate, &mut result__).from_abi::<u32>(result__)
        })
    }
    pub fn GetSurrogatePairFromCodepoint(codepoint: u32, highsurrogate: &mut u16, lowsurrogate: &mut u16) -> ::windows::core::Result<()> {
        Self::IUnicodeCharactersStatics(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), codepoint, highsurrogate, lowsurrogate).ok() })
    }
    pub fn IsHighSurrogate(codepoint: u32) -> ::windows::core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), codepoint, &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IsLowSurrogate(codepoint: u32) -> ::windows::core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), codepoint, &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IsSupplementary(codepoint: u32) -> ::windows::core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), codepoint, &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IsNoncharacter(codepoint: u32) -> ::windows::core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), codepoint, &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IsWhitespace(codepoint: u32) -> ::windows::core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), codepoint, &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IsAlphabetic(codepoint: u32) -> ::windows::core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), codepoint, &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IsCased(codepoint: u32) -> ::windows::core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), codepoint, &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IsUppercase(codepoint: u32) -> ::windows::core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), codepoint, &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IsLowercase(codepoint: u32) -> ::windows::core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), codepoint, &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IsIdStart(codepoint: u32) -> ::windows::core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), codepoint, &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IsIdContinue(codepoint: u32) -> ::windows::core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), codepoint, &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IsGraphemeBase(codepoint: u32) -> ::windows::core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), codepoint, &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IsGraphemeExtend(codepoint: u32) -> ::windows::core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), codepoint, &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn GetNumericType(codepoint: u32) -> ::windows::core::Result<UnicodeNumericType> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: UnicodeNumericType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), codepoint, &mut result__).from_abi::<UnicodeNumericType>(result__)
        })
    }
    pub fn GetGeneralCategory(codepoint: u32) -> ::windows::core::Result<UnicodeGeneralCategory> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: UnicodeGeneralCategory = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), codepoint, &mut result__).from_abi::<UnicodeGeneralCategory>(result__)
        })
    }
    pub fn IUnicodeCharactersStatics<R, F: FnOnce(&IUnicodeCharactersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UnicodeCharacters, IUnicodeCharactersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for UnicodeCharacters {
    const NAME: &'static str = "Windows.Data.Text.UnicodeCharacters";
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct UnicodeGeneralCategory(pub i32);
impl UnicodeGeneralCategory {
    pub const UppercaseLetter: UnicodeGeneralCategory = UnicodeGeneralCategory(0i32);
    pub const LowercaseLetter: UnicodeGeneralCategory = UnicodeGeneralCategory(1i32);
    pub const TitlecaseLetter: UnicodeGeneralCategory = UnicodeGeneralCategory(2i32);
    pub const ModifierLetter: UnicodeGeneralCategory = UnicodeGeneralCategory(3i32);
    pub const OtherLetter: UnicodeGeneralCategory = UnicodeGeneralCategory(4i32);
    pub const NonspacingMark: UnicodeGeneralCategory = UnicodeGeneralCategory(5i32);
    pub const SpacingCombiningMark: UnicodeGeneralCategory = UnicodeGeneralCategory(6i32);
    pub const EnclosingMark: UnicodeGeneralCategory = UnicodeGeneralCategory(7i32);
    pub const DecimalDigitNumber: UnicodeGeneralCategory = UnicodeGeneralCategory(8i32);
    pub const LetterNumber: UnicodeGeneralCategory = UnicodeGeneralCategory(9i32);
    pub const OtherNumber: UnicodeGeneralCategory = UnicodeGeneralCategory(10i32);
    pub const SpaceSeparator: UnicodeGeneralCategory = UnicodeGeneralCategory(11i32);
    pub const LineSeparator: UnicodeGeneralCategory = UnicodeGeneralCategory(12i32);
    pub const ParagraphSeparator: UnicodeGeneralCategory = UnicodeGeneralCategory(13i32);
    pub const Control: UnicodeGeneralCategory = UnicodeGeneralCategory(14i32);
    pub const Format: UnicodeGeneralCategory = UnicodeGeneralCategory(15i32);
    pub const Surrogate: UnicodeGeneralCategory = UnicodeGeneralCategory(16i32);
    pub const PrivateUse: UnicodeGeneralCategory = UnicodeGeneralCategory(17i32);
    pub const ConnectorPunctuation: UnicodeGeneralCategory = UnicodeGeneralCategory(18i32);
    pub const DashPunctuation: UnicodeGeneralCategory = UnicodeGeneralCategory(19i32);
    pub const OpenPunctuation: UnicodeGeneralCategory = UnicodeGeneralCategory(20i32);
    pub const ClosePunctuation: UnicodeGeneralCategory = UnicodeGeneralCategory(21i32);
    pub const InitialQuotePunctuation: UnicodeGeneralCategory = UnicodeGeneralCategory(22i32);
    pub const FinalQuotePunctuation: UnicodeGeneralCategory = UnicodeGeneralCategory(23i32);
    pub const OtherPunctuation: UnicodeGeneralCategory = UnicodeGeneralCategory(24i32);
    pub const MathSymbol: UnicodeGeneralCategory = UnicodeGeneralCategory(25i32);
    pub const CurrencySymbol: UnicodeGeneralCategory = UnicodeGeneralCategory(26i32);
    pub const ModifierSymbol: UnicodeGeneralCategory = UnicodeGeneralCategory(27i32);
    pub const OtherSymbol: UnicodeGeneralCategory = UnicodeGeneralCategory(28i32);
    pub const NotAssigned: UnicodeGeneralCategory = UnicodeGeneralCategory(29i32);
}
impl ::core::convert::From<i32> for UnicodeGeneralCategory {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for UnicodeGeneralCategory {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for UnicodeGeneralCategory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Data.Text.UnicodeGeneralCategory;i4)");
}
impl ::windows::core::DefaultType for UnicodeGeneralCategory {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct UnicodeNumericType(pub i32);
impl UnicodeNumericType {
    pub const None: UnicodeNumericType = UnicodeNumericType(0i32);
    pub const Decimal: UnicodeNumericType = UnicodeNumericType(1i32);
    pub const Digit: UnicodeNumericType = UnicodeNumericType(2i32);
    pub const Numeric: UnicodeNumericType = UnicodeNumericType(3i32);
}
impl ::core::convert::From<i32> for UnicodeNumericType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for UnicodeNumericType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for UnicodeNumericType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Data.Text.UnicodeNumericType;i4)");
}
impl ::windows::core::DefaultType for UnicodeNumericType {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct WordSegment(pub ::windows::core::IInspectable);
impl WordSegment {
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SourceTextSegment(&self) -> ::windows::core::Result<TextSegment> {
        let this = self;
        unsafe {
            let mut result__: TextSegment = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TextSegment>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AlternateForms(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AlternateWordForm>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<AlternateWordForm>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for WordSegment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Text.WordSegment;{d2d4ba6d-987c-4cc0-b6bd-d49a11b38f9a})");
}
unsafe impl ::windows::core::Interface for WordSegment {
    type Vtable = IWordSegment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2d4ba6d_987c_4cc0_b6bd_d49a11b38f9a);
}
impl ::windows::core::RuntimeName for WordSegment {
    const NAME: &'static str = "Windows.Data.Text.WordSegment";
}
impl ::core::convert::From<WordSegment> for ::windows::core::IUnknown {
    fn from(value: WordSegment) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&WordSegment> for ::windows::core::IUnknown {
    fn from(value: &WordSegment) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WordSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WordSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<WordSegment> for ::windows::core::IInspectable {
    fn from(value: WordSegment) -> Self {
        value.0
    }
}
impl ::core::convert::From<&WordSegment> for ::windows::core::IInspectable {
    fn from(value: &WordSegment) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WordSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WordSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for WordSegment {}
unsafe impl ::core::marker::Sync for WordSegment {}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct WordSegmentsTokenizingHandler(::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl WordSegmentsTokenizingHandler {
    pub fn new<F: FnMut(&::core::option::Option<super::super::Foundation::Collections::IIterable<WordSegment>>, &::core::option::Option<super::super::Foundation::Collections::IIterable<WordSegment>>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = WordSegmentsTokenizingHandler_box::<F> { vtable: &WordSegmentsTokenizingHandler_box::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<WordSegment>>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<WordSegment>>>(&self, precedingwords: Param0, words: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), precedingwords.into_param().abi(), words.into_param().abi()).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for WordSegmentsTokenizingHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({a5dd6357-bf2a-4c4f-a31f-29e71c6f8b35})");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for WordSegmentsTokenizingHandler {
    type Vtable = WordSegmentsTokenizingHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5dd6357_bf2a_4c4f_a31f_29e71c6f8b35);
}
#[cfg(feature = "Foundation_Collections")]
#[repr(C)]
#[doc(hidden)]
pub struct WordSegmentsTokenizingHandler_abi(pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr, precedingwords: ::windows::core::RawPtr, words: ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[cfg(feature = "Foundation_Collections")]
#[repr(C)]
struct WordSegmentsTokenizingHandler_box<F: FnMut(&::core::option::Option<super::super::Foundation::Collections::IIterable<WordSegment>>, &::core::option::Option<super::super::Foundation::Collections::IIterable<WordSegment>>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const WordSegmentsTokenizingHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
#[cfg(feature = "Foundation_Collections")]
impl<F: FnMut(&::core::option::Option<super::super::Foundation::Collections::IIterable<WordSegment>>, &::core::option::Option<super::super::Foundation::Collections::IIterable<WordSegment>>) -> ::windows::core::Result<()> + 'static> WordSegmentsTokenizingHandler_box<F> {
    const VTABLE: WordSegmentsTokenizingHandler_abi = WordSegmentsTokenizingHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<WordSegmentsTokenizingHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, precedingwords: ::windows::core::RawPtr, words: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(&*(&precedingwords as *const <super::super::Foundation::Collections::IIterable<WordSegment> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<WordSegment> as ::windows::core::DefaultType>::DefaultType), &*(&words as *const <super::super::Foundation::Collections::IIterable<WordSegment> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<WordSegment> as ::windows::core::DefaultType>::DefaultType)).into()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct WordsSegmenter(pub ::windows::core::IInspectable);
impl WordsSegmenter {
    pub fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn GetTokenAt<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0, startindex: u32) -> ::windows::core::Result<WordSegment> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), text.into_param().abi(), startindex, &mut result__).from_abi::<WordSegment>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetTokens<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<WordSegment>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<WordSegment>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Tokenize<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, WordSegmentsTokenizingHandler>>(&self, text: Param0, startindex: u32, handler: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), text.into_param().abi(), startindex, handler.into_param().abi()).ok() }
    }
    pub fn CreateWithLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(language: Param0) -> ::windows::core::Result<WordsSegmenter> {
        Self::IWordsSegmenterFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), language.into_param().abi(), &mut result__).from_abi::<WordsSegmenter>(result__)
        })
    }
    pub fn IWordsSegmenterFactory<R, F: FnOnce(&IWordsSegmenterFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WordsSegmenter, IWordsSegmenterFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for WordsSegmenter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Text.WordsSegmenter;{86b4d4d1-b2fe-4e34-a81d-66640300454f})");
}
unsafe impl ::windows::core::Interface for WordsSegmenter {
    type Vtable = IWordsSegmenter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86b4d4d1_b2fe_4e34_a81d_66640300454f);
}
impl ::windows::core::RuntimeName for WordsSegmenter {
    const NAME: &'static str = "Windows.Data.Text.WordsSegmenter";
}
impl ::core::convert::From<WordsSegmenter> for ::windows::core::IUnknown {
    fn from(value: WordsSegmenter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&WordsSegmenter> for ::windows::core::IUnknown {
    fn from(value: &WordsSegmenter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WordsSegmenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WordsSegmenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<WordsSegmenter> for ::windows::core::IInspectable {
    fn from(value: WordsSegmenter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&WordsSegmenter> for ::windows::core::IInspectable {
    fn from(value: &WordsSegmenter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WordsSegmenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WordsSegmenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for WordsSegmenter {}
unsafe impl ::core::marker::Sync for WordsSegmenter {}
