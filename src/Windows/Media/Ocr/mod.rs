#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IOcrEngine(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IOcrEngine {
    type Vtable = IOcrEngine_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1511308353, 23414, 12608, [182, 128, 136, 37, 86, 38, 131, 172]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOcrEngine_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bitmap: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging")))] usize,
    #[cfg(feature = "Globalization")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Globalization"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IOcrEngineStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IOcrEngineStatics {
    type Vtable = IOcrEngineStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1543481434, 13188, 13632, [153, 64, 105, 145, 32, 212, 40, 168]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOcrEngineStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Globalization")))] usize,
    #[cfg(feature = "Globalization")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, language: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Globalization"))] usize,
    #[cfg(feature = "Globalization")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, language: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Globalization"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IOcrLine(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IOcrLine {
    type Vtable = IOcrLine_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4432239, 58143, 14884, [137, 156, 212, 68, 189, 8, 129, 36]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOcrLine_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IOcrResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IOcrResult {
    type Vtable = IOcrResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2614244786, 5979, 15722, [146, 226, 56, 140, 32, 110, 47, 99]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOcrResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IOcrWord(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IOcrWord {
    type Vtable = IOcrWord_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1009403770, 23769, 13605, [186, 42, 35, 209, 224, 166, 138, 29]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOcrWord_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Rect) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct OcrEngine(::windows::runtime::IInspectable);
impl OcrEngine {
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub fn RecognizeAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Imaging::SoftwareBitmap>>(&self, bitmap: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<OcrResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), bitmap.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<OcrResult>>(result__)
        }
    }
    #[cfg(feature = "Globalization")]
    pub fn RecognizerLanguage(&self) -> ::windows::runtime::Result<super::super::Globalization::Language> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Globalization::Language>(result__)
        }
    }
    pub fn MaxImageDimension() -> ::windows::runtime::Result<u32> {
        Self::IOcrEngineStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
    pub fn AvailableRecognizerLanguages() -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<super::super::Globalization::Language>> {
        Self::IOcrEngineStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Globalization::Language>>(result__)
        })
    }
    #[cfg(feature = "Globalization")]
    pub fn IsLanguageSupported<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Globalization::Language>>(language: Param0) -> ::windows::runtime::Result<bool> {
        Self::IOcrEngineStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), language.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "Globalization")]
    pub fn TryCreateFromLanguage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Globalization::Language>>(language: Param0) -> ::windows::runtime::Result<OcrEngine> {
        Self::IOcrEngineStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), language.into_param().abi(), &mut result__).from_abi::<OcrEngine>(result__)
        })
    }
    pub fn TryCreateFromUserProfileLanguages() -> ::windows::runtime::Result<OcrEngine> {
        Self::IOcrEngineStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<OcrEngine>(result__)
        })
    }
    pub fn IOcrEngineStatics<R, F: FnOnce(&IOcrEngineStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<OcrEngine, IOcrEngineStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for OcrEngine {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Ocr.OcrEngine;{5a14bc41-5b76-3140-b680-8825562683ac})");
}
unsafe impl ::windows::runtime::Interface for OcrEngine {
    type Vtable = IOcrEngine_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1511308353, 23414, 12608, [182, 128, 136, 37, 86, 38, 131, 172]);
}
impl ::windows::runtime::RuntimeName for OcrEngine {
    const NAME: &'static str = "Windows.Media.Ocr.OcrEngine";
}
impl ::std::convert::From<OcrEngine> for ::windows::runtime::IUnknown {
    fn from(value: OcrEngine) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&OcrEngine> for ::windows::runtime::IUnknown {
    fn from(value: &OcrEngine) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for OcrEngine {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &OcrEngine {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<OcrEngine> for ::windows::runtime::IInspectable {
    fn from(value: OcrEngine) -> Self {
        value.0
    }
}
impl ::std::convert::From<&OcrEngine> for ::windows::runtime::IInspectable {
    fn from(value: &OcrEngine) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for OcrEngine {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a OcrEngine {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for OcrEngine {}
unsafe impl ::std::marker::Sync for OcrEngine {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct OcrLine(::windows::runtime::IInspectable);
impl OcrLine {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Words(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<OcrWord>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<OcrWord>>(result__)
        }
    }
    pub fn Text(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for OcrLine {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Ocr.OcrLine;{0043a16f-e31f-3a24-899c-d444bd088124})");
}
unsafe impl ::windows::runtime::Interface for OcrLine {
    type Vtable = IOcrLine_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4432239, 58143, 14884, [137, 156, 212, 68, 189, 8, 129, 36]);
}
impl ::windows::runtime::RuntimeName for OcrLine {
    const NAME: &'static str = "Windows.Media.Ocr.OcrLine";
}
impl ::std::convert::From<OcrLine> for ::windows::runtime::IUnknown {
    fn from(value: OcrLine) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&OcrLine> for ::windows::runtime::IUnknown {
    fn from(value: &OcrLine) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for OcrLine {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &OcrLine {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<OcrLine> for ::windows::runtime::IInspectable {
    fn from(value: OcrLine) -> Self {
        value.0
    }
}
impl ::std::convert::From<&OcrLine> for ::windows::runtime::IInspectable {
    fn from(value: &OcrLine) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for OcrLine {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a OcrLine {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for OcrLine {}
unsafe impl ::std::marker::Sync for OcrLine {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct OcrResult(::windows::runtime::IInspectable);
impl OcrResult {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lines(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<OcrLine>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<OcrLine>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TextAngle(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    pub fn Text(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for OcrResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Ocr.OcrResult;{9bd235b2-175b-3d6a-92e2-388c206e2f63})");
}
unsafe impl ::windows::runtime::Interface for OcrResult {
    type Vtable = IOcrResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2614244786, 5979, 15722, [146, 226, 56, 140, 32, 110, 47, 99]);
}
impl ::windows::runtime::RuntimeName for OcrResult {
    const NAME: &'static str = "Windows.Media.Ocr.OcrResult";
}
impl ::std::convert::From<OcrResult> for ::windows::runtime::IUnknown {
    fn from(value: OcrResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&OcrResult> for ::windows::runtime::IUnknown {
    fn from(value: &OcrResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for OcrResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &OcrResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<OcrResult> for ::windows::runtime::IInspectable {
    fn from(value: OcrResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&OcrResult> for ::windows::runtime::IInspectable {
    fn from(value: &OcrResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for OcrResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a OcrResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for OcrResult {}
unsafe impl ::std::marker::Sync for OcrResult {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct OcrWord(::windows::runtime::IInspectable);
impl OcrWord {
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::runtime::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    pub fn Text(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for OcrWord {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Ocr.OcrWord;{3c2a477a-5cd9-3525-ba2a-23d1e0a68a1d})");
}
unsafe impl ::windows::runtime::Interface for OcrWord {
    type Vtable = IOcrWord_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1009403770, 23769, 13605, [186, 42, 35, 209, 224, 166, 138, 29]);
}
impl ::windows::runtime::RuntimeName for OcrWord {
    const NAME: &'static str = "Windows.Media.Ocr.OcrWord";
}
impl ::std::convert::From<OcrWord> for ::windows::runtime::IUnknown {
    fn from(value: OcrWord) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&OcrWord> for ::windows::runtime::IUnknown {
    fn from(value: &OcrWord) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for OcrWord {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &OcrWord {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<OcrWord> for ::windows::runtime::IInspectable {
    fn from(value: OcrWord) -> Self {
        value.0
    }
}
impl ::std::convert::From<&OcrWord> for ::windows::runtime::IInspectable {
    fn from(value: &OcrWord) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for OcrWord {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a OcrWord {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for OcrWord {}
unsafe impl ::std::marker::Sync for OcrWord {}
