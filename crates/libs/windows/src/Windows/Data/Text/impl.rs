#[cfg(feature = "implement_exclusive")]
pub trait IAlternateWordFormImpl: Sized {
    fn SourceTextSegment(&self) -> ::windows::core::Result<TextSegment>;
    fn AlternateText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NormalizationFormat(&self) -> ::windows::core::Result<AlternateNormalizationFormat>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAlternateWordForm {
    const NAME: &'static str = "Windows.Data.Text.IAlternateWordForm";
}
#[cfg(feature = "implement_exclusive")]
impl IAlternateWordFormVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAlternateWordFormImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAlternateWordFormVtbl {
        unsafe extern "system" fn SourceTextSegment<Impl: IAlternateWordFormImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TextSegment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceTextSegment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AlternateText<Impl: IAlternateWordFormImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlternateText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NormalizationFormat<Impl: IAlternateWordFormImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AlternateNormalizationFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NormalizationFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAlternateWordForm>, ::windows::core::GetTrustLevel, SourceTextSegment::<Impl, IMPL_OFFSET>, AlternateText::<Impl, IMPL_OFFSET>, NormalizationFormat::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAlternateWordForm as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISelectableWordSegmentImpl: Sized {
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SourceTextSegment(&self) -> ::windows::core::Result<TextSegment>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISelectableWordSegment {
    const NAME: &'static str = "Windows.Data.Text.ISelectableWordSegment";
}
#[cfg(feature = "implement_exclusive")]
impl ISelectableWordSegmentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectableWordSegmentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISelectableWordSegmentVtbl {
        unsafe extern "system" fn Text<Impl: ISelectableWordSegmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SourceTextSegment<Impl: ISelectableWordSegmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TextSegment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceTextSegment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISelectableWordSegment>, ::windows::core::GetTrustLevel, Text::<Impl, IMPL_OFFSET>, SourceTextSegment::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISelectableWordSegment as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ISelectableWordsSegmenterImpl: Sized {
    fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetTokenAt(&self, text: &::windows::core::HSTRING, startindex: u32) -> ::windows::core::Result<SelectableWordSegment>;
    fn GetTokens(&self, text: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SelectableWordSegment>>;
    fn Tokenize(&self, text: &::windows::core::HSTRING, startindex: u32, handler: &::core::option::Option<SelectableWordSegmentsTokenizingHandler>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISelectableWordsSegmenter {
    const NAME: &'static str = "Windows.Data.Text.ISelectableWordsSegmenter";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ISelectableWordsSegmenterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectableWordsSegmenterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISelectableWordsSegmenterVtbl {
        unsafe extern "system" fn ResolvedLanguage<Impl: ISelectableWordsSegmenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResolvedLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTokenAt<Impl: ISelectableWordsSegmenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, startindex: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTokenAt(&*(&text as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), startindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTokens<Impl: ISelectableWordsSegmenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTokens(&*(&text as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tokenize<Impl: ISelectableWordsSegmenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, startindex: u32, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Tokenize(&*(&text as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), startindex, &*(&handler as *const <SelectableWordSegmentsTokenizingHandler as ::windows::core::Abi>::Abi as *const <SelectableWordSegmentsTokenizingHandler as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISelectableWordsSegmenter>, ::windows::core::GetTrustLevel, ResolvedLanguage::<Impl, IMPL_OFFSET>, GetTokenAt::<Impl, IMPL_OFFSET>, GetTokens::<Impl, IMPL_OFFSET>, Tokenize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISelectableWordsSegmenter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISelectableWordsSegmenterFactoryImpl: Sized {
    fn CreateWithLanguage(&self, language: &::windows::core::HSTRING) -> ::windows::core::Result<SelectableWordsSegmenter>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISelectableWordsSegmenterFactory {
    const NAME: &'static str = "Windows.Data.Text.ISelectableWordsSegmenterFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISelectableWordsSegmenterFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectableWordsSegmenterFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISelectableWordsSegmenterFactoryVtbl {
        unsafe extern "system" fn CreateWithLanguage<Impl: ISelectableWordsSegmenterFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithLanguage(&*(&language as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISelectableWordsSegmenterFactory>, ::windows::core::GetTrustLevel, CreateWithLanguage::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISelectableWordsSegmenterFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ISemanticTextQueryImpl: Sized {
    fn Find(&self, content: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TextSegment>>;
    fn FindInProperty(&self, propertycontent: &::windows::core::HSTRING, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TextSegment>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISemanticTextQuery {
    const NAME: &'static str = "Windows.Data.Text.ISemanticTextQuery";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ISemanticTextQueryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISemanticTextQueryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISemanticTextQueryVtbl {
        unsafe extern "system" fn Find<Impl: ISemanticTextQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, content: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Find(&*(&content as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindInProperty<Impl: ISemanticTextQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertycontent: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindInProperty(&*(&propertycontent as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISemanticTextQuery>, ::windows::core::GetTrustLevel, Find::<Impl, IMPL_OFFSET>, FindInProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISemanticTextQuery as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISemanticTextQueryFactoryImpl: Sized {
    fn Create(&self, aqsfilter: &::windows::core::HSTRING) -> ::windows::core::Result<SemanticTextQuery>;
    fn CreateWithLanguage(&self, aqsfilter: &::windows::core::HSTRING, filterlanguage: &::windows::core::HSTRING) -> ::windows::core::Result<SemanticTextQuery>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISemanticTextQueryFactory {
    const NAME: &'static str = "Windows.Data.Text.ISemanticTextQueryFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISemanticTextQueryFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISemanticTextQueryFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISemanticTextQueryFactoryVtbl {
        unsafe extern "system" fn Create<Impl: ISemanticTextQueryFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aqsfilter: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&aqsfilter as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithLanguage<Impl: ISemanticTextQueryFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aqsfilter: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, filterlanguage: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithLanguage(&*(&aqsfilter as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&filterlanguage as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISemanticTextQueryFactory>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>, CreateWithLanguage::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISemanticTextQueryFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ITextConversionGeneratorImpl: Sized {
    fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LanguageAvailableButNotInstalled(&self) -> ::windows::core::Result<bool>;
    fn GetCandidatesAsync(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>;
    fn GetCandidatesWithMaxCountAsync(&self, input: &::windows::core::HSTRING, maxcandidates: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITextConversionGenerator {
    const NAME: &'static str = "Windows.Data.Text.ITextConversionGenerator";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ITextConversionGeneratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextConversionGeneratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextConversionGeneratorVtbl {
        unsafe extern "system" fn ResolvedLanguage<Impl: ITextConversionGeneratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResolvedLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LanguageAvailableButNotInstalled<Impl: ITextConversionGeneratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LanguageAvailableButNotInstalled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCandidatesAsync<Impl: ITextConversionGeneratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCandidatesAsync(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCandidatesWithMaxCountAsync<Impl: ITextConversionGeneratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, maxcandidates: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCandidatesWithMaxCountAsync(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), maxcandidates) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextConversionGenerator>, ::windows::core::GetTrustLevel, ResolvedLanguage::<Impl, IMPL_OFFSET>, LanguageAvailableButNotInstalled::<Impl, IMPL_OFFSET>, GetCandidatesAsync::<Impl, IMPL_OFFSET>, GetCandidatesWithMaxCountAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextConversionGenerator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextConversionGeneratorFactoryImpl: Sized {
    fn Create(&self, languagetag: &::windows::core::HSTRING) -> ::windows::core::Result<TextConversionGenerator>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITextConversionGeneratorFactory {
    const NAME: &'static str = "Windows.Data.Text.ITextConversionGeneratorFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ITextConversionGeneratorFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextConversionGeneratorFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextConversionGeneratorFactoryVtbl {
        unsafe extern "system" fn Create<Impl: ITextConversionGeneratorFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languagetag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&languagetag as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextConversionGeneratorFactory>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextConversionGeneratorFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextPhonemeImpl: Sized {
    fn DisplayText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReadingText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITextPhoneme {
    const NAME: &'static str = "Windows.Data.Text.ITextPhoneme";
}
#[cfg(feature = "implement_exclusive")]
impl ITextPhonemeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextPhonemeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextPhonemeVtbl {
        unsafe extern "system" fn DisplayText<Impl: ITextPhonemeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadingText<Impl: ITextPhonemeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadingText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextPhoneme>, ::windows::core::GetTrustLevel, DisplayText::<Impl, IMPL_OFFSET>, ReadingText::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextPhoneme as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ITextPredictionGeneratorImpl: Sized {
    fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LanguageAvailableButNotInstalled(&self) -> ::windows::core::Result<bool>;
    fn GetCandidatesAsync(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>;
    fn GetCandidatesWithMaxCountAsync(&self, input: &::windows::core::HSTRING, maxcandidates: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITextPredictionGenerator {
    const NAME: &'static str = "Windows.Data.Text.ITextPredictionGenerator";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ITextPredictionGeneratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextPredictionGeneratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextPredictionGeneratorVtbl {
        unsafe extern "system" fn ResolvedLanguage<Impl: ITextPredictionGeneratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResolvedLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LanguageAvailableButNotInstalled<Impl: ITextPredictionGeneratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LanguageAvailableButNotInstalled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCandidatesAsync<Impl: ITextPredictionGeneratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCandidatesAsync(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCandidatesWithMaxCountAsync<Impl: ITextPredictionGeneratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, maxcandidates: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCandidatesWithMaxCountAsync(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), maxcandidates) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextPredictionGenerator>, ::windows::core::GetTrustLevel, ResolvedLanguage::<Impl, IMPL_OFFSET>, LanguageAvailableButNotInstalled::<Impl, IMPL_OFFSET>, GetCandidatesAsync::<Impl, IMPL_OFFSET>, GetCandidatesWithMaxCountAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextPredictionGenerator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI_Text_Core", feature = "implement_exclusive"))]
pub trait ITextPredictionGenerator2Impl: Sized {
    fn GetCandidatesWithParametersAsync(&self, input: &::windows::core::HSTRING, maxcandidates: u32, predictionoptions: TextPredictionOptions, previousstrings: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>;
    fn GetNextWordCandidatesAsync(&self, maxcandidates: u32, previousstrings: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>;
    fn InputScope(&self) -> ::windows::core::Result<super::super::UI::Text::Core::CoreTextInputScope>;
    fn SetInputScope(&self, value: super::super::UI::Text::Core::CoreTextInputScope) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI_Text_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITextPredictionGenerator2 {
    const NAME: &'static str = "Windows.Data.Text.ITextPredictionGenerator2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI_Text_Core", feature = "implement_exclusive"))]
impl ITextPredictionGenerator2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextPredictionGenerator2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextPredictionGenerator2Vtbl {
        unsafe extern "system" fn GetCandidatesWithParametersAsync<Impl: ITextPredictionGenerator2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, maxcandidates: u32, predictionoptions: TextPredictionOptions, previousstrings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCandidatesWithParametersAsync(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), maxcandidates, predictionoptions, &*(&previousstrings as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextWordCandidatesAsync<Impl: ITextPredictionGenerator2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxcandidates: u32, previousstrings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNextWordCandidatesAsync(maxcandidates, &*(&previousstrings as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InputScope<Impl: ITextPredictionGenerator2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Text::Core::CoreTextInputScope) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InputScope() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInputScope<Impl: ITextPredictionGenerator2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::UI::Text::Core::CoreTextInputScope) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInputScope(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextPredictionGenerator2>, ::windows::core::GetTrustLevel, GetCandidatesWithParametersAsync::<Impl, IMPL_OFFSET>, GetNextWordCandidatesAsync::<Impl, IMPL_OFFSET>, InputScope::<Impl, IMPL_OFFSET>, SetInputScope::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextPredictionGenerator2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextPredictionGeneratorFactoryImpl: Sized {
    fn Create(&self, languagetag: &::windows::core::HSTRING) -> ::windows::core::Result<TextPredictionGenerator>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITextPredictionGeneratorFactory {
    const NAME: &'static str = "Windows.Data.Text.ITextPredictionGeneratorFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ITextPredictionGeneratorFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextPredictionGeneratorFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextPredictionGeneratorFactoryVtbl {
        unsafe extern "system" fn Create<Impl: ITextPredictionGeneratorFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languagetag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&languagetag as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextPredictionGeneratorFactory>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextPredictionGeneratorFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ITextReverseConversionGeneratorImpl: Sized {
    fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LanguageAvailableButNotInstalled(&self) -> ::windows::core::Result<bool>;
    fn ConvertBackAsync(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITextReverseConversionGenerator {
    const NAME: &'static str = "Windows.Data.Text.ITextReverseConversionGenerator";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ITextReverseConversionGeneratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextReverseConversionGeneratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextReverseConversionGeneratorVtbl {
        unsafe extern "system" fn ResolvedLanguage<Impl: ITextReverseConversionGeneratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResolvedLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LanguageAvailableButNotInstalled<Impl: ITextReverseConversionGeneratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LanguageAvailableButNotInstalled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertBackAsync<Impl: ITextReverseConversionGeneratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertBackAsync(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextReverseConversionGenerator>, ::windows::core::GetTrustLevel, ResolvedLanguage::<Impl, IMPL_OFFSET>, LanguageAvailableButNotInstalled::<Impl, IMPL_OFFSET>, ConvertBackAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextReverseConversionGenerator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ITextReverseConversionGenerator2Impl: Sized {
    fn GetPhonemesAsync(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<TextPhoneme>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITextReverseConversionGenerator2 {
    const NAME: &'static str = "Windows.Data.Text.ITextReverseConversionGenerator2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ITextReverseConversionGenerator2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextReverseConversionGenerator2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextReverseConversionGenerator2Vtbl {
        unsafe extern "system" fn GetPhonemesAsync<Impl: ITextReverseConversionGenerator2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPhonemesAsync(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextReverseConversionGenerator2>, ::windows::core::GetTrustLevel, GetPhonemesAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextReverseConversionGenerator2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextReverseConversionGeneratorFactoryImpl: Sized {
    fn Create(&self, languagetag: &::windows::core::HSTRING) -> ::windows::core::Result<TextReverseConversionGenerator>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITextReverseConversionGeneratorFactory {
    const NAME: &'static str = "Windows.Data.Text.ITextReverseConversionGeneratorFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ITextReverseConversionGeneratorFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextReverseConversionGeneratorFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextReverseConversionGeneratorFactoryVtbl {
        unsafe extern "system" fn Create<Impl: ITextReverseConversionGeneratorFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languagetag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&languagetag as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextReverseConversionGeneratorFactory>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextReverseConversionGeneratorFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUnicodeCharactersStaticsImpl: Sized {
    fn GetCodepointFromSurrogatePair(&self, highsurrogate: u32, lowsurrogate: u32) -> ::windows::core::Result<u32>;
    fn GetSurrogatePairFromCodepoint(&self, codepoint: u32, highsurrogate: &mut u16, lowsurrogate: &mut u16) -> ::windows::core::Result<()>;
    fn IsHighSurrogate(&self, codepoint: u32) -> ::windows::core::Result<bool>;
    fn IsLowSurrogate(&self, codepoint: u32) -> ::windows::core::Result<bool>;
    fn IsSupplementary(&self, codepoint: u32) -> ::windows::core::Result<bool>;
    fn IsNoncharacter(&self, codepoint: u32) -> ::windows::core::Result<bool>;
    fn IsWhitespace(&self, codepoint: u32) -> ::windows::core::Result<bool>;
    fn IsAlphabetic(&self, codepoint: u32) -> ::windows::core::Result<bool>;
    fn IsCased(&self, codepoint: u32) -> ::windows::core::Result<bool>;
    fn IsUppercase(&self, codepoint: u32) -> ::windows::core::Result<bool>;
    fn IsLowercase(&self, codepoint: u32) -> ::windows::core::Result<bool>;
    fn IsIdStart(&self, codepoint: u32) -> ::windows::core::Result<bool>;
    fn IsIdContinue(&self, codepoint: u32) -> ::windows::core::Result<bool>;
    fn IsGraphemeBase(&self, codepoint: u32) -> ::windows::core::Result<bool>;
    fn IsGraphemeExtend(&self, codepoint: u32) -> ::windows::core::Result<bool>;
    fn GetNumericType(&self, codepoint: u32) -> ::windows::core::Result<UnicodeNumericType>;
    fn GetGeneralCategory(&self, codepoint: u32) -> ::windows::core::Result<UnicodeGeneralCategory>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUnicodeCharactersStatics {
    const NAME: &'static str = "Windows.Data.Text.IUnicodeCharactersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IUnicodeCharactersStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUnicodeCharactersStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUnicodeCharactersStaticsVtbl {
        unsafe extern "system" fn GetCodepointFromSurrogatePair<Impl: IUnicodeCharactersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, highsurrogate: u32, lowsurrogate: u32, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCodepointFromSurrogatePair(highsurrogate, lowsurrogate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSurrogatePairFromCodepoint<Impl: IUnicodeCharactersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, codepoint: u32, highsurrogate: *mut u16, lowsurrogate: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSurrogatePairFromCodepoint(codepoint, ::core::mem::transmute_copy(&highsurrogate), ::core::mem::transmute_copy(&lowsurrogate)).into()
        }
        unsafe extern "system" fn IsHighSurrogate<Impl: IUnicodeCharactersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsHighSurrogate(codepoint) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLowSurrogate<Impl: IUnicodeCharactersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLowSurrogate(codepoint) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSupplementary<Impl: IUnicodeCharactersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSupplementary(codepoint) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsNoncharacter<Impl: IUnicodeCharactersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsNoncharacter(codepoint) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsWhitespace<Impl: IUnicodeCharactersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsWhitespace(codepoint) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAlphabetic<Impl: IUnicodeCharactersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAlphabetic(codepoint) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCased<Impl: IUnicodeCharactersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCased(codepoint) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUppercase<Impl: IUnicodeCharactersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsUppercase(codepoint) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLowercase<Impl: IUnicodeCharactersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLowercase(codepoint) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsIdStart<Impl: IUnicodeCharactersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsIdStart(codepoint) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsIdContinue<Impl: IUnicodeCharactersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsIdContinue(codepoint) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsGraphemeBase<Impl: IUnicodeCharactersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsGraphemeBase(codepoint) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsGraphemeExtend<Impl: IUnicodeCharactersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsGraphemeExtend(codepoint) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumericType<Impl: IUnicodeCharactersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut UnicodeNumericType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumericType(codepoint) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGeneralCategory<Impl: IUnicodeCharactersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut UnicodeGeneralCategory) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGeneralCategory(codepoint) {
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
            ::windows::core::GetRuntimeClassName::<IUnicodeCharactersStatics>,
            ::windows::core::GetTrustLevel,
            GetCodepointFromSurrogatePair::<Impl, IMPL_OFFSET>,
            GetSurrogatePairFromCodepoint::<Impl, IMPL_OFFSET>,
            IsHighSurrogate::<Impl, IMPL_OFFSET>,
            IsLowSurrogate::<Impl, IMPL_OFFSET>,
            IsSupplementary::<Impl, IMPL_OFFSET>,
            IsNoncharacter::<Impl, IMPL_OFFSET>,
            IsWhitespace::<Impl, IMPL_OFFSET>,
            IsAlphabetic::<Impl, IMPL_OFFSET>,
            IsCased::<Impl, IMPL_OFFSET>,
            IsUppercase::<Impl, IMPL_OFFSET>,
            IsLowercase::<Impl, IMPL_OFFSET>,
            IsIdStart::<Impl, IMPL_OFFSET>,
            IsIdContinue::<Impl, IMPL_OFFSET>,
            IsGraphemeBase::<Impl, IMPL_OFFSET>,
            IsGraphemeExtend::<Impl, IMPL_OFFSET>,
            GetNumericType::<Impl, IMPL_OFFSET>,
            GetGeneralCategory::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUnicodeCharactersStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IWordSegmentImpl: Sized {
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SourceTextSegment(&self) -> ::windows::core::Result<TextSegment>;
    fn AlternateForms(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AlternateWordForm>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWordSegment {
    const NAME: &'static str = "Windows.Data.Text.IWordSegment";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IWordSegmentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWordSegmentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWordSegmentVtbl {
        unsafe extern "system" fn Text<Impl: IWordSegmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SourceTextSegment<Impl: IWordSegmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TextSegment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceTextSegment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AlternateForms<Impl: IWordSegmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlternateForms() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWordSegment>, ::windows::core::GetTrustLevel, Text::<Impl, IMPL_OFFSET>, SourceTextSegment::<Impl, IMPL_OFFSET>, AlternateForms::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWordSegment as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IWordsSegmenterImpl: Sized {
    fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetTokenAt(&self, text: &::windows::core::HSTRING, startindex: u32) -> ::windows::core::Result<WordSegment>;
    fn GetTokens(&self, text: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<WordSegment>>;
    fn Tokenize(&self, text: &::windows::core::HSTRING, startindex: u32, handler: &::core::option::Option<WordSegmentsTokenizingHandler>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWordsSegmenter {
    const NAME: &'static str = "Windows.Data.Text.IWordsSegmenter";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IWordsSegmenterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWordsSegmenterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWordsSegmenterVtbl {
        unsafe extern "system" fn ResolvedLanguage<Impl: IWordsSegmenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResolvedLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTokenAt<Impl: IWordsSegmenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, startindex: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTokenAt(&*(&text as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), startindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTokens<Impl: IWordsSegmenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTokens(&*(&text as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tokenize<Impl: IWordsSegmenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, startindex: u32, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Tokenize(&*(&text as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), startindex, &*(&handler as *const <WordSegmentsTokenizingHandler as ::windows::core::Abi>::Abi as *const <WordSegmentsTokenizingHandler as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWordsSegmenter>, ::windows::core::GetTrustLevel, ResolvedLanguage::<Impl, IMPL_OFFSET>, GetTokenAt::<Impl, IMPL_OFFSET>, GetTokens::<Impl, IMPL_OFFSET>, Tokenize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWordsSegmenter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWordsSegmenterFactoryImpl: Sized {
    fn CreateWithLanguage(&self, language: &::windows::core::HSTRING) -> ::windows::core::Result<WordsSegmenter>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWordsSegmenterFactory {
    const NAME: &'static str = "Windows.Data.Text.IWordsSegmenterFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IWordsSegmenterFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWordsSegmenterFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWordsSegmenterFactoryVtbl {
        unsafe extern "system" fn CreateWithLanguage<Impl: IWordsSegmenterFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithLanguage(&*(&language as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWordsSegmenterFactory>, ::windows::core::GetTrustLevel, CreateWithLanguage::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWordsSegmenterFactory as ::windows::core::Interface>::IID
    }
}
