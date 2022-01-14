#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait INamedResource_Impl: Sized {
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
impl INamedResource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INamedResource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INamedResource_Vtbl {
        unsafe extern "system" fn Uri<Impl: INamedResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Candidates<Impl: INamedResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Resolve<Impl: INamedResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ResolveForContext<Impl: INamedResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcecontext: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ResolveAll<Impl: INamedResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ResolveAllForContext<Impl: INamedResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcecontext: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IResourceCandidate_Impl: Sized {
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
impl IResourceCandidate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceCandidate_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceCandidate_Vtbl {
        unsafe extern "system" fn Qualifiers<Impl: IResourceCandidate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsMatch<Impl: IResourceCandidate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsMatchAsDefault<Impl: IResourceCandidate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsDefault<Impl: IResourceCandidate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ValueAsString<Impl: IResourceCandidate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetValueAsFileAsync<Impl: IResourceCandidate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetQualifierValue<Impl: IResourceCandidate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, qualifiername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
pub trait IResourceCandidate2_Impl: Sized {
    fn GetValueAsStreamAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IRandomAccessStream>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IResourceCandidate2 {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.IResourceCandidate2";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IResourceCandidate2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceCandidate2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceCandidate2_Vtbl {
        unsafe extern "system" fn GetValueAsStreamAsync<Impl: IResourceCandidate2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IResourceCandidate3_Impl: Sized {
    fn Kind(&mut self) -> ::windows::core::Result<ResourceCandidateKind>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IResourceCandidate3 {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.IResourceCandidate3";
}
#[cfg(feature = "implement_exclusive")]
impl IResourceCandidate3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceCandidate3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceCandidate3_Vtbl {
        unsafe extern "system" fn Kind<Impl: IResourceCandidate3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ResourceCandidateKind) -> ::windows::core::HRESULT {
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
pub trait IResourceContext_Impl: Sized {
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
impl IResourceContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceContext_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceContext_Vtbl {
        unsafe extern "system" fn QualifierValues<Impl: IResourceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Reset<Impl: IResourceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn ResetQualifierValues<Impl: IResourceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, qualifiernames: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResetQualifierValues(&*(&qualifiernames as *const <super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OverrideToMatch<Impl: IResourceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OverrideToMatch(&*(&result as *const <super::super::super::Foundation::Collections::IIterable<ResourceQualifier> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<ResourceQualifier> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Clone<Impl: IResourceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Languages<Impl: IResourceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLanguages<Impl: IResourceContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languages: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IResourceContextStatics_Impl: Sized {
    fn CreateMatchingContext(&mut self, result: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<ResourceQualifier>>) -> ::windows::core::Result<ResourceContext>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IResourceContextStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.IResourceContextStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IResourceContextStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceContextStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceContextStatics_Vtbl {
        unsafe extern "system" fn CreateMatchingContext<Impl: IResourceContextStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IResourceContextStatics2_Impl: Sized {
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
impl IResourceContextStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceContextStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceContextStatics2_Vtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IResourceContextStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetGlobalQualifierValue<Impl: IResourceContextStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGlobalQualifierValue(&*(&key as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ResetGlobalQualifierValues<Impl: IResourceContextStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResetGlobalQualifierValues().into()
        }
        unsafe extern "system" fn ResetGlobalQualifierValuesForSpecifiedQualifiers<Impl: IResourceContextStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, qualifiernames: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResetGlobalQualifierValuesForSpecifiedQualifiers(&*(&qualifiernames as *const <super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetForViewIndependentUse<Impl: IResourceContextStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IResourceContextStatics3_Impl: Sized {
    fn SetGlobalQualifierValueWithPersistence(&mut self, key: &::windows::core::HSTRING, value: &::windows::core::HSTRING, persistence: ResourceQualifierPersistence) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IResourceContextStatics3 {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.IResourceContextStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IResourceContextStatics3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceContextStatics3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceContextStatics3_Vtbl {
        unsafe extern "system" fn SetGlobalQualifierValueWithPersistence<Impl: IResourceContextStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, persistence: ResourceQualifierPersistence) -> ::windows::core::HRESULT {
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
pub trait IResourceContextStatics4_Impl: Sized {
    fn GetForUIContext(&mut self, context: &::core::option::Option<super::super::super::UI::UIContext>) -> ::windows::core::Result<ResourceContext>;
}
#[cfg(all(feature = "UI", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IResourceContextStatics4 {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.IResourceContextStatics4";
}
#[cfg(all(feature = "UI", feature = "implement_exclusive"))]
impl IResourceContextStatics4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceContextStatics4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceContextStatics4_Vtbl {
        unsafe extern "system" fn GetForUIContext<Impl: IResourceContextStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IResourceManager_Impl: Sized {
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
impl IResourceManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceManager_Vtbl {
        unsafe extern "system" fn MainResourceMap<Impl: IResourceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AllResourceMaps<Impl: IResourceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DefaultContext<Impl: IResourceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LoadPriFiles<Impl: IResourceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, files: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadPriFiles(&*(&files as *const <super::super::super::Foundation::Collections::IIterable<super::super::super::Storage::IStorageFile> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<super::super::super::Storage::IStorageFile> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UnloadPriFiles<Impl: IResourceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, files: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IResourceManager2_Impl: Sized {
    fn GetAllNamedResourcesForPackage(&mut self, packagename: &::windows::core::HSTRING, resourcelayoutinfo: &ResourceLayoutInfo) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<NamedResource>>;
    fn GetAllSubtreesForPackage(&mut self, packagename: &::windows::core::HSTRING, resourcelayoutinfo: &ResourceLayoutInfo) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<ResourceMap>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IResourceManager2 {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.IResourceManager2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IResourceManager2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceManager2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceManager2_Vtbl {
        unsafe extern "system" fn GetAllNamedResourcesForPackage<Impl: IResourceManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, resourcelayoutinfo: ResourceLayoutInfo, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetAllSubtreesForPackage<Impl: IResourceManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, resourcelayoutinfo: ResourceLayoutInfo, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IResourceManagerStatics_Impl: Sized {
    fn Current(&mut self) -> ::windows::core::Result<ResourceManager>;
    fn IsResourceReference(&mut self, resourcereference: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IResourceManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.IResourceManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IResourceManagerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceManagerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceManagerStatics_Vtbl {
        unsafe extern "system" fn Current<Impl: IResourceManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsResourceReference<Impl: IResourceManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcereference: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
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
pub trait IResourceMap_Impl: Sized + super::super::super::Foundation::Collections::IIterable_Impl<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, NamedResource>> + super::super::super::Foundation::Collections::IMapView_Impl<::windows::core::HSTRING, NamedResource> {
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
impl IResourceMap_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceMap_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceMap_Vtbl {
        unsafe extern "system" fn Uri<Impl: IResourceMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetValue<Impl: IResourceMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resource: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetValueForContext<Impl: IResourceMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resource: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, context: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetSubtree<Impl: IResourceMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reference: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IResourceQualifier_Impl: Sized {
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
impl IResourceQualifier_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceQualifier_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceQualifier_Vtbl {
        unsafe extern "system" fn QualifierName<Impl: IResourceQualifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn QualifierValue<Impl: IResourceQualifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsDefault<Impl: IResourceQualifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsMatch<Impl: IResourceQualifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Score<Impl: IResourceQualifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
