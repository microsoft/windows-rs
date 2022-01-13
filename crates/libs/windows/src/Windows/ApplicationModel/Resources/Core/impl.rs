#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait INamedResourceImpl: Sized {
    fn Uri(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn Candidates(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<ResourceCandidate>>;
    fn Resolve(&mut self) -> ::windows::core::Result<ResourceCandidate>;
    fn ResolveForContext(&mut self, resourcecontext: &::core::option::Option<ResourceContext>) -> ::windows::core::Result<ResourceCandidate>;
    fn ResolveAll(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<ResourceCandidate>>;
    fn ResolveAllForContext(&mut self, resourcecontext: &::core::option::Option<ResourceContext>) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<ResourceCandidate>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for INamedResource {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.INamedResource";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl INamedResourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INamedResourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INamedResourceVtbl {
        unsafe extern "system" fn Uri<Impl: INamedResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Uri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Candidates<Impl: INamedResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Candidates() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resolve<Impl: INamedResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Resolve() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResolveForContext<Impl: INamedResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcecontext: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResolveForContext(&*(&resourcecontext as *const <ResourceContext as ::windows::core::Abi>::Abi as *const <ResourceContext as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResolveAll<Impl: INamedResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResolveAll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResolveAllForContext<Impl: INamedResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcecontext: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResolveAllForContext(&*(&resourcecontext as *const <ResourceContext as ::windows::core::Abi>::Abi as *const <ResourceContext as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INamedResource, BASE_OFFSET>(),
            Uri: Uri::<Impl, IMPL_OFFSET>,
            Candidates: Candidates::<Impl, IMPL_OFFSET>,
            Resolve: Resolve::<Impl, IMPL_OFFSET>,
            ResolveForContext: ResolveForContext::<Impl, IMPL_OFFSET>,
            ResolveAll: ResolveAll::<Impl, IMPL_OFFSET>,
            ResolveAllForContext: ResolveAllForContext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INamedResource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage", feature = "implement_exclusive"))]
pub trait IResourceCandidateImpl: Sized {
    fn Qualifiers(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<ResourceQualifier>>;
    fn IsMatch(&mut self) -> ::windows::core::Result<bool>;
    fn IsMatchAsDefault(&mut self) -> ::windows::core::Result<bool>;
    fn IsDefault(&mut self) -> ::windows::core::Result<bool>;
    fn ValueAsString(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetValueAsFileAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::StorageFile>>;
    fn GetQualifierValue(&mut self, qualifiername: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IResourceCandidate {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.IResourceCandidate";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage", feature = "implement_exclusive"))]
impl IResourceCandidateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceCandidateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceCandidateVtbl {
        unsafe extern "system" fn Qualifiers<Impl: IResourceCandidateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Qualifiers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMatch<Impl: IResourceCandidateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMatch() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMatchAsDefault<Impl: IResourceCandidateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMatchAsDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDefault<Impl: IResourceCandidateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValueAsString<Impl: IResourceCandidateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValueAsString() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValueAsFileAsync<Impl: IResourceCandidateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetValueAsFileAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetQualifierValue<Impl: IResourceCandidateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, qualifiername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetQualifierValue(&*(&qualifiername as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IResourceCandidate, BASE_OFFSET>(),
            Qualifiers: Qualifiers::<Impl, IMPL_OFFSET>,
            IsMatch: IsMatch::<Impl, IMPL_OFFSET>,
            IsMatchAsDefault: IsMatchAsDefault::<Impl, IMPL_OFFSET>,
            IsDefault: IsDefault::<Impl, IMPL_OFFSET>,
            ValueAsString: ValueAsString::<Impl, IMPL_OFFSET>,
            GetValueAsFileAsync: GetValueAsFileAsync::<Impl, IMPL_OFFSET>,
            GetQualifierValue: GetQualifierValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResourceCandidate as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IResourceCandidate2Impl: Sized {
    fn GetValueAsStreamAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IRandomAccessStream>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IResourceCandidate2 {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.IResourceCandidate2";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IResourceCandidate2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceCandidate2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceCandidate2Vtbl {
        unsafe extern "system" fn GetValueAsStreamAsync<Impl: IResourceCandidate2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetValueAsStreamAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IResourceCandidate2, BASE_OFFSET>(),
            GetValueAsStreamAsync: GetValueAsStreamAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResourceCandidate2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceCandidate3Impl: Sized {
    fn Kind(&mut self) -> ::windows::core::Result<ResourceCandidateKind>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IResourceCandidate3 {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.IResourceCandidate3";
}
#[cfg(feature = "implement_exclusive")]
impl IResourceCandidate3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceCandidate3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceCandidate3Vtbl {
        unsafe extern "system" fn Kind<Impl: IResourceCandidate3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ResourceCandidateKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IResourceCandidate3, BASE_OFFSET>(), Kind: Kind::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResourceCandidate3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IResourceContextImpl: Sized {
    fn QualifierValues(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IObservableMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn ResetQualifierValues(&mut self, qualifiernames: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<()>;
    fn OverrideToMatch(&mut self, result: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<ResourceQualifier>>) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<ResourceContext>;
    fn Languages(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn SetLanguages(&mut self, languages: &::core::option::Option<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IResourceContext {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.IResourceContext";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IResourceContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceContextVtbl {
        unsafe extern "system" fn QualifierValues<Impl: IResourceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QualifierValues() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IResourceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn ResetQualifierValues<Impl: IResourceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, qualifiernames: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResetQualifierValues(&*(&qualifiernames as *const <super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OverrideToMatch<Impl: IResourceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OverrideToMatch(&*(&result as *const <super::super::super::Foundation::Collections::IIterable<ResourceQualifier> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<ResourceQualifier> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Clone<Impl: IResourceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Languages<Impl: IResourceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Languages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguages<Impl: IResourceContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languages: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLanguages(&*(&languages as *const <super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IResourceContext, BASE_OFFSET>(),
            QualifierValues: QualifierValues::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            ResetQualifierValues: ResetQualifierValues::<Impl, IMPL_OFFSET>,
            OverrideToMatch: OverrideToMatch::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
            Languages: Languages::<Impl, IMPL_OFFSET>,
            SetLanguages: SetLanguages::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResourceContext as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IResourceContextStaticsImpl: Sized {
    fn CreateMatchingContext(&mut self, result: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<ResourceQualifier>>) -> ::windows::core::Result<ResourceContext>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IResourceContextStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.IResourceContextStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IResourceContextStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceContextStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceContextStaticsVtbl {
        unsafe extern "system" fn CreateMatchingContext<Impl: IResourceContextStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMatchingContext(&*(&result as *const <super::super::super::Foundation::Collections::IIterable<ResourceQualifier> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<ResourceQualifier> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IResourceContextStatics, BASE_OFFSET>(),
            CreateMatchingContext: CreateMatchingContext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResourceContextStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IResourceContextStatics2Impl: Sized {
    fn GetForCurrentView(&mut self) -> ::windows::core::Result<ResourceContext>;
    fn SetGlobalQualifierValue(&mut self, key: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ResetGlobalQualifierValues(&mut self) -> ::windows::core::Result<()>;
    fn ResetGlobalQualifierValuesForSpecifiedQualifiers(&mut self, qualifiernames: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<()>;
    fn GetForViewIndependentUse(&mut self) -> ::windows::core::Result<ResourceContext>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IResourceContextStatics2 {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.IResourceContextStatics2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IResourceContextStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceContextStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceContextStatics2Vtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IResourceContextStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGlobalQualifierValue<Impl: IResourceContextStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGlobalQualifierValue(&*(&key as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ResetGlobalQualifierValues<Impl: IResourceContextStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResetGlobalQualifierValues().into()
        }
        unsafe extern "system" fn ResetGlobalQualifierValuesForSpecifiedQualifiers<Impl: IResourceContextStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, qualifiernames: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResetGlobalQualifierValuesForSpecifiedQualifiers(&*(&qualifiernames as *const <super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetForViewIndependentUse<Impl: IResourceContextStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForViewIndependentUse() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IResourceContextStatics2, BASE_OFFSET>(),
            GetForCurrentView: GetForCurrentView::<Impl, IMPL_OFFSET>,
            SetGlobalQualifierValue: SetGlobalQualifierValue::<Impl, IMPL_OFFSET>,
            ResetGlobalQualifierValues: ResetGlobalQualifierValues::<Impl, IMPL_OFFSET>,
            ResetGlobalQualifierValuesForSpecifiedQualifiers: ResetGlobalQualifierValuesForSpecifiedQualifiers::<Impl, IMPL_OFFSET>,
            GetForViewIndependentUse: GetForViewIndependentUse::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResourceContextStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceContextStatics3Impl: Sized {
    fn SetGlobalQualifierValueWithPersistence(&mut self, key: &::windows::core::HSTRING, value: &::windows::core::HSTRING, persistence: ResourceQualifierPersistence) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IResourceContextStatics3 {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.IResourceContextStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IResourceContextStatics3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceContextStatics3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceContextStatics3Vtbl {
        unsafe extern "system" fn SetGlobalQualifierValueWithPersistence<Impl: IResourceContextStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, persistence: ResourceQualifierPersistence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGlobalQualifierValueWithPersistence(&*(&key as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), persistence).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IResourceContextStatics3, BASE_OFFSET>(),
            SetGlobalQualifierValueWithPersistence: SetGlobalQualifierValueWithPersistence::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResourceContextStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI", feature = "implement_exclusive"))]
pub trait IResourceContextStatics4Impl: Sized {
    fn GetForUIContext(&mut self, context: &::core::option::Option<super::super::super::UI::UIContext>) -> ::windows::core::Result<ResourceContext>;
}
#[cfg(all(feature = "UI", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IResourceContextStatics4 {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.IResourceContextStatics4";
}
#[cfg(all(feature = "UI", feature = "implement_exclusive"))]
impl IResourceContextStatics4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceContextStatics4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceContextStatics4Vtbl {
        unsafe extern "system" fn GetForUIContext<Impl: IResourceContextStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForUIContext(&*(&context as *const <super::super::super::UI::UIContext as ::windows::core::Abi>::Abi as *const <super::super::super::UI::UIContext as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IResourceContextStatics4, BASE_OFFSET>(),
            GetForUIContext: GetForUIContext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResourceContextStatics4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "implement_exclusive"))]
pub trait IResourceManagerImpl: Sized {
    fn MainResourceMap(&mut self) -> ::windows::core::Result<ResourceMap>;
    fn AllResourceMaps(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ResourceMap>>;
    fn DefaultContext(&mut self) -> ::windows::core::Result<ResourceContext>;
    fn LoadPriFiles(&mut self, files: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<super::super::super::Storage::IStorageFile>>) -> ::windows::core::Result<()>;
    fn UnloadPriFiles(&mut self, files: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<super::super::super::Storage::IStorageFile>>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IResourceManager {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.IResourceManager";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "implement_exclusive"))]
impl IResourceManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceManagerVtbl {
        unsafe extern "system" fn MainResourceMap<Impl: IResourceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MainResourceMap() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllResourceMaps<Impl: IResourceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllResourceMaps() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultContext<Impl: IResourceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultContext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadPriFiles<Impl: IResourceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, files: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadPriFiles(&*(&files as *const <super::super::super::Foundation::Collections::IIterable<super::super::super::Storage::IStorageFile> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<super::super::super::Storage::IStorageFile> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UnloadPriFiles<Impl: IResourceManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, files: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnloadPriFiles(&*(&files as *const <super::super::super::Foundation::Collections::IIterable<super::super::super::Storage::IStorageFile> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<super::super::super::Storage::IStorageFile> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IResourceManager, BASE_OFFSET>(),
            MainResourceMap: MainResourceMap::<Impl, IMPL_OFFSET>,
            AllResourceMaps: AllResourceMaps::<Impl, IMPL_OFFSET>,
            DefaultContext: DefaultContext::<Impl, IMPL_OFFSET>,
            LoadPriFiles: LoadPriFiles::<Impl, IMPL_OFFSET>,
            UnloadPriFiles: UnloadPriFiles::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResourceManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IResourceManager2Impl: Sized {
    fn GetAllNamedResourcesForPackage(&mut self, packagename: &::windows::core::HSTRING, resourcelayoutinfo: &ResourceLayoutInfo) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<NamedResource>>;
    fn GetAllSubtreesForPackage(&mut self, packagename: &::windows::core::HSTRING, resourcelayoutinfo: &ResourceLayoutInfo) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<ResourceMap>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IResourceManager2 {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.IResourceManager2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IResourceManager2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceManager2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceManager2Vtbl {
        unsafe extern "system" fn GetAllNamedResourcesForPackage<Impl: IResourceManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, resourcelayoutinfo: ResourceLayoutInfo, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllNamedResourcesForPackage(&*(&packagename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&resourcelayoutinfo as *const <ResourceLayoutInfo as ::windows::core::Abi>::Abi as *const <ResourceLayoutInfo as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllSubtreesForPackage<Impl: IResourceManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, resourcelayoutinfo: ResourceLayoutInfo, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllSubtreesForPackage(&*(&packagename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&resourcelayoutinfo as *const <ResourceLayoutInfo as ::windows::core::Abi>::Abi as *const <ResourceLayoutInfo as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IResourceManager2, BASE_OFFSET>(),
            GetAllNamedResourcesForPackage: GetAllNamedResourcesForPackage::<Impl, IMPL_OFFSET>,
            GetAllSubtreesForPackage: GetAllSubtreesForPackage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResourceManager2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceManagerStaticsImpl: Sized {
    fn Current(&mut self) -> ::windows::core::Result<ResourceManager>;
    fn IsResourceReference(&mut self, resourcereference: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IResourceManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.IResourceManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IResourceManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceManagerStaticsVtbl {
        unsafe extern "system" fn Current<Impl: IResourceManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Current() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsResourceReference<Impl: IResourceManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcereference: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsResourceReference(&*(&resourcereference as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IResourceManagerStatics, BASE_OFFSET>(),
            Current: Current::<Impl, IMPL_OFFSET>,
            IsResourceReference: IsResourceReference::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResourceManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IResourceMapImpl: Sized + IIterableImpl<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, NamedResource>> + IMapViewImpl<::windows::core::HSTRING, NamedResource> {
    fn Uri(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn GetValue(&mut self, resource: &::windows::core::HSTRING) -> ::windows::core::Result<ResourceCandidate>;
    fn GetValueForContext(&mut self, resource: &::windows::core::HSTRING, context: &::core::option::Option<ResourceContext>) -> ::windows::core::Result<ResourceCandidate>;
    fn GetSubtree(&mut self, reference: &::windows::core::HSTRING) -> ::windows::core::Result<ResourceMap>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IResourceMap {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.IResourceMap";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IResourceMapVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceMapImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceMapVtbl {
        unsafe extern "system" fn Uri<Impl: IResourceMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Uri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Impl: IResourceMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resource: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetValue(&*(&resource as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValueForContext<Impl: IResourceMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resource: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, context: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetValueForContext(&*(&resource as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&context as *const <ResourceContext as ::windows::core::Abi>::Abi as *const <ResourceContext as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubtree<Impl: IResourceMapImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reference: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSubtree(&*(&reference as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IResourceMap, BASE_OFFSET>(),
            Uri: Uri::<Impl, IMPL_OFFSET>,
            GetValue: GetValue::<Impl, IMPL_OFFSET>,
            GetValueForContext: GetValueForContext::<Impl, IMPL_OFFSET>,
            GetSubtree: GetSubtree::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResourceMap as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceQualifierImpl: Sized {
    fn QualifierName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn QualifierValue(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsDefault(&mut self) -> ::windows::core::Result<bool>;
    fn IsMatch(&mut self) -> ::windows::core::Result<bool>;
    fn Score(&mut self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IResourceQualifier {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.IResourceQualifier";
}
#[cfg(feature = "implement_exclusive")]
impl IResourceQualifierVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceQualifierImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceQualifierVtbl {
        unsafe extern "system" fn QualifierName<Impl: IResourceQualifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QualifierName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QualifierValue<Impl: IResourceQualifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QualifierValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDefault<Impl: IResourceQualifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMatch<Impl: IResourceQualifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMatch() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Score<Impl: IResourceQualifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Score() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IResourceQualifier, BASE_OFFSET>(),
            QualifierName: QualifierName::<Impl, IMPL_OFFSET>,
            QualifierValue: QualifierValue::<Impl, IMPL_OFFSET>,
            IsDefault: IsDefault::<Impl, IMPL_OFFSET>,
            IsMatch: IsMatch::<Impl, IMPL_OFFSET>,
            Score: Score::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResourceQualifier as ::windows::core::Interface>::IID
    }
}
