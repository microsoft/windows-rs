#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IIndexedResourceCandidate_Impl: Sized {
    fn Type(&mut self) -> ::windows::core::Result<IndexedResourceType>;
    fn Uri(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn Metadata(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn Qualifiers(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<IndexedResourceQualifier>>;
    fn ValueAsString(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetQualifierValue(&mut self, qualifiername: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IIndexedResourceCandidate {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Management.IIndexedResourceCandidate";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IIndexedResourceCandidate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIndexedResourceCandidate_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IIndexedResourceCandidate_Vtbl {
        unsafe extern "system" fn Type<Impl: IIndexedResourceCandidate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut IndexedResourceType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Uri<Impl: IIndexedResourceCandidate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Metadata<Impl: IIndexedResourceCandidate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Qualifiers<Impl: IIndexedResourceCandidate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ValueAsString<Impl: IIndexedResourceCandidate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetQualifierValue<Impl: IIndexedResourceCandidate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, qualifiername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IIndexedResourceCandidate, BASE_OFFSET>(),
            Type: Type::<Impl, IMPL_OFFSET>,
            Uri: Uri::<Impl, IMPL_OFFSET>,
            Metadata: Metadata::<Impl, IMPL_OFFSET>,
            Qualifiers: Qualifiers::<Impl, IMPL_OFFSET>,
            ValueAsString: ValueAsString::<Impl, IMPL_OFFSET>,
            GetQualifierValue: GetQualifierValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIndexedResourceCandidate as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IIndexedResourceQualifier_Impl: Sized {
    fn QualifierName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn QualifierValue(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IIndexedResourceQualifier {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Management.IIndexedResourceQualifier";
}
#[cfg(feature = "implement_exclusive")]
impl IIndexedResourceQualifier_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIndexedResourceQualifier_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IIndexedResourceQualifier_Vtbl {
        unsafe extern "system" fn QualifierName<Impl: IIndexedResourceQualifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn QualifierValue<Impl: IIndexedResourceQualifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IIndexedResourceQualifier, BASE_OFFSET>(),
            QualifierName: QualifierName::<Impl, IMPL_OFFSET>,
            QualifierValue: QualifierValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIndexedResourceQualifier as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IResourceIndexer_Impl: Sized {
    fn IndexFilePath(&mut self, filepath: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<IndexedResourceCandidate>;
    fn IndexFileContentsAsync(&mut self, file: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<IndexedResourceCandidate>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IResourceIndexer {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Management.IResourceIndexer";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl IResourceIndexer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceIndexer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceIndexer_Vtbl {
        unsafe extern "system" fn IndexFilePath<Impl: IResourceIndexer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filepath: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IndexFileContentsAsync<Impl: IResourceIndexer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IResourceIndexer, BASE_OFFSET>(),
            IndexFilePath: IndexFilePath::<Impl, IMPL_OFFSET>,
            IndexFileContentsAsync: IndexFileContentsAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResourceIndexer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IResourceIndexerFactory_Impl: Sized {
    fn CreateResourceIndexer(&mut self, projectroot: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<ResourceIndexer>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IResourceIndexerFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Management.IResourceIndexerFactory";
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl IResourceIndexerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceIndexerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceIndexerFactory_Vtbl {
        unsafe extern "system" fn CreateResourceIndexer<Impl: IResourceIndexerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, projectroot: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IResourceIndexerFactory, BASE_OFFSET>(),
            CreateResourceIndexer: CreateResourceIndexer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResourceIndexerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IResourceIndexerFactory2_Impl: Sized {
    fn CreateResourceIndexerWithExtension(&mut self, projectroot: &::core::option::Option<super::super::super::Foundation::Uri>, extensiondllpath: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<ResourceIndexer>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IResourceIndexerFactory2 {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Management.IResourceIndexerFactory2";
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl IResourceIndexerFactory2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceIndexerFactory2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceIndexerFactory2_Vtbl {
        unsafe extern "system" fn CreateResourceIndexerWithExtension<Impl: IResourceIndexerFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, projectroot: ::windows::core::RawPtr, extensiondllpath: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IResourceIndexerFactory2, BASE_OFFSET>(),
            CreateResourceIndexerWithExtension: CreateResourceIndexerWithExtension::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResourceIndexerFactory2 as ::windows::core::Interface>::IID
    }
}
