#[cfg(feature = "implement_exclusive")]
pub trait IIndexedResourceCandidateImpl: Sized {
    fn Type(&self) -> ::windows::core::Result<IndexedResourceType>;
    fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn Metadata(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn Qualifiers(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<IndexedResourceQualifier>>;
    fn ValueAsString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetQualifierValue(&self, qualifiername: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IIndexedResourceCandidate {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Management.IIndexedResourceCandidate";
}
#[cfg(feature = "implement_exclusive")]
impl IIndexedResourceCandidateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIndexedResourceCandidateImpl, const OFFSET: isize>() -> IIndexedResourceCandidateVtbl {
        unsafe extern "system" fn Type<Impl: IIndexedResourceCandidateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut IndexedResourceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Uri<Impl: IIndexedResourceCandidateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Metadata<Impl: IIndexedResourceCandidateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Metadata() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Qualifiers<Impl: IIndexedResourceCandidateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ValueAsString<Impl: IIndexedResourceCandidateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetQualifierValue<Impl: IIndexedResourceCandidateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, qualifiername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IIndexedResourceCandidate>, ::windows::core::GetTrustLevel, Type::<Impl, OFFSET>, Uri::<Impl, OFFSET>, Metadata::<Impl, OFFSET>, Qualifiers::<Impl, OFFSET>, ValueAsString::<Impl, OFFSET>, GetQualifierValue::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IIndexedResourceQualifierImpl: Sized {
    fn QualifierName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn QualifierValue(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IIndexedResourceQualifier {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Management.IIndexedResourceQualifier";
}
#[cfg(feature = "implement_exclusive")]
impl IIndexedResourceQualifierVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIndexedResourceQualifierImpl, const OFFSET: isize>() -> IIndexedResourceQualifierVtbl {
        unsafe extern "system" fn QualifierName<Impl: IIndexedResourceQualifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn QualifierValue<Impl: IIndexedResourceQualifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IIndexedResourceQualifier>, ::windows::core::GetTrustLevel, QualifierName::<Impl, OFFSET>, QualifierValue::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IResourceIndexerImpl: Sized {
    fn IndexFilePath(&self, filepath: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<IndexedResourceCandidate>;
    fn IndexFileContentsAsync(&self, file: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<IndexedResourceCandidate>>>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IResourceIndexer {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Management.IResourceIndexer";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IResourceIndexerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceIndexerImpl, const OFFSET: isize>() -> IResourceIndexerVtbl {
        unsafe extern "system" fn IndexFilePath<Impl: IResourceIndexerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IndexFilePath(&*(&filepath as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IndexFileContentsAsync<Impl: IResourceIndexerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IndexFileContentsAsync(&*(&file as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IResourceIndexer>, ::windows::core::GetTrustLevel, IndexFilePath::<Impl, OFFSET>, IndexFileContentsAsync::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IResourceIndexerFactoryImpl: Sized {
    fn CreateResourceIndexer(&self, projectroot: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<ResourceIndexer>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IResourceIndexerFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Management.IResourceIndexerFactory";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IResourceIndexerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceIndexerFactoryImpl, const OFFSET: isize>() -> IResourceIndexerFactoryVtbl {
        unsafe extern "system" fn CreateResourceIndexer<Impl: IResourceIndexerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, projectroot: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateResourceIndexer(&*(&projectroot as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IResourceIndexerFactory>, ::windows::core::GetTrustLevel, CreateResourceIndexer::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IResourceIndexerFactory2Impl: Sized {
    fn CreateResourceIndexerWithExtension(&self, projectroot: &::core::option::Option<super::super::super::Foundation::Uri>, extensiondllpath: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<ResourceIndexer>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IResourceIndexerFactory2 {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Management.IResourceIndexerFactory2";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IResourceIndexerFactory2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceIndexerFactory2Impl, const OFFSET: isize>() -> IResourceIndexerFactory2Vtbl {
        unsafe extern "system" fn CreateResourceIndexerWithExtension<Impl: IResourceIndexerFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, projectroot: ::windows::core::RawPtr, extensiondllpath: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateResourceIndexerWithExtension(&*(&projectroot as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&extensiondllpath as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IResourceIndexerFactory2>, ::windows::core::GetTrustLevel, CreateResourceIndexerWithExtension::<Impl, OFFSET>)
    }
}
