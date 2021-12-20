#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IOcrEngine(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOcrEngine {
    type Vtable = IOcrEngineVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5a14bc41_5b76_3140_b680_8825562683ac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOcrEngineVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging")))] usize,
    #[cfg(feature = "Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Globalization"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IOcrEngineStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOcrEngineStatics {
    type Vtable = IOcrEngineStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5bffa85a_3384_3540_9940_699120d428a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOcrEngineStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Globalization")))] usize,
    #[cfg(feature = "Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Globalization"))] usize,
    #[cfg(feature = "Globalization")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Globalization"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IOcrLine(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOcrLine {
    type Vtable = IOcrLineVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0043a16f_e31f_3a24_899c_d444bd088124);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOcrLineVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IOcrResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOcrResult {
    type Vtable = IOcrResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9bd235b2_175b_3d6a_92e2_388c206e2f63);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOcrResultVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IOcrWord(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOcrWord {
    type Vtable = IOcrWordVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c2a477a_5cd9_3525_ba2a_23d1e0a68a1d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOcrWordVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Media_Ocr'*"]
#[repr(transparent)]
pub struct OcrEngine(::windows::core::IUnknown);
impl OcrEngine {
    #[doc = "*Required features: 'Media_Ocr', 'Foundation', 'Graphics_Imaging'*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub fn RecognizeAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::SoftwareBitmap>>(&self, bitmap: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<OcrResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<OcrResult>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Ocr', 'Globalization'*"]
    #[cfg(feature = "Globalization")]
    pub fn RecognizerLanguage(&self) -> ::windows::core::Result<super::super::Globalization::Language> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Globalization::Language>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Ocr'*"]
    pub fn MaxImageDimension() -> ::windows::core::Result<u32> {
        Self::IOcrEngineStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Ocr', 'Foundation_Collections', 'Globalization'*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
    pub fn AvailableRecognizerLanguages() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Globalization::Language>> {
        Self::IOcrEngineStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Globalization::Language>>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Ocr', 'Globalization'*"]
    #[cfg(feature = "Globalization")]
    pub fn IsLanguageSupported<'a, Param0: ::windows::core::IntoParam<'a, super::super::Globalization::Language>>(language: Param0) -> ::windows::core::Result<bool> {
        Self::IOcrEngineStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), language.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Ocr', 'Globalization'*"]
    #[cfg(feature = "Globalization")]
    pub fn TryCreateFromLanguage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Globalization::Language>>(language: Param0) -> ::windows::core::Result<OcrEngine> {
        Self::IOcrEngineStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), language.into_param().abi(), &mut result__).from_abi::<OcrEngine>(result__)
        })
    }
    #[doc = "*Required features: 'Media_Ocr'*"]
    pub fn TryCreateFromUserProfileLanguages() -> ::windows::core::Result<OcrEngine> {
        Self::IOcrEngineStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<OcrEngine>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IOcrEngineStatics<R, F: FnOnce(&IOcrEngineStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<OcrEngine, IOcrEngineStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for OcrEngine {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for OcrEngine {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OcrEngine {}
impl ::core::fmt::Debug for OcrEngine {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OcrEngine").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for OcrEngine {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Ocr.OcrEngine;{5a14bc41-5b76-3140-b680-8825562683ac})");
}
unsafe impl ::windows::core::Interface for OcrEngine {
    type Vtable = IOcrEngineVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5a14bc41_5b76_3140_b680_8825562683ac);
}
impl ::windows::core::RuntimeName for OcrEngine {
    const NAME: &'static str = "Windows.Media.Ocr.OcrEngine";
}
impl ::core::convert::From<OcrEngine> for ::windows::core::IUnknown {
    fn from(value: OcrEngine) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OcrEngine> for ::windows::core::IUnknown {
    fn from(value: &OcrEngine) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for OcrEngine {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &OcrEngine {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<OcrEngine> for ::windows::core::IInspectable {
    fn from(value: OcrEngine) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OcrEngine> for ::windows::core::IInspectable {
    fn from(value: &OcrEngine) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for OcrEngine {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &OcrEngine {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for OcrEngine {}
unsafe impl ::core::marker::Sync for OcrEngine {}
#[doc = "*Required features: 'Media_Ocr'*"]
#[repr(transparent)]
pub struct OcrLine(::windows::core::IUnknown);
impl OcrLine {
    #[doc = "*Required features: 'Media_Ocr', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Words(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<OcrWord>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<OcrWord>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Ocr'*"]
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for OcrLine {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for OcrLine {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OcrLine {}
impl ::core::fmt::Debug for OcrLine {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OcrLine").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for OcrLine {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Ocr.OcrLine;{0043a16f-e31f-3a24-899c-d444bd088124})");
}
unsafe impl ::windows::core::Interface for OcrLine {
    type Vtable = IOcrLineVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0043a16f_e31f_3a24_899c_d444bd088124);
}
impl ::windows::core::RuntimeName for OcrLine {
    const NAME: &'static str = "Windows.Media.Ocr.OcrLine";
}
impl ::core::convert::From<OcrLine> for ::windows::core::IUnknown {
    fn from(value: OcrLine) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OcrLine> for ::windows::core::IUnknown {
    fn from(value: &OcrLine) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for OcrLine {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &OcrLine {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<OcrLine> for ::windows::core::IInspectable {
    fn from(value: OcrLine) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OcrLine> for ::windows::core::IInspectable {
    fn from(value: &OcrLine) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for OcrLine {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &OcrLine {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for OcrLine {}
unsafe impl ::core::marker::Sync for OcrLine {}
#[doc = "*Required features: 'Media_Ocr'*"]
#[repr(transparent)]
pub struct OcrResult(::windows::core::IUnknown);
impl OcrResult {
    #[doc = "*Required features: 'Media_Ocr', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lines(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<OcrLine>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<OcrLine>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Ocr', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn TextAngle(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Ocr'*"]
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for OcrResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for OcrResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OcrResult {}
impl ::core::fmt::Debug for OcrResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OcrResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for OcrResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Ocr.OcrResult;{9bd235b2-175b-3d6a-92e2-388c206e2f63})");
}
unsafe impl ::windows::core::Interface for OcrResult {
    type Vtable = IOcrResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9bd235b2_175b_3d6a_92e2_388c206e2f63);
}
impl ::windows::core::RuntimeName for OcrResult {
    const NAME: &'static str = "Windows.Media.Ocr.OcrResult";
}
impl ::core::convert::From<OcrResult> for ::windows::core::IUnknown {
    fn from(value: OcrResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OcrResult> for ::windows::core::IUnknown {
    fn from(value: &OcrResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for OcrResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &OcrResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<OcrResult> for ::windows::core::IInspectable {
    fn from(value: OcrResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OcrResult> for ::windows::core::IInspectable {
    fn from(value: &OcrResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for OcrResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &OcrResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for OcrResult {}
unsafe impl ::core::marker::Sync for OcrResult {}
#[doc = "*Required features: 'Media_Ocr'*"]
#[repr(transparent)]
pub struct OcrWord(::windows::core::IUnknown);
impl OcrWord {
    #[doc = "*Required features: 'Media_Ocr', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: 'Media_Ocr'*"]
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for OcrWord {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for OcrWord {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OcrWord {}
impl ::core::fmt::Debug for OcrWord {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OcrWord").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for OcrWord {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Ocr.OcrWord;{3c2a477a-5cd9-3525-ba2a-23d1e0a68a1d})");
}
unsafe impl ::windows::core::Interface for OcrWord {
    type Vtable = IOcrWordVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c2a477a_5cd9_3525_ba2a_23d1e0a68a1d);
}
impl ::windows::core::RuntimeName for OcrWord {
    const NAME: &'static str = "Windows.Media.Ocr.OcrWord";
}
impl ::core::convert::From<OcrWord> for ::windows::core::IUnknown {
    fn from(value: OcrWord) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OcrWord> for ::windows::core::IUnknown {
    fn from(value: &OcrWord) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for OcrWord {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &OcrWord {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<OcrWord> for ::windows::core::IInspectable {
    fn from(value: OcrWord) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OcrWord> for ::windows::core::IInspectable {
    fn from(value: &OcrWord) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for OcrWord {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &OcrWord {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for OcrWord {}
unsafe impl ::core::marker::Sync for OcrWord {}
