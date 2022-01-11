#[cfg(all(feature = "Foundation", feature = "Globalization", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
pub trait IOcrEngineImpl: Sized {
    fn RecognizeAsync(&self, bitmap: &::core::option::Option<super::super::Graphics::Imaging::SoftwareBitmap>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<OcrResult>>;
    fn RecognizerLanguage(&self) -> ::windows::core::Result<super::super::Globalization::Language>;
}
#[cfg(all(feature = "Foundation", feature = "Globalization", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IOcrEngine {
    const NAME: &'static str = "Windows.Media.Ocr.IOcrEngine";
}
#[cfg(all(feature = "Foundation", feature = "Globalization", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl IOcrEngineVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOcrEngineImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOcrEngineVtbl {
        unsafe extern "system" fn RecognizeAsync<Impl: IOcrEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RecognizerLanguage<Impl: IOcrEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOcrEngine>, ::windows::core::GetTrustLevel, RecognizeAsync::<Impl, IMPL_OFFSET>, RecognizerLanguage::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOcrEngine as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Globalization", feature = "implement_exclusive"))]
pub trait IOcrEngineStaticsImpl: Sized {
    fn MaxImageDimension(&self) -> ::windows::core::Result<u32>;
    fn AvailableRecognizerLanguages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Globalization::Language>>;
    fn IsLanguageSupported(&self, language: &::core::option::Option<super::super::Globalization::Language>) -> ::windows::core::Result<bool>;
    fn TryCreateFromLanguage(&self, language: &::core::option::Option<super::super::Globalization::Language>) -> ::windows::core::Result<OcrEngine>;
    fn TryCreateFromUserProfileLanguages(&self) -> ::windows::core::Result<OcrEngine>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Globalization", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IOcrEngineStatics {
    const NAME: &'static str = "Windows.Media.Ocr.IOcrEngineStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Globalization", feature = "implement_exclusive"))]
impl IOcrEngineStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOcrEngineStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOcrEngineStaticsVtbl {
        unsafe extern "system" fn MaxImageDimension<Impl: IOcrEngineStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AvailableRecognizerLanguages<Impl: IOcrEngineStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsLanguageSupported<Impl: IOcrEngineStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryCreateFromLanguage<Impl: IOcrEngineStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryCreateFromUserProfileLanguages<Impl: IOcrEngineStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IOcrEngineStatics>,
            ::windows::core::GetTrustLevel,
            MaxImageDimension::<Impl, IMPL_OFFSET>,
            AvailableRecognizerLanguages::<Impl, IMPL_OFFSET>,
            IsLanguageSupported::<Impl, IMPL_OFFSET>,
            TryCreateFromLanguage::<Impl, IMPL_OFFSET>,
            TryCreateFromUserProfileLanguages::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOcrEngineStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IOcrLineImpl: Sized {
    fn Words(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<OcrWord>>;
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IOcrLine {
    const NAME: &'static str = "Windows.Media.Ocr.IOcrLine";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IOcrLineVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOcrLineImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOcrLineVtbl {
        unsafe extern "system" fn Words<Impl: IOcrLineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Text<Impl: IOcrLineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOcrLine>, ::windows::core::GetTrustLevel, Words::<Impl, IMPL_OFFSET>, Text::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOcrLine as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IOcrResultImpl: Sized {
    fn Lines(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<OcrLine>>;
    fn TextAngle(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IOcrResult {
    const NAME: &'static str = "Windows.Media.Ocr.IOcrResult";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IOcrResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOcrResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOcrResultVtbl {
        unsafe extern "system" fn Lines<Impl: IOcrResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TextAngle<Impl: IOcrResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Text<Impl: IOcrResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOcrResult>, ::windows::core::GetTrustLevel, Lines::<Impl, IMPL_OFFSET>, TextAngle::<Impl, IMPL_OFFSET>, Text::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOcrResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IOcrWordImpl: Sized {
    fn BoundingRect(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IOcrWord {
    const NAME: &'static str = "Windows.Media.Ocr.IOcrWord";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IOcrWordVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOcrWordImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOcrWordVtbl {
        unsafe extern "system" fn BoundingRect<Impl: IOcrWordImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Text<Impl: IOcrWordImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOcrWord>, ::windows::core::GetTrustLevel, BoundingRect::<Impl, IMPL_OFFSET>, Text::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOcrWord as ::windows::core::Interface>::IID
    }
}
