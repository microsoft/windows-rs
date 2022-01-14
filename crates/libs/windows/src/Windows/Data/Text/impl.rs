#[cfg(feature = "implement_exclusive")]
pub trait IAlternateWordForm_Impl: Sized {
    fn SourceTextSegment(&mut self) -> ::windows::core::Result<TextSegment>;
    fn AlternateText(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NormalizationFormat(&mut self) -> ::windows::core::Result<AlternateNormalizationFormat>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAlternateWordForm {
    const NAME: &'static str = "Windows.Data.Text.IAlternateWordForm";
}
#[cfg(feature = "implement_exclusive")]
impl IAlternateWordForm_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAlternateWordForm_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAlternateWordForm_Vtbl {
        unsafe extern "system" fn SourceTextSegment<Impl: IAlternateWordForm_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TextSegment) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AlternateText<Impl: IAlternateWordForm_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NormalizationFormat<Impl: IAlternateWordForm_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AlternateNormalizationFormat) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAlternateWordForm, BASE_OFFSET>(),
            SourceTextSegment: SourceTextSegment::<Impl, IMPL_OFFSET>,
            AlternateText: AlternateText::<Impl, IMPL_OFFSET>,
            NormalizationFormat: NormalizationFormat::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAlternateWordForm as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISelectableWordSegment_Impl: Sized {
    fn Text(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SourceTextSegment(&mut self) -> ::windows::core::Result<TextSegment>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISelectableWordSegment {
    const NAME: &'static str = "Windows.Data.Text.ISelectableWordSegment";
}
#[cfg(feature = "implement_exclusive")]
impl ISelectableWordSegment_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectableWordSegment_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISelectableWordSegment_Vtbl {
        unsafe extern "system" fn Text<Impl: ISelectableWordSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SourceTextSegment<Impl: ISelectableWordSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TextSegment) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISelectableWordSegment, BASE_OFFSET>(),
            Text: Text::<Impl, IMPL_OFFSET>,
            SourceTextSegment: SourceTextSegment::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISelectableWordSegment as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ISelectableWordsSegmenter_Impl: Sized {
    fn ResolvedLanguage(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetTokenAt(&mut self, text: &::windows::core::HSTRING, startindex: u32) -> ::windows::core::Result<SelectableWordSegment>;
    fn GetTokens(&mut self, text: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SelectableWordSegment>>;
    fn Tokenize(&mut self, text: &::windows::core::HSTRING, startindex: u32, handler: &::core::option::Option<SelectableWordSegmentsTokenizingHandler>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISelectableWordsSegmenter {
    const NAME: &'static str = "Windows.Data.Text.ISelectableWordsSegmenter";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ISelectableWordsSegmenter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectableWordsSegmenter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISelectableWordsSegmenter_Vtbl {
        unsafe extern "system" fn ResolvedLanguage<Impl: ISelectableWordsSegmenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetTokenAt<Impl: ISelectableWordsSegmenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, startindex: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetTokens<Impl: ISelectableWordsSegmenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Tokenize<Impl: ISelectableWordsSegmenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, startindex: u32, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Tokenize(&*(&text as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), startindex, &*(&handler as *const <SelectableWordSegmentsTokenizingHandler as ::windows::core::Abi>::Abi as *const <SelectableWordSegmentsTokenizingHandler as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISelectableWordsSegmenter, BASE_OFFSET>(),
            ResolvedLanguage: ResolvedLanguage::<Impl, IMPL_OFFSET>,
            GetTokenAt: GetTokenAt::<Impl, IMPL_OFFSET>,
            GetTokens: GetTokens::<Impl, IMPL_OFFSET>,
            Tokenize: Tokenize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISelectableWordsSegmenter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISelectableWordsSegmenterFactory_Impl: Sized {
    fn CreateWithLanguage(&mut self, language: &::windows::core::HSTRING) -> ::windows::core::Result<SelectableWordsSegmenter>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISelectableWordsSegmenterFactory {
    const NAME: &'static str = "Windows.Data.Text.ISelectableWordsSegmenterFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISelectableWordsSegmenterFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectableWordsSegmenterFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISelectableWordsSegmenterFactory_Vtbl {
        unsafe extern "system" fn CreateWithLanguage<Impl: ISelectableWordsSegmenterFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISelectableWordsSegmenterFactory, BASE_OFFSET>(),
            CreateWithLanguage: CreateWithLanguage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISelectableWordsSegmenterFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ISemanticTextQuery_Impl: Sized {
    fn Find(&mut self, content: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TextSegment>>;
    fn FindInProperty(&mut self, propertycontent: &::windows::core::HSTRING, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TextSegment>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISemanticTextQuery {
    const NAME: &'static str = "Windows.Data.Text.ISemanticTextQuery";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ISemanticTextQuery_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISemanticTextQuery_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISemanticTextQuery_Vtbl {
        unsafe extern "system" fn Find<Impl: ISemanticTextQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, content: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindInProperty<Impl: ISemanticTextQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertycontent: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISemanticTextQuery, BASE_OFFSET>(),
            Find: Find::<Impl, IMPL_OFFSET>,
            FindInProperty: FindInProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISemanticTextQuery as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISemanticTextQueryFactory_Impl: Sized {
    fn Create(&mut self, aqsfilter: &::windows::core::HSTRING) -> ::windows::core::Result<SemanticTextQuery>;
    fn CreateWithLanguage(&mut self, aqsfilter: &::windows::core::HSTRING, filterlanguage: &::windows::core::HSTRING) -> ::windows::core::Result<SemanticTextQuery>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISemanticTextQueryFactory {
    const NAME: &'static str = "Windows.Data.Text.ISemanticTextQueryFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISemanticTextQueryFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISemanticTextQueryFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISemanticTextQueryFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: ISemanticTextQueryFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aqsfilter: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateWithLanguage<Impl: ISemanticTextQueryFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aqsfilter: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, filterlanguage: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISemanticTextQueryFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            CreateWithLanguage: CreateWithLanguage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISemanticTextQueryFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ITextConversionGenerator_Impl: Sized {
    fn ResolvedLanguage(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LanguageAvailableButNotInstalled(&mut self) -> ::windows::core::Result<bool>;
    fn GetCandidatesAsync(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>;
    fn GetCandidatesWithMaxCountAsync(&mut self, input: &::windows::core::HSTRING, maxcandidates: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITextConversionGenerator {
    const NAME: &'static str = "Windows.Data.Text.ITextConversionGenerator";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ITextConversionGenerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextConversionGenerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextConversionGenerator_Vtbl {
        unsafe extern "system" fn ResolvedLanguage<Impl: ITextConversionGenerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LanguageAvailableButNotInstalled<Impl: ITextConversionGenerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetCandidatesAsync<Impl: ITextConversionGenerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetCandidatesWithMaxCountAsync<Impl: ITextConversionGenerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, maxcandidates: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITextConversionGenerator, BASE_OFFSET>(),
            ResolvedLanguage: ResolvedLanguage::<Impl, IMPL_OFFSET>,
            LanguageAvailableButNotInstalled: LanguageAvailableButNotInstalled::<Impl, IMPL_OFFSET>,
            GetCandidatesAsync: GetCandidatesAsync::<Impl, IMPL_OFFSET>,
            GetCandidatesWithMaxCountAsync: GetCandidatesWithMaxCountAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextConversionGenerator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextConversionGeneratorFactory_Impl: Sized {
    fn Create(&mut self, languagetag: &::windows::core::HSTRING) -> ::windows::core::Result<TextConversionGenerator>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITextConversionGeneratorFactory {
    const NAME: &'static str = "Windows.Data.Text.ITextConversionGeneratorFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ITextConversionGeneratorFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextConversionGeneratorFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextConversionGeneratorFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: ITextConversionGeneratorFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languagetag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITextConversionGeneratorFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextConversionGeneratorFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextPhoneme_Impl: Sized {
    fn DisplayText(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReadingText(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITextPhoneme {
    const NAME: &'static str = "Windows.Data.Text.ITextPhoneme";
}
#[cfg(feature = "implement_exclusive")]
impl ITextPhoneme_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextPhoneme_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextPhoneme_Vtbl {
        unsafe extern "system" fn DisplayText<Impl: ITextPhoneme_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReadingText<Impl: ITextPhoneme_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITextPhoneme, BASE_OFFSET>(),
            DisplayText: DisplayText::<Impl, IMPL_OFFSET>,
            ReadingText: ReadingText::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextPhoneme as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ITextPredictionGenerator_Impl: Sized {
    fn ResolvedLanguage(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LanguageAvailableButNotInstalled(&mut self) -> ::windows::core::Result<bool>;
    fn GetCandidatesAsync(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>;
    fn GetCandidatesWithMaxCountAsync(&mut self, input: &::windows::core::HSTRING, maxcandidates: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITextPredictionGenerator {
    const NAME: &'static str = "Windows.Data.Text.ITextPredictionGenerator";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ITextPredictionGenerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextPredictionGenerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextPredictionGenerator_Vtbl {
        unsafe extern "system" fn ResolvedLanguage<Impl: ITextPredictionGenerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LanguageAvailableButNotInstalled<Impl: ITextPredictionGenerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetCandidatesAsync<Impl: ITextPredictionGenerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetCandidatesWithMaxCountAsync<Impl: ITextPredictionGenerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, maxcandidates: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITextPredictionGenerator, BASE_OFFSET>(),
            ResolvedLanguage: ResolvedLanguage::<Impl, IMPL_OFFSET>,
            LanguageAvailableButNotInstalled: LanguageAvailableButNotInstalled::<Impl, IMPL_OFFSET>,
            GetCandidatesAsync: GetCandidatesAsync::<Impl, IMPL_OFFSET>,
            GetCandidatesWithMaxCountAsync: GetCandidatesWithMaxCountAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextPredictionGenerator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI_Text_Core", feature = "implement_exclusive"))]
pub trait ITextPredictionGenerator2_Impl: Sized {
    fn GetCandidatesWithParametersAsync(&mut self, input: &::windows::core::HSTRING, maxcandidates: u32, predictionoptions: TextPredictionOptions, previousstrings: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>;
    fn GetNextWordCandidatesAsync(&mut self, maxcandidates: u32, previousstrings: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>;
    fn InputScope(&mut self) -> ::windows::core::Result<super::super::UI::Text::Core::CoreTextInputScope>;
    fn SetInputScope(&mut self, value: super::super::UI::Text::Core::CoreTextInputScope) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI_Text_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITextPredictionGenerator2 {
    const NAME: &'static str = "Windows.Data.Text.ITextPredictionGenerator2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI_Text_Core", feature = "implement_exclusive"))]
impl ITextPredictionGenerator2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextPredictionGenerator2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextPredictionGenerator2_Vtbl {
        unsafe extern "system" fn GetCandidatesWithParametersAsync<Impl: ITextPredictionGenerator2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, maxcandidates: u32, predictionoptions: TextPredictionOptions, previousstrings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetNextWordCandidatesAsync<Impl: ITextPredictionGenerator2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxcandidates: u32, previousstrings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn InputScope<Impl: ITextPredictionGenerator2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Text::Core::CoreTextInputScope) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetInputScope<Impl: ITextPredictionGenerator2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::UI::Text::Core::CoreTextInputScope) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInputScope(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITextPredictionGenerator2, BASE_OFFSET>(),
            GetCandidatesWithParametersAsync: GetCandidatesWithParametersAsync::<Impl, IMPL_OFFSET>,
            GetNextWordCandidatesAsync: GetNextWordCandidatesAsync::<Impl, IMPL_OFFSET>,
            InputScope: InputScope::<Impl, IMPL_OFFSET>,
            SetInputScope: SetInputScope::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextPredictionGenerator2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextPredictionGeneratorFactory_Impl: Sized {
    fn Create(&mut self, languagetag: &::windows::core::HSTRING) -> ::windows::core::Result<TextPredictionGenerator>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITextPredictionGeneratorFactory {
    const NAME: &'static str = "Windows.Data.Text.ITextPredictionGeneratorFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ITextPredictionGeneratorFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextPredictionGeneratorFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextPredictionGeneratorFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: ITextPredictionGeneratorFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languagetag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITextPredictionGeneratorFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextPredictionGeneratorFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ITextReverseConversionGenerator_Impl: Sized {
    fn ResolvedLanguage(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LanguageAvailableButNotInstalled(&mut self) -> ::windows::core::Result<bool>;
    fn ConvertBackAsync(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITextReverseConversionGenerator {
    const NAME: &'static str = "Windows.Data.Text.ITextReverseConversionGenerator";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ITextReverseConversionGenerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextReverseConversionGenerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextReverseConversionGenerator_Vtbl {
        unsafe extern "system" fn ResolvedLanguage<Impl: ITextReverseConversionGenerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LanguageAvailableButNotInstalled<Impl: ITextReverseConversionGenerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ConvertBackAsync<Impl: ITextReverseConversionGenerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITextReverseConversionGenerator, BASE_OFFSET>(),
            ResolvedLanguage: ResolvedLanguage::<Impl, IMPL_OFFSET>,
            LanguageAvailableButNotInstalled: LanguageAvailableButNotInstalled::<Impl, IMPL_OFFSET>,
            ConvertBackAsync: ConvertBackAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextReverseConversionGenerator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ITextReverseConversionGenerator2_Impl: Sized {
    fn GetPhonemesAsync(&mut self, input: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<TextPhoneme>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITextReverseConversionGenerator2 {
    const NAME: &'static str = "Windows.Data.Text.ITextReverseConversionGenerator2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ITextReverseConversionGenerator2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextReverseConversionGenerator2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextReverseConversionGenerator2_Vtbl {
        unsafe extern "system" fn GetPhonemesAsync<Impl: ITextReverseConversionGenerator2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITextReverseConversionGenerator2, BASE_OFFSET>(),
            GetPhonemesAsync: GetPhonemesAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextReverseConversionGenerator2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextReverseConversionGeneratorFactory_Impl: Sized {
    fn Create(&mut self, languagetag: &::windows::core::HSTRING) -> ::windows::core::Result<TextReverseConversionGenerator>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITextReverseConversionGeneratorFactory {
    const NAME: &'static str = "Windows.Data.Text.ITextReverseConversionGeneratorFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ITextReverseConversionGeneratorFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextReverseConversionGeneratorFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextReverseConversionGeneratorFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: ITextReverseConversionGeneratorFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languagetag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITextReverseConversionGeneratorFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextReverseConversionGeneratorFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUnicodeCharactersStatics_Impl: Sized {
    fn GetCodepointFromSurrogatePair(&mut self, highsurrogate: u32, lowsurrogate: u32) -> ::windows::core::Result<u32>;
    fn GetSurrogatePairFromCodepoint(&mut self, codepoint: u32, highsurrogate: &mut u16, lowsurrogate: &mut u16) -> ::windows::core::Result<()>;
    fn IsHighSurrogate(&mut self, codepoint: u32) -> ::windows::core::Result<bool>;
    fn IsLowSurrogate(&mut self, codepoint: u32) -> ::windows::core::Result<bool>;
    fn IsSupplementary(&mut self, codepoint: u32) -> ::windows::core::Result<bool>;
    fn IsNoncharacter(&mut self, codepoint: u32) -> ::windows::core::Result<bool>;
    fn IsWhitespace(&mut self, codepoint: u32) -> ::windows::core::Result<bool>;
    fn IsAlphabetic(&mut self, codepoint: u32) -> ::windows::core::Result<bool>;
    fn IsCased(&mut self, codepoint: u32) -> ::windows::core::Result<bool>;
    fn IsUppercase(&mut self, codepoint: u32) -> ::windows::core::Result<bool>;
    fn IsLowercase(&mut self, codepoint: u32) -> ::windows::core::Result<bool>;
    fn IsIdStart(&mut self, codepoint: u32) -> ::windows::core::Result<bool>;
    fn IsIdContinue(&mut self, codepoint: u32) -> ::windows::core::Result<bool>;
    fn IsGraphemeBase(&mut self, codepoint: u32) -> ::windows::core::Result<bool>;
    fn IsGraphemeExtend(&mut self, codepoint: u32) -> ::windows::core::Result<bool>;
    fn GetNumericType(&mut self, codepoint: u32) -> ::windows::core::Result<UnicodeNumericType>;
    fn GetGeneralCategory(&mut self, codepoint: u32) -> ::windows::core::Result<UnicodeGeneralCategory>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUnicodeCharactersStatics {
    const NAME: &'static str = "Windows.Data.Text.IUnicodeCharactersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IUnicodeCharactersStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUnicodeCharactersStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUnicodeCharactersStatics_Vtbl {
        unsafe extern "system" fn GetCodepointFromSurrogatePair<Impl: IUnicodeCharactersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, highsurrogate: u32, lowsurrogate: u32, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetSurrogatePairFromCodepoint<Impl: IUnicodeCharactersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, codepoint: u32, highsurrogate: *mut u16, lowsurrogate: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSurrogatePairFromCodepoint(codepoint, ::core::mem::transmute_copy(&highsurrogate), ::core::mem::transmute_copy(&lowsurrogate)).into()
        }
        unsafe extern "system" fn IsHighSurrogate<Impl: IUnicodeCharactersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsLowSurrogate<Impl: IUnicodeCharactersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsSupplementary<Impl: IUnicodeCharactersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsNoncharacter<Impl: IUnicodeCharactersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsWhitespace<Impl: IUnicodeCharactersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsAlphabetic<Impl: IUnicodeCharactersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsCased<Impl: IUnicodeCharactersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsUppercase<Impl: IUnicodeCharactersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsLowercase<Impl: IUnicodeCharactersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsIdStart<Impl: IUnicodeCharactersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsIdContinue<Impl: IUnicodeCharactersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsGraphemeBase<Impl: IUnicodeCharactersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsGraphemeExtend<Impl: IUnicodeCharactersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetNumericType<Impl: IUnicodeCharactersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut UnicodeNumericType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetGeneralCategory<Impl: IUnicodeCharactersStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut UnicodeGeneralCategory) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUnicodeCharactersStatics, BASE_OFFSET>(),
            GetCodepointFromSurrogatePair: GetCodepointFromSurrogatePair::<Impl, IMPL_OFFSET>,
            GetSurrogatePairFromCodepoint: GetSurrogatePairFromCodepoint::<Impl, IMPL_OFFSET>,
            IsHighSurrogate: IsHighSurrogate::<Impl, IMPL_OFFSET>,
            IsLowSurrogate: IsLowSurrogate::<Impl, IMPL_OFFSET>,
            IsSupplementary: IsSupplementary::<Impl, IMPL_OFFSET>,
            IsNoncharacter: IsNoncharacter::<Impl, IMPL_OFFSET>,
            IsWhitespace: IsWhitespace::<Impl, IMPL_OFFSET>,
            IsAlphabetic: IsAlphabetic::<Impl, IMPL_OFFSET>,
            IsCased: IsCased::<Impl, IMPL_OFFSET>,
            IsUppercase: IsUppercase::<Impl, IMPL_OFFSET>,
            IsLowercase: IsLowercase::<Impl, IMPL_OFFSET>,
            IsIdStart: IsIdStart::<Impl, IMPL_OFFSET>,
            IsIdContinue: IsIdContinue::<Impl, IMPL_OFFSET>,
            IsGraphemeBase: IsGraphemeBase::<Impl, IMPL_OFFSET>,
            IsGraphemeExtend: IsGraphemeExtend::<Impl, IMPL_OFFSET>,
            GetNumericType: GetNumericType::<Impl, IMPL_OFFSET>,
            GetGeneralCategory: GetGeneralCategory::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUnicodeCharactersStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IWordSegment_Impl: Sized {
    fn Text(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SourceTextSegment(&mut self) -> ::windows::core::Result<TextSegment>;
    fn AlternateForms(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AlternateWordForm>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWordSegment {
    const NAME: &'static str = "Windows.Data.Text.IWordSegment";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IWordSegment_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWordSegment_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWordSegment_Vtbl {
        unsafe extern "system" fn Text<Impl: IWordSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SourceTextSegment<Impl: IWordSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TextSegment) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AlternateForms<Impl: IWordSegment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWordSegment, BASE_OFFSET>(),
            Text: Text::<Impl, IMPL_OFFSET>,
            SourceTextSegment: SourceTextSegment::<Impl, IMPL_OFFSET>,
            AlternateForms: AlternateForms::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWordSegment as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IWordsSegmenter_Impl: Sized {
    fn ResolvedLanguage(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetTokenAt(&mut self, text: &::windows::core::HSTRING, startindex: u32) -> ::windows::core::Result<WordSegment>;
    fn GetTokens(&mut self, text: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<WordSegment>>;
    fn Tokenize(&mut self, text: &::windows::core::HSTRING, startindex: u32, handler: &::core::option::Option<WordSegmentsTokenizingHandler>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWordsSegmenter {
    const NAME: &'static str = "Windows.Data.Text.IWordsSegmenter";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IWordsSegmenter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWordsSegmenter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWordsSegmenter_Vtbl {
        unsafe extern "system" fn ResolvedLanguage<Impl: IWordsSegmenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetTokenAt<Impl: IWordsSegmenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, startindex: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetTokens<Impl: IWordsSegmenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Tokenize<Impl: IWordsSegmenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, startindex: u32, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Tokenize(&*(&text as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), startindex, &*(&handler as *const <WordSegmentsTokenizingHandler as ::windows::core::Abi>::Abi as *const <WordSegmentsTokenizingHandler as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWordsSegmenter, BASE_OFFSET>(),
            ResolvedLanguage: ResolvedLanguage::<Impl, IMPL_OFFSET>,
            GetTokenAt: GetTokenAt::<Impl, IMPL_OFFSET>,
            GetTokens: GetTokens::<Impl, IMPL_OFFSET>,
            Tokenize: Tokenize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWordsSegmenter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWordsSegmenterFactory_Impl: Sized {
    fn CreateWithLanguage(&mut self, language: &::windows::core::HSTRING) -> ::windows::core::Result<WordsSegmenter>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWordsSegmenterFactory {
    const NAME: &'static str = "Windows.Data.Text.IWordsSegmenterFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IWordsSegmenterFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWordsSegmenterFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWordsSegmenterFactory_Vtbl {
        unsafe extern "system" fn CreateWithLanguage<Impl: IWordsSegmenterFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, language: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWordsSegmenterFactory, BASE_OFFSET>(),
            CreateWithLanguage: CreateWithLanguage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWordsSegmenterFactory as ::windows::core::Interface>::IID
    }
}
