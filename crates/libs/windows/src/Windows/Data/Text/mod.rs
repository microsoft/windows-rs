#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Data_Text'*"]
#[repr(transparent)]
pub struct AlternateNormalizationFormat(pub i32);
impl AlternateNormalizationFormat {
    pub const NotNormalized: Self = Self(0i32);
    pub const Number: Self = Self(1i32);
    pub const Currency: Self = Self(3i32);
    pub const Date: Self = Self(4i32);
    pub const Time: Self = Self(5i32);
}
impl ::core::marker::Copy for AlternateNormalizationFormat {}
impl ::core::clone::Clone for AlternateNormalizationFormat {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AlternateNormalizationFormat {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AlternateNormalizationFormat {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AlternateNormalizationFormat {}
unsafe impl ::windows::core::RuntimeType for AlternateNormalizationFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Data.Text.AlternateNormalizationFormat;i4)");
}
impl ::windows::core::DefaultType for AlternateNormalizationFormat {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Data_Text'*"]
#[repr(transparent)]
pub struct AlternateWordForm(::windows::core::IUnknown);
impl AlternateWordForm {
    #[doc = "*Required features: 'Data_Text'*"]
    pub fn SourceTextSegment(&self) -> ::windows::core::Result<TextSegment> {
        let this = self;
        unsafe {
            let mut result__: TextSegment = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TextSegment>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Text'*"]
    pub fn AlternateText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Text'*"]
    pub fn NormalizationFormat(&self) -> ::windows::core::Result<AlternateNormalizationFormat> {
        let this = self;
        unsafe {
            let mut result__: AlternateNormalizationFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AlternateNormalizationFormat>(result__)
        }
    }
}
impl ::core::clone::Clone for AlternateWordForm {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AlternateWordForm {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AlternateWordForm {}
unsafe impl ::windows::core::RuntimeType for AlternateWordForm {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Text.AlternateWordForm;{47396c1e-51b9-4207-9146-248e636a1d1d})");
}
unsafe impl ::windows::core::Interface for AlternateWordForm {
    type Vtable = IAlternateWordFormVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47396c1e_51b9_4207_9146_248e636a1d1d);
}
impl ::windows::core::RuntimeName for AlternateWordForm {
    const NAME: &'static str = "Windows.Data.Text.AlternateWordForm";
}
impl ::core::convert::From<AlternateWordForm> for ::windows::core::IUnknown {
    fn from(value: AlternateWordForm) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AlternateWordForm> for ::windows::core::IUnknown {
    fn from(value: &AlternateWordForm) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AlternateWordForm {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AlternateWordForm {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AlternateWordForm> for ::windows::core::IInspectable {
    fn from(value: AlternateWordForm) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AlternateWordForm> for ::windows::core::IInspectable {
    fn from(value: &AlternateWordForm) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AlternateWordForm {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AlternateWordForm {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AlternateWordForm {}
unsafe impl ::core::marker::Sync for AlternateWordForm {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAlternateWordForm(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAlternateWordForm {
    type Vtable = IAlternateWordFormVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47396c1e_51b9_4207_9146_248e636a1d1d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAlternateWordFormVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TextSegment) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AlternateNormalizationFormat) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISelectableWordSegment(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISelectableWordSegment {
    type Vtable = ISelectableWordSegmentVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x916a4cb7_8aa7_4c78_b374_5dedb752e60b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectableWordSegmentVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TextSegment) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISelectableWordsSegmenter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISelectableWordsSegmenter {
    type Vtable = ISelectableWordsSegmenterVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6dc31e7_4b13_45c5_8897_7d71269e085d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectableWordsSegmenterVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, startindex: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, startindex: u32, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISelectableWordsSegmenterFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISelectableWordsSegmenterFactory {
    type Vtable = ISelectableWordsSegmenterFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c7a7648_6057_4339_bc70_f210010a4150);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectableWordsSegmenterFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISemanticTextQuery(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISemanticTextQuery {
    type Vtable = ISemanticTextQueryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a1cab51_1fb2_4909_80b8_35731a2b3e7f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISemanticTextQueryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertycontent: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISemanticTextQueryFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISemanticTextQueryFactory {
    type Vtable = ISemanticTextQueryFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x238c0503_f995_4587_8777_a2b7d80acfef);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISemanticTextQueryFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aqsfilter: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aqsfilter: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, filterlanguage: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextConversionGenerator(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextConversionGenerator {
    type Vtable = ITextConversionGeneratorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03606a5e_2aa9_4ab6_af8b_a562b63a8992);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextConversionGeneratorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, maxcandidates: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextConversionGeneratorFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextConversionGeneratorFactory {
    type Vtable = ITextConversionGeneratorFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcaa3781_3083_49ab_be15_56dfbbb74d6f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextConversionGeneratorFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languagetag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextPhoneme(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextPhoneme {
    type Vtable = ITextPhonemeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9362a40a_9b7a_4569_94cf_d84f2f38cf9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextPhonemeVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextPredictionGenerator(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextPredictionGenerator {
    type Vtable = ITextPredictionGeneratorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5eacab07_abf1_4cb6_9d9e_326f2b468756);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextPredictionGeneratorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, maxcandidates: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextPredictionGenerator2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextPredictionGenerator2 {
    type Vtable = ITextPredictionGenerator2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb84723b8_2c77_486a_900a_a3453eedc15d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextPredictionGenerator2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, maxcandidates: u32, predictionoptions: TextPredictionOptions, previousstrings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxcandidates: u32, previousstrings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "UI_Text_Core")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Text::Core::CoreTextInputScope) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text_Core"))] usize,
    #[cfg(feature = "UI_Text_Core")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::UI::Text::Core::CoreTextInputScope) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text_Core"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextPredictionGeneratorFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextPredictionGeneratorFactory {
    type Vtable = ITextPredictionGeneratorFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7257b416_8ba2_4751_9d30_9d85435653a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextPredictionGeneratorFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languagetag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextReverseConversionGenerator(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextReverseConversionGenerator {
    type Vtable = ITextReverseConversionGeneratorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51e7f514_9c51_4d86_ae1b_b498fbad8313);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextReverseConversionGeneratorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextReverseConversionGenerator2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextReverseConversionGenerator2 {
    type Vtable = ITextReverseConversionGenerator2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1aafd2ec_85d6_46fd_828a_3a4830fa6e18);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextReverseConversionGenerator2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextReverseConversionGeneratorFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextReverseConversionGeneratorFactory {
    type Vtable = ITextReverseConversionGeneratorFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63bed326_1fda_41f6_89d5_23ddea3c729a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextReverseConversionGeneratorFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languagetag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUnicodeCharactersStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUnicodeCharactersStatics {
    type Vtable = IUnicodeCharactersStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97909e87_9291_4f91_b6c8_b6e359d7a7fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnicodeCharactersStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, highsurrogate: u32, lowsurrogate: u32, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, codepoint: u32, highsurrogate: *mut u16, lowsurrogate: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut UnicodeNumericType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut UnicodeGeneralCategory) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWordSegment(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWordSegment {
    type Vtable = IWordSegmentVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2d4ba6d_987c_4cc0_b6bd_d49a11b38f9a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWordSegmentVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TextSegment) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWordsSegmenter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWordsSegmenter {
    type Vtable = IWordsSegmenterVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86b4d4d1_b2fe_4e34_a81d_66640300454f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWordsSegmenterVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, startindex: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, startindex: u32, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWordsSegmenterFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWordsSegmenterFactory {
    type Vtable = IWordsSegmenterFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6977274_fc35_455c_8bfb_6d7f4653ca97);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWordsSegmenterFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Data_Text'*"]
#[repr(transparent)]
pub struct SelectableWordSegment(::windows::core::IUnknown);
impl SelectableWordSegment {
    #[doc = "*Required features: 'Data_Text'*"]
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Text'*"]
    pub fn SourceTextSegment(&self) -> ::windows::core::Result<TextSegment> {
        let this = self;
        unsafe {
            let mut result__: TextSegment = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TextSegment>(result__)
        }
    }
}
impl ::core::clone::Clone for SelectableWordSegment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SelectableWordSegment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SelectableWordSegment {}
unsafe impl ::windows::core::RuntimeType for SelectableWordSegment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Text.SelectableWordSegment;{916a4cb7-8aa7-4c78-b374-5dedb752e60b})");
}
unsafe impl ::windows::core::Interface for SelectableWordSegment {
    type Vtable = ISelectableWordSegmentVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x916a4cb7_8aa7_4c78_b374_5dedb752e60b);
}
impl ::windows::core::RuntimeName for SelectableWordSegment {
    const NAME: &'static str = "Windows.Data.Text.SelectableWordSegment";
}
impl ::core::convert::From<SelectableWordSegment> for ::windows::core::IUnknown {
    fn from(value: SelectableWordSegment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SelectableWordSegment> for ::windows::core::IUnknown {
    fn from(value: &SelectableWordSegment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SelectableWordSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SelectableWordSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SelectableWordSegment> for ::windows::core::IInspectable {
    fn from(value: SelectableWordSegment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SelectableWordSegment> for ::windows::core::IInspectable {
    fn from(value: &SelectableWordSegment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SelectableWordSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SelectableWordSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SelectableWordSegment {}
unsafe impl ::core::marker::Sync for SelectableWordSegment {}
#[doc = "*Required features: 'Data_Text', 'Foundation_Collections'*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct SelectableWordSegmentsTokenizingHandler(pub ::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl SelectableWordSegmentsTokenizingHandler {
    pub fn new<F: FnMut(&::core::option::Option<super::super::Foundation::Collections::IIterable<SelectableWordSegment>>, &::core::option::Option<super::super::Foundation::Collections::IIterable<SelectableWordSegment>>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = SelectableWordSegmentsTokenizingHandlerBox::<F> { vtable: &SelectableWordSegmentsTokenizingHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: 'Data_Text', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<SelectableWordSegment>>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<SelectableWordSegment>>>(&self, precedingwords: Param0, words: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), precedingwords.into_param().abi(), words.into_param().abi()).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
#[repr(C)]
struct SelectableWordSegmentsTokenizingHandlerBox<F: FnMut(&::core::option::Option<super::super::Foundation::Collections::IIterable<SelectableWordSegment>>, &::core::option::Option<super::super::Foundation::Collections::IIterable<SelectableWordSegment>>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const SelectableWordSegmentsTokenizingHandlerVtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
#[cfg(feature = "Foundation_Collections")]
impl<F: FnMut(&::core::option::Option<super::super::Foundation::Collections::IIterable<SelectableWordSegment>>, &::core::option::Option<super::super::Foundation::Collections::IIterable<SelectableWordSegment>>) -> ::windows::core::Result<()> + 'static> SelectableWordSegmentsTokenizingHandlerBox<F> {
    const VTABLE: SelectableWordSegmentsTokenizingHandlerVtbl = SelectableWordSegmentsTokenizingHandlerVtbl(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, precedingwords: ::windows::core::RawPtr, words: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&precedingwords as *const <super::super::Foundation::Collections::IIterable<SelectableWordSegment> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<SelectableWordSegment> as ::windows::core::DefaultType>::DefaultType),
            &*(&words as *const <super::super::Foundation::Collections::IIterable<SelectableWordSegment> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<SelectableWordSegment> as ::windows::core::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for SelectableWordSegmentsTokenizingHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for SelectableWordSegmentsTokenizingHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for SelectableWordSegmentsTokenizingHandler {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for SelectableWordSegmentsTokenizingHandler {
    type Vtable = SelectableWordSegmentsTokenizingHandlerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a3dfc9c_aede_4dc7_9e6c_41c044bd3592);
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for SelectableWordSegmentsTokenizingHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{3a3dfc9c-aede-4dc7-9e6c-41c044bd3592}");
}
#[cfg(feature = "Foundation_Collections")]
#[repr(C)]
#[doc(hidden)]
pub struct SelectableWordSegmentsTokenizingHandlerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, precedingwords: ::windows::core::RawPtr, words: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc = "*Required features: 'Data_Text'*"]
#[repr(transparent)]
pub struct SelectableWordsSegmenter(::windows::core::IUnknown);
impl SelectableWordsSegmenter {
    #[doc = "*Required features: 'Data_Text'*"]
    pub fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Text'*"]
    pub fn GetTokenAt<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0, startindex: u32) -> ::windows::core::Result<SelectableWordSegment> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), text.into_param().abi(), startindex, &mut result__).from_abi::<SelectableWordSegment>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Text', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetTokens<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SelectableWordSegment>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<SelectableWordSegment>>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Text', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Tokenize<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, SelectableWordSegmentsTokenizingHandler>>(&self, text: Param0, startindex: u32, handler: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), text.into_param().abi(), startindex, handler.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Data_Text'*"]
    pub fn CreateWithLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(language: Param0) -> ::windows::core::Result<SelectableWordsSegmenter> {
        Self::ISelectableWordsSegmenterFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), language.into_param().abi(), &mut result__).from_abi::<SelectableWordsSegmenter>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISelectableWordsSegmenterFactory<R, F: FnOnce(&ISelectableWordsSegmenterFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SelectableWordsSegmenter, ISelectableWordsSegmenterFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SelectableWordsSegmenter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SelectableWordsSegmenter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SelectableWordsSegmenter {}
unsafe impl ::windows::core::RuntimeType for SelectableWordsSegmenter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Text.SelectableWordsSegmenter;{f6dc31e7-4b13-45c5-8897-7d71269e085d})");
}
unsafe impl ::windows::core::Interface for SelectableWordsSegmenter {
    type Vtable = ISelectableWordsSegmenterVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6dc31e7_4b13_45c5_8897_7d71269e085d);
}
impl ::windows::core::RuntimeName for SelectableWordsSegmenter {
    const NAME: &'static str = "Windows.Data.Text.SelectableWordsSegmenter";
}
impl ::core::convert::From<SelectableWordsSegmenter> for ::windows::core::IUnknown {
    fn from(value: SelectableWordsSegmenter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SelectableWordsSegmenter> for ::windows::core::IUnknown {
    fn from(value: &SelectableWordsSegmenter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SelectableWordsSegmenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SelectableWordsSegmenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SelectableWordsSegmenter> for ::windows::core::IInspectable {
    fn from(value: SelectableWordsSegmenter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SelectableWordsSegmenter> for ::windows::core::IInspectable {
    fn from(value: &SelectableWordsSegmenter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SelectableWordsSegmenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SelectableWordsSegmenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SelectableWordsSegmenter {}
unsafe impl ::core::marker::Sync for SelectableWordsSegmenter {}
#[doc = "*Required features: 'Data_Text'*"]
#[repr(transparent)]
pub struct SemanticTextQuery(::windows::core::IUnknown);
impl SemanticTextQuery {
    #[doc = "*Required features: 'Data_Text', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Find<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, content: Param0) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TextSegment>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), content.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<TextSegment>>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Text', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindInProperty<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, propertycontent: Param0, propertyname: Param1) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TextSegment>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), propertycontent.into_param().abi(), propertyname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<TextSegment>>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Text'*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(aqsfilter: Param0) -> ::windows::core::Result<SemanticTextQuery> {
        Self::ISemanticTextQueryFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), aqsfilter.into_param().abi(), &mut result__).from_abi::<SemanticTextQuery>(result__)
        })
    }
    #[doc = "*Required features: 'Data_Text'*"]
    pub fn CreateWithLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(aqsfilter: Param0, filterlanguage: Param1) -> ::windows::core::Result<SemanticTextQuery> {
        Self::ISemanticTextQueryFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), aqsfilter.into_param().abi(), filterlanguage.into_param().abi(), &mut result__).from_abi::<SemanticTextQuery>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISemanticTextQueryFactory<R, F: FnOnce(&ISemanticTextQueryFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SemanticTextQuery, ISemanticTextQueryFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SemanticTextQuery {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SemanticTextQuery {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SemanticTextQuery {}
unsafe impl ::windows::core::RuntimeType for SemanticTextQuery {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Text.SemanticTextQuery;{6a1cab51-1fb2-4909-80b8-35731a2b3e7f})");
}
unsafe impl ::windows::core::Interface for SemanticTextQuery {
    type Vtable = ISemanticTextQueryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a1cab51_1fb2_4909_80b8_35731a2b3e7f);
}
impl ::windows::core::RuntimeName for SemanticTextQuery {
    const NAME: &'static str = "Windows.Data.Text.SemanticTextQuery";
}
impl ::core::convert::From<SemanticTextQuery> for ::windows::core::IUnknown {
    fn from(value: SemanticTextQuery) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SemanticTextQuery> for ::windows::core::IUnknown {
    fn from(value: &SemanticTextQuery) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SemanticTextQuery {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SemanticTextQuery {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SemanticTextQuery> for ::windows::core::IInspectable {
    fn from(value: SemanticTextQuery) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SemanticTextQuery> for ::windows::core::IInspectable {
    fn from(value: &SemanticTextQuery) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SemanticTextQuery {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SemanticTextQuery {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SemanticTextQuery {}
unsafe impl ::core::marker::Sync for SemanticTextQuery {}
#[doc = "*Required features: 'Data_Text'*"]
#[repr(transparent)]
pub struct TextConversionGenerator(::windows::core::IUnknown);
impl TextConversionGenerator {
    #[doc = "*Required features: 'Data_Text'*"]
    pub fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Text'*"]
    pub fn LanguageAvailableButNotInstalled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Text', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetCandidatesAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, input: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Text', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetCandidatesWithMaxCountAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, input: Param0, maxcandidates: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), input.into_param().abi(), maxcandidates, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Text'*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(languagetag: Param0) -> ::windows::core::Result<TextConversionGenerator> {
        Self::ITextConversionGeneratorFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), languagetag.into_param().abi(), &mut result__).from_abi::<TextConversionGenerator>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITextConversionGeneratorFactory<R, F: FnOnce(&ITextConversionGeneratorFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TextConversionGenerator, ITextConversionGeneratorFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TextConversionGenerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TextConversionGenerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TextConversionGenerator {}
unsafe impl ::windows::core::RuntimeType for TextConversionGenerator {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Text.TextConversionGenerator;{03606a5e-2aa9-4ab6-af8b-a562b63a8992})");
}
unsafe impl ::windows::core::Interface for TextConversionGenerator {
    type Vtable = ITextConversionGeneratorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03606a5e_2aa9_4ab6_af8b_a562b63a8992);
}
impl ::windows::core::RuntimeName for TextConversionGenerator {
    const NAME: &'static str = "Windows.Data.Text.TextConversionGenerator";
}
impl ::core::convert::From<TextConversionGenerator> for ::windows::core::IUnknown {
    fn from(value: TextConversionGenerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TextConversionGenerator> for ::windows::core::IUnknown {
    fn from(value: &TextConversionGenerator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TextConversionGenerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TextConversionGenerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TextConversionGenerator> for ::windows::core::IInspectable {
    fn from(value: TextConversionGenerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TextConversionGenerator> for ::windows::core::IInspectable {
    fn from(value: &TextConversionGenerator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TextConversionGenerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TextConversionGenerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TextConversionGenerator {}
unsafe impl ::core::marker::Sync for TextConversionGenerator {}
#[doc = "*Required features: 'Data_Text'*"]
#[repr(transparent)]
pub struct TextPhoneme(::windows::core::IUnknown);
impl TextPhoneme {
    #[doc = "*Required features: 'Data_Text'*"]
    pub fn DisplayText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Text'*"]
    pub fn ReadingText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for TextPhoneme {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TextPhoneme {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TextPhoneme {}
unsafe impl ::windows::core::RuntimeType for TextPhoneme {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Text.TextPhoneme;{9362a40a-9b7a-4569-94cf-d84f2f38cf9b})");
}
unsafe impl ::windows::core::Interface for TextPhoneme {
    type Vtable = ITextPhonemeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9362a40a_9b7a_4569_94cf_d84f2f38cf9b);
}
impl ::windows::core::RuntimeName for TextPhoneme {
    const NAME: &'static str = "Windows.Data.Text.TextPhoneme";
}
impl ::core::convert::From<TextPhoneme> for ::windows::core::IUnknown {
    fn from(value: TextPhoneme) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TextPhoneme> for ::windows::core::IUnknown {
    fn from(value: &TextPhoneme) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TextPhoneme {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TextPhoneme {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TextPhoneme> for ::windows::core::IInspectable {
    fn from(value: TextPhoneme) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TextPhoneme> for ::windows::core::IInspectable {
    fn from(value: &TextPhoneme) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TextPhoneme {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TextPhoneme {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TextPhoneme {}
unsafe impl ::core::marker::Sync for TextPhoneme {}
#[doc = "*Required features: 'Data_Text'*"]
#[repr(transparent)]
pub struct TextPredictionGenerator(::windows::core::IUnknown);
impl TextPredictionGenerator {
    #[doc = "*Required features: 'Data_Text'*"]
    pub fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Text'*"]
    pub fn LanguageAvailableButNotInstalled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Text', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetCandidatesAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, input: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Text', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetCandidatesWithMaxCountAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, input: Param0, maxcandidates: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), input.into_param().abi(), maxcandidates, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Text', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetCandidatesWithParametersAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, input: Param0, maxcandidates: u32, predictionoptions: TextPredictionOptions, previousstrings: Param3) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>> {
        let this = &::windows::core::Interface::cast::<ITextPredictionGenerator2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), input.into_param().abi(), maxcandidates, predictionoptions, previousstrings.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Text', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetNextWordCandidatesAsync<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, maxcandidates: u32, previousstrings: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>> {
        let this = &::windows::core::Interface::cast::<ITextPredictionGenerator2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), maxcandidates, previousstrings.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Text', 'UI_Text_Core'*"]
    #[cfg(feature = "UI_Text_Core")]
    pub fn InputScope(&self) -> ::windows::core::Result<super::super::UI::Text::Core::CoreTextInputScope> {
        let this = &::windows::core::Interface::cast::<ITextPredictionGenerator2>(self)?;
        unsafe {
            let mut result__: super::super::UI::Text::Core::CoreTextInputScope = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Text::Core::CoreTextInputScope>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Text', 'UI_Text_Core'*"]
    #[cfg(feature = "UI_Text_Core")]
    pub fn SetInputScope(&self, value: super::super::UI::Text::Core::CoreTextInputScope) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextPredictionGenerator2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Data_Text'*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(languagetag: Param0) -> ::windows::core::Result<TextPredictionGenerator> {
        Self::ITextPredictionGeneratorFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), languagetag.into_param().abi(), &mut result__).from_abi::<TextPredictionGenerator>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITextPredictionGeneratorFactory<R, F: FnOnce(&ITextPredictionGeneratorFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TextPredictionGenerator, ITextPredictionGeneratorFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TextPredictionGenerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TextPredictionGenerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TextPredictionGenerator {}
unsafe impl ::windows::core::RuntimeType for TextPredictionGenerator {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Text.TextPredictionGenerator;{5eacab07-abf1-4cb6-9d9e-326f2b468756})");
}
unsafe impl ::windows::core::Interface for TextPredictionGenerator {
    type Vtable = ITextPredictionGeneratorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5eacab07_abf1_4cb6_9d9e_326f2b468756);
}
impl ::windows::core::RuntimeName for TextPredictionGenerator {
    const NAME: &'static str = "Windows.Data.Text.TextPredictionGenerator";
}
impl ::core::convert::From<TextPredictionGenerator> for ::windows::core::IUnknown {
    fn from(value: TextPredictionGenerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TextPredictionGenerator> for ::windows::core::IUnknown {
    fn from(value: &TextPredictionGenerator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TextPredictionGenerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TextPredictionGenerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TextPredictionGenerator> for ::windows::core::IInspectable {
    fn from(value: TextPredictionGenerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TextPredictionGenerator> for ::windows::core::IInspectable {
    fn from(value: &TextPredictionGenerator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TextPredictionGenerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TextPredictionGenerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TextPredictionGenerator {}
unsafe impl ::core::marker::Sync for TextPredictionGenerator {}
#[doc = "*Required features: 'Data_Text'*"]
#[repr(transparent)]
pub struct TextPredictionOptions(pub u32);
impl TextPredictionOptions {
    pub const None: Self = Self(0u32);
    pub const Predictions: Self = Self(1u32);
    pub const Corrections: Self = Self(2u32);
}
impl ::core::marker::Copy for TextPredictionOptions {}
impl ::core::clone::Clone for TextPredictionOptions {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TextPredictionOptions {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TextPredictionOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TextPredictionOptions {}
unsafe impl ::windows::core::RuntimeType for TextPredictionOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Data.Text.TextPredictionOptions;u4)");
}
impl ::windows::core::DefaultType for TextPredictionOptions {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Data_Text'*"]
#[repr(transparent)]
pub struct TextReverseConversionGenerator(::windows::core::IUnknown);
impl TextReverseConversionGenerator {
    #[doc = "*Required features: 'Data_Text'*"]
    pub fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Text'*"]
    pub fn LanguageAvailableButNotInstalled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Text', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ConvertBackAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, input: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Text', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetPhonemesAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, input: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<TextPhoneme>>> {
        let this = &::windows::core::Interface::cast::<ITextReverseConversionGenerator2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), input.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<TextPhoneme>>>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Text'*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(languagetag: Param0) -> ::windows::core::Result<TextReverseConversionGenerator> {
        Self::ITextReverseConversionGeneratorFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), languagetag.into_param().abi(), &mut result__).from_abi::<TextReverseConversionGenerator>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITextReverseConversionGeneratorFactory<R, F: FnOnce(&ITextReverseConversionGeneratorFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TextReverseConversionGenerator, ITextReverseConversionGeneratorFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TextReverseConversionGenerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TextReverseConversionGenerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TextReverseConversionGenerator {}
unsafe impl ::windows::core::RuntimeType for TextReverseConversionGenerator {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Text.TextReverseConversionGenerator;{51e7f514-9c51-4d86-ae1b-b498fbad8313})");
}
unsafe impl ::windows::core::Interface for TextReverseConversionGenerator {
    type Vtable = ITextReverseConversionGeneratorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51e7f514_9c51_4d86_ae1b_b498fbad8313);
}
impl ::windows::core::RuntimeName for TextReverseConversionGenerator {
    const NAME: &'static str = "Windows.Data.Text.TextReverseConversionGenerator";
}
impl ::core::convert::From<TextReverseConversionGenerator> for ::windows::core::IUnknown {
    fn from(value: TextReverseConversionGenerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TextReverseConversionGenerator> for ::windows::core::IUnknown {
    fn from(value: &TextReverseConversionGenerator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TextReverseConversionGenerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TextReverseConversionGenerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TextReverseConversionGenerator> for ::windows::core::IInspectable {
    fn from(value: TextReverseConversionGenerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TextReverseConversionGenerator> for ::windows::core::IInspectable {
    fn from(value: &TextReverseConversionGenerator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TextReverseConversionGenerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TextReverseConversionGenerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TextReverseConversionGenerator {}
unsafe impl ::core::marker::Sync for TextReverseConversionGenerator {}
#[repr(C)]
#[doc = "*Required features: 'Data_Text'*"]
pub struct TextSegment {
    pub StartPosition: u32,
    pub Length: u32,
}
impl ::core::marker::Copy for TextSegment {}
impl ::core::clone::Clone for TextSegment {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TextSegment {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for TextSegment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Data.Text.TextSegment;u4;u4)");
}
impl ::windows::core::DefaultType for TextSegment {
    type DefaultType = Self;
}
impl ::core::cmp::PartialEq for TextSegment {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TextSegment>()) == 0 }
    }
}
impl ::core::cmp::Eq for TextSegment {}
impl ::core::default::Default for TextSegment {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Data_Text'*"]
pub struct UnicodeCharacters {}
impl UnicodeCharacters {
    #[doc = "*Required features: 'Data_Text'*"]
    pub fn GetCodepointFromSurrogatePair(highsurrogate: u32, lowsurrogate: u32) -> ::windows::core::Result<u32> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), highsurrogate, lowsurrogate, &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: 'Data_Text'*"]
    pub fn GetSurrogatePairFromCodepoint(codepoint: u32, highsurrogate: &mut u16, lowsurrogate: &mut u16) -> ::windows::core::Result<()> {
        Self::IUnicodeCharactersStatics(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), codepoint, highsurrogate, lowsurrogate).ok() })
    }
    #[doc = "*Required features: 'Data_Text'*"]
    pub fn IsHighSurrogate(codepoint: u32) -> ::windows::core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), codepoint, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Data_Text'*"]
    pub fn IsLowSurrogate(codepoint: u32) -> ::windows::core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), codepoint, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Data_Text'*"]
    pub fn IsSupplementary(codepoint: u32) -> ::windows::core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), codepoint, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Data_Text'*"]
    pub fn IsNoncharacter(codepoint: u32) -> ::windows::core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), codepoint, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Data_Text'*"]
    pub fn IsWhitespace(codepoint: u32) -> ::windows::core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), codepoint, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Data_Text'*"]
    pub fn IsAlphabetic(codepoint: u32) -> ::windows::core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), codepoint, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Data_Text'*"]
    pub fn IsCased(codepoint: u32) -> ::windows::core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), codepoint, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Data_Text'*"]
    pub fn IsUppercase(codepoint: u32) -> ::windows::core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), codepoint, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Data_Text'*"]
    pub fn IsLowercase(codepoint: u32) -> ::windows::core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), codepoint, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Data_Text'*"]
    pub fn IsIdStart(codepoint: u32) -> ::windows::core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), codepoint, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Data_Text'*"]
    pub fn IsIdContinue(codepoint: u32) -> ::windows::core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), codepoint, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Data_Text'*"]
    pub fn IsGraphemeBase(codepoint: u32) -> ::windows::core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), codepoint, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Data_Text'*"]
    pub fn IsGraphemeExtend(codepoint: u32) -> ::windows::core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), codepoint, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Data_Text'*"]
    pub fn GetNumericType(codepoint: u32) -> ::windows::core::Result<UnicodeNumericType> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: UnicodeNumericType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), codepoint, &mut result__).from_abi::<UnicodeNumericType>(result__)
        })
    }
    #[doc = "*Required features: 'Data_Text'*"]
    pub fn GetGeneralCategory(codepoint: u32) -> ::windows::core::Result<UnicodeGeneralCategory> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__: UnicodeGeneralCategory = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), codepoint, &mut result__).from_abi::<UnicodeGeneralCategory>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUnicodeCharactersStatics<R, F: FnOnce(&IUnicodeCharactersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UnicodeCharacters, IUnicodeCharactersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for UnicodeCharacters {
    const NAME: &'static str = "Windows.Data.Text.UnicodeCharacters";
}
#[doc = "*Required features: 'Data_Text'*"]
#[repr(transparent)]
pub struct UnicodeGeneralCategory(pub i32);
impl UnicodeGeneralCategory {
    pub const UppercaseLetter: Self = Self(0i32);
    pub const LowercaseLetter: Self = Self(1i32);
    pub const TitlecaseLetter: Self = Self(2i32);
    pub const ModifierLetter: Self = Self(3i32);
    pub const OtherLetter: Self = Self(4i32);
    pub const NonspacingMark: Self = Self(5i32);
    pub const SpacingCombiningMark: Self = Self(6i32);
    pub const EnclosingMark: Self = Self(7i32);
    pub const DecimalDigitNumber: Self = Self(8i32);
    pub const LetterNumber: Self = Self(9i32);
    pub const OtherNumber: Self = Self(10i32);
    pub const SpaceSeparator: Self = Self(11i32);
    pub const LineSeparator: Self = Self(12i32);
    pub const ParagraphSeparator: Self = Self(13i32);
    pub const Control: Self = Self(14i32);
    pub const Format: Self = Self(15i32);
    pub const Surrogate: Self = Self(16i32);
    pub const PrivateUse: Self = Self(17i32);
    pub const ConnectorPunctuation: Self = Self(18i32);
    pub const DashPunctuation: Self = Self(19i32);
    pub const OpenPunctuation: Self = Self(20i32);
    pub const ClosePunctuation: Self = Self(21i32);
    pub const InitialQuotePunctuation: Self = Self(22i32);
    pub const FinalQuotePunctuation: Self = Self(23i32);
    pub const OtherPunctuation: Self = Self(24i32);
    pub const MathSymbol: Self = Self(25i32);
    pub const CurrencySymbol: Self = Self(26i32);
    pub const ModifierSymbol: Self = Self(27i32);
    pub const OtherSymbol: Self = Self(28i32);
    pub const NotAssigned: Self = Self(29i32);
}
impl ::core::marker::Copy for UnicodeGeneralCategory {}
impl ::core::clone::Clone for UnicodeGeneralCategory {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UnicodeGeneralCategory {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for UnicodeGeneralCategory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UnicodeGeneralCategory {}
unsafe impl ::windows::core::RuntimeType for UnicodeGeneralCategory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Data.Text.UnicodeGeneralCategory;i4)");
}
impl ::windows::core::DefaultType for UnicodeGeneralCategory {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Data_Text'*"]
#[repr(transparent)]
pub struct UnicodeNumericType(pub i32);
impl UnicodeNumericType {
    pub const None: Self = Self(0i32);
    pub const Decimal: Self = Self(1i32);
    pub const Digit: Self = Self(2i32);
    pub const Numeric: Self = Self(3i32);
}
impl ::core::marker::Copy for UnicodeNumericType {}
impl ::core::clone::Clone for UnicodeNumericType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UnicodeNumericType {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for UnicodeNumericType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UnicodeNumericType {}
unsafe impl ::windows::core::RuntimeType for UnicodeNumericType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Data.Text.UnicodeNumericType;i4)");
}
impl ::windows::core::DefaultType for UnicodeNumericType {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Data_Text'*"]
#[repr(transparent)]
pub struct WordSegment(::windows::core::IUnknown);
impl WordSegment {
    #[doc = "*Required features: 'Data_Text'*"]
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Text'*"]
    pub fn SourceTextSegment(&self) -> ::windows::core::Result<TextSegment> {
        let this = self;
        unsafe {
            let mut result__: TextSegment = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TextSegment>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Text', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AlternateForms(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AlternateWordForm>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<AlternateWordForm>>(result__)
        }
    }
}
impl ::core::clone::Clone for WordSegment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WordSegment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WordSegment {}
unsafe impl ::windows::core::RuntimeType for WordSegment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Text.WordSegment;{d2d4ba6d-987c-4cc0-b6bd-d49a11b38f9a})");
}
unsafe impl ::windows::core::Interface for WordSegment {
    type Vtable = IWordSegmentVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2d4ba6d_987c_4cc0_b6bd_d49a11b38f9a);
}
impl ::windows::core::RuntimeName for WordSegment {
    const NAME: &'static str = "Windows.Data.Text.WordSegment";
}
impl ::core::convert::From<WordSegment> for ::windows::core::IUnknown {
    fn from(value: WordSegment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WordSegment> for ::windows::core::IUnknown {
    fn from(value: &WordSegment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WordSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &WordSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WordSegment> for ::windows::core::IInspectable {
    fn from(value: WordSegment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WordSegment> for ::windows::core::IInspectable {
    fn from(value: &WordSegment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WordSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &WordSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WordSegment {}
unsafe impl ::core::marker::Sync for WordSegment {}
#[doc = "*Required features: 'Data_Text', 'Foundation_Collections'*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct WordSegmentsTokenizingHandler(pub ::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl WordSegmentsTokenizingHandler {
    pub fn new<F: FnMut(&::core::option::Option<super::super::Foundation::Collections::IIterable<WordSegment>>, &::core::option::Option<super::super::Foundation::Collections::IIterable<WordSegment>>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = WordSegmentsTokenizingHandlerBox::<F> { vtable: &WordSegmentsTokenizingHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: 'Data_Text', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<WordSegment>>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<WordSegment>>>(&self, precedingwords: Param0, words: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), precedingwords.into_param().abi(), words.into_param().abi()).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
#[repr(C)]
struct WordSegmentsTokenizingHandlerBox<F: FnMut(&::core::option::Option<super::super::Foundation::Collections::IIterable<WordSegment>>, &::core::option::Option<super::super::Foundation::Collections::IIterable<WordSegment>>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const WordSegmentsTokenizingHandlerVtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
#[cfg(feature = "Foundation_Collections")]
impl<F: FnMut(&::core::option::Option<super::super::Foundation::Collections::IIterable<WordSegment>>, &::core::option::Option<super::super::Foundation::Collections::IIterable<WordSegment>>) -> ::windows::core::Result<()> + 'static> WordSegmentsTokenizingHandlerBox<F> {
    const VTABLE: WordSegmentsTokenizingHandlerVtbl = WordSegmentsTokenizingHandlerVtbl(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, precedingwords: ::windows::core::RawPtr, words: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(&*(&precedingwords as *const <super::super::Foundation::Collections::IIterable<WordSegment> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<WordSegment> as ::windows::core::DefaultType>::DefaultType), &*(&words as *const <super::super::Foundation::Collections::IIterable<WordSegment> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<WordSegment> as ::windows::core::DefaultType>::DefaultType)).into()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for WordSegmentsTokenizingHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for WordSegmentsTokenizingHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for WordSegmentsTokenizingHandler {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for WordSegmentsTokenizingHandler {
    type Vtable = WordSegmentsTokenizingHandlerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5dd6357_bf2a_4c4f_a31f_29e71c6f8b35);
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for WordSegmentsTokenizingHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{a5dd6357-bf2a-4c4f-a31f-29e71c6f8b35}");
}
#[cfg(feature = "Foundation_Collections")]
#[repr(C)]
#[doc(hidden)]
pub struct WordSegmentsTokenizingHandlerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, precedingwords: ::windows::core::RawPtr, words: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc = "*Required features: 'Data_Text'*"]
#[repr(transparent)]
pub struct WordsSegmenter(::windows::core::IUnknown);
impl WordsSegmenter {
    #[doc = "*Required features: 'Data_Text'*"]
    pub fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Text'*"]
    pub fn GetTokenAt<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0, startindex: u32) -> ::windows::core::Result<WordSegment> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), text.into_param().abi(), startindex, &mut result__).from_abi::<WordSegment>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Text', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetTokens<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<WordSegment>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<WordSegment>>(result__)
        }
    }
    #[doc = "*Required features: 'Data_Text', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Tokenize<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, WordSegmentsTokenizingHandler>>(&self, text: Param0, startindex: u32, handler: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), text.into_param().abi(), startindex, handler.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Data_Text'*"]
    pub fn CreateWithLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(language: Param0) -> ::windows::core::Result<WordsSegmenter> {
        Self::IWordsSegmenterFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), language.into_param().abi(), &mut result__).from_abi::<WordsSegmenter>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWordsSegmenterFactory<R, F: FnOnce(&IWordsSegmenterFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WordsSegmenter, IWordsSegmenterFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for WordsSegmenter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WordsSegmenter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WordsSegmenter {}
unsafe impl ::windows::core::RuntimeType for WordsSegmenter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Data.Text.WordsSegmenter;{86b4d4d1-b2fe-4e34-a81d-66640300454f})");
}
unsafe impl ::windows::core::Interface for WordsSegmenter {
    type Vtable = IWordsSegmenterVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86b4d4d1_b2fe_4e34_a81d_66640300454f);
}
impl ::windows::core::RuntimeName for WordsSegmenter {
    const NAME: &'static str = "Windows.Data.Text.WordsSegmenter";
}
impl ::core::convert::From<WordsSegmenter> for ::windows::core::IUnknown {
    fn from(value: WordsSegmenter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WordsSegmenter> for ::windows::core::IUnknown {
    fn from(value: &WordsSegmenter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WordsSegmenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &WordsSegmenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WordsSegmenter> for ::windows::core::IInspectable {
    fn from(value: WordsSegmenter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WordsSegmenter> for ::windows::core::IInspectable {
    fn from(value: &WordsSegmenter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WordsSegmenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &WordsSegmenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WordsSegmenter {}
unsafe impl ::core::marker::Sync for WordsSegmenter {}
