#[cfg(feature = "implement_exclusive")]
pub trait INamedResourceImpl: Sized {
    fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn Candidates(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<ResourceCandidate>>;
    fn Resolve(&self) -> ::windows::core::Result<ResourceCandidate>;
    fn ResolveForContext(&self, resourcecontext: &::core::option::Option<ResourceContext>) -> ::windows::core::Result<ResourceCandidate>;
    fn ResolveAll(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<ResourceCandidate>>;
    fn ResolveAllForContext(&self, resourcecontext: &::core::option::Option<ResourceContext>) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<ResourceCandidate>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INamedResource {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.INamedResource";
}
#[cfg(feature = "implement_exclusive")]
impl INamedResourceVtbl {
    pub const fn new<Impl: INamedResourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INamedResourceVtbl {
        unsafe extern "system" fn Uri<Impl: INamedResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Uri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Candidates<Impl: INamedResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Candidates() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resolve<Impl: INamedResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Resolve() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResolveForContext<Impl: INamedResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourcecontext: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ResolveForContext(&*(&resourcecontext as *const <ResourceContext as ::windows::core::Abi>::Abi as *const <ResourceContext as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResolveAll<Impl: INamedResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ResolveAll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResolveAllForContext<Impl: INamedResourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourcecontext: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ResolveAllForContext(&*(&resourcecontext as *const <ResourceContext as ::windows::core::Abi>::Abi as *const <ResourceContext as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INamedResource>, base.5, Uri::<Impl, OFFSET>, Candidates::<Impl, OFFSET>, Resolve::<Impl, OFFSET>, ResolveForContext::<Impl, OFFSET>, ResolveAll::<Impl, OFFSET>, ResolveAllForContext::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceCandidateImpl: Sized {
    fn Qualifiers(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<ResourceQualifier>>;
    fn IsMatch(&self) -> ::windows::core::Result<bool>;
    fn IsMatchAsDefault(&self) -> ::windows::core::Result<bool>;
    fn IsDefault(&self) -> ::windows::core::Result<bool>;
    fn ValueAsString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetValueAsFileAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::StorageFile>>;
    fn GetQualifierValue(&self, qualifiername: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IResourceCandidate {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.IResourceCandidate";
}
#[cfg(feature = "implement_exclusive")]
impl IResourceCandidateVtbl {
    pub const fn new<Impl: IResourceCandidateImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IResourceCandidateVtbl {
        unsafe extern "system" fn Qualifiers<Impl: IResourceCandidateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Qualifiers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMatch<Impl: IResourceCandidateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsMatch() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMatchAsDefault<Impl: IResourceCandidateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsMatchAsDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDefault<Impl: IResourceCandidateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValueAsString<Impl: IResourceCandidateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ValueAsString() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValueAsFileAsync<Impl: IResourceCandidateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetValueAsFileAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetQualifierValue<Impl: IResourceCandidateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, qualifiername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetQualifierValue(&*(&qualifiername as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IResourceCandidate>, base.5, Qualifiers::<Impl, OFFSET>, IsMatch::<Impl, OFFSET>, IsMatchAsDefault::<Impl, OFFSET>, IsDefault::<Impl, OFFSET>, ValueAsString::<Impl, OFFSET>, GetValueAsFileAsync::<Impl, OFFSET>, GetQualifierValue::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceCandidate2Impl: Sized {
    fn GetValueAsStreamAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IRandomAccessStream>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IResourceCandidate2 {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.IResourceCandidate2";
}
#[cfg(feature = "implement_exclusive")]
impl IResourceCandidate2Vtbl {
    pub const fn new<Impl: IResourceCandidate2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IResourceCandidate2Vtbl {
        unsafe extern "system" fn GetValueAsStreamAsync<Impl: IResourceCandidate2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetValueAsStreamAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IResourceCandidate2>, base.5, GetValueAsStreamAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceCandidate3Impl: Sized {
    fn Kind(&self) -> ::windows::core::Result<ResourceCandidateKind>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IResourceCandidate3 {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.IResourceCandidate3";
}
#[cfg(feature = "implement_exclusive")]
impl IResourceCandidate3Vtbl {
    pub const fn new<Impl: IResourceCandidate3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IResourceCandidate3Vtbl {
        unsafe extern "system" fn Kind<Impl: IResourceCandidate3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ResourceCandidateKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IResourceCandidate3>, base.5, Kind::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceContextImpl: Sized {
    fn QualifierValues(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IObservableMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn ResetQualifierValues(&self, qualifiernames: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<()>;
    fn OverrideToMatch(&self, result: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<ResourceQualifier>>) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<ResourceContext>;
    fn Languages(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn SetLanguages(&self, languages: &::core::option::Option<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IResourceContext {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.IResourceContext";
}
#[cfg(feature = "implement_exclusive")]
impl IResourceContextVtbl {
    pub const fn new<Impl: IResourceContextImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IResourceContextVtbl {
        unsafe extern "system" fn QualifierValues<Impl: IResourceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QualifierValues() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IResourceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn ResetQualifierValues<Impl: IResourceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, qualifiernames: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ResetQualifierValues(&*(&qualifiernames as *const <super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OverrideToMatch<Impl: IResourceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).OverrideToMatch(&*(&result as *const <super::super::super::Foundation::Collections::IIterable<ResourceQualifier> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<ResourceQualifier> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Clone<Impl: IResourceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Languages<Impl: IResourceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Languages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguages<Impl: IResourceContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, languages: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLanguages(&*(&languages as *const <super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IResourceContext>, base.5, QualifierValues::<Impl, OFFSET>, Reset::<Impl, OFFSET>, ResetQualifierValues::<Impl, OFFSET>, OverrideToMatch::<Impl, OFFSET>, Clone::<Impl, OFFSET>, Languages::<Impl, OFFSET>, SetLanguages::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceContextStaticsImpl: Sized {
    fn CreateMatchingContext(&self, result: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<ResourceQualifier>>) -> ::windows::core::Result<ResourceContext>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IResourceContextStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.IResourceContextStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IResourceContextStaticsVtbl {
    pub const fn new<Impl: IResourceContextStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IResourceContextStaticsVtbl {
        unsafe extern "system" fn CreateMatchingContext<Impl: IResourceContextStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateMatchingContext(&*(&result as *const <super::super::super::Foundation::Collections::IIterable<ResourceQualifier> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<ResourceQualifier> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IResourceContextStatics>, base.5, CreateMatchingContext::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceContextStatics2Impl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<ResourceContext>;
    fn SetGlobalQualifierValue(&self, key: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ResetGlobalQualifierValues(&self) -> ::windows::core::Result<()>;
    fn ResetGlobalQualifierValuesForSpecifiedQualifiers(&self, qualifiernames: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<()>;
    fn GetForViewIndependentUse(&self) -> ::windows::core::Result<ResourceContext>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IResourceContextStatics2 {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.IResourceContextStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IResourceContextStatics2Vtbl {
    pub const fn new<Impl: IResourceContextStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IResourceContextStatics2Vtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IResourceContextStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGlobalQualifierValue<Impl: IResourceContextStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetGlobalQualifierValue(&*(&key as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ResetGlobalQualifierValues<Impl: IResourceContextStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ResetGlobalQualifierValues().into()
        }
        unsafe extern "system" fn ResetGlobalQualifierValuesForSpecifiedQualifiers<Impl: IResourceContextStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, qualifiernames: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ResetGlobalQualifierValuesForSpecifiedQualifiers(&*(&qualifiernames as *const <super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetForViewIndependentUse<Impl: IResourceContextStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForViewIndependentUse() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IResourceContextStatics2>, base.5, GetForCurrentView::<Impl, OFFSET>, SetGlobalQualifierValue::<Impl, OFFSET>, ResetGlobalQualifierValues::<Impl, OFFSET>, ResetGlobalQualifierValuesForSpecifiedQualifiers::<Impl, OFFSET>, GetForViewIndependentUse::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceContextStatics3Impl: Sized {
    fn SetGlobalQualifierValueWithPersistence(&self, key: &::windows::core::HSTRING, value: &::windows::core::HSTRING, persistence: ResourceQualifierPersistence) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IResourceContextStatics3 {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.IResourceContextStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IResourceContextStatics3Vtbl {
    pub const fn new<Impl: IResourceContextStatics3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IResourceContextStatics3Vtbl {
        unsafe extern "system" fn SetGlobalQualifierValueWithPersistence<Impl: IResourceContextStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, persistence: ResourceQualifierPersistence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetGlobalQualifierValueWithPersistence(&*(&key as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), persistence).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IResourceContextStatics3>, base.5, SetGlobalQualifierValueWithPersistence::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceContextStatics4Impl: Sized {
    fn GetForUIContext(&self, context: &::core::option::Option<super::super::super::UI::UIContext>) -> ::windows::core::Result<ResourceContext>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IResourceContextStatics4 {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.IResourceContextStatics4";
}
#[cfg(feature = "implement_exclusive")]
impl IResourceContextStatics4Vtbl {
    pub const fn new<Impl: IResourceContextStatics4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IResourceContextStatics4Vtbl {
        unsafe extern "system" fn GetForUIContext<Impl: IResourceContextStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForUIContext(&*(&context as *const <super::super::super::UI::UIContext as ::windows::core::Abi>::Abi as *const <super::super::super::UI::UIContext as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IResourceContextStatics4>, base.5, GetForUIContext::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceManagerImpl: Sized {
    fn MainResourceMap(&self) -> ::windows::core::Result<ResourceMap>;
    fn AllResourceMaps(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ResourceMap>>;
    fn DefaultContext(&self) -> ::windows::core::Result<ResourceContext>;
    fn LoadPriFiles(&self, files: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<super::super::super::Storage::IStorageFile>>) -> ::windows::core::Result<()>;
    fn UnloadPriFiles(&self, files: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<super::super::super::Storage::IStorageFile>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IResourceManager {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.IResourceManager";
}
#[cfg(feature = "implement_exclusive")]
impl IResourceManagerVtbl {
    pub const fn new<Impl: IResourceManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IResourceManagerVtbl {
        unsafe extern "system" fn MainResourceMap<Impl: IResourceManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MainResourceMap() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllResourceMaps<Impl: IResourceManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllResourceMaps() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultContext<Impl: IResourceManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DefaultContext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadPriFiles<Impl: IResourceManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, files: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).LoadPriFiles(&*(&files as *const <super::super::super::Foundation::Collections::IIterable<super::super::super::Storage::IStorageFile> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<super::super::super::Storage::IStorageFile> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UnloadPriFiles<Impl: IResourceManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, files: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).UnloadPriFiles(&*(&files as *const <super::super::super::Foundation::Collections::IIterable<super::super::super::Storage::IStorageFile> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<super::super::super::Storage::IStorageFile> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IResourceManager>, base.5, MainResourceMap::<Impl, OFFSET>, AllResourceMaps::<Impl, OFFSET>, DefaultContext::<Impl, OFFSET>, LoadPriFiles::<Impl, OFFSET>, UnloadPriFiles::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceManager2Impl: Sized {
    fn GetAllNamedResourcesForPackage(&self, packagename: &::windows::core::HSTRING, resourcelayoutinfo: &ResourceLayoutInfo) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<NamedResource>>;
    fn GetAllSubtreesForPackage(&self, packagename: &::windows::core::HSTRING, resourcelayoutinfo: &ResourceLayoutInfo) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<ResourceMap>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IResourceManager2 {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.IResourceManager2";
}
#[cfg(feature = "implement_exclusive")]
impl IResourceManager2Vtbl {
    pub const fn new<Impl: IResourceManager2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IResourceManager2Vtbl {
        unsafe extern "system" fn GetAllNamedResourcesForPackage<Impl: IResourceManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, packagename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, resourcelayoutinfo: ResourceLayoutInfo, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAllNamedResourcesForPackage(&*(&packagename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&resourcelayoutinfo as *const <ResourceLayoutInfo as ::windows::core::Abi>::Abi as *const <ResourceLayoutInfo as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllSubtreesForPackage<Impl: IResourceManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, packagename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, resourcelayoutinfo: ResourceLayoutInfo, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAllSubtreesForPackage(&*(&packagename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&resourcelayoutinfo as *const <ResourceLayoutInfo as ::windows::core::Abi>::Abi as *const <ResourceLayoutInfo as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IResourceManager2>, base.5, GetAllNamedResourcesForPackage::<Impl, OFFSET>, GetAllSubtreesForPackage::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceManagerStaticsImpl: Sized {
    fn Current(&self) -> ::windows::core::Result<ResourceManager>;
    fn IsResourceReference(&self, resourcereference: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IResourceManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.IResourceManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IResourceManagerStaticsVtbl {
    pub const fn new<Impl: IResourceManagerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IResourceManagerStaticsVtbl {
        unsafe extern "system" fn Current<Impl: IResourceManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Current() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsResourceReference<Impl: IResourceManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourcereference: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsResourceReference(&*(&resourcereference as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IResourceManagerStatics>, base.5, Current::<Impl, OFFSET>, IsResourceReference::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IResourceMapImpl: Sized + IIterableImpl<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, NamedResource>> + IMapViewImpl<::windows::core::HSTRING, NamedResource> {
    fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn GetValue(&self, resource: &::windows::core::HSTRING) -> ::windows::core::Result<ResourceCandidate>;
    fn GetValueForContext(&self, resource: &::windows::core::HSTRING, context: &::core::option::Option<ResourceContext>) -> ::windows::core::Result<ResourceCandidate>;
    fn GetSubtree(&self, reference: &::windows::core::HSTRING) -> ::windows::core::Result<ResourceMap>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IResourceMap {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.IResourceMap";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IResourceMapVtbl {
    pub const fn new<Impl: IResourceMapImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IResourceMapVtbl {
        unsafe extern "system" fn Uri<Impl: IResourceMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Uri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Impl: IResourceMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resource: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetValue(&*(&resource as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValueForContext<Impl: IResourceMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resource: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, context: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetValueForContext(&*(&resource as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&context as *const <ResourceContext as ::windows::core::Abi>::Abi as *const <ResourceContext as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubtree<Impl: IResourceMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reference: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSubtree(&*(&reference as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IResourceMap>, base.5, Uri::<Impl, OFFSET>, GetValue::<Impl, OFFSET>, GetValueForContext::<Impl, OFFSET>, GetSubtree::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceQualifierImpl: Sized {
    fn QualifierName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn QualifierValue(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsDefault(&self) -> ::windows::core::Result<bool>;
    fn IsMatch(&self) -> ::windows::core::Result<bool>;
    fn Score(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IResourceQualifier {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.IResourceQualifier";
}
#[cfg(feature = "implement_exclusive")]
impl IResourceQualifierVtbl {
    pub const fn new<Impl: IResourceQualifierImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IResourceQualifierVtbl {
        unsafe extern "system" fn QualifierName<Impl: IResourceQualifierImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QualifierName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QualifierValue<Impl: IResourceQualifierImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QualifierValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDefault<Impl: IResourceQualifierImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMatch<Impl: IResourceQualifierImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsMatch() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Score<Impl: IResourceQualifierImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Score() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IResourceQualifier>, base.5, QualifierName::<Impl, OFFSET>, QualifierValue::<Impl, OFFSET>, IsDefault::<Impl, OFFSET>, IsMatch::<Impl, OFFSET>, Score::<Impl, OFFSET>)
    }
}
