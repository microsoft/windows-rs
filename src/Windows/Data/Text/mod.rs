#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Data_Text`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AlternateNormalizationFormat(pub i32);
impl AlternateNormalizationFormat {
    pub const NotNormalized: AlternateNormalizationFormat = AlternateNormalizationFormat(0i32);
    pub const Number: AlternateNormalizationFormat = AlternateNormalizationFormat(1i32);
    pub const Currency: AlternateNormalizationFormat = AlternateNormalizationFormat(3i32);
    pub const Date: AlternateNormalizationFormat = AlternateNormalizationFormat(4i32);
    pub const Time: AlternateNormalizationFormat = AlternateNormalizationFormat(5i32);
}
impl ::std::convert::From<i32> for AlternateNormalizationFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AlternateNormalizationFormat {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AlternateNormalizationFormat {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Data.Text.AlternateNormalizationFormat;i4)");
}
#[doc = "*Required features: `Data_Text`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AlternateWordForm(::windows::runtime::IInspectable);
impl AlternateWordForm {
    #[doc = "*Required features: `Data_Text`*"]
    pub fn SourceTextSegment(&self) -> ::windows::runtime::Result<TextSegment> {
        let this = self;
        unsafe {
            let mut result__: TextSegment = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TextSegment>(result__)
        }
    }
    #[doc = "*Required features: `Data_Text`*"]
    pub fn AlternateText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Text`*"]
    pub fn NormalizationFormat(&self) -> ::windows::runtime::Result<AlternateNormalizationFormat> {
        let this = self;
        unsafe {
            let mut result__: AlternateNormalizationFormat = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AlternateNormalizationFormat>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AlternateWordForm {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Data.Text.AlternateWordForm;{47396c1e-51b9-4207-9146-248e636a1d1d})");
}
unsafe impl ::windows::runtime::Interface for AlternateWordForm {
    type Vtable = IAlternateWordForm_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1194945566, 20921, 16903, [145, 70, 36, 142, 99, 106, 29, 29]);
}
impl ::windows::runtime::RuntimeName for AlternateWordForm {
    const NAME: &'static str = "Windows.Data.Text.AlternateWordForm";
}
impl ::std::convert::From<AlternateWordForm> for ::windows::runtime::IUnknown {
    fn from(value: AlternateWordForm) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AlternateWordForm> for ::windows::runtime::IUnknown {
    fn from(value: &AlternateWordForm) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AlternateWordForm {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AlternateWordForm {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<AlternateWordForm> for ::windows::runtime::IInspectable {
    fn from(value: AlternateWordForm) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AlternateWordForm> for ::windows::runtime::IInspectable {
    fn from(value: &AlternateWordForm) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AlternateWordForm {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AlternateWordForm {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AlternateWordForm {}
unsafe impl ::std::marker::Sync for AlternateWordForm {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IAlternateWordForm(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAlternateWordForm {
    type Vtable = IAlternateWordForm_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1194945566, 20921, 16903, [145, 70, 36, 142, 99, 106, 29, 29]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAlternateWordForm_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut TextSegment) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AlternateNormalizationFormat) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISelectableWordSegment(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISelectableWordSegment {
    type Vtable = ISelectableWordSegment_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2439662775, 35495, 19576, [179, 116, 93, 237, 183, 82, 230, 11]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectableWordSegment_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut TextSegment) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISelectableWordsSegmenter(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISelectableWordsSegmenter {
    type Vtable = ISelectableWordsSegmenter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4141625831, 19219, 17861, [136, 151, 125, 113, 38, 158, 8, 93]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectableWordsSegmenter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, text: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, startindex: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, text: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, text: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, startindex: u32, handler: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISelectableWordsSegmenterFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISelectableWordsSegmenterFactory {
    type Vtable = ISelectableWordsSegmenterFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2356835912, 24663, 17209, [188, 112, 242, 16, 1, 10, 65, 80]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectableWordsSegmenterFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, language: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISemanticTextQuery(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISemanticTextQuery {
    type Vtable = ISemanticTextQuery_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1780263761, 8114, 18697, [128, 184, 53, 115, 26, 43, 62, 127]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISemanticTextQuery_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, content: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propertycontent: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, propertyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISemanticTextQueryFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISemanticTextQueryFactory {
    type Vtable = ISemanticTextQueryFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(596378883, 63893, 17799, [135, 119, 162, 183, 216, 10, 207, 239]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISemanticTextQueryFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, aqsfilter: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, aqsfilter: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, filterlanguage: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ITextConversionGenerator(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextConversionGenerator {
    type Vtable = ITextConversionGenerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(56650334, 10921, 19126, [175, 139, 165, 98, 182, 58, 137, 146]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextConversionGenerator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, maxcandidates: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ITextConversionGeneratorFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextConversionGeneratorFactory {
    type Vtable = ITextConversionGeneratorFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4239013761, 12419, 18859, [190, 21, 86, 223, 187, 183, 77, 111]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextConversionGeneratorFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, languagetag: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ITextPhoneme(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextPhoneme {
    type Vtable = ITextPhoneme_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2472715274, 39802, 17769, [148, 207, 216, 79, 47, 56, 207, 155]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextPhoneme_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ITextPredictionGenerator(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextPredictionGenerator {
    type Vtable = ITextPredictionGenerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1588374279, 44017, 19638, [157, 158, 50, 111, 43, 70, 135, 86]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextPredictionGenerator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, maxcandidates: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ITextPredictionGenerator2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextPredictionGenerator2 {
    type Vtable = ITextPredictionGenerator2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3091669944, 11383, 18538, [144, 10, 163, 69, 62, 237, 193, 93]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextPredictionGenerator2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, maxcandidates: u32, predictionoptions: TextPredictionOptions, previousstrings: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, maxcandidates: u32, previousstrings: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "UI_Text_Core")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::UI::Text::Core::CoreTextInputScope) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Text_Core"))] usize,
    #[cfg(feature = "UI_Text_Core")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::UI::Text::Core::CoreTextInputScope) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Text_Core"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ITextPredictionGeneratorFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextPredictionGeneratorFactory {
    type Vtable = ITextPredictionGeneratorFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1918350358, 35746, 18257, [157, 48, 157, 133, 67, 86, 83, 162]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextPredictionGeneratorFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, languagetag: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ITextReverseConversionGenerator(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextReverseConversionGenerator {
    type Vtable = ITextReverseConversionGenerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1374156052, 40017, 19846, [174, 27, 180, 152, 251, 173, 131, 19]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextReverseConversionGenerator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ITextReverseConversionGenerator2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextReverseConversionGenerator2 {
    type Vtable = ITextReverseConversionGenerator2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(447730412, 34262, 18173, [130, 138, 58, 72, 48, 250, 110, 24]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextReverseConversionGenerator2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ITextReverseConversionGeneratorFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextReverseConversionGeneratorFactory {
    type Vtable = ITextReverseConversionGeneratorFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1673450278, 8154, 16886, [137, 213, 35, 221, 234, 60, 114, 154]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextReverseConversionGeneratorFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, languagetag: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IUnicodeCharactersStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUnicodeCharactersStatics {
    type Vtable = IUnicodeCharactersStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2542837383, 37521, 20369, [182, 200, 182, 227, 89, 215, 167, 251]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnicodeCharactersStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, highsurrogate: u32, lowsurrogate: u32, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, codepoint: u32, highsurrogate: *mut u16, lowsurrogate: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, codepoint: u32, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, codepoint: u32, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, codepoint: u32, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, codepoint: u32, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, codepoint: u32, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, codepoint: u32, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, codepoint: u32, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, codepoint: u32, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, codepoint: u32, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, codepoint: u32, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, codepoint: u32, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, codepoint: u32, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, codepoint: u32, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, codepoint: u32, result__: *mut UnicodeNumericType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, codepoint: u32, result__: *mut UnicodeGeneralCategory) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IWordSegment(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWordSegment {
    type Vtable = IWordSegment_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3537156717, 39036, 19648, [182, 189, 212, 154, 17, 179, 143, 154]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWordSegment_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut TextSegment) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IWordsSegmenter(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWordsSegmenter {
    type Vtable = IWordsSegmenter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2259997905, 45822, 20020, [168, 29, 102, 100, 3, 0, 69, 79]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWordsSegmenter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, text: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, startindex: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, text: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, text: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, startindex: u32, handler: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IWordsSegmenterFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWordsSegmenterFactory {
    type Vtable = IWordsSegmenterFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3868684916, 64565, 17756, [139, 251, 109, 127, 70, 83, 202, 151]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWordsSegmenterFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, language: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Data_Text`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SelectableWordSegment(::windows::runtime::IInspectable);
impl SelectableWordSegment {
    #[doc = "*Required features: `Data_Text`*"]
    pub fn Text(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Text`*"]
    pub fn SourceTextSegment(&self) -> ::windows::runtime::Result<TextSegment> {
        let this = self;
        unsafe {
            let mut result__: TextSegment = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TextSegment>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SelectableWordSegment {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Data.Text.SelectableWordSegment;{916a4cb7-8aa7-4c78-b374-5dedb752e60b})");
}
unsafe impl ::windows::runtime::Interface for SelectableWordSegment {
    type Vtable = ISelectableWordSegment_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2439662775, 35495, 19576, [179, 116, 93, 237, 183, 82, 230, 11]);
}
impl ::windows::runtime::RuntimeName for SelectableWordSegment {
    const NAME: &'static str = "Windows.Data.Text.SelectableWordSegment";
}
impl ::std::convert::From<SelectableWordSegment> for ::windows::runtime::IUnknown {
    fn from(value: SelectableWordSegment) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SelectableWordSegment> for ::windows::runtime::IUnknown {
    fn from(value: &SelectableWordSegment) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SelectableWordSegment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SelectableWordSegment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SelectableWordSegment> for ::windows::runtime::IInspectable {
    fn from(value: SelectableWordSegment) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SelectableWordSegment> for ::windows::runtime::IInspectable {
    fn from(value: &SelectableWordSegment) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SelectableWordSegment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SelectableWordSegment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SelectableWordSegment {}
unsafe impl ::std::marker::Sync for SelectableWordSegment {}
#[cfg(feature = "Foundation_Collections")]
#[doc = "*Required features: `Data_Text`, `Foundation_Collections`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SelectableWordSegmentsTokenizingHandler(::windows::runtime::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl SelectableWordSegmentsTokenizingHandler {
    pub fn new<F: FnMut(&::std::option::Option<super::super::Foundation::Collections::IIterable<SelectableWordSegment>>, &::std::option::Option<super::super::Foundation::Collections::IIterable<SelectableWordSegment>>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = SelectableWordSegmentsTokenizingHandler_box::<F> {
            vtable: &SelectableWordSegmentsTokenizingHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Data_Text`, `Foundation_Collections`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<SelectableWordSegment>>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<SelectableWordSegment>>>(&self, precedingwords: Param0, words: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::std::mem::transmute_copy(this), precedingwords.into_param().abi(), words.into_param().abi()).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::RuntimeType for SelectableWordSegmentsTokenizingHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({3a3dfc9c-aede-4dc7-9e6c-41c044bd3592})");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::Interface for SelectableWordSegmentsTokenizingHandler {
    type Vtable = SelectableWordSegmentsTokenizingHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(977140892, 44766, 19911, [158, 108, 65, 192, 68, 189, 53, 146]);
}
#[cfg(feature = "Foundation_Collections")]
#[repr(C)]
#[doc(hidden)]
pub struct SelectableWordSegmentsTokenizingHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, precedingwords: ::windows::runtime::RawPtr, words: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[cfg(feature = "Foundation_Collections")]
#[repr(C)]
struct SelectableWordSegmentsTokenizingHandler_box<F: FnMut(&::std::option::Option<super::super::Foundation::Collections::IIterable<SelectableWordSegment>>, &::std::option::Option<super::super::Foundation::Collections::IIterable<SelectableWordSegment>>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const SelectableWordSegmentsTokenizingHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
#[cfg(feature = "Foundation_Collections")]
impl<F: FnMut(&::std::option::Option<super::super::Foundation::Collections::IIterable<SelectableWordSegment>>, &::std::option::Option<super::super::Foundation::Collections::IIterable<SelectableWordSegment>>) -> ::windows::runtime::Result<()> + 'static> SelectableWordSegmentsTokenizingHandler_box<F> {
    const VTABLE: SelectableWordSegmentsTokenizingHandler_abi = SelectableWordSegmentsTokenizingHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<SelectableWordSegmentsTokenizingHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, precedingwords: ::windows::runtime::RawPtr, words: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&precedingwords as *const <super::super::Foundation::Collections::IIterable<SelectableWordSegment> as ::windows::runtime::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<SelectableWordSegment> as ::windows::runtime::Abi>::DefaultType),
            &*(&words as *const <super::super::Foundation::Collections::IIterable<SelectableWordSegment> as ::windows::runtime::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<SelectableWordSegment> as ::windows::runtime::Abi>::DefaultType),
        )
        .into()
    }
}
#[doc = "*Required features: `Data_Text`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SelectableWordsSegmenter(::windows::runtime::IInspectable);
impl SelectableWordsSegmenter {
    #[doc = "*Required features: `Data_Text`*"]
    pub fn ResolvedLanguage(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Text`*"]
    pub fn GetTokenAt<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, text: Param0, startindex: u32) -> ::windows::runtime::Result<SelectableWordSegment> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), text.into_param().abi(), startindex, &mut result__).from_abi::<SelectableWordSegment>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Data_Text`, `Foundation_Collections`*"]
    pub fn GetTokens<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, text: Param0) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<SelectableWordSegment>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<SelectableWordSegment>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Data_Text`, `Foundation_Collections`*"]
    pub fn Tokenize<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, SelectableWordSegmentsTokenizingHandler>>(&self, text: Param0, startindex: u32, handler: Param2) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), text.into_param().abi(), startindex, handler.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Text`*"]
    pub fn CreateWithLanguage<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(language: Param0) -> ::windows::runtime::Result<SelectableWordsSegmenter> {
        Self::ISelectableWordsSegmenterFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), language.into_param().abi(), &mut result__).from_abi::<SelectableWordsSegmenter>(result__)
        })
    }
    pub fn ISelectableWordsSegmenterFactory<R, F: FnOnce(&ISelectableWordsSegmenterFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SelectableWordsSegmenter, ISelectableWordsSegmenterFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SelectableWordsSegmenter {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Data.Text.SelectableWordsSegmenter;{f6dc31e7-4b13-45c5-8897-7d71269e085d})");
}
unsafe impl ::windows::runtime::Interface for SelectableWordsSegmenter {
    type Vtable = ISelectableWordsSegmenter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4141625831, 19219, 17861, [136, 151, 125, 113, 38, 158, 8, 93]);
}
impl ::windows::runtime::RuntimeName for SelectableWordsSegmenter {
    const NAME: &'static str = "Windows.Data.Text.SelectableWordsSegmenter";
}
impl ::std::convert::From<SelectableWordsSegmenter> for ::windows::runtime::IUnknown {
    fn from(value: SelectableWordsSegmenter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SelectableWordsSegmenter> for ::windows::runtime::IUnknown {
    fn from(value: &SelectableWordsSegmenter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SelectableWordsSegmenter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SelectableWordsSegmenter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SelectableWordsSegmenter> for ::windows::runtime::IInspectable {
    fn from(value: SelectableWordsSegmenter) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SelectableWordsSegmenter> for ::windows::runtime::IInspectable {
    fn from(value: &SelectableWordsSegmenter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SelectableWordsSegmenter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SelectableWordsSegmenter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SelectableWordsSegmenter {}
unsafe impl ::std::marker::Sync for SelectableWordsSegmenter {}
#[doc = "*Required features: `Data_Text`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SemanticTextQuery(::windows::runtime::IInspectable);
impl SemanticTextQuery {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Data_Text`, `Foundation_Collections`*"]
    pub fn Find<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, content: Param0) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<TextSegment>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), content.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<TextSegment>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Data_Text`, `Foundation_Collections`*"]
    pub fn FindInProperty<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, propertycontent: Param0, propertyname: Param1) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<TextSegment>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), propertycontent.into_param().abi(), propertyname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<TextSegment>>(result__)
        }
    }
    #[doc = "*Required features: `Data_Text`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(aqsfilter: Param0) -> ::windows::runtime::Result<SemanticTextQuery> {
        Self::ISemanticTextQueryFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), aqsfilter.into_param().abi(), &mut result__).from_abi::<SemanticTextQuery>(result__)
        })
    }
    #[doc = "*Required features: `Data_Text`*"]
    pub fn CreateWithLanguage<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(aqsfilter: Param0, filterlanguage: Param1) -> ::windows::runtime::Result<SemanticTextQuery> {
        Self::ISemanticTextQueryFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), aqsfilter.into_param().abi(), filterlanguage.into_param().abi(), &mut result__).from_abi::<SemanticTextQuery>(result__)
        })
    }
    pub fn ISemanticTextQueryFactory<R, F: FnOnce(&ISemanticTextQueryFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SemanticTextQuery, ISemanticTextQueryFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SemanticTextQuery {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Data.Text.SemanticTextQuery;{6a1cab51-1fb2-4909-80b8-35731a2b3e7f})");
}
unsafe impl ::windows::runtime::Interface for SemanticTextQuery {
    type Vtable = ISemanticTextQuery_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1780263761, 8114, 18697, [128, 184, 53, 115, 26, 43, 62, 127]);
}
impl ::windows::runtime::RuntimeName for SemanticTextQuery {
    const NAME: &'static str = "Windows.Data.Text.SemanticTextQuery";
}
impl ::std::convert::From<SemanticTextQuery> for ::windows::runtime::IUnknown {
    fn from(value: SemanticTextQuery) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SemanticTextQuery> for ::windows::runtime::IUnknown {
    fn from(value: &SemanticTextQuery) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SemanticTextQuery {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SemanticTextQuery {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SemanticTextQuery> for ::windows::runtime::IInspectable {
    fn from(value: SemanticTextQuery) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SemanticTextQuery> for ::windows::runtime::IInspectable {
    fn from(value: &SemanticTextQuery) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SemanticTextQuery {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SemanticTextQuery {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SemanticTextQuery {}
unsafe impl ::std::marker::Sync for SemanticTextQuery {}
#[doc = "*Required features: `Data_Text`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct TextConversionGenerator(::windows::runtime::IInspectable);
impl TextConversionGenerator {
    #[doc = "*Required features: `Data_Text`*"]
    pub fn ResolvedLanguage(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Text`*"]
    pub fn LanguageAvailableButNotInstalled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Data_Text`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetCandidatesAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, input: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Data_Text`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetCandidatesWithMaxCountAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, input: Param0, maxcandidates: u32) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), input.into_param().abi(), maxcandidates, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>>(result__)
        }
    }
    #[doc = "*Required features: `Data_Text`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(languagetag: Param0) -> ::windows::runtime::Result<TextConversionGenerator> {
        Self::ITextConversionGeneratorFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), languagetag.into_param().abi(), &mut result__).from_abi::<TextConversionGenerator>(result__)
        })
    }
    pub fn ITextConversionGeneratorFactory<R, F: FnOnce(&ITextConversionGeneratorFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<TextConversionGenerator, ITextConversionGeneratorFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TextConversionGenerator {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Data.Text.TextConversionGenerator;{03606a5e-2aa9-4ab6-af8b-a562b63a8992})");
}
unsafe impl ::windows::runtime::Interface for TextConversionGenerator {
    type Vtable = ITextConversionGenerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(56650334, 10921, 19126, [175, 139, 165, 98, 182, 58, 137, 146]);
}
impl ::windows::runtime::RuntimeName for TextConversionGenerator {
    const NAME: &'static str = "Windows.Data.Text.TextConversionGenerator";
}
impl ::std::convert::From<TextConversionGenerator> for ::windows::runtime::IUnknown {
    fn from(value: TextConversionGenerator) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&TextConversionGenerator> for ::windows::runtime::IUnknown {
    fn from(value: &TextConversionGenerator) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for TextConversionGenerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &TextConversionGenerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<TextConversionGenerator> for ::windows::runtime::IInspectable {
    fn from(value: TextConversionGenerator) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TextConversionGenerator> for ::windows::runtime::IInspectable {
    fn from(value: &TextConversionGenerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for TextConversionGenerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a TextConversionGenerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for TextConversionGenerator {}
unsafe impl ::std::marker::Sync for TextConversionGenerator {}
#[doc = "*Required features: `Data_Text`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct TextPhoneme(::windows::runtime::IInspectable);
impl TextPhoneme {
    #[doc = "*Required features: `Data_Text`*"]
    pub fn DisplayText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Text`*"]
    pub fn ReadingText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TextPhoneme {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Data.Text.TextPhoneme;{9362a40a-9b7a-4569-94cf-d84f2f38cf9b})");
}
unsafe impl ::windows::runtime::Interface for TextPhoneme {
    type Vtable = ITextPhoneme_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2472715274, 39802, 17769, [148, 207, 216, 79, 47, 56, 207, 155]);
}
impl ::windows::runtime::RuntimeName for TextPhoneme {
    const NAME: &'static str = "Windows.Data.Text.TextPhoneme";
}
impl ::std::convert::From<TextPhoneme> for ::windows::runtime::IUnknown {
    fn from(value: TextPhoneme) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&TextPhoneme> for ::windows::runtime::IUnknown {
    fn from(value: &TextPhoneme) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for TextPhoneme {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &TextPhoneme {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<TextPhoneme> for ::windows::runtime::IInspectable {
    fn from(value: TextPhoneme) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TextPhoneme> for ::windows::runtime::IInspectable {
    fn from(value: &TextPhoneme) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for TextPhoneme {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a TextPhoneme {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for TextPhoneme {}
unsafe impl ::std::marker::Sync for TextPhoneme {}
#[doc = "*Required features: `Data_Text`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct TextPredictionGenerator(::windows::runtime::IInspectable);
impl TextPredictionGenerator {
    #[doc = "*Required features: `Data_Text`*"]
    pub fn ResolvedLanguage(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Text`*"]
    pub fn LanguageAvailableButNotInstalled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Data_Text`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetCandidatesAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, input: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Data_Text`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetCandidatesWithMaxCountAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, input: Param0, maxcandidates: u32) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), input.into_param().abi(), maxcandidates, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>>(result__)
        }
    }
    #[doc = "*Required features: `Data_Text`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(languagetag: Param0) -> ::windows::runtime::Result<TextPredictionGenerator> {
        Self::ITextPredictionGeneratorFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), languagetag.into_param().abi(), &mut result__).from_abi::<TextPredictionGenerator>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Data_Text`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetCandidatesWithParametersAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(
        &self,
        input: Param0,
        maxcandidates: u32,
        predictionoptions: TextPredictionOptions,
        previousstrings: Param3,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>> {
        let this = &::windows::runtime::Interface::cast::<ITextPredictionGenerator2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), input.into_param().abi(), maxcandidates, predictionoptions, previousstrings.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Data_Text`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetNextWordCandidatesAsync<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(&self, maxcandidates: u32, previousstrings: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>> {
        let this = &::windows::runtime::Interface::cast::<ITextPredictionGenerator2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), maxcandidates, previousstrings.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>>(result__)
        }
    }
    #[cfg(feature = "UI_Text_Core")]
    #[doc = "*Required features: `Data_Text`, `UI_Text_Core`*"]
    pub fn InputScope(&self) -> ::windows::runtime::Result<super::super::UI::Text::Core::CoreTextInputScope> {
        let this = &::windows::runtime::Interface::cast::<ITextPredictionGenerator2>(self)?;
        unsafe {
            let mut result__: super::super::UI::Text::Core::CoreTextInputScope = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Text::Core::CoreTextInputScope>(result__)
        }
    }
    #[cfg(feature = "UI_Text_Core")]
    #[doc = "*Required features: `Data_Text`, `UI_Text_Core`*"]
    pub fn SetInputScope(&self, value: super::super::UI::Text::Core::CoreTextInputScope) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextPredictionGenerator2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn ITextPredictionGeneratorFactory<R, F: FnOnce(&ITextPredictionGeneratorFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<TextPredictionGenerator, ITextPredictionGeneratorFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TextPredictionGenerator {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Data.Text.TextPredictionGenerator;{5eacab07-abf1-4cb6-9d9e-326f2b468756})");
}
unsafe impl ::windows::runtime::Interface for TextPredictionGenerator {
    type Vtable = ITextPredictionGenerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1588374279, 44017, 19638, [157, 158, 50, 111, 43, 70, 135, 86]);
}
impl ::windows::runtime::RuntimeName for TextPredictionGenerator {
    const NAME: &'static str = "Windows.Data.Text.TextPredictionGenerator";
}
impl ::std::convert::From<TextPredictionGenerator> for ::windows::runtime::IUnknown {
    fn from(value: TextPredictionGenerator) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&TextPredictionGenerator> for ::windows::runtime::IUnknown {
    fn from(value: &TextPredictionGenerator) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for TextPredictionGenerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &TextPredictionGenerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<TextPredictionGenerator> for ::windows::runtime::IInspectable {
    fn from(value: TextPredictionGenerator) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TextPredictionGenerator> for ::windows::runtime::IInspectable {
    fn from(value: &TextPredictionGenerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for TextPredictionGenerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a TextPredictionGenerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for TextPredictionGenerator {}
unsafe impl ::std::marker::Sync for TextPredictionGenerator {}
#[doc = "*Required features: `Data_Text`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TextPredictionOptions(pub u32);
impl TextPredictionOptions {
    pub const None: TextPredictionOptions = TextPredictionOptions(0u32);
    pub const Predictions: TextPredictionOptions = TextPredictionOptions(1u32);
    pub const Corrections: TextPredictionOptions = TextPredictionOptions(2u32);
}
impl ::std::convert::From<u32> for TextPredictionOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TextPredictionOptions {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for TextPredictionOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Data.Text.TextPredictionOptions;u4)");
}
impl ::std::ops::BitOr for TextPredictionOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for TextPredictionOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for TextPredictionOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for TextPredictionOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for TextPredictionOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Data_Text`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct TextReverseConversionGenerator(::windows::runtime::IInspectable);
impl TextReverseConversionGenerator {
    #[doc = "*Required features: `Data_Text`*"]
    pub fn ResolvedLanguage(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Text`*"]
    pub fn LanguageAvailableButNotInstalled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Data_Text`, `Foundation`*"]
    pub fn ConvertBackAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, input: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `Data_Text`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(languagetag: Param0) -> ::windows::runtime::Result<TextReverseConversionGenerator> {
        Self::ITextReverseConversionGeneratorFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), languagetag.into_param().abi(), &mut result__).from_abi::<TextReverseConversionGenerator>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Data_Text`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetPhonemesAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, input: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<TextPhoneme>>> {
        let this = &::windows::runtime::Interface::cast::<ITextReverseConversionGenerator2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<TextPhoneme>>>(result__)
        }
    }
    pub fn ITextReverseConversionGeneratorFactory<R, F: FnOnce(&ITextReverseConversionGeneratorFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<TextReverseConversionGenerator, ITextReverseConversionGeneratorFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TextReverseConversionGenerator {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Data.Text.TextReverseConversionGenerator;{51e7f514-9c51-4d86-ae1b-b498fbad8313})");
}
unsafe impl ::windows::runtime::Interface for TextReverseConversionGenerator {
    type Vtable = ITextReverseConversionGenerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1374156052, 40017, 19846, [174, 27, 180, 152, 251, 173, 131, 19]);
}
impl ::windows::runtime::RuntimeName for TextReverseConversionGenerator {
    const NAME: &'static str = "Windows.Data.Text.TextReverseConversionGenerator";
}
impl ::std::convert::From<TextReverseConversionGenerator> for ::windows::runtime::IUnknown {
    fn from(value: TextReverseConversionGenerator) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&TextReverseConversionGenerator> for ::windows::runtime::IUnknown {
    fn from(value: &TextReverseConversionGenerator) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for TextReverseConversionGenerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &TextReverseConversionGenerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<TextReverseConversionGenerator> for ::windows::runtime::IInspectable {
    fn from(value: TextReverseConversionGenerator) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TextReverseConversionGenerator> for ::windows::runtime::IInspectable {
    fn from(value: &TextReverseConversionGenerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for TextReverseConversionGenerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a TextReverseConversionGenerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for TextReverseConversionGenerator {}
unsafe impl ::std::marker::Sync for TextReverseConversionGenerator {}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Data_Text`*"]
pub struct TextSegment {
    pub StartPosition: u32,
    pub Length: u32,
}
impl TextSegment {}
impl ::std::default::Default for TextSegment {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TextSegment {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TextSegment").field("StartPosition", &self.StartPosition).field("Length", &self.Length).finish()
    }
}
impl ::std::cmp::PartialEq for TextSegment {
    fn eq(&self, other: &Self) -> bool {
        self.StartPosition == other.StartPosition && self.Length == other.Length
    }
}
impl ::std::cmp::Eq for TextSegment {}
unsafe impl ::windows::runtime::Abi for TextSegment {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for TextSegment {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Data.Text.TextSegment;u4;u4)");
}
#[doc = "*Required features: `Data_Text`*"]
pub struct UnicodeCharacters {}
impl UnicodeCharacters {
    #[doc = "*Required features: `Data_Text`*"]
    pub fn GetCodepointFromSurrogatePair(highsurrogate: u32, lowsurrogate: u32) -> ::windows::runtime::Result<u32> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), highsurrogate, lowsurrogate, &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Data_Text`*"]
    pub fn GetSurrogatePairFromCodepoint(codepoint: u32, highsurrogate: &mut u16, lowsurrogate: &mut u16) -> ::windows::runtime::Result<()> {
        Self::IUnicodeCharactersStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), codepoint, highsurrogate, lowsurrogate).ok() })
    }
    #[doc = "*Required features: `Data_Text`*"]
    pub fn IsHighSurrogate(codepoint: u32) -> ::windows::runtime::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), codepoint, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Data_Text`*"]
    pub fn IsLowSurrogate(codepoint: u32) -> ::windows::runtime::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), codepoint, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Data_Text`*"]
    pub fn IsSupplementary(codepoint: u32) -> ::windows::runtime::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), codepoint, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Data_Text`*"]
    pub fn IsNoncharacter(codepoint: u32) -> ::windows::runtime::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), codepoint, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Data_Text`*"]
    pub fn IsWhitespace(codepoint: u32) -> ::windows::runtime::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), codepoint, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Data_Text`*"]
    pub fn IsAlphabetic(codepoint: u32) -> ::windows::runtime::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), codepoint, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Data_Text`*"]
    pub fn IsCased(codepoint: u32) -> ::windows::runtime::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), codepoint, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Data_Text`*"]
    pub fn IsUppercase(codepoint: u32) -> ::windows::runtime::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), codepoint, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Data_Text`*"]
    pub fn IsLowercase(codepoint: u32) -> ::windows::runtime::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), codepoint, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Data_Text`*"]
    pub fn IsIdStart(codepoint: u32) -> ::windows::runtime::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), codepoint, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Data_Text`*"]
    pub fn IsIdContinue(codepoint: u32) -> ::windows::runtime::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), codepoint, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Data_Text`*"]
    pub fn IsGraphemeBase(codepoint: u32) -> ::windows::runtime::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), codepoint, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Data_Text`*"]
    pub fn IsGraphemeExtend(codepoint: u32) -> ::windows::runtime::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), codepoint, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Data_Text`*"]
    pub fn GetNumericType(codepoint: u32) -> ::windows::runtime::Result<UnicodeNumericType> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: UnicodeNumericType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), codepoint, &mut result__).from_abi::<UnicodeNumericType>(result__)
        })
    }
    #[doc = "*Required features: `Data_Text`*"]
    pub fn GetGeneralCategory(codepoint: u32) -> ::windows::runtime::Result<UnicodeGeneralCategory> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: UnicodeGeneralCategory = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), codepoint, &mut result__).from_abi::<UnicodeGeneralCategory>(result__)
        })
    }
    pub fn IUnicodeCharactersStatics<R, F: FnOnce(&IUnicodeCharactersStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<UnicodeCharacters, IUnicodeCharactersStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for UnicodeCharacters {
    const NAME: &'static str = "Windows.Data.Text.UnicodeCharacters";
}
#[doc = "*Required features: `Data_Text`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for UnicodeGeneralCategory {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UnicodeGeneralCategory {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UnicodeGeneralCategory {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Data.Text.UnicodeGeneralCategory;i4)");
}
#[doc = "*Required features: `Data_Text`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UnicodeNumericType(pub i32);
impl UnicodeNumericType {
    pub const None: UnicodeNumericType = UnicodeNumericType(0i32);
    pub const Decimal: UnicodeNumericType = UnicodeNumericType(1i32);
    pub const Digit: UnicodeNumericType = UnicodeNumericType(2i32);
    pub const Numeric: UnicodeNumericType = UnicodeNumericType(3i32);
}
impl ::std::convert::From<i32> for UnicodeNumericType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UnicodeNumericType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UnicodeNumericType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Data.Text.UnicodeNumericType;i4)");
}
#[doc = "*Required features: `Data_Text`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct WordSegment(::windows::runtime::IInspectable);
impl WordSegment {
    #[doc = "*Required features: `Data_Text`*"]
    pub fn Text(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Text`*"]
    pub fn SourceTextSegment(&self) -> ::windows::runtime::Result<TextSegment> {
        let this = self;
        unsafe {
            let mut result__: TextSegment = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TextSegment>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Data_Text`, `Foundation_Collections`*"]
    pub fn AlternateForms(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<AlternateWordForm>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<AlternateWordForm>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WordSegment {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Data.Text.WordSegment;{d2d4ba6d-987c-4cc0-b6bd-d49a11b38f9a})");
}
unsafe impl ::windows::runtime::Interface for WordSegment {
    type Vtable = IWordSegment_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3537156717, 39036, 19648, [182, 189, 212, 154, 17, 179, 143, 154]);
}
impl ::windows::runtime::RuntimeName for WordSegment {
    const NAME: &'static str = "Windows.Data.Text.WordSegment";
}
impl ::std::convert::From<WordSegment> for ::windows::runtime::IUnknown {
    fn from(value: WordSegment) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&WordSegment> for ::windows::runtime::IUnknown {
    fn from(value: &WordSegment) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for WordSegment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &WordSegment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<WordSegment> for ::windows::runtime::IInspectable {
    fn from(value: WordSegment) -> Self {
        value.0
    }
}
impl ::std::convert::From<&WordSegment> for ::windows::runtime::IInspectable {
    fn from(value: &WordSegment) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for WordSegment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a WordSegment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for WordSegment {}
unsafe impl ::std::marker::Sync for WordSegment {}
#[cfg(feature = "Foundation_Collections")]
#[doc = "*Required features: `Data_Text`, `Foundation_Collections`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct WordSegmentsTokenizingHandler(::windows::runtime::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl WordSegmentsTokenizingHandler {
    pub fn new<F: FnMut(&::std::option::Option<super::super::Foundation::Collections::IIterable<WordSegment>>, &::std::option::Option<super::super::Foundation::Collections::IIterable<WordSegment>>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = WordSegmentsTokenizingHandler_box::<F> {
            vtable: &WordSegmentsTokenizingHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Data_Text`, `Foundation_Collections`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<WordSegment>>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<WordSegment>>>(&self, precedingwords: Param0, words: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::std::mem::transmute_copy(this), precedingwords.into_param().abi(), words.into_param().abi()).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::RuntimeType for WordSegmentsTokenizingHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({a5dd6357-bf2a-4c4f-a31f-29e71c6f8b35})");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::Interface for WordSegmentsTokenizingHandler {
    type Vtable = WordSegmentsTokenizingHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2782749527, 48938, 19535, [163, 31, 41, 231, 28, 111, 139, 53]);
}
#[cfg(feature = "Foundation_Collections")]
#[repr(C)]
#[doc(hidden)]
pub struct WordSegmentsTokenizingHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, precedingwords: ::windows::runtime::RawPtr, words: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[cfg(feature = "Foundation_Collections")]
#[repr(C)]
struct WordSegmentsTokenizingHandler_box<F: FnMut(&::std::option::Option<super::super::Foundation::Collections::IIterable<WordSegment>>, &::std::option::Option<super::super::Foundation::Collections::IIterable<WordSegment>>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const WordSegmentsTokenizingHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
#[cfg(feature = "Foundation_Collections")]
impl<F: FnMut(&::std::option::Option<super::super::Foundation::Collections::IIterable<WordSegment>>, &::std::option::Option<super::super::Foundation::Collections::IIterable<WordSegment>>) -> ::windows::runtime::Result<()> + 'static> WordSegmentsTokenizingHandler_box<F> {
    const VTABLE: WordSegmentsTokenizingHandler_abi = WordSegmentsTokenizingHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<WordSegmentsTokenizingHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, precedingwords: ::windows::runtime::RawPtr, words: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&precedingwords as *const <super::super::Foundation::Collections::IIterable<WordSegment> as ::windows::runtime::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<WordSegment> as ::windows::runtime::Abi>::DefaultType),
            &*(&words as *const <super::super::Foundation::Collections::IIterable<WordSegment> as ::windows::runtime::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<WordSegment> as ::windows::runtime::Abi>::DefaultType),
        )
        .into()
    }
}
#[doc = "*Required features: `Data_Text`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct WordsSegmenter(::windows::runtime::IInspectable);
impl WordsSegmenter {
    #[doc = "*Required features: `Data_Text`*"]
    pub fn ResolvedLanguage(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Data_Text`*"]
    pub fn GetTokenAt<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, text: Param0, startindex: u32) -> ::windows::runtime::Result<WordSegment> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), text.into_param().abi(), startindex, &mut result__).from_abi::<WordSegment>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Data_Text`, `Foundation_Collections`*"]
    pub fn GetTokens<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, text: Param0) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<WordSegment>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<WordSegment>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Data_Text`, `Foundation_Collections`*"]
    pub fn Tokenize<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, WordSegmentsTokenizingHandler>>(&self, text: Param0, startindex: u32, handler: Param2) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), text.into_param().abi(), startindex, handler.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Data_Text`*"]
    pub fn CreateWithLanguage<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(language: Param0) -> ::windows::runtime::Result<WordsSegmenter> {
        Self::IWordsSegmenterFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), language.into_param().abi(), &mut result__).from_abi::<WordsSegmenter>(result__)
        })
    }
    pub fn IWordsSegmenterFactory<R, F: FnOnce(&IWordsSegmenterFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<WordsSegmenter, IWordsSegmenterFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WordsSegmenter {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Data.Text.WordsSegmenter;{86b4d4d1-b2fe-4e34-a81d-66640300454f})");
}
unsafe impl ::windows::runtime::Interface for WordsSegmenter {
    type Vtable = IWordsSegmenter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2259997905, 45822, 20020, [168, 29, 102, 100, 3, 0, 69, 79]);
}
impl ::windows::runtime::RuntimeName for WordsSegmenter {
    const NAME: &'static str = "Windows.Data.Text.WordsSegmenter";
}
impl ::std::convert::From<WordsSegmenter> for ::windows::runtime::IUnknown {
    fn from(value: WordsSegmenter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&WordsSegmenter> for ::windows::runtime::IUnknown {
    fn from(value: &WordsSegmenter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for WordsSegmenter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &WordsSegmenter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<WordsSegmenter> for ::windows::runtime::IInspectable {
    fn from(value: WordsSegmenter) -> Self {
        value.0
    }
}
impl ::std::convert::From<&WordsSegmenter> for ::windows::runtime::IInspectable {
    fn from(value: &WordsSegmenter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for WordsSegmenter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a WordsSegmenter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for WordsSegmenter {}
unsafe impl ::std::marker::Sync for WordsSegmenter {}
