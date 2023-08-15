#[doc(hidden)]
#[repr(transparent)]
pub struct IOcrEngine(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOcrEngine {
    type Vtable = IOcrEngine_Vtbl;
}
impl ::core::clone::Clone for IOcrEngine {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IOcrEngine {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5a14bc41_5b76_3140_b680_8825562683ac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOcrEngine_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub RecognizeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging")))]
    RecognizeAsync: usize,
    #[cfg(feature = "Globalization")]
    pub RecognizerLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    RecognizerLanguage: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOcrEngineStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOcrEngineStatics {
    type Vtable = IOcrEngineStatics_Vtbl;
}
impl ::core::clone::Clone for IOcrEngineStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IOcrEngineStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5bffa85a_3384_3540_9940_699120d428a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOcrEngineStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub MaxImageDimension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
    pub AvailableRecognizerLanguages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Globalization")))]
    AvailableRecognizerLanguages: usize,
    #[cfg(feature = "Globalization")]
    pub IsLanguageSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    IsLanguageSupported: usize,
    #[cfg(feature = "Globalization")]
    pub TryCreateFromLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    TryCreateFromLanguage: usize,
    pub TryCreateFromUserProfileLanguages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOcrLine(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOcrLine {
    type Vtable = IOcrLine_Vtbl;
}
impl ::core::clone::Clone for IOcrLine {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IOcrLine {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0043a16f_e31f_3a24_899c_d444bd088124);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOcrLine_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Words: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Words: usize,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOcrResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOcrResult {
    type Vtable = IOcrResult_Vtbl;
}
impl ::core::clone::Clone for IOcrResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IOcrResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9bd235b2_175b_3d6a_92e2_388c206e2f63);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOcrResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Lines: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Lines: usize,
    #[cfg(feature = "Foundation")]
    pub TextAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TextAngle: usize,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOcrWord(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOcrWord {
    type Vtable = IOcrWord_Vtbl;
}
impl ::core::clone::Clone for IOcrWord {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IOcrWord {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3c2a477a_5cd9_3525_ba2a_23d1e0a68a1d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOcrWord_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub BoundingRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BoundingRect: usize,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Media_Ocr\"`*"]
#[repr(transparent)]
pub struct OcrEngine(::windows_core::IUnknown);
impl OcrEngine {
    #[doc = "*Required features: `\"Foundation\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub fn RecognizeAsync<P0>(&self, bitmap: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<OcrResult>>
    where
        P0: ::windows_core::IntoParam<super::super::Graphics::Imaging::SoftwareBitmap>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RecognizeAsync)(::windows_core::Interface::as_raw(this), bitmap.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Globalization\"`*"]
    #[cfg(feature = "Globalization")]
    pub fn RecognizerLanguage(&self) -> ::windows_core::Result<super::super::Globalization::Language> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RecognizerLanguage)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxImageDimension() -> ::windows_core::Result<u32> {
        Self::IOcrEngineStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxImageDimension)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Globalization\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization"))]
    pub fn AvailableRecognizerLanguages() -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Globalization::Language>> {
        Self::IOcrEngineStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AvailableRecognizerLanguages)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Globalization\"`*"]
    #[cfg(feature = "Globalization")]
    pub fn IsLanguageSupported<P0>(language: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<super::super::Globalization::Language>,
    {
        Self::IOcrEngineStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsLanguageSupported)(::windows_core::Interface::as_raw(this), language.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Globalization\"`*"]
    #[cfg(feature = "Globalization")]
    pub fn TryCreateFromLanguage<P0>(language: P0) -> ::windows_core::Result<OcrEngine>
    where
        P0: ::windows_core::IntoParam<super::super::Globalization::Language>,
    {
        Self::IOcrEngineStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryCreateFromLanguage)(::windows_core::Interface::as_raw(this), language.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn TryCreateFromUserProfileLanguages() -> ::windows_core::Result<OcrEngine> {
        Self::IOcrEngineStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryCreateFromUserProfileLanguages)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IOcrEngineStatics<R, F: FnOnce(&IOcrEngineStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<OcrEngine, IOcrEngineStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for OcrEngine {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Ocr.OcrEngine;{5a14bc41-5b76-3140-b680-8825562683ac})");
}
impl ::core::clone::Clone for OcrEngine {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for OcrEngine {
    type Vtable = IOcrEngine_Vtbl;
}
unsafe impl ::windows_core::ComInterface for OcrEngine {
    const IID: ::windows_core::GUID = <IOcrEngine as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for OcrEngine {
    const NAME: &'static str = "Windows.Media.Ocr.OcrEngine";
}
::windows_core::imp::interface_hierarchy!(OcrEngine, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for OcrEngine {}
unsafe impl ::core::marker::Sync for OcrEngine {}
#[doc = "*Required features: `\"Media_Ocr\"`*"]
#[repr(transparent)]
pub struct OcrLine(::windows_core::IUnknown);
impl OcrLine {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Words(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<OcrWord>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Words)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Text(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Text)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for OcrLine {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Ocr.OcrLine;{0043a16f-e31f-3a24-899c-d444bd088124})");
}
impl ::core::clone::Clone for OcrLine {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for OcrLine {
    type Vtable = IOcrLine_Vtbl;
}
unsafe impl ::windows_core::ComInterface for OcrLine {
    const IID: ::windows_core::GUID = <IOcrLine as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for OcrLine {
    const NAME: &'static str = "Windows.Media.Ocr.OcrLine";
}
::windows_core::imp::interface_hierarchy!(OcrLine, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for OcrLine {}
unsafe impl ::core::marker::Sync for OcrLine {}
#[doc = "*Required features: `\"Media_Ocr\"`*"]
#[repr(transparent)]
pub struct OcrResult(::windows_core::IUnknown);
impl OcrResult {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lines(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<OcrLine>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Lines)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TextAngle(&self) -> ::windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TextAngle)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Text(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Text)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for OcrResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Ocr.OcrResult;{9bd235b2-175b-3d6a-92e2-388c206e2f63})");
}
impl ::core::clone::Clone for OcrResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for OcrResult {
    type Vtable = IOcrResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for OcrResult {
    const IID: ::windows_core::GUID = <IOcrResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for OcrResult {
    const NAME: &'static str = "Windows.Media.Ocr.OcrResult";
}
::windows_core::imp::interface_hierarchy!(OcrResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for OcrResult {}
unsafe impl ::core::marker::Sync for OcrResult {}
#[doc = "*Required features: `\"Media_Ocr\"`*"]
#[repr(transparent)]
pub struct OcrWord(::windows_core::IUnknown);
impl OcrWord {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows_core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BoundingRect)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Text(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Text)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for OcrWord {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Ocr.OcrWord;{3c2a477a-5cd9-3525-ba2a-23d1e0a68a1d})");
}
impl ::core::clone::Clone for OcrWord {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for OcrWord {
    type Vtable = IOcrWord_Vtbl;
}
unsafe impl ::windows_core::ComInterface for OcrWord {
    const IID: ::windows_core::GUID = <IOcrWord as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for OcrWord {
    const NAME: &'static str = "Windows.Media.Ocr.OcrWord";
}
::windows_core::imp::interface_hierarchy!(OcrWord, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for OcrWord {}
unsafe impl ::core::marker::Sync for OcrWord {}
