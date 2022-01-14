#[cfg(all(feature = "Foundation", feature = "Globalization", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
pub trait IOcrEngine_Impl: Sized {
    fn RecognizeAsync(&mut self, bitmap: &::core::option::Option<super::super::Graphics::Imaging::SoftwareBitmap>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<OcrResult>>;
    fn RecognizerLanguage(&mut self) -> ::windows::core::Result<super::super::Globalization::Language>;
}
#[cfg(all(feature = "Foundation", feature = "Globalization", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IOcrEngine {
    const NAME: &'static str = "Windows.Media.Ocr.IOcrEngine";
}
#[cfg(all(feature = "Foundation", feature = "Globalization", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl IOcrEngine_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOcrEngine_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOcrEngine_Vtbl {
        unsafe extern "system" fn RecognizeAsync<Impl: IOcrEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecognizeAsync(&*(&bitmap as *const <super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecognizerLanguage<Impl: IOcrEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecognizerLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IOcrEngine, BASE_OFFSET>(),
            RecognizeAsync: RecognizeAsync::<Impl, IMPL_OFFSET>,
            RecognizerLanguage: RecognizerLanguage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOcrEngine as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Globalization", feature = "implement_exclusive"))]
pub trait IOcrEngineStatics_Impl: Sized {
    fn MaxImageDimension(&mut self) -> ::windows::core::Result<u32>;
    fn AvailableRecognizerLanguages(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Globalization::Language>>;
    fn IsLanguageSupported(&mut self, language: &::core::option::Option<super::super::Globalization::Language>) -> ::windows::core::Result<bool>;
    fn TryCreateFromLanguage(&mut self, language: &::core::option::Option<super::super::Globalization::Language>) -> ::windows::core::Result<OcrEngine>;
    fn TryCreateFromUserProfileLanguages(&mut self) -> ::windows::core::Result<OcrEngine>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Globalization", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IOcrEngineStatics {
    const NAME: &'static str = "Windows.Media.Ocr.IOcrEngineStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Globalization", feature = "implement_exclusive"))]
impl IOcrEngineStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOcrEngineStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOcrEngineStatics_Vtbl {
        unsafe extern "system" fn MaxImageDimension<Impl: IOcrEngineStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxImageDimension() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AvailableRecognizerLanguages<Impl: IOcrEngineStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AvailableRecognizerLanguages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLanguageSupported<Impl: IOcrEngineStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLanguageSupported(&*(&language as *const <super::super::Globalization::Language as ::windows::core::Abi>::Abi as *const <super::super::Globalization::Language as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryCreateFromLanguage<Impl: IOcrEngineStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryCreateFromLanguage(&*(&language as *const <super::super::Globalization::Language as ::windows::core::Abi>::Abi as *const <super::super::Globalization::Language as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryCreateFromUserProfileLanguages<Impl: IOcrEngineStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryCreateFromUserProfileLanguages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IOcrEngineStatics, BASE_OFFSET>(),
            MaxImageDimension: MaxImageDimension::<Impl, IMPL_OFFSET>,
            AvailableRecognizerLanguages: AvailableRecognizerLanguages::<Impl, IMPL_OFFSET>,
            IsLanguageSupported: IsLanguageSupported::<Impl, IMPL_OFFSET>,
            TryCreateFromLanguage: TryCreateFromLanguage::<Impl, IMPL_OFFSET>,
            TryCreateFromUserProfileLanguages: TryCreateFromUserProfileLanguages::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOcrEngineStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IOcrLine_Impl: Sized {
    fn Words(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<OcrWord>>;
    fn Text(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IOcrLine {
    const NAME: &'static str = "Windows.Media.Ocr.IOcrLine";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IOcrLine_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOcrLine_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOcrLine_Vtbl {
        unsafe extern "system" fn Words<Impl: IOcrLine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Words() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Text<Impl: IOcrLine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Text() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IOcrLine, BASE_OFFSET>(),
            Words: Words::<Impl, IMPL_OFFSET>,
            Text: Text::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOcrLine as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IOcrResult_Impl: Sized {
    fn Lines(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<OcrLine>>;
    fn TextAngle(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn Text(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IOcrResult {
    const NAME: &'static str = "Windows.Media.Ocr.IOcrResult";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IOcrResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOcrResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOcrResult_Vtbl {
        unsafe extern "system" fn Lines<Impl: IOcrResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Lines() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TextAngle<Impl: IOcrResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TextAngle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Text<Impl: IOcrResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Text() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IOcrResult, BASE_OFFSET>(),
            Lines: Lines::<Impl, IMPL_OFFSET>,
            TextAngle: TextAngle::<Impl, IMPL_OFFSET>,
            Text: Text::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOcrResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IOcrWord_Impl: Sized {
    fn BoundingRect(&mut self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn Text(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IOcrWord {
    const NAME: &'static str = "Windows.Media.Ocr.IOcrWord";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IOcrWord_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOcrWord_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOcrWord_Vtbl {
        unsafe extern "system" fn BoundingRect<Impl: IOcrWord_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BoundingRect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Text<Impl: IOcrWord_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Text() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IOcrWord, BASE_OFFSET>(),
            BoundingRect: BoundingRect::<Impl, IMPL_OFFSET>,
            Text: Text::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOcrWord as ::windows::core::Interface>::IID
    }
}
